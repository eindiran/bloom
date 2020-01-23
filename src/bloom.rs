/*!
 * bloom.rs
 *
 * Collates the various bloom filter types into a single Rust module.
 */
pub use self::a2buffering_bloom_filter::A2BufferingBloomFilter;
pub use self::bigenerational_bloom_filter::BigenerationalBloomFilter;
pub use self::bloom_filter::BloomFilter;
pub use self::counting_bloom_filter::CountingBloomFilter;
pub use self::generational_bloom_filter::GenerationalBloomFilter;
pub use self::generations::Generation;
pub use self::generations::GenerationValues;
pub use self::scope_decay_bloom_filter::ScopeDecayBloomFilter;

mod a2buffering_bloom_filter;
mod bigenerational_bloom_filter;
mod bloom_filter;
mod counting_bloom_filter;
mod generational_bloom_filter;
mod generations;
mod scope_decay_bloom_filter;
