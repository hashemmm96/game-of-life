# Game of life
Game of life implementation written in Rust.

Part of my journey in learning Rust.

# Pattern files
The patterns are based on the example patterns [here](http://pi.math.cornell.edu/~lipa/mec/lesson6.html).

* test-\*: Patterns used in unit tests.
* triomino1: Should end at generation 2.
* triomino2: repeats a pattern of a vertical line followed by a horizontal line.
* triomino3: Stabilizes into a square after only one generation.
* triomino4: Should end at generation 2.
* tetromino: Sequence with more generations which ends by repeating a plus and an O.
* glider: Repeats every fourth generation by moving to the northwest.
* f-pentomino: Pattern that starts repeating after 1103 generations. Good to see the expansion of the grid.