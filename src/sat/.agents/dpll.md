Now we are going to extend the lazy backtracking algorithm described
in `.agents/lazy.md` and implemented in `lazy_backtracking.rs` to also
take advantage of forced literals, making a DPLL solver.  This is Algorithm
D of Knuth 4B 7.2.2.2.

# Task

This should follow a similar API to that defined in the file
`lazy_backtracking.rs`.

1) There is a file-internal data structure holding the current state. 
2) It has a new method that is used to instantiate the contents of the
   data structure given an input SatProblem, as defined by sat_problem.rs.
   This also defines how literals are represented.
3) It also has a self-consuming solve method that returns a solution
   assignment, if found, or None if not.

However, the data structure and algorithm are different.

Your task is the following (further instructions to follow):

1) Create the data structure in a new file, `dpll.rs`.
2) Implement the `new` method that takes a SatProblem and initialzes
   the contents of the data structure.
3) Implement the `solve` algorithm on the data structure.
4) Add a public `solve_via_lazy_backtracking` that provides the public
   interface to solving a problem.

Please use red/green TDD for this process.

# Changes in the data structure and algorithm

The algorithm is based on the same lazy datastructure as `lazy_backtracking.rs`,
in particular the idea of watching literals, but with a few additions:

1. The algorithm looks for unit clauses before deciding what variable to
   branch on, since unit clauses force a particular choice.
2. Rather than always considering the variables in order during the search,
   the algorithm chooses between the available variables.

The data structures are extended by introducing indices h_1, ..., h_n so that
the variable whose value is being set at depth d is x_h_d rather than x_d.

The not yet set variables whose watch lists are not empty are kept in a
circular list called the active ring; the idea is that when it is deciding
what variable to branch on, it first checks through the active ring to try
to find any unit clauses, and only if we go all the way around the ring
without finding one do we resort to 2-way branching.  The active ring
is represented by associating a `next` field with each variable, with
x_next(k) the successor to x_k in the ring.  The ring is accessed via 'head'
and 'tail' pointers h and t at the left and right, with h = next(t).
If the ring is empty, t = 0 and h is undefined.

# Initializing the data structure

The basic set up of the data structure, ignoring the active ring, is as
in the lazy backtracking algorithm.  You can either copy that from
`lazy_backtracking.rs` or implement from scratch; however, we may want to
change it in the future, so don't factor it into a common library called by both.

In addition, there is some initialization for the new components described
in step D1 below.

# DPLL algorithm

The algorithm is as follows:

If the variables are x_1 ... x_n, represented with lazy data structures and an 
active ring as explained above, this algorithm finds a solution if and only if 
the clauses are satisfiable. It records its current progress in an array 
h_1 ... h_n of indices and an array m_0 ... m_n of “moves,” whose significance 
is explained below.

D1. [Initialize.] Set m0 ← d ← h ← t ← 0, and do the following for 
  k = n, n − 1, ... , 1: Set x_k ← −1 (denoting an unset value); if w_{2k} != 0 
  or w_{2k+1} ̸= 0, set next(k) ← h, h ← k, and if t = 0 also set t ← k. 
  Finally, if t != 0, complete the active ring by setting NEXT(t) ← h.
D2. [Success?] Terminate if t = 0 (all clauses are satisfied) and return the
  solution. Otherwise set k ← t.
D3. [Look for unit clauses.] Set h ← next(k) and use the subroutine
  `check_unit_clauses` f ← [2h is a unit] + 2[2h + 1 is a unit]. If f = 3, go
  to D7. If f = 1 or 2, set md+1 ← f + 3, t ← k, and go to D5. Otherwise, if
  h != t, set k ← h and repeat this step.
D4. [Two-way branch.] Set h ← next(t) and m_{d+1} ← [w_{2h} = 0 or w_{2h+1} != 0].
D5. [Move on.] Set d ← d + 1, h_d ← k ← h. If t = k, set t ← 0; otherwise 
  delete variable k from the ring by setting next(t) ← h ← next(k).
D6. [Update watches.] Set b ← (md + 1) mod 2, x_k ← b, and clear the watch list
  for ¬x_k by calling the subroutine `clear_watch_lists`.  Return to D2.
D7. [Backtrack.] Set t ← k. While m_d ≥ 2, set k ← h_d, x_k ← −1; if W_{2k} != 0 or
W_{2k+1} != 0, set next(k) ← h, h ← k, next(t) ← h; and set d ← d − 1.
D8. [Failure?] If d > 0, set m_d ← 3 − m_d, k ← h_d, and return to D6.

The move codes have the following meanings:
* m_j = 0 means we’re trying x_h_j = 1 and haven’t yet tried x_h_j = 0.
* m_j = 1 means we’re trying x_h_j = 0 and haven’t yet tried x_h_j = 1.
* m_j = 2 means we’re trying x_hj = 1 after x_h_j = 0 has failed.
* mj = 3 means we’re trying x_h_j = 0 after x_h_j = 1 has failed.
* mj = 4 means we’re trying x_h_j = 1 because it’s forced by a unit clause.
* mj = 5 means we’re trying x_h_j = 0 because it’s forced by a unit clause.

In step D2, you will have to use these to construct the solution vector to
return.

## `check_unit_clauses`

The purpose of this subroutine is to return 1 or 0 according to whether or not
literal l is or is not being watched in some clause whose literals are entirely
false -- that is, it returns 1 if the literal is in a unit clause.

An implementation is:
Set j ← w(l), then do the following steps while j != 0: 
(i) Set p ← start(j) + 1;
(ii) if p = start(j − 1), return 1; 
(iii) if l(p) is false (that is, if x_|L(p)| = L(p) & 1), set p ← p + 1 and 
  repeat (ii);
(iv) set j ← link(j).  
If j becomes zero, return 0.

## `clear_watch_lists`

The purpose of this subroutine is to clear the watch lists for the
variable ¬x_k in step D6.

An implementation is
Set l ← 2k + b, j ← w(l), w(l) ← 0, and do the following steps while j != 0: 
(i) Set j′ ← link(j), i ← start(j), p ← i + 1; 
(ii) while L(p) is false, set p ← p + 1 (see `check_unit_clauses`; this loop 
  will end before p = start(j − 1)); 
(iii) set l′ ← l(p), l(p) ← l, l(i) ← l′;
(iv) set p ← w(l′) and q ← w(¬l′) , and go to (vi) if p != 0 or q != 0 or 
  x_|l′ | ≥ 0;
(v) if t = 0, set t ← h ← |l′| and next(t) ← h, otherwise set next(|l′ |) ← h, 
  h ← |l′|, next(t) ← h (thus inserting |l′| = l′ ≫ 1 into the ring as its 
  new head); 
(vi) set link(j) ← p, w(l′) ← j (thus inserting j into the watch list of l′); 
(vii) set j ← j′.

The tricky part here is to remember that t can be zero in step (v).

Recall that |l| means the variable referred to be literal l, e.g., 
x_{l >> 1} (or, equivalently, x_{l / 2}), and ¬l, the literal representing
the same variable but notted, can be found by l xor 1.

However, as was the case for Algorithm B, the way Algorithm D is structured 
above is problematical, because it makes use of a complicated set of nested
gotos, and rust does not support goto.   Some other notes:

* Some of the interfaces are clunky, such as returning 1 or 0 from 
  `check_unit_clauses` instead of something more explicit.  You should consider
  reformulating them to be cleaner.
* It isn't clear if it is really necessary to set x_k <- -1 at various stages.
  Since doing so would require x_k to not be represented as a vector of booleans,
  if it isn't necessary it would be preferable not to do so.

# Methods on the data structure

* `new`:  The purpose of the `new` procedure is to set up the data 
  structures for a particular SAT problem.
* `solve`:  For a non-empty set of clauses, this attempts to solve the SAT problem
  in the already initialized data structure.

# Task
1) Analyze the subroutine interfaces and look for an opportunity to make them
   cleaner, clearer, and easier to test.
2) Recast the algorithm in a way more amentable to implementation as a method,
   but be careful to try to avoid the inefficient 'loop over state enums'
   approach for efficiency.
3) Implement and test.

Your implementation should be well factored, efficient, tested, and clear.
