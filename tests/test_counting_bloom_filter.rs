/**
 * test_counting_bloom_filter.rs
 *
 * Test that the murmur3 implementation of a Counting Bloom Filter is
 * reasonably performant and the expected behavior is seen wrt to false
 * positive rate.
 */
use bloom::bloom::CountingBloomFilter as CountingBloomFilter;

#[test]
/// Test that the getter for hash_count is visible from outside the crate
fn test_get_hash_count() {
    let bf: CountingBloomFilter = CountingBloomFilter::new(10000, 0.01);
    assert!(bf.get_hash_count() > 0);
}

#[test]
/// Test that the getter for false_positive_rate is visible from outside the crate
fn test_get_false_positive_rate() {
    let bf: CountingBloomFilter = CountingBloomFilter::new(10000, 0.01);
    assert_eq!(bf.get_false_positive_rate(), 0.01);
}

#[test]
/// Test that the getter for expected_inserts is visible from outside the crate
fn test_get_expected_inserts() {
    let bf: CountingBloomFilter = CountingBloomFilter::new(10000, 0.01);
    assert_eq!(bf.get_expected_inserts(), 10000);
}

#[test]
/// Test that the getter for actual_inserts is visible from outside the crate
fn test_get_actual_inserts() {
    let bf: CountingBloomFilter = CountingBloomFilter::new(10000, 0.01);
    assert_eq!(bf.get_actual_inserts(), 0);
}

#[test]
/// Ensure that the false positive rate is close to the actual value
fn test_false_positive_rate() {
    let mut bf: CountingBloomFilter = CountingBloomFilter::new(10000, 0.01);
    for i in 0..10000 {
        bf.insert(&i.to_string());
    }
    let mut false_positives: u64 = 0;
    for i in 10000..100000 {
        if bf.check(&i.to_string()) {
            false_positives += 1;
        }
    }
    assert!((false_positives as f64) < (90000.0 * 0.011));
}
