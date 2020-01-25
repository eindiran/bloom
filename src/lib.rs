/*!
 * lib.rs
 *
 * Implements a Murmur3-based bloom filter variations:
 *      BloomFilter               -- a standard bloom filter.
 *      CountingBloomFilter       -- a bloom filter which supports deleting items, using counters
 *                                   instead of bits.
 *      ScopeDecayBloomFilter     -- a scope decay bloom filter, supporting gradual resetting of bits
 *                                   over time.
 *      GenerationalBloomFilter   -- use N generations of bloom filters to support a moving window of data.
 *      BigenerationalBloomFilter -- use 2 alternating generations of bloom filters to support a
 *                                   moving window of data.
 *      A2BufferingBloomFilter    -- use a segmented bloom filter, relying on the active-active
 *                                   buffering strategy to keep recent elements in the current
 *                                   segment.
 */
pub mod bloom; // Export the module defined in bloom.rs
