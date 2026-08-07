This project is to explore the use of various basic SAT solvers following
Knuth Volume 4B.  It is written in rust, and uses jj for source control.

The first step is to set up some basic data structures to represent SAT
problems, and handle input and output.

A SAT problem consists of `n` variables arranged into `m` clauses.

* Variables are written `x_1` ... `x_n`, inclusive in a tex-like notation.
* A literal represents a variable or it's negation, where negation is
  represented where negation is represented by placing a bar over the
  number.  Thus, for example, `1` represents the literal for the 
  variable `x_1`, and `1̅` is the literal for the negation of
  `x_1` (`not x_1`)
* A clause consists of some number of distinct literals ored together.
  For example, the clause representing `not x_1 or x_2 or x_3`
  can be represented as the set of literals `1̅ 2 3`.
* The SAT problem is the anding together of `m` such clauses.
  The overall task is to find some setting of `x_1`...`x_n` such that
  every clause is true.

We are going to develop implementations to represent 
individual clauses and SAT problems.
* Clauses should be represented by a rust struct with the following 
  properties:
    * The literal for $x_i$ is represented by the number $2 i$.
    * The literal for $not x_i$ is represented by the number $2 i + 1$.
    * Each clause should internally store the literals in sorted order.
      There is no need to add or remove literals, and we should avoid
      the overhead of supporting that.  Therefore, the clause representing
      `not x_1 or x_2 or x_3` would be represented by the literal values
      [3, 4, 6].
    * The empty clause is possible.
    * There should be a new function to create new clauses from
      a slice of literals.  It should return an error if the literals
      are not distinct.
    * There should be an implementation of Debug and Display
      that show the literals in order using the number/number with
      bar notation.  For example, the clause representing 
      `not x_1 or x_2 or x_3` would be displayed as `{1̅ 2 3}`.
    * There should be a method to return the number of literals in the
      clause.
    * Automatically implement PartialEq, Eq, and Hash.
* A SAT problem should be represented as a struct containing a (possibly
    empty) list of clauses.
    * There should be a constructor that takes a slice of clauses.
    * There should be a constructor that takes a slice of vecs of literals,
      returning an error if clause construction fails.
    * There should be a method to display the number of clauses.
    * There should be implementations of Debug and Display that take
      advantage of the Debug and Display implementations of Clause.
    * There should also be a display_latex method that returns a
      tex formula representing the formula.  For example:
      $\left(x_1 \vee {\bar x}_2 \vee x_4\right) \wedge 
       \left(x_1 \vee {\bar x}_2 \vee {\bar x}_4\right))$.

Your task is to create a file `sat_problem.rs` that will contain
representations of SAT problems and clauses.
Your implementation should include good unit tests and documentation.