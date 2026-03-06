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
4) Add a public `solve_via_lazy_backtracking` that provides the public
   interface to solving a problem.

Please use red/green TDD for this process.

# Data structure description and contents.

Our new data structure for each cell p has only one field, l(p); the other 
fields f(p), b(p), c(p) used in backtracking.rs are no longer necessary, 
nor do we need 2n + 2 special cells.   Therefore, we only need 2n + m cells.  

As before we will represent clauses sequentially in l(p), with the 
literals of Clause C_j beginning at start(j) for 1 ≤ j ≤ m. The watched literal
will be the one in start(j); and a new field, link(j), will be the number of 
another clause with the same watched
literal (or 0, if C_j is the last such clause). Moreover, our new algorithm won’t
need size(j). Instead, we can assume that the final literal of Cj is in location
start(j − 1) − 1, provided that we define start(0) appropriately.

In addition, a w(l) array is needed to hold the head of a singly linked list
of all clauses that watch a literal l.  It holds only the head because the
link array provides the rest of the watchees.

# Initializing the data structure

The purpose of the `new` procedure is to 

Consider the following SAT problem:

R' = {x_1, x_2, ¬x_3}, {x_2, x_3, ¬x_4}, {x_1, x_3, x_4},
     {x_2, ¬x_1, x_4}, {x_3, ¬x_1, ¬x_2}, {x_4, ¬x_2, ¬x_3}
     {¬x_1, ¬x_3, ¬x_4}

This problem admits two solutions: {not 1, 2, 3, 4} and {not 1, 2, not 3, 4}.
If we were to add the clause { x_1, ¬x_2, ¬x_4}, it admits no solution.

If this is provided as a SatProblem instance, the contents of the data
structure after calling `new` should be:

p    = 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
L(p) = 3 9 7 8 7 5 6 5 3 4  3  8  2  8  6  9  6  4  7  4  2
and start(j) = 21 − 3j for 0 ≤ j ≤ 7; w(2) = 3, w(3) = 7, 
w(4) = 4, w(5) = 0, w(6) = 5, w(7) = 1, w(8) = 6, w(9) = 2. 
Also link(j) = 0 for 1 ≤ j ≤ 7 in this case.
 
# `solve` 

For a non-empty set of clauses, this attempts to solve the SAT problem
in the already initialized data structure.

It records it's progress in an array m(1)..m(d) of moves described later.

## Algorithm B

The basic algorithm is:

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

Where the meanings of different values of m(d) are:
* m(j) = 0 means we’re trying x_j) = 1 and haven’t yet tried x_j = 0.
* m(j) = 1 means we’re trying x_j = 0 and haven’t yet tried x_j = 1.
* m(j) = 2 means we’re trying x_j = 1 after x_j = 0 has failed.
* m(j) = 3 means we’re trying x_j = 0 after x_j = 1 has failed.
* m(j) = 4 means we’re trying x_j = 1 when not x_j doesn’t appear.
* m(j) = 5 means we’re trying x_j = 0 when x_j doesn’t appear.

Thus, a satisfying assignment can be reconstructed from the m_d in B2 by setting
x(j) ← 1 ⊕ (mj & 1) for 1 ≤ j ≤ d.

## Watch

The subroutine `watch` does the following:

Set j ← w(not l). While j != 0, a literal other than not l should be watched in 
clause j, so we do the following: Set i ← start(j), i′ ← start(j − 1), 
j′ ← link(j), k ← i + 1. While k < i′, set l′ ← l(k); if l′ isn’t false 
(that is, if |l′| > d or l′ + m|l′ | is even, where |l′| can be computed
by l' / 2) set L(i) ← l′, L(k) ← not l, link(j) ← w(l′) , w(l′) ← j, j ← j′, 
and exit the loop on k; otherwise set k ← k + 1 and continue that loop. 
If k reaches i′, however, we cannot stop watching not l; so we set w(not l) ← j, 
exit the loop on j, and go on to step B5.

However, the way Algorithm B is structured above is problematical, because
it makes use of a complicated set of nested gotos, and rust does not support
goto.  In addition, the `watch` subroutine above is not well encapsluated
or testable.  Your task for impelementing Algorithm B is:

1) Determine how to rewrite `watch` so that it has a good, testable interface
   as a method on the struct.
2) Recast the algorithm in a way more amentable to implementation as a method,
   but be careful to try to avoid the inefficient 'loop over state enums'
   approach for efficiency.
3) Implement and test.

# Overall

Your implementation should be well factored, efficient, tested, and clear.
