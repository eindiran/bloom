/**
 * scope_decay_bloom_filter.rs
 *
 * Implements a Murmur3-based scope decay bloom filter:
 *      ScopeDecayBloomFilter -- a scope decay bloom filter, supporting gradual resetting of bits
 *                               over time.
 */
use bit_vec::BitVec;
use fasthash::murmur3;
use rand::distributions::{Distribution, Uniform};

/// ScopeDecayBloomFilter
///     * bit_arr:             Bit array
///     * len:                 Length of bit array
///     * hash_count:          Hash count
///     * false_positive_rate: False positive rate
///     * bit_reset_rate:      Bit reset rate
///     * expected_inserts:    Expected inserts
///     * actual_inserts:      Actual inserts
pub struct ScopeDecayBloomFilter {
    bit_arr: BitVec,
    len: u64,
    hash_count: u64,
    false_positive_rate: f64,
    bit_reset_rate: f64,
    expected_inserts: u64,
    actual_inserts: u64,
}

/// Implementation of a standard bloom filter, using a bit array.
impl ScopeDecayBloomFilter {
    /// Getter for the hashing iteration count (hash_count)
    pub fn get_hash_count(&self) -> u64 {
        return self.hash_count;
    }

    /// Getter for the false positive rate (false_positive_rate)
    pub fn get_false_positive_rate(&self) -> f64 {
        return self.false_positive_rate;
    }

    /// Getter for the bit reset rate (bit_reset_rate)
    pub fn get_bit_reset_rate(&self) -> f64 {
        return self.bit_reset_rate;
    }

    /// Getter for the expected number of inserts (expected_inserts)
    pub fn get_expected_inserts(&self) -> u64 {
        return self.expected_inserts;
    }

    /// Getter for the current number of inserts performed (actual_inserts)
    pub fn get_actual_inserts(&self) -> u64 {
        return self.actual_inserts;
    }

    /// Given a desired false positive rate, calculate the length of the BitVec required
    /// See 'm' in this SO answer: https://stackoverflow.com/a/22467497
    /// m = ceil(-n*ln(p) / (ln(2)^2))
    fn calculate_len(expected_inserts: f64, false_positive_rate: f64) -> u64 {
        let two: f64 = 2.0;
        return ((-1.0 * (expected_inserts) * false_positive_rate.ln()) / two.ln().powf(two)).ceil()
            as u64;
    }

    /// Calculate the number of hashes required
    /// See 'k' in this SO answer: https://stackoverflow.com/a/22467497
    /// k = ceil(m/n * ln(2))
    fn calculate_hash_count(expected_inserts: f64, len: u64) -> u64 {
        let two: f64 = 2.0;
        return (((len as f64) / expected_inserts) * two.ln()).ceil() as u64;
    }

    /// Return a single usize value, representing an index to be marked or checked
    fn get_hash_index(i: u32, item: &str, len: u64) -> usize {
        let digest_val: u128 = murmur3::hash128_with_seed(item, i); // Compute a murmur3 seeded hash
        let bit_index: u64 = digest_val as u64 % len; // Mod the len of the BitVec
        return bit_index as usize;
    }

    /// Create a new ScopeDecayBloomFilter
    pub fn new(
        expected_inserts: u64,
        false_positive_rate: f64,
        bit_reset_rate: f64,
    ) -> ScopeDecayBloomFilter {
        if false_positive_rate <= 0.0 {
            panic!(
                "False positive rate must be a positive number. Currently: {}",
                false_positive_rate
            );
        } else if expected_inserts < 1 {
            panic!(
                "Expected number of inserts must be a positive number. Currently: {}",
                expected_inserts
            );
        } else if bit_reset_rate < 0.0 || bit_reset_rate > 1.0 {
            panic!(
                "Bit reset rate must be a positive number between 0.0 and 1.0 inclusive. Currently: {}",
                bit_reset_rate
            );
        }

        let len: u64 =
            ScopeDecayBloomFilter::calculate_len(expected_inserts as f64, false_positive_rate);
        let hash_count: u64 =
            ScopeDecayBloomFilter::calculate_hash_count(expected_inserts as f64, len);

        ScopeDecayBloomFilter {
            bit_arr: BitVec::from_elem(len as usize, false), // Create the whole BitVec zeroed-out
            len: len,
            hash_count: hash_count,
            false_positive_rate: false_positive_rate,
            bit_reset_rate: bit_reset_rate,
            expected_inserts: expected_inserts,
            actual_inserts: 0,
        }
    }

    /// Insert a new element into the ScopeDecayBloomFilter
    /// Will initiate a new decay event if the actual inserts exceed the expected inserts
    pub fn insert(&mut self, item: &str) {
        if self.actual_inserts > self.expected_inserts {
            self.decay();
        }
        for i in 0..self.hash_count {
            let bit_index: usize = ScopeDecayBloomFilter::get_hash_index(i as u32, item, self.len);
            self.bit_arr.set(bit_index, true); // Set the relevant index to '1'
        }
        self.actual_inserts += 1;
    }

    /// Check whether an element is probably in the filter or not
    pub fn check(&self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let bit_index: usize = ScopeDecayBloomFilter::get_hash_index(i as u32, item, self.len);
            // Check if the relevant index is set
            if !self.bit_arr[bit_index as usize] {
                return false;
            }
        }
        return true;
    }

    /// Decay n bits randomly from the bit array
    /// Will NOT retry when hiting a zeroed out bit
    fn decay_nbits(&mut self, nbits: u64) {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..self.len);
        let mut decay_counter: u64 = 0;
        while decay_counter < nbits {
            let throw = die.sample(&mut rng);
            self.bit_arr.set(throw as usize, false);
            decay_counter += 1;
        }
    }

    /// Decay using the bit reset rate
    pub fn decay(&mut self) {
        if !self.bit_arr.none() {
            let nbits = self.bit_reset_rate * self.len as f64;
            self.decay_nbits(nbits as u64);
        }
    }

    /// Emtpy out the ScopeDecayBloomFilter
    pub fn empty(&mut self) {
        self.bit_arr.clear();
        self.actual_inserts = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that we can create a new ScopeDecayBloomFilter using ScopeDecayBloomFilter::new()
    /// and that all getters work and return the expected values
    fn test_new() {
        let bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(3, 0.05, 0.1);
        assert_eq!(bf.get_expected_inserts(), 3);
        assert_eq!(bf.get_actual_inserts(), 0);
        assert_eq!(bf.get_false_positive_rate(), 0.05);
        assert!(bf.get_hash_count() > 0);
    }

    #[test]
    /// Test that we can insert a string into a ScopeDecayBloomFilter
    fn test_insert() {
        let s = "This is a test string"; // Inserted
        let s2 = "This is another string"; // Inserted
        let s3 = "This is a third string"; // Not inserted
        let mut bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(2, 0.05, 0.1);
        bf.insert(&s);
        bf.insert(&s2);
        assert!(bf.check(&s)); // Included
        assert!(bf.check(&s2)); // Included
        assert!(!bf.check(&s3)); // Not included
    }

    #[test]
    /// Test that the get_hash_index method works as expected
    fn test_get_hash_index() {
        let s = "This is a test string";
        let i: u32 = 32;
        assert_eq!(ScopeDecayBloomFilter::get_hash_index(i, &s, 10), 4);
    }

    #[test]
    /// Test that the actual_inserts counter works as expected
    fn test_count_actual_inserts() {
        let mut bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(100, 0.05, 0.1);
        assert_eq!(bf.get_actual_inserts(), 0);
        for i in 1..100 {
            bf.insert(&i.to_string());
            assert_eq!(bf.get_actual_inserts(), i);
        }
    }

    #[test]
    /// Test that check behaves like we expect
    fn test_check() {
        let mut bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(100, 0.05, 0.1);
        for i in 1..100 {
            bf.insert(&i.to_string());
        }
        for j in 1..100 {
            assert!(bf.check(&j.to_string()));
        }
        let mut false_positives: u64 = 0;
        for k in 101..200 {
            if bf.check(&k.to_string()) {
                false_positives += 1;
            }
        }
        assert!(false_positives < 6); // Slightly more than 5%
    }

    #[test]
    /// Test decay
    fn test_decay() {
        let mut bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(100, 0.05, 0.1);
        for i in 1..100 {
            bf.insert(&i.to_string());
        }
        let old_count = bf.bit_arr.iter().filter(|x| *x).count();
        bf.decay();
        assert!(old_count > bf.bit_arr.iter().filter(|x| *x).count());
    }

    #[test]
    /// Test that empty behaves like we expect
    fn test_empty() {
        let mut bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(100, 0.05, 0.1);
        for i in 1..100 {
            bf.insert(&i.to_string());
        }
        for j in 1..100 {
            assert!(bf.check(&j.to_string()));
        }
        bf.empty();
        for k in 1..100 {
            assert!(!bf.check(&k.to_string()));
        }
    }

    #[test]
    /// Test that we get the expected value for calculating the length of the
    /// BitVec using the computation:
    /// m = ceil(-n*ln(p) / (ln(2)^2))
    fn test_calculate_len() {
        let m: u64 = ScopeDecayBloomFilter::calculate_len(100 as f64, 0.01);
        assert_eq!(m, 959);
        let m2: u64 = ScopeDecayBloomFilter::calculate_len(1000 as f64, 0.001);
        assert_eq!(m2, 14378);
        let m3: u64 = ScopeDecayBloomFilter::calculate_len(5 as f64, 0.1);
        assert_eq!(m3, 24);
        let m4: u64 = ScopeDecayBloomFilter::calculate_len(10000 as f64, 0.01);
        assert_eq!(m4, 95851);
        let m5: u64 = ScopeDecayBloomFilter::calculate_len(216553 as f64, 0.01);
        assert_eq!(m5, 2075674);
    }

    #[test]
    /// Test that we get the expected value for calculating the number of iterations
    /// if hashing we need to perform
    /// k = ceil(m/n * ln(2))
    fn test_calculate_hash_count() {
        let k: u64 = ScopeDecayBloomFilter::calculate_hash_count(100 as f64, 959);
        assert_eq!(k, 7);
        let k2: u64 = ScopeDecayBloomFilter::calculate_hash_count(1000 as f64, 14378);
        assert_eq!(k2, 10);
        let k3: u64 = ScopeDecayBloomFilter::calculate_hash_count(5 as f64, 24);
        assert_eq!(k3, 4);
        let k4: u64 = ScopeDecayBloomFilter::calculate_hash_count(10000 as f64, 95851);
        assert_eq!(k4, 7);
        let k5: u64 = ScopeDecayBloomFilter::calculate_hash_count(216553 as f64, 2075674);
        assert_eq!(k5, 7);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass zero as the expected_inserts value
    /// Because expected_inserts is u64, the negative case is free
    fn test_invalid_inserts() {
        #[allow(unused_variables)]
        let bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(0, 0.001, 0.0);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass 0% as the desired false positive rate
    fn test_invalid_fpr_zero() {
        #[allow(unused_variables)]
        let bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(1, 0.0, 0.0);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass a negative percentage as the desired false positive rate
    fn test_invalid_fpr_negative() {
        #[allow(unused_variables)]
        let bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(1, -0.03, 0.0);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass a negative percentage as the desired bit reset rate
    fn test_invalid_brr_negative() {
        #[allow(unused_variables)]
        let bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(1, 0.1, -0.7);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass a bit reset rate > 1.0
    fn test_invalid_brr_large() {
        #[allow(unused_variables)]
        let bf: ScopeDecayBloomFilter = ScopeDecayBloomFilter::new(1, 0.1, 33.7);
    }
}
