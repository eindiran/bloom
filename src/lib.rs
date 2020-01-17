/*!
 * lib.rs
 *
 * Implements a Murmur3-based bloom filter variations:
 *      BloomFilter             -- a standard bloom filter.
 *      CountingBloomFilter     -- a bloom filter which supports deleting items.
 *      ScopeDecayBloomFilter   -- a scope decay bloom filter, supporting gradual resetting of bits
 *                                 over time.
 *      GenerationalBloomFilter -- use 2 bloom filters to support a moving window of data.
 */
pub mod bloom; // Export the module defined in bloom.rs
