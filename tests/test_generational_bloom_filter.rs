/**
 * test_generational_bloom_filter.rs
 *
 * Integration tests for GenerationalBloomFilter.
 */
use bloom::bloom::GenerationalBloomFilter;

#[test]
/// Test that the getter for hash_count is visible from outside the crate
fn test_get_hash_count() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01);
    assert!(bf.get_hash_count() > 0);
}

#[test]
/// Test that the getter for false_positive_rate is visible from outside the crate
fn test_get_false_positive_rate() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01);
    assert_eq!(bf.get_false_positive_rate(), 0.01);
}

#[test]
/// Test that the getter for expected_inserts is visible from outside the crate;
/// this is the number of inserts that will occur in each generation
fn test_get_expected_inserts() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01);
    assert_eq!(bf.get_expected_inserts(), 10000);
}

#[test]
/// Test that the getter for actual_inserts is visible from outside the crate
fn test_get_actual_inserts() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01);
    assert_eq!(bf.get_actual_inserts(), 0);
}

#[test]
/// Ensure that the false positive rate is close to the actual value
fn test_false_positive_rate() {
    let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01);
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

#[test]
/// A end-to-end test with a forced recycle
fn test_e2e() {
    let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(100, 0.001);
    // Fill 'er up
    for i in 0..199 {
        bf.insert(&i.to_string());
    }
    // Make sure all of our entries are still present
    for j in 0..199 {
        assert!(bf.check(&j.to_string()));
    }
    // Continue adding new entries to force a recycle/generational turn over
    for k in 200..300 {
        bf.insert(&k.to_string());
    }
    // Now check on 0..100, which should have been recycled
    let mut false_positives: u64 = 0;
    for l in 0..100 {
        if bf.check(&l.to_string()) {
            false_positives += 1;
        }
    }
    assert!(false_positives <= 1);
}
