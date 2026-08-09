We want to create some helper routines to create SAT representations of
some problems.

1) waerden(j, k, n): is there a binary sequence x_1,...,x_n that contains
   neither j equally spaced 0s nor k equally spaced 1s.
   For example, for waerden(3, 3, 8) one solution is 00110011.
   But waerden(3, 3, 9) has no solutions.
2) langford(n): Given the 2n numbers {1, 1, 2, 2, ..., n, n}, try to find
   an arrangement so that exactly k numbers appear between the two appearances
   of digit k.  For example, for n = 3, [2, 3, 1, 2, 1, 3].

Your task is to create rust functions to convert these to SAT problems.

# Description of problems

## Waerden

For j, k, n > 0, the set of clauses is

waerden(j, k, n) = {x_i or x_(i+d) or · · · or x_(i+(j−1)d)} for 1 ≤ i ≤ n − (j−1)d, d ≥ 1
 union {not x_i or not x_(i+d) or · · · or not x_(i+(k−1)d)} for 1 ≤ i ≤ n − (k−1)d, d ≥ 1 . 

## Langford

We can formulate langford as an exact cover problem that we then
translate into SAT.  For example, for n=3, we have nine items with eight options:

d_1 s_1 s_3
d_1 s_2 s_4
d_1 s_3 s_5
d_1 s_4 s_6
d_2 s_1 s_4
d_2 s_2 s_5
d_2 s_3 s_6
d_3 s_1 s_5
d_3 s_2 s_6

where d_i s_j s_k means place digit 'i' into positions 'j' and 'k'.

This can then be converted to a SAT problem  using the symmetric binary
function to specify which options can be combined, where the symmetric
binary function

S_m(y_1, ..., y_n) = exactly m of the y_i are true, the rest are false.

If we let x_1 ... x_9 be the 9 options above, then

langford(3) = S_1(x_1, x_2, x_3, x_4) and S_1(x_5, x_6, x_7) and
                S_1(x_1, x_5, x_8) and S_1(x_2, x_6, x_9) and
                S_1(x_1, x_3, x_7) and S_1(x_2, x_4, x_5) and
                S_1(x_3, x_6, x_8) and S_1(x_4, x_7, x_9) and
                S_1(x_8, x_9)

Consider S_1(x_4, x_7) -- this specifies that exactly one of
d_1 s_4 s_6 and d_2 s_3 s_6 must be chosen because chosing both
would mean both 1 and 2 would have to be placed in slot 6, and only
one number can be present. S_1(x_8, x_9) on the other hand, is present
because it requires that exactly one of d_3 s_1 s_5 and d_3 s_2 s_6 is
present, since the digit 3 must be set in exactly 2 positions, not 4.

But then we need a way to express S_1(x_1, ..., x_n).  A simple way
is to use 1 + (n choose 2) clauses:

S_1(y_1, ..., y_p) = (y_1 or ... or y_p) and {not y_j or not y_k for 1 <= j < k <= p}

# Task

Please create a new rust file `sample_problems.rs` that exports two
functions:

* pub fn waerden(j: u8, k: u8, n: u8) -> Result<SatProblem, ...>;
* pub fn langford(n: u8) -> Result<SatProblem, ...>

where you define some reasonable error for invalid inputs (j, k, n <= 0).
You should use red/green TDD with good unit tests.  Make sure you
understand the problem statements above, and ask to clarify any issues (or
if the problem statement is incorrect).
