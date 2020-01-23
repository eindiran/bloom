/*!
 * generations.rs
 *
 * Implements a generation enum and a Generation type alias for use with Generational Bloom
 * Filter variants.
 */

/// Create an enum to control which generation of bloom filter
/// is the active or current generation.
#[derive(PartialEq, Debug)]
pub enum GenerationValues {
    A,
    B,
}

/// Type alias for GenerationValues
pub type Generation = GenerationValues;
