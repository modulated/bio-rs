# Bio

Bioinformatics library written to help solve the Rosalind problems.

## TODO
- replace all `panic!` macros with proper error handling and custom type

## Notes
- FASTA algo
    1. generate index table ID: HashMap<u8, Vec<usize>>
    2. generate SQ: difference Q and ID: Vec<Vec<i8>>
    3. creaete freq dist of SQ
    4. use high freq points as anchors