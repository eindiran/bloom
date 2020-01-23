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

impl GenerationValues {
    /// Return the generation name as a string
    pub fn as_str(&self) -> &'static str {
        match *self {
            GenerationValues::A => "A",
            GenerationValues::B => "B",
        }
    }
}

/// Type alias for GenerationValues
pub type Generation = GenerationValues;
