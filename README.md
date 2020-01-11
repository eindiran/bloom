# bloom
A murmur3-based implementation of a few bloom filter variants in Rust. Supports each of the following:

  * A standard bloom filter (`BloomFilter`)
  * A counting bloom filter (`CountingBloomFilter`)
  * A scope decay bloom filter (`ScopeDecayBloomFilter`)

_Note_: You probably shouldn't use this for anything since I don't really know Rust...
