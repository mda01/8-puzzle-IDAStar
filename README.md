
# IDA* project for the N-puzzle game!

*My new personal project!*

I did something similar in Java in 2019, and I'm now trying to do it in Rust!
It will be the occasion for me to learn this language :D

## What is the N-puzzle game?

The N-puzzle is a sliding puzzle game played on a square grid with numbered tiles and one empty space. The most common version is the 15-puzzle, which uses a 4×4 grid with tiles numbered 1-15 and one blank space.

The goal is to arrange the tiles in numerical order (1-15 reading left to right, top to bottom) by sliding tiles into the empty space. You can only move tiles that are adjacent to the empty space, making it a constraint-based puzzle that requires strategic thinking to solve efficiently.

The puzzle is named "N-puzzle" because it can be generalized to different grid sizes - an 8-puzzle uses a 3×3 grid, a 15-puzzle uses 4×4, and so on, where N represents the highest numbered tile.

## Different approachs to solve the N-puzzle

Finding a solution to N-puzzle with the fewest number of moves has been shown to be NP-Complete, so we need to rely on heuristics to reduce the search space. I won't go too much into details, but the most common ones that perform well are the Manhattan Distance, and Linear Conflicts which is an improvement of Manhattan Distance.

The choice of algorithm will also be crucial, in order to not use too much RAM and keep a reasonable execution time. For this project, the algorithms A* and IDA* are used.

### Heuristics

#### Manhattan distance

The Manhattan distance heuristic is a popular admissible heuristic to solve the N-puzzle efficiently.

For each tile in the current puzzle state, calculate the Manhattan distance from its current position to its goal position, then sum these distances across all tiles.
Manhattan distance between two positions is the sum of the absolute differences of their coordinates:

If a tile is at position `(x₁, y₁)` and needs to be at `(x₂, y₂)`
Manhattan distance = `|x₁ - x₂| + |y₁ - y₂|`

#### Linear Conflicts

The Linear Conflicts heuristic is an enhancement to the Manhattan distance heuristic that accounts for tiles that are in their correct row or column but in the wrong order relative to each other.

Linear Conflicts = `Manhattan Distance + 2 × (number of linear conflicts)`

A linear conflict occurs when two tiles are:

1. Both in their correct row or column (but not necessarily both)
2. In the wrong order relative to each other
3. Would need to "pass by" each other to reach their goal positions

This heuristic is very powerful for solving N-puzzle!

### Algorithms

#### A*

A* is A best-first search algorithm that uses an heuristic to find optimal paths by exploring nodes with the lowest estimated total cost, maintaining all nodes in memory.

It's the fastest of the two algorithms, but may use a lot of RAM due to keeping all nodes in the memory.

#### IDA*

IDA* is an iterative deepening version of A* that performs depth-first search with successively increasing cost thresholds, using minimal memory but potentially re-exploring nodes.

This algorithm is a bit slower (30%) than A*, but uses a lot less RAM. For harder test cases, this can prove to be useful.

## How did I develop this?

### Learning Rust

For this project, I had to learn Rust, which was not too much of a challenge. The only confusing issue I faced was that variables/structs are often moved so I had to clone them. My code is probably still suboptimal due to this, but it will be improved in future releases.

### Actual implementation

I implemented most of the code myself from scratch. The only exceptions are the A* method which was partially AI-generated, and the linear conflict heuristic which is entierly AI-generated.

The usage of Claude AI proved to be very helpful for the last commits, because now that I learned how to code in Rust, it was easy to generate precisely what I needed.

I wrote unit tests along with the code, for some parts of the code I used test-driven development to ensure the methods did what was expected.

## Results

The common test cases used in this repo are solvable from 0 steps to 50 steps. these are the ones named `puzzle<XX>.txt`, XX being the minimum steps to solve the puzzle. All these test cases can be solved with the code from this repo :) The 49 and 50 take around 10s to be solved.

The `unsolvable` puzzles have not been tackled yet, but there are algorithms to determine if a N-puzzle is solvable or not.

The puzzles `4x4-hard` can be solved easily with the code from this repo :) They take between 1 and 4 seconds to be solved.

The puzzles `4x4-78` and `4x4-80` proved to be too difficult to solve only with A*/IDA* + Linear Conflicts, better heuristics such as Pattern Database (precomputed database of patterns) should be used in this case.
