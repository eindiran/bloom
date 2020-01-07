#![allow(unused_parens)]

/**
 * lib.rs
 *
 * Implements a Murmur3-based Bloom Filter.
 */
extern crate bit_vec;
extern crate fasthash;

use bit_vec::BitVec;
use fasthash::murmur3;


// BloomFilter struct:
//    * Bit array
//    * Length of bit array
//    * Hash count
//    * False positive rate
//    * Expected inserts
//    * Actual inserts
pub struct BloomFilter {
    bit_arr: BitVec,
    bit_len: u64,
    hash_count: u64,
    false_positive_rate: f64,
    expected_inserts: u64,
    actual_inserts: u64
}

impl BloomFilter {

    pub fn get_hash_count(&self) -> u64 {
        // Getter for hash_count
        return self.hash_count;
    }

    pub fn get_false_positive_rate(&self) -> f64 {
        // Getter for false_positive_rate
        return self.false_positive_rate;
    }

    pub fn get_expected_inserts(&self) -> u64 {
        // Getter for expected_inserts
        return self.expected_inserts;
    }

    pub fn get_actual_inserts(&self) -> u64 {
        // Getter for actual_inserts
        return self.actual_inserts;
    }

    fn calculate_bit_arr_len(expected_inserts: f64, false_positive_rate: f64) -> u64 {
        // Given a desired false positive rate, calculate the length of the BitVec required
        // See 'm' in this SO answer: https://stackoverflow.com/a/22467497
        // m = ceil(-n*ln(p) / (ln(2)^2))
        let two: f64 = 2.0;
        return ((-1.0 * (expected_inserts) * false_positive_rate.ln()) / two.ln().powf(two)).ceil() as u64;
    }

    fn calculate_hash_count(expected_inserts: f64, bit_len: u64) -> u64 {
        // Calculate the number of hashes required
        // See 'k' in this SO answer: https://stackoverflow.com/a/22467497
        // k = ceil(m/n * ln(2))
        let two: f64 = 2.0;
        return (((bit_len as f64) / expected_inserts) * two.ln()).ceil() as u64;
    }

    fn get_hash_index(i: u32, item: &str, len: u64) -> usize {
        // Return a single usize value, representing an index to be marked or checked
        let digest_val: u128 = murmur3::hash128_with_seed(item, i);  // Compute a murmur3 seeded hash
        let bit_arr_index: u64 = digest_val as u64 % len;            // Mod the len of the BitVec
        return bit_arr_index as usize;
    }

    pub fn new(expected_inserts: u64, false_positive_rate: f64) -> BloomFilter {
        if (false_positive_rate <= 0.0) {
            panic!("False positive rate must be a positive number. Currently: {}", false_positive_rate);
        } else if (expected_inserts < 1) {
            panic!("Expected number of inserts must be a positive number. Currently: {}", expected_inserts);
        }

        let bit_len: u64 = BloomFilter::calculate_bit_arr_len(expected_inserts as f64, false_positive_rate);
        let hash_count: u64 = BloomFilter::calculate_hash_count(expected_inserts as f64, bit_len);

        BloomFilter {
            bit_arr: BitVec::from_elem(bit_len as usize, false), // Create the whole BitVec zeroed-out
            bit_len: bit_len,
            hash_count: hash_count,
            false_positive_rate: false_positive_rate,
            expected_inserts: expected_inserts,
            actual_inserts: 0
        }
    }

    pub fn insert(&mut self, item: &str) {
        // Insert a new element into the BloomFilter
        for i in 0..self.hash_count {
            let bit_arr_index: usize = BloomFilter::get_hash_index(i as u32, item, self.bit_len);
            self.bit_arr.set(bit_arr_index, true); // Set the relevant index to '1'
        }
        self.actual_inserts += 1;
    }

    pub fn check(&self, item: &str) -> bool {
        // Check whether an element is probably in the filter or not
        for i in 0..self.hash_count {
            let bit_arr_index: usize = BloomFilter::get_hash_index(i as u32, item, self.bit_len);
            // Check if the relevant index is set
            if self.bit_arr[bit_arr_index as usize] == false {
                return false;
            }
        }
        // If all values checked matched then the item is
        // likely in the BloomFilter
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test that we can create a new BloomFilter using BloomFilter::new()
        // and that all getters work and return the expected values
        let bf: BloomFilter = BloomFilter::new(3, 0.05);
        assert_eq!(bf.get_expected_inserts(), 3);
        assert_eq!(bf.get_actual_inserts(), 0);
        assert_eq!(bf.get_false_positive_rate(), 0.05);
        assert!(bf.get_hash_count() > 0);
    }

    #[test]
    fn test_insert() {
        // Test that we can insert a string into a BloomFilter
        let s = "This is a test string";    // Inserted
        let s2 = "This is another string";  // Inserted
        let s3 = "This is a third string";  // Not inserted
        let mut bf: BloomFilter = BloomFilter::new(2, 0.05);
        bf.insert(&s);
        bf.insert(&s2);
        assert!(bf.check(&s));    // Included
        assert!(bf.check(&s2));   // Included
        assert!(!bf.check(&s3));  // Not included
    }

    #[test]
    fn test_get_hash_index() {
        // Test that the get_hash_index method works as expected
        let s = "This is a test string";
        let i: u32 = 32;
        assert_eq!(BloomFilter::get_hash_index(i, &s, 10), 4);
    }

    #[test]
    fn test_count_actual_inserts() {
        // Test that the actual_inserts counter works as expected
        let mut bf: BloomFilter = BloomFilter::new(100, 0.05);
        assert_eq!(bf.get_actual_inserts(), 0);
        for i in 1..100 {
            bf.insert(&i.to_string());
            assert_eq!(bf.get_actual_inserts(), i);
        }
    }

    #[test]
    fn test_check() {
        // Test that check behaves like we expect
        let mut bf: BloomFilter = BloomFilter::new(100, 0.05);
        for i in 1..100 {
            bf.insert(&i.to_string());
        }
        for g in 1..100 {
            assert!(bf.check(&g.to_string()));
        }
        let mut false_positives: u64 = 0;
        for h in 101..200 {
            if (bf.check(&h.to_string())) {
                false_positives += 1;
            }
        }
        assert!(false_positives < 6); // Slightly more than 5%
    }

    #[test]
    fn test_calculate_bit_arr_len() {
        // Test that we get the expected value for calculating the length of the
        // BitVec using the computation:
        // m = ceil(-n*ln(p) / (ln(2)^2))
        let m: u64 = BloomFilter::calculate_bit_arr_len(100 as f64, 0.01);
        assert_eq!(m, 959);
        let m2: u64 = BloomFilter::calculate_bit_arr_len(1000 as f64, 0.001);
        assert_eq!(m2, 14378);
        let m3: u64 = BloomFilter::calculate_bit_arr_len(5 as f64, 0.1);
        assert_eq!(m3, 24);
        let m4: u64 = BloomFilter::calculate_bit_arr_len(10000 as f64, 0.01);
        assert_eq!(m4, 95851);
        let m5: u64 = BloomFilter::calculate_bit_arr_len(216553 as f64, 0.01);
        assert_eq!(m5, 2075674);
    }

    #[test]
    fn test_calculate_hash_count() {
        // Test that we get the expected value for calculating the number of iterations
        // if hashing we need to perform
        // k = ceil(m/n * ln(2))
        let k: u64 = BloomFilter::calculate_hash_count(100 as f64, 959);
        assert_eq!(k, 7);
        let k2: u64 = BloomFilter::calculate_hash_count(1000 as f64, 14378);
        assert_eq!(k2, 10);
        let k3: u64 = BloomFilter::calculate_hash_count(5 as f64, 24);
        assert_eq!(k3, 4);
        let k4: u64 = BloomFilter::calculate_hash_count(10000 as f64, 95851);
        assert_eq!(k4, 7);
        let k5: u64 = BloomFilter::calculate_hash_count(216553 as f64, 2075674);
        assert_eq!(k5, 7);
    }

    #[test]
    #[should_panic]
    fn test_invalid_inserts() {
        // Test that we can't pass zero as the expected_inserts value
        // Because expected_inserts is u64, the negative case is free
        #[allow(unused_variables)]
        let bf: BloomFilter = BloomFilter::new(0, 0.001);
    }

    #[test]
    #[should_panic]
    fn test_invalid_fpr_zero() {
        // Test that we can't pass 0% as the desired false positive rate
        #[allow(unused_variables)]
        let bf: BloomFilter = BloomFilter::new(1, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_fpr_negative() {
        // Test that we can't pass a negative percentage as the desired false positive rate
        #[allow(unused_variables)]
        let bf: BloomFilter = BloomFilter::new(1, -0.03);
    }
}
