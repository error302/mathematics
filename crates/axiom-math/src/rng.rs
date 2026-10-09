//! AXIOM RNG v1 (Section 14.2).
//!
//! A frozen, explicit byte-stream specification:
//! `block_c = SHA256(D || S || BE64(c))` with `D = b"AXIOM-RNG-v1\0"`.
//! Each 32-byte block yields four big-endian `u64` draws, consumed in order.
//! Integer ranges use rejection sampling against `L = 2^64 - (2^64 mod n)`.
//!
//! This is a deterministic replay stream, not a cryptographic protocol.

use sha2::{Digest, Sha256};

/// Domain separator: ASCII `AXIOM-RNG-v1` followed by one zero byte.
pub const DOMAIN: &[u8] = b"AXIOM-RNG-v1\0";
/// Generator version identifier recorded in every replay identity.
pub const GENERATOR_VERSION: &str = "rng-sha256-v1";

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SeedError {
    #[error("seed must be exactly 64 lowercase hexadecimal characters")]
    InvalidFormat,
}

/// Validates and decodes a 32-byte seed given as 64 lowercase hex characters.
pub fn decode_seed(seed_hex: &str) -> Result<[u8; 32], SeedError> {
    if seed_hex.len() != 64
        || !seed_hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(SeedError::InvalidFormat);
    }
    let bytes = hex::decode(seed_hex).map_err(|_| SeedError::InvalidFormat)?;
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

/// Source of raw `u64` draws. Abstracted so tests can inject synthetic streams.
pub trait DrawSource {
    fn next_u64(&mut self) -> u64;
}

/// The AXIOM RNG v1 stream.
#[derive(Debug, Clone)]
pub struct AxiomRng {
    seed: [u8; 32],
    counter: u64,
    block: [u64; 4],
    index: usize,
    draws: u64,
}

impl AxiomRng {
    pub fn new(seed: [u8; 32]) -> Self {
        AxiomRng {
            seed,
            counter: 0,
            block: [0; 4],
            index: 4,
            draws: 0,
        }
    }

    pub fn from_hex(seed_hex: &str) -> Result<Self, SeedError> {
        Ok(Self::new(decode_seed(seed_hex)?))
    }

    /// Computes block `c` for this seed.
    pub fn block_bytes(seed: &[u8; 32], counter: u64) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(DOMAIN);
        h.update(seed);
        h.update(counter.to_be_bytes());
        h.finalize().into()
    }

    /// Number of raw draws consumed so far (useful for diagnostics/budgets).
    pub fn draws_consumed(&self) -> u64 {
        self.draws
    }

    fn refill(&mut self) {
        let bytes = Self::block_bytes(&self.seed, self.counter);
        for (i, chunk) in bytes.chunks_exact(8).enumerate() {
            let mut b = [0u8; 8];
            b.copy_from_slice(chunk);
            self.block[i] = u64::from_be_bytes(b);
        }
        self.counter = self
            .counter
            .checked_add(1)
            .expect("AXIOM RNG counter exhausted");
        self.index = 0;
    }
}

impl DrawSource for AxiomRng {
    fn next_u64(&mut self) -> u64 {
        if self.index >= 4 {
            self.refill();
        }
        let v = self.block[self.index];
        self.index += 1;
        self.draws += 1;
        v
    }
}

/// Uniform integer in the inclusive range `[a, b]` using AXIOM rejection sampling.
///
/// Panics if `a > b` (a template configuration error, validated before generation).
pub fn uniform_inclusive<R: DrawSource + ?Sized>(rng: &mut R, a: i128, b: i128) -> i128 {
    assert!(a <= b, "invalid range [{a}, {b}]");
    let n: u128 = (b - a) as u128 + 1;
    assert!(n <= 1u128 << 64, "range larger than 2^64");
    if n == 1u128 << 64 {
        return a + rng.next_u64() as i128;
    }
    let two64: u128 = 1u128 << 64;
    let limit: u128 = two64 - (two64 % n);
    loop {
        let u = rng.next_u64() as u128;
        if u >= limit {
            continue;
        }
        return a + (u % n) as i128;
    }
}

/// Fisher–Yates shuffle: index from last down to 1, drawing uniformly in `[0, index]`.
pub fn shuffle<T, R: DrawSource + ?Sized>(rng: &mut R, items: &mut [T]) {
    if items.len() < 2 {
        return;
    }
    let mut i = items.len() - 1;
    while i >= 1 {
        let j = uniform_inclusive(rng, 0, i as i128) as usize;
        items.swap(i, j);
        i -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    struct Synthetic(Vec<u64>, usize);
    impl DrawSource for Synthetic {
        fn next_u64(&mut self) -> u64 {
            let v = self.0[self.1];
            self.1 += 1;
            v
        }
    }

    #[test]
    fn zero_seed_reference_block() {
        // Independent SHA-256 reference vector from the source of truth, Section 14.2.
        let seed = decode_seed(ZERO).unwrap();
        let block = AxiomRng::block_bytes(&seed, 0);
        assert_eq!(
            hex::encode(block),
            "955ce261c1f93ec7622e45849d2c4eeaa6cecc71079e1940869f63878bb8f0e5"
        );
        let mut rng = AxiomRng::new(seed);
        let draws: Vec<u64> = (0..4).map(|_| rng.next_u64()).collect();
        assert_eq!(
            draws,
            vec![
                10762726119002685127,
                7074668500520554218,
                12019769241329604928,
                9700581556195225829
            ]
        );
    }

    #[test]
    fn zero_seed_first_sample_one_to_nine() {
        let mut rng = AxiomRng::from_hex(ZERO).unwrap();
        assert_eq!(uniform_inclusive(&mut rng, 1, 9), 2);
    }

    #[test]
    fn rejection_case_from_spec() {
        // L = 18446744073709551610 for n = 10; u64::MAX must be rejected, then 7 -> 7.
        let mut s = Synthetic(vec![18446744073709551615, 7], 0);
        assert_eq!(uniform_inclusive(&mut s, 0, 9), 7);
        assert_eq!(s.1, 2);
    }

    #[test]
    fn full_range_uses_draw_directly() {
        let mut s = Synthetic(vec![5], 0);
        let a = -(1i128 << 63);
        let b = a + (1i128 << 64) - 1;
        assert_eq!(uniform_inclusive(&mut s, a, b), a + 5);
    }

    #[test]
    fn cross_block_draws_are_ordered() {
        let seed = decode_seed(ZERO).unwrap();
        let mut rng = AxiomRng::new(seed);
        for _ in 0..4 {
            rng.next_u64();
        }
        let fifth = rng.next_u64();
        let block1 = AxiomRng::block_bytes(&seed, 1);
        let mut b = [0u8; 8];
        b.copy_from_slice(&block1[0..8]);
        assert_eq!(fifth, u64::from_be_bytes(b));
    }

    #[test]
    fn rejects_bad_seeds() {
        assert!(decode_seed("00").is_err());
        assert!(decode_seed(&"A".repeat(64)).is_err());
        assert!(decode_seed(&"g".repeat(64)).is_err());
        assert!(decode_seed(&"f".repeat(64)).is_ok());
    }

    #[test]
    fn shuffle_is_deterministic() {
        let mut a = AxiomRng::from_hex(ZERO).unwrap();
        let mut b = AxiomRng::from_hex(ZERO).unwrap();
        let mut x: Vec<u32> = (0..10).collect();
        let mut y: Vec<u32> = (0..10).collect();
        shuffle(&mut a, &mut x);
        shuffle(&mut b, &mut y);
        assert_eq!(x, y);
        let mut sorted = x.clone();
        sorted.sort();
        assert_eq!(sorted, (0..10).collect::<Vec<_>>());
    }
}
