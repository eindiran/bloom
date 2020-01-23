# bloom
A murmur3-based implementation of a few bloom filter variants in Rust. Supports each of the following:

  * A standard [bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) (`BloomFilter`)
  * A [counting bloom filter](https://en.wikipedia.org/wiki/Counting_Bloom_filter) (`CountingBloomFilter`)
  * A [scope decay bloom filter](https://cis.temple.edu/~jiewu/research/publications/Publication_files/NAS_Li.pdf) (`ScopeDecayBloomFilter`)
  * A generational bloom filter (`GenerationalBloomFilter`), which supports an arbitrary number of generations.
  * A bigenerational bloom filter (`BigenerationalBloomFilter`), which supports 2 alternating generations.
  * An [Active-Active Buffering](https://www.researchgate.net/publication/220073582_Aging_Bloom_Filter_with_Two_Active_Buffers_for_Dynamic_Sets) bloom filter (`A2BufferingBloomFilter`)

_Note_: You probably shouldn't use this for anything since I don't really know Rust... Additionally, the library is in significant flux and the interface is not yet stable.
