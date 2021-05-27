# dive-shannon
Calculate Shannon's entropy a site in MSA.

## Description  
* Calculates Shannon`s entropy[1] per site in Multiple Sequence Alignment.  

## Dependencies 
``` 
[dependencies]
colored = "2.0" 
``` 

## Compilation 
You can compile the program with `Cargo`. 

[e.g.]  

``` 
% cd dive-shannon-main
% build --release
```

## Input file format 
Aligned Multi-FASTA format in amino acid seqeuences. 

See the example files in `demo` directory. 

## Useage 
* `-i` : The input file name. 
* `-o` : The output file name.

[e.g.] 

``` 
% ./dive-shannon -i input.fasta -o output.txt -c no 
``` 
Type `-h` to see another available options. 

## References 
1. Shannon, Claude E. "A mathematical theory of communication." The Bell system technical journal 27.3 (1948): 379-423.
