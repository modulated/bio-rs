# Bio

Bioinformatics library written to help solve the Rosalind problems.

## TODO
- replace all `panic!` macros with proper error handling and custom type

## Notes
- probability of matching Q (query) within D (database) is: 
    - p = prob of each base all mult
    - n = D.len - Q.len + 1
    - q = 1 - p
    = binom(x) = (p + q)^2
- approximate binomial with poisson 
    - if n > 100, p < 0.01 and np < 1
    - lambda = np

- Prob of L consecutive matches in Q and D
    - m = Q.len - L + 1
    - w = D.len - L + 1
    - E-value = mw0.25^L


- FASTA algo
    1. generate index table ID: HashMap<u8, Vec<usize>>
    2. generate SQ: difference Q and ID: Vec<Vec<i8>>
    3. creaete freq dist of SQ
    4. use high freq points as anchors