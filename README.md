# Bio-rs

Bioinformatics library in Rust focusing on performance and correctness.

## Implementation Checkpoints
### Stage 1
- [ ] 2-bit compressed encoding - seperate for RNA + DNA
- [ ] Use &[u8]
- [ ] FASTA parsing
- [ ] Reverse complement
- [ ] GC content
- [ ] Mem-mapped encoding (memmap2 crate)
- [ ] FASTQ filtering (parralelise)
### Stage 2
- [ ] k-mer counting including canonical k-mers
- [ ] Hash-based minimiser index for sliding window k-mers
- [ ] Exact match finder
- [ ] Needleman-Wunsch alignment with banding
### Stage 3
- [ ] SIMD-optimised Smith-Waterman local sequence alignment
- [ ] FM-index
- [ ] BAM parsing
- [ ] VCF parsing

## TODO
- [ ] replace all `panic!` macros with proper error handling and custom type

## Performance Optimization TODOs
### Memory Allocation Optimizations
- [ ] Pre-allocate Vec capacity in complement_slice and transcribe operations
- [ ] Eliminate excessive cloning in reverse_complement (avoid String conversion)
- [ ] Reduce temporary Vec allocations in BWT implementation
- [ ] Use map_in_place for transcribe operation instead of push loops

### Algorithmic Efficiency Improvements
- [ ] Replace O(n²) BWT with suffix array-based O(n log n) algorithm
- [ ] Optimize k-mer generation to avoid recursive cloning pattern
- [ ] Share reverse complement computation across ORF reading frames
- [ ] Implement more efficient HashMap operations for k-mer counting

### SIMD and Parallelization
- [ ] Add SIMD optimizations for nucleotide counting operations
- [ ] Implement SIMD for complement operations on byte arrays
- [ ] Add SIMD for GC content calculation
- [ ] Parallelize FASTA/FASTQ file parsing with chunk processing
- [ ] Parallelize k-mer counting across large sequences
- [ ] Parallelize multiple ORF frame analysis

### Data Structure and Encoding Optimizations
- [ ] Implement 2-bit nucleotide encoding for 75% memory reduction
- [ ] Add memory-mapped I/O for large genomic files
- [ ] Eliminate String operations in performance-critical paths
- [ ] Optimize display formatting to reduce String allocations
- [ ] Consider cache-friendly data layouts for sequence operations

## Notes
- FASTA algo
    1. generate index table ID: HashMap<u8, Vec<usize>>
    2. generate SQ: difference Q and ID: Vec<Vec<i8>>
    3. creaete freq dist of SQ
    4. use high freq points as anchors
