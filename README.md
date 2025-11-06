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

## Notes
- FASTA algo
    1. generate index table ID: HashMap<u8, Vec<usize>>
    2. generate SQ: difference Q and ID: Vec<Vec<i8>>
    3. creaete freq dist of SQ
    4. use high freq points as anchors
