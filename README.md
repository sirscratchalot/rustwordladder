# Simple word ladder

Learning project for Rust, some basic graph theory and depth first walks.
All words in the provided input file need to be the same length.
The program attempts to find the longest walk in a file of words, words that differ only by one letter are considered adjacent.
Provided file: knowngraph.txt can be used to start a quick test-walk.

### Build:
´cargo build --release´

### Run

´./wordladder knowngraph.txt´

### Multi threaded execution (4 threads):

´./wordladder knowngraph.txt 4´


From input:

chfa
hall
grph
ball
call
hell
erik
ekka
extr
wall
bekk
bxtr
sall
salk
walk


The output would be:

New max length vector found: 8
Longest graph identified is: 8
hell
hall
ball
call
wall
walk
salk
sall

