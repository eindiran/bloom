/**
 * bloom.rs
 *
 * Collates the various bloom filter types into a single Rust module.
 */

pub use self::bloom_filter::BloomFilter;
pub use self::counting_bloom_filter::CountingBloomFilter;
pub use self::scope_decay_bloom_filter::ScopeDecayBloomFilter;

mod bloom_filter;
mod counting_bloom_filter;
mod scope_decay_bloom_filter;
