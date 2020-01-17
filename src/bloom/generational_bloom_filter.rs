/**
 * generational_bloom_filter.rs
 *
 * Implements a Murmur3-based generational bloom filter:
 *      GenerationalBloomFilter  -- a generational bloom filter, which is essentially 2 bloom filters
 *                                  used together to provide alternating "generations". The older
 *                                  generation is recycled when the newer generation becomes full,
 *                                  in a pattern similar to Blue-Green deployment.
 *                                  Does ~not~ support manual deletion.
 */
use crate::bloom::BloomFilter;

/// Create an enum to control which generation of bloom filter
/// is the active or current generation.
#[derive(PartialEq, Debug)]
enum GenerationValues {
    A,
    B,
}

/// Type alias for the generation values
type Generation = GenerationValues;

/// GenerationalBloomFilter struct:
///    * generation_a:        A single generation's BloomFilter, owned ptr
///    * generation_b:        A single generation's BloomFilter, owned ptr
///    * current_gen:         Annotate which generation is the current generation for inserts
///    * hash_count:          Hash count
///    * false_positive_rate: False positive rate
///    * expected_inserts:    Expected inserts per generation
///    * actual_inserts:      Actual inserts, for the current generation
pub struct GenerationalBloomFilter {
    generation_a: BloomFilter,
    generation_b: BloomFilter,
    current_gen: Generation,
    hash_count: u64,
    false_positive_rate: f64,
    expected_inserts: u64,
    actual_inserts: u64,
}

/// Implementation of a generational bloom filter
impl GenerationalBloomFilter {
    /// Getter for hash_count
    pub fn get_hash_count(&self) -> u64 {
        return self.hash_count;
    }

    /// Getter for false_positive_rate
    pub fn get_false_positive_rate(&self) -> f64 {
        return self.false_positive_rate;
    }

    /// Getter for expected_inserts
    pub fn get_expected_inserts(&self) -> u64 {
        return self.expected_inserts;
    }

    /// Getter for actual_inserts
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

    /// Create a new BloomFilter
    pub fn new(expected_inserts: u64, false_positive_rate: f64) -> GenerationalBloomFilter {
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
        }

        let len: u64 =
            GenerationalBloomFilter::calculate_len(expected_inserts as f64, false_positive_rate);
        let hash_count: u64 =
            GenerationalBloomFilter::calculate_hash_count(expected_inserts as f64, len);

        GenerationalBloomFilter {
            generation_a: BloomFilter::new(expected_inserts, false_positive_rate),
            generation_b: BloomFilter::new(expected_inserts, false_positive_rate),
            current_gen: Generation::A,
            hash_count: hash_count,
            false_positive_rate: false_positive_rate,
            expected_inserts: expected_inserts,
            actual_inserts: 0,
        }
    }

    /// Recycle & switch the current/active generation
    fn recycle(&mut self) {
        if self.current_gen == Generation::A {
            self.generation_b.empty();
            self.current_gen = Generation::B;
        } else if self.current_gen == Generation::B {
            self.generation_a.empty();
            self.current_gen = Generation::A;
        } else {
            panic!(
                "Encountered illegal value for current_gen: {:?}",
                self.current_gen
            );
        }
    }

    /// Insert a new element into the current generation BloomFilter
    pub fn insert(&mut self, item: &str) {
        if self.actual_inserts + 1 > self.expected_inserts {
            // Recycle the previous generation
            self.recycle();
            self.actual_inserts = 0;
        }
        if self.current_gen == Generation::A {
            self.generation_a.insert(item);
        } else if self.current_gen == Generation::B {
            self.generation_b.insert(item);
        } else {
            panic!(
                "Encountered illegal value for current_gen: {:?}",
                self.current_gen
            );
        }
        self.actual_inserts += 1;
    }

    /// Check whether an element is probably in the filter or not
    pub fn check(&self, item: &str) -> bool {
        return self.generation_a.check(item) || self.generation_b.check(item);
    }

    /// Empty out the entire data structure (ie both generations)
    pub fn empty(&mut self) {
        self.recycle();
        self.recycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that we can create a new GenerationalBloomFilter using GenerationalBloomFilter::new()
    /// and that all getters work and return the expected values
    fn test_new() {
        let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(3, 0.05);
        assert_eq!(bf.get_expected_inserts(), 3);
        assert_eq!(bf.get_actual_inserts(), 0);
        assert_eq!(bf.get_false_positive_rate(), 0.05);
        assert!(bf.get_hash_count() > 0);
    }

    #[test]
    /// Test that we can insert a string into a GenerationalBloomFilter
    fn test_insert() {
        let s = "This is a test string"; // Inserted
        let s2 = "This is another string"; // Inserted
        let s3 = "This is a third string"; // Not inserted
        let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(2, 0.05);
        bf.insert(&s);
        bf.insert(&s2);
        assert!(bf.check(&s)); // Included
        assert!(bf.check(&s2)); // Included
        assert!(!bf.check(&s3)); // Not included
    }

    #[test]
    /// Test that the actual_inserts counter works as expected
    fn test_count_actual_inserts() {
        let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(100, 0.05);
        assert_eq!(bf.get_actual_inserts(), 0);
        for i in 1..100 {
            bf.insert(&i.to_string());
            assert_eq!(bf.get_actual_inserts(), i);
        }
    }

    #[test]
    /// Test that check behaves like we expect
    fn test_check() {
        let mut bf: GenerationalBloomFilter = GenerationalBloomFilter::new(100, 0.05);
        for i in 1..100 {
            bf.insert(&i.to_string());
        }
        for g in 1..100 {
            assert!(bf.check(&g.to_string()));
        }
        let mut false_positives: u64 = 0;
        for h in 101..200 {
            if bf.check(&h.to_string()) {
                false_positives += 1;
            }
        }
        assert!(false_positives < 6); // Slightly more than 5%
    }

    #[test]
    /// Test that we get the expected value for calculating the length of the
    /// BitVec using the computation:
    /// m = ceil(-n*ln(p) / (ln(2)^2))
    fn test_calculate_len() {
        let m: u64 = GenerationalBloomFilter::calculate_len(100 as f64, 0.01);
        assert_eq!(m, 959);
        let m2: u64 = GenerationalBloomFilter::calculate_len(1000 as f64, 0.001);
        assert_eq!(m2, 14378);
        let m3: u64 = GenerationalBloomFilter::calculate_len(5 as f64, 0.1);
        assert_eq!(m3, 24);
        let m4: u64 = GenerationalBloomFilter::calculate_len(10000 as f64, 0.01);
        assert_eq!(m4, 95851);
        let m5: u64 = GenerationalBloomFilter::calculate_len(216553 as f64, 0.01);
        assert_eq!(m5, 2075674);
    }

    #[test]
    /// Test that we get the expected value for calculating the number of iterations
    /// if hashing we need to perform
    /// k = ceil(m/n * ln(2))
    fn test_calculate_hash_count() {
        let k: u64 = GenerationalBloomFilter::calculate_hash_count(100 as f64, 959);
        assert_eq!(k, 7);
        let k2: u64 = GenerationalBloomFilter::calculate_hash_count(1000 as f64, 14378);
        assert_eq!(k2, 10);
        let k3: u64 = GenerationalBloomFilter::calculate_hash_count(5 as f64, 24);
        assert_eq!(k3, 4);
        let k4: u64 = GenerationalBloomFilter::calculate_hash_count(10000 as f64, 95851);
        assert_eq!(k4, 7);
        let k5: u64 = GenerationalBloomFilter::calculate_hash_count(216553 as f64, 2075674);
        assert_eq!(k5, 7);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass zero as the expected_inserts value
    /// Because expected_inserts is u64, the negative case is free
    fn test_invalid_inserts() {
        #[allow(unused_variables)]
        let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(0, 0.001);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass 0% as the desired false positive rate
    fn test_invalid_fpr_zero() {
        #[allow(unused_variables)]
        let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(1, 0.0);
    }

    #[test]
    #[should_panic]
    /// Test that we can't pass a negative percentage as the desired false positive rate
    fn test_invalid_fpr_negative() {
        #[allow(unused_variables)]
        let bf: GenerationalBloomFilter = GenerationalBloomFilter::new(1, -0.03);
    }
}
