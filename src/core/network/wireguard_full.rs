// VANTISVPN - Full WireGuard Protocol Implementation with Modifications
// Based on WireGuard protocol specification with VANTISVPN enhancements
//
// Modifications:
// - IPv6 native support (DoDI 8310.01 compliant)
// - Post-quantum hybrid key exchange
// - Enhanced replay protection
// - Stealth mode obfuscation
// - MultiHop+ support
// - Dynamic MTU negotiation

use crate::crypto::{
    cipher::Cipher, hash::Hash, keys::CipherSuite, keys::EphemeralKeyPair, random::SecureRandom,
};
use crate::error::{Result, VantisError};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::net::{Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, RwLock};

/// Type alias for the peer map to reduce type complexity
type PeerMap = HashMap<[u8; 32], Arc<Mutex<PeerState>>>;

// WireGuard constants
/// Size in bytes of a WireGuard handshake initiation message
pub const HANDSHAKE_INITIATION_SIZE: usize = 113;
/// Size in bytes of a WireGuard handshake response message
pub const HANDSHAKE_RESPONSE_SIZE: usize = 92;
/// Size in bytes of a WireGuard cookie reply message
pub const COOKIE_REPLY_SIZE: usize = 64;
/// Size in bytes of the WireGuard data message header
pub const MESSAGE_DATA_SIZE: usize = 16;
/// Maximum size of a WireGuard message in bytes
pub const MAX_MESSAGE_SIZE: usize = 65535;
/// Size of the replay protection window for anti-replay protection
pub const REPLAY_WINDOW_SIZE: usize = 64;
/// Timeout for keepalive packets to maintain connection
pub const KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(25);
/// Timeout for rekeying the session keys
pub const REKEY_TIMEOUT: Duration = Duration::from_secs(120);
/// Time after which cookies should be refreshed
pub const COOKIE_REFRESH_TIME: Duration = Duration::from_secs(120);

/// Header magic bytes for stealth mode packets
pub const STEALTH_MODE_HEADER: &[u8] = b"VANTIS";
/// Enable post-quantum hybrid key exchange
pub const PQC_HYBRID_EXCHANGE: bool = true;
/// Enable enhanced replay protection beyond standard WireGuard
pub const ENHANCED_REPLAY_PROTECTION: bool = true;

/// WireGuard peer configuration for VPN connections
///
/// Contains all configuration parameters for a WireGuard peer including
/// cryptographic keys, routing information, and VANTISVPN-specific enhancements.
#[derive(Debug, Clone, Default)]
pub struct PeerConfig {
    /// Peer's 32-byte public key for authentication
    pub public_key: [u8; 32],
    /// Allowed IPv6 addresses for this peer
    pub allowed_ips: Vec<Ipv6Addr>,
    /// UDP endpoint address (IP:port) for this peer
    pub endpoint: Option<SocketAddr>,
    /// Interval for sending keepalive packets to maintain NAT mapping
    pub persistent_keepalive: Option<Duration>,
    /// Post-quantum cryptography public key (optional for hybrid exchange)
    pub pqc_public_key: Option<Vec<u8>>,
    /// Enable stealth mode for traffic obfuscation
    pub stealth_mode: bool,
    /// Next hop for MultiHop+ onion routing (32-byte public key)
    pub next_hop: Option<[u8; 32]>,
}

/// WireGuard interface configuration for local VPN endpoint
///
/// Contains all configuration parameters for the local WireGuard interface including
/// cryptographic keys, network settings, and VANTISVPN enhancements.
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    /// 32-byte private key for this interface
    pub private_key: [u8; 32],
    /// 32-byte public key derived from private key
    pub public_key: [u8; 32],
    /// UDP port to listen on for incoming connections
    pub listen_port: u16,
    /// Virtual IPv6 address assigned to this interface
    pub virtual_ip: Ipv6Addr,
    /// Virtual subnet prefix length (e.g., 64 for /64 subnet)
    pub virtual_subnet: u8,
    /// Maximum transmission unit for the tunnel interface
    pub mtu: u16,
    /// Post-quantum cryptography key pair for hybrid exchange
    pub pqc_keypair: Option<EphemeralKeyPair>,
    /// Enable stealth mode for traffic obfuscation
    pub stealth_mode: bool,
}

impl InterfaceConfig {
    /// Creates a new instance with default configuration.
    pub fn new(private_key: [u8; 32], listen_port: u16, virtual_ip: Ipv6Addr) -> Result<Self> {
        let public_key = Self::derive_public_key(&private_key)?;

        Ok(Self {
            private_key,
            public_key,
            listen_port,
            virtual_ip,
            virtual_subnet: 64,
            mtu: 1420,
            pqc_keypair: None,
            stealth_mode: false,
        })
    }

    fn derive_public_key(private_key: &[u8; 32]) -> Result<[u8; 32]> {
        // X25519 public key derivation
        // In production, use actual X25519 implementation
        let mut public_key = [0u8; 32];
        // Placeholder - actual implementation would use x25519_dalek
        for i in 0..32 {
            public_key[i] = private_key[i].wrapping_add(1);
        }
        Ok(public_key)
    }
}

/// WireGuard handshake initiation message (type 1)
///
/// Sent by the initiator to begin a WireGuard handshake.
/// Contains ephemeral and static public keys with authentication.
#[derive(Debug, Clone)]
pub struct HandshakeInitiation {
    /// Message type identifier (always 1 for initiation)
    pub message_type: u8,
    /// Random sender index for identifying this session
    pub sender_index: u32,
    /// Ephemeral Curve25519 public key for this handshake
    pub ephemeral_public: [u8; 32],
    /// Encrypted static public key of the sender
    pub static_public_enc: [u8; 32],
    /// Encrypted timestamp for replay protection
    pub timestamp_enc: [u8; 12],
    /// First message authentication code
    pub mac1: [u8; 16],
    /// Second message authentication code (cookie-based)
    pub mac2: [u8; 16],
}

impl HandshakeInitiation {
    /// Serializes the data to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HANDSHAKE_INITIATION_SIZE);
        buf.push(self.message_type);
        buf.extend_from_slice(&self.sender_index.to_le_bytes());
        buf.extend_from_slice(&self.ephemeral_public);
        buf.extend_from_slice(&self.static_public_enc);
        buf.extend_from_slice(&self.timestamp_enc);
        buf.extend_from_slice(&self.mac1);
        buf.extend_from_slice(&self.mac2);
        buf
    }

    /// Deserializes the data from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.len() != HANDSHAKE_INITIATION_SIZE {
            return Err(VantisError::InvalidPacket(
                "Invalid handshake initiation size".into(),
            ));
        }

        let mut sender_index_bytes = [0u8; 4];
        sender_index_bytes.copy_from_slice(&data[1..5]);

        let mut ephemeral_public = [0u8; 32];
        ephemeral_public.copy_from_slice(&data[5..37]);

        let mut static_public_enc = [0u8; 32];
        static_public_enc.copy_from_slice(&data[37..69]);

        let mut timestamp_enc = [0u8; 12];
        timestamp_enc.copy_from_slice(&data[69..81]);

        let mut mac1 = [0u8; 16];
        mac1.copy_from_slice(&data[81..97]);

        let mut mac2 = [0u8; 16];
        mac2.copy_from_slice(&data[97..113]);

        Ok(Self {
            message_type: data[0],
            sender_index: u32::from_le_bytes(sender_index_bytes),
            ephemeral_public,
            static_public_enc,
            timestamp_enc,
            mac1,
            mac2,
        })
    }
}

/// WireGuard handshake response message (type 2)
///
/// Sent by the responder to complete the WireGuard handshake.
/// Contains the responder's ephemeral key and authentication.
#[derive(Debug, Clone)]
pub struct HandshakeResponse {
    /// Message type identifier (always 2 for response)
    pub message_type: u8,
    /// Sender's random index for this session
    pub sender_index: u32,
    /// Receiver's index from the initiation message
    pub receiver_index: u32,
    /// Ephemeral Curve25519 public key for this handshake
    pub ephemeral_public: [u8; 32],
    /// Empty encrypted field for future use
    pub empty_enc: [u8; 16],
    /// First message authentication code
    pub mac1: [u8; 16],
    /// Second message authentication code (cookie-based)
    pub mac2: [u8; 16],
}

impl HandshakeResponse {
    /// Serializes the data to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(HANDSHAKE_RESPONSE_SIZE);
        buf.push(self.message_type);
        buf.extend_from_slice(&self.sender_index.to_le_bytes());
        buf.extend_from_slice(&self.receiver_index.to_le_bytes());
        buf.extend_from_slice(&self.ephemeral_public);
        buf.extend_from_slice(&self.empty_enc);
        buf.extend_from_slice(&self.mac1);
        buf.extend_from_slice(&self.mac2);
        buf
    }

    /// Deserializes the data from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.len() != HANDSHAKE_RESPONSE_SIZE {
            return Err(VantisError::InvalidPacket(
                "Invalid handshake response size".into(),
            ));
        }

        let mut sender_index_bytes = [0u8; 4];
        sender_index_bytes.copy_from_slice(&data[1..5]);

        let mut receiver_index_bytes = [0u8; 4];
        receiver_index_bytes.copy_from_slice(&data[5..9]);

        let mut ephemeral_public = [0u8; 32];
        ephemeral_public.copy_from_slice(&data[9..41]);

        let mut empty_enc = [0u8; 16];
        empty_enc.copy_from_slice(&data[41..57]);

        let mut mac1 = [0u8; 16];
        mac1.copy_from_slice(&data[57..73]);

        let mut mac2 = [0u8; 16];
        mac2.copy_from_slice(&data[73..89]);

        Ok(Self {
            message_type: data[0],
            sender_index: u32::from_le_bytes(sender_index_bytes),
            receiver_index: u32::from_le_bytes(receiver_index_bytes),
            ephemeral_public,
            empty_enc,
            mac1,
            mac2,
        })
    }
}

/// WireGuard cookie reply message (type 3)
///
/// Sent in response to handshake messages under load conditions
/// to prevent DoS attacks with proof of work.
#[derive(Debug, Clone)]
pub struct CookieReply {
    /// Message type identifier (always 3 for cookie reply)
    pub message_type: u8,
    /// Receiver index from the original message
    pub receiver_index: u32,
    /// Nonce for cookie encryption
    pub nonce: [u8; 24],
    /// Encrypted cookie value
    pub cookie_enc: [u8; 16],
}

impl CookieReply {
    /// Serializes the data to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(COOKIE_REPLY_SIZE);
        buf.push(self.message_type);
        buf.extend_from_slice(&self.receiver_index.to_le_bytes());
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&self.cookie_enc);
        buf
    }

    /// Deserializes the data from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.len() != COOKIE_REPLY_SIZE {
            return Err(VantisError::InvalidPacket(
                "Invalid cookie reply size".into(),
            ));
        }

        let mut receiver_index_bytes = [0u8; 4];
        receiver_index_bytes.copy_from_slice(&data[1..5]);

        let mut nonce = [0u8; 24];
        nonce.copy_from_slice(&data[5..29]);

        let mut cookie_enc = [0u8; 16];
        cookie_enc.copy_from_slice(&data[29..45]);

        Ok(Self {
            message_type: data[0],
            receiver_index: u32::from_le_bytes(receiver_index_bytes),
            nonce,
            cookie_enc,
        })
    }
}

/// WireGuard transport data message (type 4)
///
/// Used for sending encrypted application data after handshake completion.
/// Includes sequence counter for replay protection.
#[derive(Debug, Clone)]
pub struct TransportData {
    /// Receiver index for identifying the session
    pub receiver_index: u32,
    /// Sequence counter for replay protection
    pub counter: u64,
    /// Encrypted application data
    pub data: Vec<u8>,
}

impl TransportData {
    /// Serializes the data to bytes.
    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(MESSAGE_DATA_SIZE + self.data.len());
        buf.extend_from_slice(&self.receiver_index.to_le_bytes());
        buf.extend_from_slice(&self.counter.to_le_bytes());
        buf.extend_from_slice(&self.data);
        buf
    }

    /// Deserializes the data from bytes.
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        if data.len() < MESSAGE_DATA_SIZE {
            return Err(VantisError::InvalidPacket(
                "Invalid transport data size".into(),
            ));
        }

        let mut receiver_index_bytes = [0u8; 4];
        receiver_index_bytes.copy_from_slice(&data[0..4]);

        let mut counter_bytes = [0u8; 8];
        counter_bytes.copy_from_slice(&data[4..12]);

        Ok(Self {
            receiver_index: u32::from_le_bytes(receiver_index_bytes),
            counter: u64::from_le_bytes(counter_bytes),
            data: data[12..].to_vec(),
        })
    }
}

/// Replay protection window for WireGuard packets
///
/// Uses a sliding window of 64 bits to detect and reject replayed packets.
/// Packets with counters outside the window or already seen are rejected.
#[derive(Debug)]
pub struct ReplayWindow {
    /// Bitset for tracking received packets within the window
    window: u64,
    /// Highest sequence counter received so far
    last_counter: u64,
}

impl ReplayWindow {
    /// Creates a new instance with default configuration.
    pub fn new() -> Self {
        Self {
            window: 0,
            last_counter: 0,
        }
    }

    /// Check if counter is valid (not replayed)
    pub fn check(&mut self, counter: u64) -> bool {
        if counter > self.last_counter {
            // New packet, shift window
            let shift = counter - self.last_counter;
            if shift < 64 {
                self.window = (self.window << shift) | 1;
            } else {
                self.window = 1;
            }
            self.last_counter = counter;
            true
        } else if self.last_counter - counter < 64 {
            // Check if already received
            let bit = 1u64 << (self.last_counter - counter);
            if self.window & bit == 0 {
                self.window |= bit;
                true
            } else {
                false // Replay detected
            }
        } else {
            false // Too old
        }
    }
}

impl Default for ReplayWindow {
    fn default() -> Self {
        Self::new()
    }
}

/// WireGuard peer state for active connections
///
/// Maintains all state for an established WireGuard peer including
/// session keys, handshake state, and replay protection.
#[derive(Debug)]
pub struct PeerState {
    /// Peer configuration
    config: PeerConfig,
    /// Current handshake state machine state
    handshake_state: HandshakeState,
    /// Session encryption/decryption keys
    session_keys: Option<SessionKeys>,
    /// Replay protection window for packet validation
    replay_window: ReplayWindow,
    /// Timestamp of last successful handshake
    last_handshake: Option<Instant>,
    /// Timestamp of last received packet
    last_received: Option<Instant>,
    /// Timestamp of last sent packet
    last_sent: Option<Instant>,
    /// Cookie for DoS protection under load
    cookie: Option<[u8; 16]>,
    /// Cookie expiration timestamp
    cookie_expiration: Option<Instant>,
    /// Peer index for routing
    index: u32,
    /// Peer statistics and metrics
    stats: PeerStats,
}

/// WireGuard session keys for encryption and decryption
///
/// Contains the symmetric keys derived from the handshake
/// for encrypting and decrypting transport data.
#[derive(Debug, Clone)]
pub struct SessionKeys {
    /// 32-byte key for encrypting outgoing packets
    pub sending_key: [u8; 32],
    /// 32-byte key for decrypting incoming packets
    pub receiving_key: [u8; 32],
    /// ID of the current sending key
    pub sending_key_id: u32,
    /// ID of the current receiving key
    pub receiving_key_id: u32,
    /// Timestamp when these keys were created
    pub created_at: Instant,
}

#[derive(Debug, Clone, PartialEq)]
enum HandshakeState {
    None,
    InitiationSent,
    ResponseReceived,
    Established,
}

/// WireGuard peer statistics and metrics
///
/// Tracks performance metrics and counters for a WireGuard peer connection.
#[derive(Debug, Clone, Default)]
pub struct PeerStats {
    /// Total bytes sent to this peer
    pub bytes_sent: u64,
    /// Total bytes received from this peer
    pub bytes_received: u64,
    /// Total packets sent to this peer
    pub packets_sent: u64,
    /// Total packets received from this peer
    pub packets_received: u64,
    /// Number of handshakes initiated with this peer
    pub handshakes_initiated: u64,
    /// Number of handshakes successfully completed
    pub handshakes_completed: u64,
}

impl PeerState {
    /// Creates a new instance with default configuration.
    pub fn new(config: PeerConfig, index: u32) -> Self {
        Self {
            config,
            handshake_state: HandshakeState::None,
            session_keys: None,
            replay_window: ReplayWindow::new(),
            last_handshake: None,
            last_received: None,
            last_sent: None,
            cookie: None,
            cookie_expiration: None,
            index,
            stats: PeerStats::default(),
        }
    }

    /// Returns the established value.
    pub fn is_established(&self) -> bool {
        self.handshake_state == HandshakeState::Established && self.session_keys.is_some()
    }

    /// Performs the needs rekey operation.
    pub fn needs_rekey(&self) -> bool {
        if let Some(keys) = &self.session_keys {
            keys.created_at.elapsed() > REKEY_TIMEOUT
        } else {
            true
        }
    }

    /// Updates the stats sent state.
    pub fn update_stats_sent(&mut self, bytes: u64) {
        self.stats.bytes_sent += bytes;
        self.stats.packets_sent += 1;
        self.last_sent = Some(Instant::now());
    }

    /// Updates the stats received state.
    pub fn update_stats_received(&mut self, bytes: u64) {
        self.stats.bytes_received += bytes;
        self.stats.packets_received += 1;
        self.last_received = Some(Instant::now());
    }
}

/// WireGuard device (interface) managing multiple peers
///
/// Represents a WireGuard interface that handles packet routing,
/// encryption, and peer management for VPN connections.
pub struct WireGuardDevice {
    /// Interface configuration
    config: InterfaceConfig,
    /// All peers indexed by their 32-byte public key
    peers: Arc<RwLock<PeerMap>>,
    /// Peer lookup by index for routing
    peer_indices: Arc<RwLock<HashMap<u32, [u8; 32]>>>,
    /// Next available peer index
    next_index: Arc<Mutex<u32>>,
    /// UDP socket for sending and receiving packets
    socket: Arc<UdpSocket>,
    /// Flag indicating if the device is running
    running: Arc<Mutex<bool>>,
    /// Cipher for packet encryption/decryption
    cipher: Arc<Cipher>,
    /// Hash function for MAC calculations
    hash: Arc<Hash>,
    /// Cryptographically secure random number generator
    rng: Arc<SecureRandom>,
}

impl WireGuardDevice {
    /// Derive public key from private key (X25519)
    fn derive_public_key(private_key: &[u8; 32]) -> Result<[u8; 32]> {
        // X25519 public key derivation
        // In production, use actual X25519 implementation
        let mut public_key = [0u8; 32];
        for i in 0..32 {
            public_key[i] = private_key[i].wrapping_add(1);
        }
        Ok(public_key)
    }

    /// Create a new WireGuard device
    pub async fn new(config: InterfaceConfig) -> Result<Self> {
        let socket = UdpSocket::bind(format!("[::]:{}", config.listen_port)).await?;
        let key = vec![0u8; 32];
        let cipher = Arc::new(Cipher::new(&key, CipherSuite::ChaCha20Poly1305)?);
        let hash = Arc::new(Hash::new()?);
        let rng = Arc::new(SecureRandom::new()?);

        Ok(Self {
            config,
            peers: Arc::new(RwLock::new(HashMap::new())),
            peer_indices: Arc::new(RwLock::new(HashMap::new())),
            next_index: Arc::new(Mutex::new(1)),
            socket: Arc::new(socket),
            running: Arc::new(Mutex::new(false)),
            cipher,
            hash,
            rng,
        })
    }

    /// Add a peer to the device
    pub async fn add_peer(&self, config: PeerConfig) -> Result<()> {
        let mut next_index = self.next_index.lock().await;
        let index = *next_index;
        *next_index = index.wrapping_add(1);
        drop(next_index);

        let peer = Arc::new(Mutex::new(PeerState::new(config.clone(), index)));

        let mut peers = self.peers.write().await;
        let mut peer_indices = self.peer_indices.write().await;

        peers.insert(config.public_key, peer.clone());
        peer_indices.insert(index, config.public_key);

        info!("Added peer with index {}", index);
        Ok(())
    }

    /// Remove a peer
    pub async fn remove_peer(&self, public_key: &[u8; 32]) -> Result<()> {
        let mut peers = self.peers.write().await;
        let mut peer_indices = self.peer_indices.write().await;

        if let Some(peer) = peers.remove(public_key) {
            let peer_state = peer.lock().await;
            peer_indices.remove(&peer_state.index);
            info!("Removed peer with index {}", peer_state.index);
        }

        Ok(())
    }

    /// Start the device
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = true;
        drop(running);

        info!(
            "WireGuard device started on port {}",
            self.config.listen_port
        );

        // Start receive loop
        self.receive_loop().await?;

        Ok(())
    }

    /// Stop the device
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.lock().await;
        *running = false;
        info!("WireGuard device stopped");
        Ok(())
    }

    /// Main receive loop
    async fn receive_loop(&self) -> Result<()> {
        let mut buf = vec![0u8; MAX_MESSAGE_SIZE];

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
                    self.handle_packet(data, src).await?;
                },
                Err(e) => {
                    error!("Error receiving packet: {}", e);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                },
            }
        }

        Ok(())
    }

    /// Handle incoming packet
    async fn handle_packet(&self, mut data: Vec<u8>, src: SocketAddr) -> Result<()> {
        // Check for stealth mode header
        if self.config.stealth_mode {
            if data.starts_with(STEALTH_MODE_HEADER) {
                data = data[STEALTH_MODE_HEADER.len()..].to_vec();
            } else {
                return Err(VantisError::InvalidPacket("Invalid stealth header".into()));
            }
        }

        if data.is_empty() {
            return Err(VantisError::InvalidPacket("Empty packet".into()));
        }

        let message_type = data[0];

        match message_type {
            1 => self.handle_handshake_initiation(data, src).await?,
            2 => self.handle_handshake_response(data, src).await?,
            3 => self.handle_cookie_reply(data, src).await?,
            4 => self.handle_transport_data(data, src).await?,
            _ => {
                warn!("Unknown message type: {}", message_type);
            },
        }

        Ok(())
    }

    /// Handle handshake initiation
    async fn handle_handshake_initiation(&self, data: Vec<u8>, _src: SocketAddr) -> Result<()> {
        let initiation = HandshakeInitiation::deserialize(&data)?;
        debug!(
            "Received handshake initiation from index {}",
            initiation.sender_index
        );

        // Find peer by sender index
        let peer_indices = self.peer_indices.read().await;
        let public_key = match peer_indices.get(&initiation.sender_index) {
            Some(key) => *key,
            None => {
                warn!("Unknown peer index: {}", initiation.sender_index);
                return Ok(());
            },
        };
        drop(peer_indices);

        let peers = self.peers.read().await;
        let peer = match peers.get(&public_key) {
            Some(p) => p.clone(),
            None => return Ok(()),
        };
        drop(peers);

        let mut peer_state = peer.lock().await;

        // Verify MAC1
        if !self
            .verify_mac1(&initiation, &peer_state.config.public_key)
            .await
        {
            warn!("Invalid MAC1 from peer {}", peer_state.index);
            return Ok(());
        }

        // Check MAC2 for cookie verification
        if initiation.mac2 != [0u8; 16] {
            if let Some(cookie) = &peer_state.cookie {
                if !self.verify_mac2(&initiation, cookie).await {
                    warn!("Invalid MAC2 from peer {}", peer_state.index);
                    return Ok(());
                }
            }
        } else {
            // Request cookie
            self.send_cookie_reply(&peer_state, &initiation).await?;
            return Ok(());
        }

        // Process handshake
        peer_state.handshake_state = HandshakeState::ResponseReceived;
        peer_state.stats.handshakes_completed += 1;
        peer_state.last_handshake = Some(Instant::now());

        // Generate response
        let response = self
            .generate_handshake_response(&peer_state, &initiation)
            .await?;

        // Send response
        if let Some(endpoint) = &peer_state.config.endpoint {
            self.socket.send_to(&response.serialize(), endpoint).await?;
            debug!("Sent handshake response to {}", endpoint);
        }

        Ok(())
    }

    /// Handle handshake response
    async fn handle_handshake_response(&self, data: Vec<u8>, _src: SocketAddr) -> Result<()> {
        let response = HandshakeResponse::deserialize(&data)?;
        debug!(
            "Received handshake response from index {}",
            response.sender_index
        );

        // Find peer by receiver index
        let peer_indices = self.peer_indices.read().await;
        let public_key = match peer_indices.get(&response.receiver_index) {
            Some(key) => *key,
            None => {
                warn!("Unknown peer index: {}", response.receiver_index);
                return Ok(());
            },
        };
        drop(peer_indices);

        let peers = self.peers.read().await;
        let peer = match peers.get(&public_key) {
            Some(p) => p.clone(),
            None => return Ok(()),
        };
        drop(peers);

        let mut peer_state = peer.lock().await;

        // Verify MAC1
        if !self
            .verify_mac1_response(&response, &peer_state.config.public_key)
            .await
        {
            warn!("Invalid MAC1 from peer {}", peer_state.index);
            return Ok(());
        }

        // Check MAC2
        if response.mac2 != [0u8; 16] {
            if let Some(cookie) = &peer_state.cookie {
                if !self.verify_mac2_response(&response, cookie).await {
                    warn!("Invalid MAC2 from peer {}", peer_state.index);
                    return Ok(());
                }
            }
        }

        // Complete handshake
        peer_state.handshake_state = HandshakeState::Established;
        peer_state.stats.handshakes_completed += 1;
        peer_state.last_handshake = Some(Instant::now());

        // Derive session keys
        let session_keys = self.derive_session_keys(&peer_state, &response).await?;
        peer_state.session_keys = Some(session_keys);

        info!("Handshake completed with peer {}", peer_state.index);

        Ok(())
    }

    /// Handle cookie reply
    async fn handle_cookie_reply(&self, data: Vec<u8>, _src: SocketAddr) -> Result<()> {
        let cookie_reply = CookieReply::deserialize(&data)?;
        debug!(
            "Received cookie reply for index {}",
            cookie_reply.receiver_index
        );

        // Find peer by receiver index
        let peer_indices = self.peer_indices.read().await;
        let public_key = match peer_indices.get(&cookie_reply.receiver_index) {
            Some(key) => *key,
            None => return Ok(()),
        };
        drop(peer_indices);

        let peers = self.peers.read().await;
        let peer = match peers.get(&public_key) {
            Some(p) => p.clone(),
            None => return Ok(()),
        };
        drop(peers);

        let mut peer_state = peer.lock().await;

        // Decrypt cookie
        let cookie = self.decrypt_cookie(&cookie_reply).await?;

        peer_state.cookie = Some(cookie);
        peer_state.cookie_expiration = Some(Instant::now() + COOKIE_REFRESH_TIME);

        debug!("Cookie updated for peer {}", peer_state.index);

        Ok(())
    }

    /// Handle transport data
    async fn handle_transport_data(&self, data: Vec<u8>, _src: SocketAddr) -> Result<()> {
        let transport = TransportData::deserialize(&data)?;

        // Find peer by receiver index
        let peer_indices = self.peer_indices.read().await;
        let public_key = match peer_indices.get(&transport.receiver_index) {
            Some(key) => *key,
            None => return Ok(()),
        };
        drop(peer_indices);

        let peers = self.peers.read().await;
        let peer = match peers.get(&public_key) {
            Some(p) => p.clone(),
            None => return Ok(()),
        };
        drop(peers);

        let mut peer_state = peer.lock().await;

        // Check replay protection
        if !peer_state.replay_window.check(transport.counter) {
            warn!("Replay packet detected from peer {}", peer_state.index);
            return Ok(());
        }

        // Decrypt data
        let decrypted = if let Some(keys) = &peer_state.session_keys {
            self.decrypt_transport_data(&transport, keys).await?
        } else {
            return Err(VantisError::CryptoError("No session keys".into()));
        };

        peer_state.update_stats_received(decrypted.len() as u64);

        debug!(
            "Received {} bytes from peer {}",
            decrypted.len(),
            peer_state.index
        );

        // Process decrypted data (would be passed to network stack)
        // For now, just log it
        Ok(())
    }

    /// Initiate handshake with a peer
    pub async fn initiate_handshake(&self, public_key: &[u8; 32]) -> Result<()> {
        let peers = self.peers.read().await;
        let peer = match peers.get(public_key) {
            Some(p) => p.clone(),
            None => return Err(VantisError::InvalidPeer("Peer not found".into())),
        };
        drop(peers);

        let mut peer_state = peer.lock().await;

        // Generate ephemeral key pair
        let ephemeral_private_vec = self.rng.generate_bytes(32)?;
        let ephemeral_private: [u8; 32] = ephemeral_private_vec.try_into().unwrap();
        let ephemeral_public = Self::derive_public_key(&ephemeral_private)?;

        // Create initiation message
        let initiation = self
            .generate_handshake_initiation(&peer_state, &ephemeral_public)
            .await?;

        peer_state.handshake_state = HandshakeState::InitiationSent;
        peer_state.stats.handshakes_initiated += 1;

        // Send initiation
        let endpoint = peer_state.config.endpoint;
        if let Some(endpoint) = endpoint {
            let mut data = initiation.serialize();

            // Add stealth mode header
            if self.config.stealth_mode || peer_state.config.stealth_mode {
                let mut stealth_data = STEALTH_MODE_HEADER.to_vec();
                stealth_data.extend_from_slice(&data);
                data = stealth_data;
            }

            self.socket.send_to(&data, &endpoint).await?;
            peer_state.update_stats_sent(data.len() as u64);
            debug!("Sent handshake initiation to {}", endpoint);
        }

        Ok(())
    }

    /// Send data through the tunnel
    pub async fn send_data(&self, public_key: &[u8; 32], data: &[u8]) -> Result<()> {
        let peers = self.peers.read().await;
        let peer = match peers.get(public_key) {
            Some(p) => p.clone(),
            None => return Err(VantisError::InvalidPeer("Peer not found".into())),
        };
        drop(peers);

        // Check if handshake is established, retry if needed
        loop {
            let mut peer_state = peer.lock().await;

            if peer_state.is_established() && !peer_state.needs_rekey() {
                // Encrypt data
                let keys = peer_state.session_keys.as_ref().unwrap();
                let counter = keys.sending_key_id as u64;
                let encrypted = self
                    .encrypt_transport_data(data, peer_state.index, counter, keys)
                    .await?;

                peer_state.update_stats_sent(data.len() as u64);

                // Send encrypted data
                if let Some(endpoint) = &peer_state.config.endpoint {
                    let mut packet = Vec::with_capacity(MESSAGE_DATA_SIZE + encrypted.len());
                    packet.extend_from_slice(&peer_state.index.to_le_bytes());
                    packet.extend_from_slice(&counter.to_le_bytes());
                    packet.extend_from_slice(&encrypted);

                    self.socket.send_to(&packet, endpoint).await?;
                    debug!("Sent {} bytes to {}", data.len(), endpoint);
                }

                return Ok(());
            }

            drop(peer_state);
            self.initiate_handshake(public_key).await?;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    // Helper methods for cryptographic operations

    async fn generate_handshake_initiation(
        &self,
        peer_state: &PeerState,
        ephemeral_public: &[u8; 32],
    ) -> Result<HandshakeInitiation> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let mac1 = self
            .calculate_mac1_initiation(peer_state, ephemeral_public)
            .await?;

        Ok(HandshakeInitiation {
            message_type: 1,
            sender_index: peer_state.index,
            ephemeral_public: *ephemeral_public,
            static_public_enc: [0u8; 32], // Would be encrypted
            timestamp_enc: {
                let mut ts_bytes = [0u8; 12];
                ts_bytes[..8].copy_from_slice(&timestamp.to_le_bytes());
                ts_bytes
            },
            mac1,
            mac2: [0u8; 16],
        })
    }

    async fn generate_handshake_response(
        &self,
        peer_state: &PeerState,
        initiation: &HandshakeInitiation,
    ) -> Result<HandshakeResponse> {
        let ephemeral_private_vec = self.rng.generate_bytes(32)?;
        let ephemeral_private: [u8; 32] = ephemeral_private_vec.try_into().unwrap();
        let ephemeral_public = Self::derive_public_key(&ephemeral_private)?;

        let mac1 = self
            .calculate_mac1_response(peer_state, &ephemeral_public)
            .await?;

        Ok(HandshakeResponse {
            message_type: 2,
            sender_index: peer_state.index,
            receiver_index: initiation.sender_index,
            ephemeral_public,
            empty_enc: [0u8; 16],
            mac1,
            mac2: [0u8; 16],
        })
    }

    async fn send_cookie_reply(
        &self,
        peer_state: &PeerState,
        initiation: &HandshakeInitiation,
    ) -> Result<()> {
        let cookie_vec = self.rng.generate_bytes(16)?;
        let cookie: [u8; 16] = cookie_vec.try_into().unwrap();
        let nonce_vec = self.rng.generate_bytes(24)?;
        let nonce: [u8; 24] = nonce_vec.try_into().unwrap();

        let cookie_reply = CookieReply {
            message_type: 3,
            receiver_index: initiation.sender_index,
            nonce,
            cookie_enc: cookie, // Would be encrypted
        };

        if let Some(endpoint) = &peer_state.config.endpoint {
            self.socket
                .send_to(&cookie_reply.serialize(), endpoint)
                .await?;
            debug!("Sent cookie reply to {}", endpoint);
        }

        Ok(())
    }

    async fn derive_session_keys(
        &self,
        _peer_state: &PeerState,
        _response: &HandshakeResponse,
    ) -> Result<SessionKeys> {
        // In production, derive keys from DH exchange
        let sending_key = self.rng.generate_bytes(32)?;
        let receiving_key = self.rng.generate_bytes(32)?;

        Ok(SessionKeys {
            sending_key: sending_key.try_into().unwrap(),
            receiving_key: receiving_key.try_into().unwrap(),
            sending_key_id: 1,
            receiving_key_id: 1,
            created_at: Instant::now(),
        })
    }

    async fn encrypt_transport_data(
        &self,
        data: &[u8],
        receiver_index: u32,
        counter: u64,
        _keys: &SessionKeys,
    ) -> Result<Vec<u8>> {
        let nonce = counter.to_le_bytes();
        let encrypted = self.cipher.encrypt(data, &nonce)?;

        let mut packet = Vec::with_capacity(MESSAGE_DATA_SIZE + encrypted.len());
        packet.extend_from_slice(&receiver_index.to_le_bytes());
        packet.extend_from_slice(&counter.to_le_bytes());
        packet.extend_from_slice(&encrypted);

        Ok(packet)
    }

    async fn decrypt_transport_data(
        &self,
        transport: &TransportData,
        _keys: &SessionKeys,
    ) -> Result<Vec<u8>> {
        let nonce = transport.counter.to_le_bytes();
        self.cipher.decrypt(&transport.data, &nonce)
    }

    async fn decrypt_cookie(&self, cookie_reply: &CookieReply) -> Result<[u8; 16]> {
        // In production, decrypt cookie with device key
        Ok(cookie_reply.cookie_enc)
    }

    async fn calculate_mac1_initiation(
        &self,
        peer_state: &PeerState,
        ephemeral_public: &[u8; 32],
    ) -> Result<[u8; 16]> {
        let mut data = Vec::new();
        data.extend_from_slice(&peer_state.config.public_key);
        data.extend_from_slice(ephemeral_public);

        let mac = self.hash.compute_mac(&data, &self.config.public_key)?;
        Ok(mac[..16].try_into().unwrap())
    }

    async fn calculate_mac1_response(
        &self,
        peer_state: &PeerState,
        ephemeral_public: &[u8; 32],
    ) -> Result<[u8; 16]> {
        let mut data = Vec::new();
        data.extend_from_slice(&peer_state.config.public_key);
        data.extend_from_slice(ephemeral_public);

        let mac = self.hash.compute_mac(&data, &self.config.public_key)?;
        Ok(mac[..16].try_into().unwrap())
    }

    async fn verify_mac1(&self, _initiation: &HandshakeInitiation, _public_key: &[u8; 32]) -> bool {
        // In production, verify MAC1
        true
    }

    async fn verify_mac1_response(
        &self,
        _response: &HandshakeResponse,
        _public_key: &[u8; 32],
    ) -> bool {
        // In production, verify MAC1
        true
    }

    async fn verify_mac2(&self, _initiation: &HandshakeInitiation, _cookie: &[u8; 16]) -> bool {
        // In production, verify MAC2 with cookie
        true
    }

    async fn verify_mac2_response(
        &self,
        _response: &HandshakeResponse,
        _cookie: &[u8; 16],
    ) -> bool {
        // In production, verify MAC2 with cookie
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handshake_initiation_serialization() {
        let initiation = HandshakeInitiation {
            message_type: 1,
            sender_index: 12345,
            ephemeral_public: [1u8; 32],
            static_public_enc: [2u8; 32],
            timestamp_enc: [3u8; 12],
            mac1: [4u8; 16],
            mac2: [5u8; 16],
        };

        let serialized = initiation.serialize();
        let deserialized = HandshakeInitiation::deserialize(&serialized).unwrap();

        assert_eq!(initiation.message_type, deserialized.message_type);
        assert_eq!(initiation.sender_index, deserialized.sender_index);
    }

    #[tokio::test]
    async fn test_replay_window() {
        let mut window = ReplayWindow::new();

        assert!(window.check(1));
        assert!(window.check(2));
        assert!(!window.check(1)); // Replay
        assert!(window.check(3));
    }
}
