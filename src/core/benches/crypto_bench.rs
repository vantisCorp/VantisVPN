// Benchmark Suite for VANTISVPN Cryptographic Operations
// Measures performance of critical cryptographic functions

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vantis_core::crypto::keys::EphemeralKeyPair;
use vantis_core::crypto::cipher::{Cipher, CipherSuite, EncryptionContext, DecryptionContext};
use vantis_core::crypto::hash::Hash;
use vantis_core::crypto::random::SecureRandom;

fn bench_key_generation(c: &mut Criterion) {
    c.bench_function("key_generation", |b| {
        b.iter(|| {
            black_box(EphemeralKeyPair::new().unwrap())
        })
    });
}

fn bench_encryption(c: &mut Criterion) {
    let key_pair = EphemeralKeyPair::new().unwrap();
    let mut encrypt_ctx = EncryptionContext::new(key_pair.public_key().as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let data = vec![0u8; 1024]; // 1KB of data
    
    c.bench_function("encryption_1kb", |b| {
        b.iter(|| {
            black_box(encrypt_ctx.encrypt_packet(black_box(&data)).unwrap())
        })
    });
}

fn bench_decryption(c: &mut Criterion) {
    let key_pair = EphemeralKeyPair::new().unwrap();
    let mut encrypt_ctx = EncryptionContext::new(key_pair.public_key().as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let mut decrypt_ctx = DecryptionContext::new(key_pair.public_key().as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let data = vec![0u8; 1024];
    let encrypted = encrypt_ctx.encrypt_packet(&data).unwrap();
    
    c.bench_function("decryption_1kb", |b| {
        b.iter(|| {
            black_box(decrypt_ctx.decrypt_packet(black_box(&encrypted)).unwrap())
        })
    });
}

fn bench_hash_computation(c: &mut Criterion) {
    let hash_instance = Hash::new().unwrap();
    let data = vec![0u8; 1024];
    
    c.bench_function("hash_1kb", |b| {
        b.iter(|| {
            black_box(hash_instance.compute(black_box(&data)).unwrap())
        })
    });
}

fn bench_random_generation(c: &mut Criterion) {
    let rng = SecureRandom::new().unwrap();
    
    c.bench_function("random_32_bytes", |b| {
        b.iter(|| {
            black_box(rng.generate_bytes(32).unwrap())
        })
    });
}

fn bench_large_encryption(c: &mut Criterion) {
    let key_pair = EphemeralKeyPair::new().unwrap();
    let mut encrypt_ctx = EncryptionContext::new(key_pair.public_key().as_bytes(), CipherSuite::ChaCha20Poly1305).unwrap();
    let data = vec![0u8; 65536]; // 64KB of data
    
    c.bench_function("encryption_64kb", |b| {
        b.iter(|| {
            black_box(encrypt_ctx.encrypt_packet(black_box(&data)).unwrap())
        })
    });
}

criterion_group!(
    benches,
    bench_key_generation,
    bench_encryption,
    bench_decryption,
    bench_hash_computation,
    bench_random_generation,
    bench_large_encryption
);
criterion_main!(benches);