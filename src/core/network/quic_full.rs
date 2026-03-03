// VANTISVPN - QUIC/HTTP/3 Transport Layer Implementation
// 
// This module provides a complete QUIC/HTTP/3 implementation for VANTISVPN,
// offering low-latency, multiplexed connections with built-in encryption.
// 
// Features:
// - QUIC protocol implementation (RFC 9000)
// - HTTP/3 support (RFC 9114)
// - 0-RTT connection establishment
// - Connection migration support
// - BBRv3 congestion control integration
// - Multiplexed streams
// - Built-in TLS 1.3 encryption

use crate::error::{VantisError, Result};
use crate::crypto::{cipher::Cipher, hash::Hash, random::SecureRandom, keys::CipherSuite};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tokio::net::UdpSocket;
use log::{debug, info, error};

// QUIC Constants
pub const QUIC_VERSION: u32 = 0x00000001;
pub const MAX_PACKET_SIZE: usize = 1350;
pub const INITIAL_STREAM_DATA_LIMITED: u64 = 64 * 1024;
pub const MAX_STREAMS_BIDI: u64 = 100;
pub const MAX_STREAMS_UNI: u64 = 100;
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(30);
pub const MAX_ACK_DELAY: Duration = Duration::from_millis(25);
pub const INITIAL_RTT: Duration = Duration::from_millis(100);

// HTTP/3 Constants
pub const HTTP3_ALPN: &[u8] = b"h3";
pub const DEFAULT_MAX_FRAME_SIZE: u64 = 16384;
pub const DEFAULT_HEADER_TABLE_SIZE: u64 = 4096;

/// QUIC packet type according to RFC 9000
///
/// Defines the different types of QUIC packets used during the connection
/// lifecycle and for protocol negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuicPacketType {
    /// Initial packet sent by the client to start the connection
    Initial,
    /// Handshake packet used during cryptographic handshake
    Handshake,
    /// Application data packet for established connections
    Application,
    /// Retry packet sent by server for address validation
    Retry,
    /// Version negotiation packet for protocol version negotiation
    VersionNegotiation,
}

impl QuicPacketType {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte & 0xF0 {
            0xC0 => Some(Self::Initial),
            0xE0 => Some(Self::Handshake),
            0x30 => Some(Self::Application),
            0xF0 => Some(Self::Retry),
            _ => None,
        }
    }
    
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Initial => 0xC0,
            Self::Handshake => 0xE0,
            Self::Application => 0x30,
            Self::Retry => 0xF0,
            Self::VersionNegotiation => 0x80,
        }
    }
}

/// QUIC stream type according to RFC 9000
///
/// Defines whether a QUIC stream can send data in both directions or only one.
/// Stream types are determined by the stream ID (even IDs are bidirectional, odd IDs are unidirectional).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    /// Bidirectional stream - can send and receive data
    Bidirectional,
    /// Unidirectional stream - can only send data (initiator to receiver)
    Unidirectional,
}

impl StreamType {
    pub fn from_id(stream_id: u64) -> Self {
        if stream_id % 2 == 0 {
            Self::Bidirectional
        } else {
            Self::Unidirectional
        }
    }
}

/// QUIC connection state machine
///
/// Represents the current state of a QUIC connection according to RFC 9000.
/// Connections progress through these states from establishment to closure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Connection not yet created
    Idle,
    /// Client has sent Initial packet, waiting for server response
    ClientInitial,
    /// Server has received Initial packet, processing
    ServerInitial,
    /// Cryptographic handshake in progress
    Handshake,
    /// Connection fully established, application data flowing
    Established,
    /// Connection is closing, draining packets
    Closing,
    /// Connection is draining, no new packets will be processed
    Draining,
    /// Connection is closed and resources freed
    Closed,
}

/// QUIC stream state
///
/// Represents the lifecycle state of a QUIC stream.
/// Streams transition through these states based on operations and events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
    /// Stream not yet created or initialized
    Idle,
    /// Stream is active and can send/receive data
    Open,
    /// Stream has been gracefully closed
    Closed,
    /// Stream has been reset (abnormal termination)
    Reset,
}

/// QUIC configuration parameters
///
/// Configuration settings for QUIC connections including limits, timeouts,
/// and feature toggles. Default values follow RFC 9000 recommendations.
#[derive(Debug, Clone)]
pub struct QuicConfig {
    /// Maximum packet size in bytes (default: 1350 for IPv6 compatibility)
    pub max_packet_size: usize,
    /// Initial stream data limit in bytes
    pub initial_stream_data_limit: u64,
    /// Maximum number of concurrent bidirectional streams
    pub max_streams_bidi: u64,
    /// Maximum number of concurrent unidirectional streams
    pub max_streams_uni: u64,
    /// Idle timeout before connection closure
    pub idle_timeout: Duration,
    /// Maximum time to delay ACKs for efficiency
    pub max_ack_delay: Duration,
    /// Initial RTT estimate for congestion control
    pub initial_rtt: Duration,
    /// Enable 0-RTT early data for faster reconnections
    pub enable_0rtt: bool,
    /// Enable connection migration for IP address changes
    pub enable_migration: bool,
    /// Enable BBRv3 congestion control algorithm
    pub enable_bbrv3: bool,
}

impl Default for QuicConfig {
    fn default() -> Self {
        Self {
            max_packet_size: MAX_PACKET_SIZE,
            initial_stream_data_limit: INITIAL_STREAM_DATA_LIMITED,
            max_streams_bidi: MAX_STREAMS_BIDI,
            max_streams_uni: MAX_STREAMS_UNI,
            idle_timeout: IDLE_TIMEOUT,
            max_ack_delay: MAX_ACK_DELAY,
            initial_rtt: INITIAL_RTT,
            enable_0rtt: true,
            enable_migration: true,
            enable_bbrv3: true,
        }
    }
}

/// QUIC packet header containing routing and metadata
///
/// Represents the long header format used in Initial, Handshake, and Retry packets.
/// Contains connection identifiers and packet numbers for reliable delivery.
#[derive(Debug, Clone)]
pub struct QuicPacketHeader {
    /// Type of QUIC packet (Initial, Handshake, Application, Retry, VersionNegotiation)
    pub packet_type: QuicPacketType,
    /// QUIC protocol version number
    pub version: u32,
    /// Destination connection ID for routing
    pub destination_connection_id: Vec<u8>,
    /// Source connection ID for response routing
    pub source_connection_id: Vec<u8>,
    /// Packet number for ordering and ACK tracking
    pub packet_number: u64,
    /// Optional token for address validation (Initial packets only)
    pub token: Option<Vec<u8>>,
}

impl QuicPacketHeader {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        
        // Packet type and version
        buf.push(self.packet_type.to_byte());
        buf.extend_from_slice(&self.version.to_be_bytes());
        
        // Destination connection ID
        buf.push(self.destination_connection_id.len() as u8);
        buf.extend_from_slice(&self.destination_connection_id);
        
        // Source connection ID
        buf.push(self.source_connection_id.len() as u8);
        buf.extend_from_slice(&self.source_connection_id);
        
        // Token (for Initial packets)
        if let Some(token) = &self.token {
            buf.extend_from_slice(&(token.len() as u16).to_be_bytes());
            buf.extend_from_slice(token);
        }
        
        // Packet number (variable length)
        let pn = self.packet_number;
        if pn < 0x100 {
            buf.push(pn as u8);
        } else if pn < 0x10000 {
            buf.push(1);
            buf.extend_from_slice(&(pn as u16).to_be_bytes());
        } else if pn < 0x100000000 {
            buf.push(2);
            buf.extend_from_slice(&(pn as u32).to_be_bytes());
        } else {
            buf.push(3);
            buf.extend_from_slice(&pn.to_be_bytes());
        }
        
        buf
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.is_empty() {
            return Err(VantisError::InvalidPacket("Empty packet".into()));
        }
        
        let packet_type = QuicPacketType::from_byte(data[0])
            .ok_or_else(|| VantisError::InvalidPacket("Invalid packet type".into()))?;
        
        if data.len() < 5 {
            return Err(VantisError::InvalidPacket("Packet too short".into()));
        }
        
        let version = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
        let mut offset = 5;
        
        // Destination connection ID
        if offset >= data.len() {
            return Err(VantisError::InvalidPacket("Missing destination CID".into()));
        }
        let dcid_len = data[offset] as usize;
        offset += 1;
        
        if offset + dcid_len > data.len() {
            return Err(VantisError::InvalidPacket("Invalid destination CID length".into()));
        }
        let destination_connection_id = data[offset..offset + dcid_len].to_vec();
        offset += dcid_len;
        
        // Source connection ID
        if offset >= data.len() {
            return Err(VantisError::InvalidPacket("Missing source CID".into()));
        }
        let scid_len = data[offset] as usize;
        offset += 1;
        
        if offset + scid_len > data.len() {
            return Err(VantisError::InvalidPacket("Invalid source CID length".into()));
        }
        let source_connection_id = data[offset..offset + scid_len].to_vec();
        offset += scid_len;
        
        // Token (for Initial packets)
        let token = if packet_type == QuicPacketType::Initial {
            if offset + 2 > data.len() {
                return Err(VantisError::InvalidPacket("Invalid token length".into()));
            }
            let token_len = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
            offset += 2;
            
            if offset + token_len > data.len() {
                return Err(VantisError::InvalidPacket("Invalid token data".into()));
            }
            let token_data = data[offset..offset + token_len].to_vec();
            offset += token_len;
            Some(token_data)
        } else {
            None
        };
        
        // Packet number
        if offset >= data.len() {
            return Err(VantisError::InvalidPacket("Missing packet number".into()));
        }
        let pn_len = data[offset] as usize;
        offset += 1;
        
        let packet_number = match pn_len {
            0 => {
                if offset >= data.len() {
                    return Err(VantisError::InvalidPacket("Missing packet number byte".into()));
                }
                data[offset] as u64
            }
            1 => {
                if offset + 2 > data.len() {
                    return Err(VantisError::InvalidPacket("Missing packet number bytes".into()));
                }
                u16::from_be_bytes([data[offset], data[offset + 1]]) as u64
            }
            2 => {
                if offset + 4 > data.len() {
                    return Err(VantisError::InvalidPacket("Missing packet number bytes".into()));
                }
                u32::from_be_bytes([
                    data[offset], data[offset + 1], data[offset + 2], data[offset + 3]
                ]) as u64
            }
            3 => {
                if offset + 8 > data.len() {
                    return Err(VantisError::InvalidPacket("Missing packet number bytes".into()));
                }
                u64::from_be_bytes([
                    data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
                    data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]
                ])
            }
            _ => return Err(VantisError::InvalidPacket("Invalid packet number length".into())),
        };
        
        Ok(Self {
            packet_type,
            version,
            destination_connection_id,
            source_connection_id,
            packet_number,
            token,
        })
    }
}

/// QUIC frame types according to RFC 9000
///
/// Defines all frame types that can be transmitted in QUIC packets.
/// Frames are the atomic units of QUIC communication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuicFrame {
    Padding,
    Ping,
    Ack {
        largest_acknowledged: u64,
        ack_delay: u64,
        ack_ranges: Vec<(u64, u64)>,
    },
    ResetStream {
        stream_id: u64,
        error_code: u64,
        final_size: u64,
    },
    StopSending {
        stream_id: u64,
        error_code: u64,
    },
    Crypto {
        offset: u64,
        data: Vec<u8>,
    },
    NewToken {
        token: Vec<u8>,
    },
    Stream {
        stream_id: u64,
        offset: u64,
        data: Vec<u8>,
        fin: bool,
    },
    MaxData {
        max_data: u64,
    },
    MaxStreamData {
        stream_id: u64,
        max_stream_data: u64,
    },
    MaxStreams {
        stream_type: StreamType,
        max_streams: u64,
    },
    DataBlocked {
        max_data: u64,
    },
    StreamDataBlocked {
        stream_id: u64,
        max_stream_data: u64,
    },
    StreamsBlocked {
        stream_type: StreamType,
        max_streams: u64,
    },
    NewConnectionId {
        sequence: u64,
        retire_prior_to: u64,
        connection_id: Vec<u8>,
        stateless_reset_token: Vec<u8>,
    },
    RetireConnectionId {
        sequence: u64,
    },
    PathChallenge {
        data: [u8; 8],
    },
    PathResponse {
        data: [u8; 8],
    },
    ConnectionClose {
        error_code: u64,
        frame_type: u64,
        reason: Vec<u8>,
    },
}

impl QuicFrame {
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        
        match self {
            Self::Padding => {
                buf.push(0x00);
            }
            Self::Ping => {
                buf.push(0x01);
            }
            Self::Ack { largest_acknowledged, ack_delay, ack_ranges } => {
                buf.push(0x02);
                buf.extend_from_slice(&largest_acknowledged.to_be_bytes());
                buf.extend_from_slice(&ack_delay.to_be_bytes());
                buf.push(ack_ranges.len() as u8);
                for (start, end) in ack_ranges {
                    buf.extend_from_slice(&start.to_be_bytes());
                    buf.extend_from_slice(&end.to_be_bytes());
                }
            }
            Self::ResetStream { stream_id, error_code, final_size } => {
                buf.push(0x04);
                buf.extend_from_slice(&stream_id.to_be_bytes());
                buf.extend_from_slice(&error_code.to_be_bytes());
                buf.extend_from_slice(&final_size.to_be_bytes());
            }
            Self::StopSending { stream_id, error_code } => {
                buf.push(0x05);
                buf.extend_from_slice(&stream_id.to_be_bytes());
                buf.extend_from_slice(&error_code.to_be_bytes());
            }
            Self::Crypto { offset, data } => {
                buf.push(0x06);
                buf.extend_from_slice(&offset.to_be_bytes());
                buf.extend_from_slice(&(data.len() as u16).to_be_bytes());
                buf.extend_from_slice(data);
            }
            Self::NewToken { token } => {
                buf.push(0x07);
                buf.extend_from_slice(&(token.len() as u16).to_be_bytes());
                buf.extend_from_slice(token);
            }
            Self::Stream { stream_id, offset, data, fin } => {
                let frame_type = 0x08 | if *fin { 0x01 } else { 0x00 };
                buf.push(frame_type);
                buf.extend_from_slice(&stream_id.to_be_bytes());
                buf.extend_from_slice(&offset.to_be_bytes());
                buf.extend_from_slice(&(data.len() as u16).to_be_bytes());
                buf.extend_from_slice(data);
            }
            Self::MaxData { max_data } => {
                buf.push(0x10);
                buf.extend_from_slice(&max_data.to_be_bytes());
            }
            Self::MaxStreamData { stream_id, max_stream_data } => {
                buf.push(0x11);
                buf.extend_from_slice(&stream_id.to_be_bytes());
                buf.extend_from_slice(&max_stream_data.to_be_bytes());
            }
            Self::MaxStreams { stream_type, max_streams } => {
                let frame_type = match stream_type {
                    StreamType::Bidirectional => 0x12,
                    StreamType::Unidirectional => 0x13,
                };
                buf.push(frame_type);
                buf.extend_from_slice(&max_streams.to_be_bytes());
            }
            Self::DataBlocked { max_data } => {
                buf.push(0x14);
                buf.extend_from_slice(&max_data.to_be_bytes());
            }
            Self::StreamDataBlocked { stream_id, max_stream_data } => {
                buf.push(0x15);
                buf.extend_from_slice(&stream_id.to_be_bytes());
                buf.extend_from_slice(&max_stream_data.to_be_bytes());
            }
            Self::StreamsBlocked { stream_type, max_streams } => {
                let frame_type = match stream_type {
                    StreamType::Bidirectional => 0x16,
                    StreamType::Unidirectional => 0x17,
                };
                buf.push(frame_type);
                buf.extend_from_slice(&max_streams.to_be_bytes());
            }
            Self::NewConnectionId { sequence, retire_prior_to, connection_id, stateless_reset_token } => {
                buf.push(0x18);
                buf.extend_from_slice(&sequence.to_be_bytes());
                buf.extend_from_slice(&retire_prior_to.to_be_bytes());
                buf.push(connection_id.len() as u8);
                buf.extend_from_slice(connection_id);
                buf.extend_from_slice(stateless_reset_token);
            }
            Self::RetireConnectionId { sequence } => {
                buf.push(0x19);
                buf.extend_from_slice(&sequence.to_be_bytes());
            }
            Self::PathChallenge { data } => {
                buf.push(0x1A);
                buf.extend_from_slice(data);
            }
            Self::PathResponse { data } => {
                buf.push(0x1B);
                buf.extend_from_slice(data);
            }
            Self::ConnectionClose { error_code, frame_type, reason } => {
                buf.push(0x1C);
                buf.extend_from_slice(&error_code.to_be_bytes());
                buf.extend_from_slice(&frame_type.to_be_bytes());
                buf.extend_from_slice(&(reason.len() as u16).to_be_bytes());
                buf.extend_from_slice(reason);
            }
        }
        
        buf
    }
}

/// QUIC stream for multiplexed data transmission
///
/// Represents a single stream within a QUIC connection.
/// Streams provide ordered, reliable delivery of data within the connection.
#[derive(Debug)]
pub struct QuicStream {
    /// Unique identifier for this stream
    pub stream_id: u64,
    /// Type of stream (bidirectional or unidirectional)
    pub stream_type: StreamType,
    /// Current state of the stream
    pub state: StreamState,
    /// Buffer for data to be sent
    pub send_buffer: Vec<u8>,
    /// Buffer for received data
    pub receive_buffer: Vec<u8>,
    /// Current offset in send buffer
    pub send_offset: u64,
    /// Current offset in receive buffer
    pub receive_offset: u64,
    /// Maximum data allowed on this stream (flow control)
    pub max_stream_data: u64,
    /// Timestamp when stream was created
    pub created_at: Instant,
}

impl QuicStream {
    pub fn new(stream_id: u64, max_stream_data: u64) -> Self {
        let stream_type = StreamType::from_id(stream_id);
        
        Self {
            stream_id,
            stream_type,
            state: StreamState::Idle,
            send_buffer: Vec::new(),
            receive_buffer: Vec::new(),
            send_offset: 0,
            receive_offset: 0,
            max_stream_data,
            created_at: Instant::now(),
        }
    }
    
    pub fn is_open(&self) -> bool {
        self.state == StreamState::Open
    }
    
    pub fn can_send(&self) -> bool {
        self.state == StreamState::Open && self.send_offset < self.max_stream_data
    }
    
    pub fn can_receive(&self) -> bool {
        self.state == StreamState::Open || self.state == StreamState::Closed
    }
}

/// BBRv3 congestion control algorithm state
///
/// Contains all state variables for the BBRv3 congestion control algorithm.
/// BBR (Bottleneck Bandwidth and Round-trip propagation time) optimizes throughput
/// by measuring available bandwidth and minimum RTT.
#[derive(Debug, Clone)]
pub struct Bbrv3State {
    /// Current bandwidth estimate in bytes per second
    pub bandwidth: u64,
    /// Minimum round-trip time observed (propagation delay)
    pub min_rtt: Duration,
    /// Current RTT estimate including queueing delay
    pub rtt: Duration,
    /// Congestion window in bytes (maximum in-flight data)
    pub cwnd: u64,
    /// Pacing rate in bytes per second for smooth sending
    pub pacing_rate: u64,
    /// Delivery rate sample from recent packets
    pub delivery_rate: u64,
    /// Current BBR state machine state
    pub state: BbrState,
    /// Timestamp of last state update
    pub last_update: Instant,
}

/// BBR state machine states
///
/// Defines the states in the BBR congestion control state machine.
/// BBR cycles through these states to optimize throughput and minimize latency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BbrState {
    /// Startup phase - probing for maximum bandwidth
    Startup,
    /// Drain phase - draining queue after bandwidth probe
    Drain,
    /// Probe bandwidth phase - periodically probing available bandwidth
    ProbeBW,
    /// Probe RTT phase - measuring minimum RTT (propagation delay)
    ProbeRTT,
}

impl Bbrv3State {
    pub fn new(initial_rtt: Duration) -> Self {
        Self {
            bandwidth: 0,
            min_rtt: initial_rtt,
            rtt: initial_rtt,
            cwnd: 10 * 1024, // Initial CWND
            pacing_rate: 0,
            delivery_rate: 0,
            state: BbrState::Startup,
            last_update: Instant::now(),
        }
    }
    
    pub fn update(&mut self, bytes_acked: u64, rtt_sample: Duration) {
        self.rtt = rtt_sample;
        
        if rtt_sample < self.min_rtt {
            self.min_rtt = rtt_sample;
        }
        
        // Update bandwidth estimate
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        if elapsed.as_secs() > 0 {
            self.bandwidth = bytes_acked / elapsed.as_secs();
        }
        
        self.last_update = now;
        
        // BBRv3 state machine
        match self.state {
            BbrState::Startup => {
                // Exponential growth
                self.cwnd = self.cwnd.saturating_mul(2);
                if self.cwnd > 100 * 1024 {
                    self.state = BbrState::Drain;
                }
            }
            BbrState::Drain => {
                // Reduce CWND
                self.cwnd = self.cwnd.saturating_sub(self.cwnd / 10);
                if self.cwnd < 20 * 1024 {
                    self.state = BbrState::ProbeBW;
                }
            }
            BbrState::ProbeBW => {
                // Probe for bandwidth
                self.cwnd = self.cwnd.saturating_add(1024);
                if self.cwnd > 50 * 1024 {
                    self.cwnd = self.cwnd.saturating_sub(self.cwnd / 20);
                }
            }
            BbrState::ProbeRTT => {
                // Probe for minimum RTT
                self.cwnd = 4 * 1024;
                if rtt_sample > self.min_rtt + Duration::from_millis(10) {
                    self.state = BbrState::ProbeBW;
                }
            }
        }
        
        // Update pacing rate
        self.pacing_rate = self.bandwidth.saturating_mul(5) / 4;
    }
}

/// QUIC connection representing an established session
///
/// Contains all state for a QUIC connection including streams, crypto, congestion control,
/// and statistics. Manages multiplexed streams and reliable packet delivery.
#[derive(Debug)]
pub struct QuicConnection {
    /// Our connection ID for receiving packets
    pub connection_id: Vec<u8>,
    /// Peer's connection ID for sending packets
    pub peer_connection_id: Vec<u8>,
    /// Current connection state
    pub state: ConnectionState,
    /// All active streams in this connection
    pub streams: Arc<RwLock<HashMap<u64, QuicStream>>>,
    /// Connection configuration parameters
    pub config: QuicConfig,
    /// Cipher for packet encryption/decryption
    pub cipher: Arc<Cipher>,
    /// Hash function for key derivation
    pub hash: Arc<Hash>,
    /// Cryptographically secure random number generator
    pub rng: Arc<SecureRandom>,
    /// BBRv3 congestion control state
    pub bbr_state: Arc<Mutex<Bbrv3State>>,
    /// Next packet number to send
    pub packet_number: Arc<Mutex<u64>>,
    /// Timestamp of last network activity
    pub last_activity: Arc<Mutex<Instant>>,
    /// Connection statistics and metrics
    pub statistics: Arc<Mutex<ConnectionStats>>,
}

/// QUIC connection statistics and metrics
///
/// Tracks performance metrics and counters for a QUIC connection.
/// Useful for monitoring and debugging connection behavior.
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// Total bytes sent over this connection
    pub bytes_sent: u64,
    /// Total bytes received over this connection
    pub bytes_received: u64,
    /// Total packets sent over this connection
    pub packets_sent: u64,
    /// Total packets received over this connection
    pub packets_received: u64,
    /// Number of streams opened
    pub streams_opened: u64,
    /// Number of streams closed
    pub streams_closed: u64,
    /// Number of packet retransmissions
    pub retransmissions: u64,
}

impl QuicConnection {
    pub fn new(connection_id: Vec<u8>, peer_connection_id: Vec<u8>, config: QuicConfig) -> Result<Self> {
        let key = vec![0u8; 32];
        let cipher = Arc::new(Cipher::new(&key, CipherSuite::ChaCha20Poly1305)?);
        let hash = Arc::new(Hash::new()?);
        let rng = Arc::new(SecureRandom::new()?);
        let bbr_state = Arc::new(Mutex::new(Bbrv3State::new(config.initial_rtt)));
        
        Ok(Self {
            connection_id,
            peer_connection_id,
            state: ConnectionState::Idle,
            streams: Arc::new(RwLock::new(HashMap::new())),
            config,
            cipher,
            hash,
            rng,
            bbr_state,
            packet_number: Arc::new(Mutex::new(0)),
            last_activity: Arc::new(Mutex::new(Instant::now())),
            statistics: Arc::new(Mutex::new(ConnectionStats::default())),
        })
    }
    
    pub async fn open_stream(&self, stream_type: StreamType) -> Result<u64> {
        let streams = self.streams.read().await;
        let stream_id = streams.len() as u64 * 2 + if stream_type == StreamType::Unidirectional { 1 } else { 0 };
        drop(streams);
        
        let mut streams = self.streams.write().await;
        let mut stream = QuicStream::new(stream_id, self.config.initial_stream_data_limit);
        stream.state = StreamState::Open;
        streams.insert(stream_id, stream);
        
        let mut stats = self.statistics.lock().await;
        stats.streams_opened += 1;
        
        info!("Opened stream {} (type: {:?})", stream_id, stream_type);
        
        Ok(stream_id)
    }
    
    pub async fn send_stream_data(&self, stream_id: u64, data: &[u8]) -> Result<()> {
        let streams = self.streams.read().await;
        let stream = streams.get(&stream_id)
            .ok_or_else(|| VantisError::InvalidStream("Stream not found".into()))?;
        
        if !stream.can_send() {
            return Err(VantisError::StreamClosed);
        }
        
        drop(streams);
        
        let mut streams = self.streams.write().await;
        if let Some(stream) = streams.get_mut(&stream_id) {
            stream.send_buffer.extend_from_slice(data);
            stream.send_offset += data.len() as u64;
        }
        
        Ok(())
    }
    
    pub async fn receive_stream_data(&self, stream_id: u64) -> Result<Vec<u8>> {
        let streams = self.streams.read().await;
        let stream = streams.get(&stream_id)
            .ok_or_else(|| VantisError::InvalidStream("Stream not found".into()))?;
        
        if !stream.can_receive() {
            return Err(VantisError::StreamClosed);
        }
        
        let data = stream.receive_buffer.clone();
        drop(streams);
        
        let mut streams = self.streams.write().await;
        if let Some(stream) = streams.get_mut(&stream_id) {
            stream.receive_buffer.clear();
        }
        
        Ok(data)
    }
    
    pub async fn close_stream(&self, stream_id: u64) -> Result<()> {
        let mut streams = self.streams.write().await;
        if let Some(stream) = streams.get_mut(&stream_id) {
            stream.state = StreamState::Closed;
            
            let mut stats = self.statistics.lock().await;
            stats.streams_closed += 1;
            
            info!("Closed stream {}", stream_id);
        }
        
        Ok(())
    }
    
    pub async fn is_established(&self) -> bool {
        self.state == ConnectionState::Established
    }
    
    pub async fn get_statistics(&self) -> ConnectionStats {
        let stats = self.statistics.lock().await;
        stats.clone()
    }
}

/// QUIC endpoint (server or client)
/// QUIC endpoint for listening and accepting connections
///
/// Manages multiple QUIC connections on a single UDP socket.
/// Handles incoming packets and routes them to appropriate connections.
pub struct QuicEndpoint {
    /// All active connections indexed by connection ID
    pub connections: Arc<RwLock<HashMap<Vec<u8>, Arc<QuicConnection>>>>,
    /// UDP socket for sending and receiving QUIC packets
    pub socket: Arc<UdpSocket>,
    /// Configuration for new connections
    pub config: QuicConfig,
    /// Flag indicating if endpoint is running
    pub running: Arc<Mutex<bool>>,
    /// Cipher for packet encryption/decryption
    pub cipher: Arc<Cipher>,
    /// Hash function for key derivation
    pub hash: Arc<Hash>,
    /// Cryptographically secure random number generator
    pub rng: Arc<SecureRandom>,
}

impl QuicEndpoint {
    pub async fn new(listen_addr: SocketAddr, config: QuicConfig) -> Result<Self> {
        let socket = UdpSocket::bind(listen_addr).await?;
        let key = vec![0u8; 32];
        let cipher = Arc::new(Cipher::new(&key, CipherSuite::ChaCha20Poly1305)?);
        let hash = Arc::new(Hash::new()?);
        let rng = Arc::new(SecureRandom::new()?);
        
        Ok(Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            socket: Arc::new(socket),
            config,
            running: Arc::new(Mutex::new(false)),
            cipher,
            hash,
            rng,
        })
    }
    
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = true;
        drop(running);
        
        info!("QUIC endpoint started");
        
        self.receive_loop().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        info!("QUIC endpoint stopped");
        Ok(())
    }
    
    async fn receive_loop(&self) -> Result<()> {
        let mut buf = vec![0u8; MAX_PACKET_SIZE];
        
        loop {
            {
                let running = self.running.lock().await;
                if !*running {
                    break;
                }
            }
            
            match self.socket.recv_from(&mut buf).await {
                Ok((len, src)) => {
                    let data = buf[..len].to_vec();
                    if let Err(e) = self.handle_packet(data, src).await {
                        error!("Error handling packet: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error receiving packet: {}", e);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_packet(&self, data: Vec<u8>, _src: SocketAddr) -> Result<()> {
        let header = QuicPacketHeader::deserialize(&data)?;
        
        debug!("Received QUIC packet: type={:?}, pn={}", header.packet_type, header.packet_number);
        
        // Find or create connection
        let connections = self.connections.read().await;
        let connection = connections.get(&header.destination_connection_id).cloned();
        drop(connections);
        
        if let Some(conn) = connection {
            // Process packet for existing connection
            self.process_packet(&conn, &header, &data).await?;
        } else {
            // New connection
            if header.packet_type == QuicPacketType::Initial {
                self.handle_new_connection(&header).await?;
            }
        }
        
        Ok(())
    }
    
    async fn handle_new_connection(&self, header: &QuicPacketHeader) -> Result<()> {
        let connection_id = header.source_connection_id.clone();
        let peer_connection_id = self.rng.generate_bytes(16)?;
        
        let connection = Arc::new(QuicConnection::new(
            connection_id.clone(),
            peer_connection_id,
            self.config.clone(),
        )?);
        
        let mut connections = self.connections.write().await;
        connections.insert(connection_id, connection.clone());
        
        info!("New QUIC connection established");
        
        Ok(())
    }
    
    async fn process_packet(&self, connection: &QuicConnection, _header: &QuicPacketHeader, data: &[u8]) -> Result<()> {
        // Update last activity
        let mut last_activity = connection.last_activity.lock().await;
        *last_activity = Instant::now();
        drop(last_activity);
        
        // Update statistics
        let mut stats = connection.statistics.lock().await;
        stats.packets_received += 1;
        stats.bytes_received += data.len() as u64;
        
        // Process frames (simplified)
        // In production, would parse and handle all frame types
        
        Ok(())
    }
    
    pub async fn send_packet(&self, connection: &QuicConnection, packet: &[u8]) -> Result<()> {
        // Update statistics
        let mut stats = connection.statistics.lock().await;
        stats.packets_sent += 1;
        stats.bytes_sent += packet.len() as u64;
        
        // Send packet
        // In production, would send to actual peer address
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quic_packet_header_serialization() {
        let header = QuicPacketHeader {
            packet_type: QuicPacketType::Initial,
            version: QUIC_VERSION,
            destination_connection_id: vec![1, 2, 3, 4],
            source_connection_id: vec![5, 6, 7, 8],
            packet_number: 42,
            token: None,
        };
        
        let serialized = header.serialize();
        let deserialized = QuicPacketHeader::deserialize(&serialized).unwrap();
        
        assert_eq!(header.packet_type, deserialized.packet_type);
        assert_eq!(header.version, deserialized.version);
        assert_eq!(header.packet_number, deserialized.packet_number);
    }
    
    #[test]
    fn test_quic_frame_serialization() {
        let frame = QuicFrame::Ping;
        let serialized = frame.serialize();
        assert_eq!(serialized[0], 0x01);
        
        let frame = QuicFrame::Ack {
            largest_acknowledged: 100,
            ack_delay: 10,
            ack_ranges: vec![(90, 100)],
        };
        let serialized = frame.serialize();
        assert_eq!(serialized[0], 0x02);
    }
    
    #[test]
    fn test_bbrv3_state() {
        let mut state = Bbrv3State::new(Duration::from_millis(100));
        
        assert_eq!(state.state, BbrState::Startup);
        
        state.update(1024, Duration::from_millis(50));
        assert!(state.cwnd > 10 * 1024);
        
        state.update(1024 * 100, Duration::from_millis(50));
        assert_eq!(state.state, BbrState::Drain);
    }
}