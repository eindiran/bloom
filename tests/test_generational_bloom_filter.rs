/**
 * test_generational_bloom_filter.rs
 *
 * Integration tests for GenerationalBloomFilter.
 */
use bloom::bloom::GenerationalBloomFilter;

#[test]
/// Test that we can check the number of generations via the getter
fn test_get_num_generations() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 3);
    assert_eq!(bf.get_num_generations(), 3);
    let bf2: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 10);
    assert_eq!(bf2.get_num_generations(), 10);
}

#[test]
/// Test that the getter for hash_count is visible from outside the crate
fn test_get_hash_count() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 3);
    assert!(bf.get_hash_count() > 0);
}

#[test]
/// Test that the getter for false_positive_rate is visible from outside the crate
fn test_get_false_positive_rate() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 3);
    assert_eq!(bf.get_false_positive_rate(), 0.01);
}

#[test]
/// Test that the getter for expected_inserts is visible from outside the crate;
/// this is the number of inserts that will occur in each generation
fn test_get_expected_inserts() {
    let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 3);
    assert_eq!(bf.get_expected_inserts(), 10000);
}

#[test]
/// Test that the getter for actual_inserts is visible from outside the crate
fn test_get_actual_inserts() {
    let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 3);
    assert_eq!(bf.get_actual_inserts(), 0);
    let test_str = "This is a test string";
    bf.insert(&test_str.to_string());
    assert_eq!(bf.get_actual_inserts(), 1);
}

#[test]
/// Ensure that the false positive rate is close to the actual value
fn test_false_positive_rate() {
    let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(10000, 0.01, 3);
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
