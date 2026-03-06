We are going to implement a lazy backtracking Algorithm, specifically
Algorithm B of TAOCP vol. 4b 7.2.2.2.

This should follow a similar API to that defined in the file backtracking.rs:

1) There is a file-internal data structure holding the current state. 
2) It has a new method that is used to instantiate the contents of the
   data structure given an input SatProblem, as defined by sat_problem.rs.
   This also defines how literals are represented.
3) It also has a self-consuming solve method that returns a solution
   assignment, if found, or None if not.

However, the data structure and algorithm are different.

Your task is the following (further instructions to follow):

1) Create the data structure in a new file, `lazy_backtracking.rs`.
2) Implement the `new` method that takes a SatProblem and initialzes
   the contents of the data structure.
3) Implement the `solve` algorithm on the data structure.
4) Add a public `solve_via_public_backtracking` that provides the public
   interface to solving a problem.

Please use red/green TDD for this process.

# Data structure description and contents.

Our new data structure for each cell p has only one field, l(p); the other 
fields f(p), b(p), c(p) used in backtracking.rs are no longer necessary, 
nor do we need 2n + 2 special cells. As before we will represent clauses 
sequentially, with the literals of Clause C_j beginning at
start(j) for 1 ≤ j ≤ m. The watched literal will be the one in start(j); and a
new field, link(j), will be the number of another clause with the same watched
literal (or 0, if C_j is the last such clause). Moreover, our new algorithm won’t
need size(j). Instead, we can assume that the final literal of Cj is in location
start(j − 1) − 1, provided that we define start(0) appropriately.

In addition, a w(l) array is needed to hold the head of a singly linked list
of all clauses that watch a literal l.  It holds only the head because the
link array provides the rest of the watchees.

# Initializing the data structure

# `solve` Algorithm

For a non-empty set of clauses, this attempts to solve the SAT problem
in the already initialized data structure.

It records it's progress in an array m(1)..m(d) of moves described later.

B1. [Initialize.] Set d ← 1.  d is the current depth.
B2. [Rejoice or choose.] If d > n, terminate successfully. Otherwise set 
  m(d) ← [w(2d) = 0 or w(2d+1) != 0] and l ← 2d + m(d).
B3. [Remove ¯ l if possible.] For all j such that not l is watched in Cj , watch another
   literal of Cj . But go to B5 if that can’t be done. Do so by executing
   the subroutine 'watch', described below.
B4. [Advance.] Set W(not l) ← 0, d ← d + 1, and return to B2.
B5. [Try again.] If m(d) < 2, set m(d) ← 3 − m(d), l ← 2d + (m(d) & 1), and go to B3.
B6. [Backtrack.] Terminate unsuccessfully if d = 1 (the clauses are unsatisfiable).
    Otherwise set d ← d − 1 and go back to B5.

