# CEGAR Algorithm

This file describes how to use CEGAR to detect whether not a given graph
has a hamiltonian cycle.  It is taken from Knuth Fascicle 8a (Hamiltonian paths).

> **This file is a faithful transcription and keeps Knuth's conventions.**  The
> implementation adapts them in two places, both documented in plan.md: variables
> are numbered from 0 rather than from 2 (rustsat's `Var` is 0-based and we do not
> need `0` as a sentinel, since arcs are looked up with `graph.find_edge` instead
> of a dense `ADJ` matrix), and literals are `rustsat::types::Lit` rather than raw
> `2k` / `2k+1` integers.  The ⊕1 pairing of `uv` with `vu` is preserved, because
> the asymmetry clauses and the cut clauses both depend on it.  Where a step below
> says `l ⊕ 2`, the implementation flips bit 0 of the *variable* index.

## Basic principle

The basic principle is to use a SAT solver to find a cycle cover of a graph --
a set of one or more cycles in which each vertex appears exactly once.  A
Hamiltonian cycle is then a cycle cover with only one element.

If there is no cycle cover, the graph clearly doesn't have a Hamiltonian cycle.
And if we find a single element cover, that's clearly the Hamiltonian cycle.
But often we are working with multiple cycles, and so we need a way to use
that information to make progress towards finding a single cycle.

If C is a cycle that doesn't include every vertex, a Hamiltonian cycle must
include an arc from a vertex in C to a vertex not in C.  So if we add clauses
that require at least one such arc, and run the SAT solver again (incrementally,
keeping clauses it has already added) and keep doing this we will eventually
find a Hamiltonian cycle or conclude there isn't one.

Therefore, we begin with a set of clauses representing the cycle cover
(cycle_cover(G)) then successively add cut clauses to it until we either find
the Hamiltonian cycle or exclude it: 
  cycle_cover(G) and cut(C_1) and cut(C_2) and ...

## Representation 

The input Graph is undirected, but we will internally consider directed
arcs since we are constructing an oriented path. 

### Cut clauses

Cut clauses for a cycle C (that doesn't include all vertices) are fairly obvious:

cut(C) = {uv | u ∈ C, v / ∈ C, u -> v}

### Clauses encoding the cycle cover

Given an undirected Graph G, we encode the cover requirements as

1. Asymmetry clauses that say that for any edge u <-> v in G we can't have
   both u -> v and v -> u in the cover.  This prevents cycles of length 2.
2. At-least-once clauses that say at least one arc leads in to each vertex,
   and that one leads out; recall we are finding Hamiltonian cycles rather
   than paths.
3. The at-most-one clauses which say tht at most one arc leads in to each
   vertex, and at most one leads out.

For example, consider the graph A <-> B <-> C.  We represent each ordered
edge A -> B as a pair AB.

1. There is one clause per edge: (not AB or not BA) and (not BC or not CB).
2. There are two at-least-once clauses per vertex, one for outgoing and one
   for ingoing.  For vertex A the ingoing clause is (BA) and the ingoing is 
   (AB).  For the entire graph, the clauses are: 
   (AB) and (BA)  and (BA or BC) and (AB or CB) and (CB) and (BC)
3. Representing the at-most is more complex.  We shall use d \choose 2
   binary clauses for each vertex (where d is the degree of the vertex) that
   forbid both of each pair of vertices.  For the above graph, only vertex B is
   interesting, and the clauses are (not AB or not BC) for ingoing and
   (not BA or not BC) for outgoing.  Other representations of the
   at-most-one clauses are possible, but the binary representation works well. 

### Merging cycle covers

While the above process is sufficient to find cycle covers, significant
improvements can be made if we include the ability to merge cycles.  We can
merge cycles whenever two adjacent vertices of one cycle are respectively
adjacent to two vertices of another cycle.

What data structures make it easy to do this? If there are n vertices, numbered 
0 to n − 1, it turns out to be convenient to have two arrays, SUCC[v] and 
PRED[v], for 0 ≤ v < n, indicating the next or previous vertex in v’s cycle, as
well as an array CID[v] to tell the number of the cycle to which v belongs. 
(Cycles are numbered 1, 2, . . . , t.) For example, we might have 3 cycles
represented as:

      v = 0  1  2  3  4  5  6  7  8  9 10 11 12
NAME[v] = A  B  C  D  E  F  G  H  I  J  K  L  M
SUCC[v] = 2 11  6  0  8 10  3  5  1  4 12  9  7
PRED[v] = 3  8  0  6  9  7  2 12  4 11  5  1 10
CID[v] =  1  2  1  1  2  3  1  3  2  2  3  2  3

here NAME[v] are arbitrary names attached to each vertex.

Two arrays provide a sparse-set representation of the currently active cycles,
which are CYC[0], . . . , CYC[t − 1]; CLOC[c] is the current location of cycle 
c in the CYC array. We also keep track of HEAD[c], an arbitrary vertex in 
cycle c.

The merging process absorbs one cycle into another, after which the absorbed 
cycle essentially disappears. For example, the above will be accompanied by the
values CYC[0] = 1, CYC[1] = 2, CYC[2] = 3, t = 3, CLOC[1] = 0, CLOC[2] = 1,
CLOC[3] = 2, HEAD[1] = 0, HEAD[2] = 1, HEAD[3] = 5, before C1 and C2 are
combined to form C′ 1. But afterwards we’ll have CYC[0] = 1, CYC[1] = 3, t = 2,
CLOC[1] = 0, CLOC[3] = 1, SUCC[0] = 1, SUCC[8] = 2, PRED[1] = 0, PRED[2] =
8; and all occurrences of ‘2’ in the CID array will have been changed to ‘1’.

### Clause conventions

If the given graph has m edges, there are 2m Boolean variables in our SAT
clauses. It will be convenient to number them from 2 to 2m + 1, not from 1
to 2m and not from 0 to 2m − 1, because 
(i) 0 is useful as a special “sentinel” value; 
(ii) we can arrange things so that variable uv is number k if and only if
     variable vu is number k ⊕ 1. 
     
We shall work with an adjacency matrix ADJ, whose rows and columns are
indexed by vertices of the graph. If u is not adjacent to v, ADJ[u][v] = 0;
otherwise ADJ[u][v] is the number of the Boolean variable uv that corresponds
to the arc u -> v.

Recall that we represent the literlas for the boolean variable k were
repsented internally by the numbers 2k and 2k + 1, where literal 2k was 
“positive” and literal 2k + 1 was “negative.” Thus the 4m possible
literals are numbered from 4 to 4m + 3. 

## Algorithm

The heart of the algorithm is the following CEGAR cut algorithm.  Before
executing it, however, it is wise to check some basic preconditions:

1) The graph is connected and nonempty.
2) Each vertex is at least of degree 2.
3) The graph can contain no bridges or cut vertices.

The latter is more complicated. Since the focus of this project is on CEGAR
it is okay to use existing implementations for these checks if they are
available on the graph library rather than implementing from scratch.

petgraph 0.8.3 provides all three: `algo::connected_components`, `algo::bridges`
and `algo::articulation_points`.  Two further checks belong here that Knuth has
no reason to mention, because they are artifacts of the graph type rather than of
the mathematics: `UnGraph` permits **self-loops** and **parallel edges**, and both
break the arc-variable mapping — a self-loop gets two variables naming one vertex,
and parallel edges get independent variables for a single adjacency.

Each of these failures genuinely proves that no Hamiltonian cycle exists, so the
implementation reports `Ok(None)` rather than an error, recording which check
fired.  A configuration flag skips the checks entirely, so that tests can drive
otherwise-rejected graphs through the SAT path, and so that the checks' value can
be measured rather than assumed.

When the query is a Hamiltonian *path*, these checks run on the reduced graph G′,
not on G.  That is not merely convenient but necessary: G may well be disconnected
or have cut vertices while G′ does not, and it is G′ that the solver sees.  It is
also sound, since any graph with a Hamiltonian cycle is 2-connected, so an
obstruction in G′ does prove that G has no Hamiltonian path.

### Algorithm C (CEGAR cuts)

Given a graph G and a SAT solver, this algorithm either finds a Hamiltonian 
cycle or proves that G doesn’t have any.

C1. [Initialize.] Set ADJ[u][v] = 0 for 0 ≤ u, v < n, where n is the number of
    vertices. Then set N ← 2 and, for each u -> v with u < v, set 
    ADJ[u][v] = N , ADJ[v][u] ← N + 1, N ← N + 2. (The Boolean variables 
    are [2 .. N).
C2. [Create basic clauses.] Contribute the asymmetry clauses, the at-least-one
    clauses, and the at-most-one clauses to the solver, as described above, thus
    specifying the constraints of a cycle cover.
C3. [Solve.] Run the solver. If the clauses are unsatisfiable, stop the algorithm
    with no solution found.
C4. [Find the cycles.] For each true variable uv in the solution, set SUCC[u] = v
    and PRED[v] = u. Also set CID[v] = 0 for 0 ≤ v < n. Then set t = v = 0,
    and do the following while v < n: “If CID[v] = 0, t = t+1, CYC[t − 1] = t,
    CLOC[t] = t − 1, HEAD[t] = v, CID[v] = t, u ← SUCC[v], and repeatedly
    set CID[u] = t, u = SUCC[u] until u = v. Then set v = v + 1.”
C5. [Done?] Stop (successfully) if t = 1, because SUCC defines an n-cycle.
    Return the cycle found.
C6. [Merge?] Use the algorithm (detailed below) to merge adjacent cycles and
    reduce t, until no further merging is possible.
C7. [Done?] Stop (successfully) if t = 1, because SUCC defines an n-cycle.
    Return the cycle.
C8. [Add cut clauses.] Contribute the clauses Cut(C_j) to the solver for 
    0 ≤ j < t, where C_j denotes the vertices of CYC[j]. If t > 2, also 
    contribute Cut(complement C_j), for 0 ≤ j < t. Stop (unsuccessfully,
    returning that no cycle is present) if any of those clauses have size 
    less than 2. Otherwise return to step C3

#### Step C2: Defining the cycle cover clauses

Step C1 will have already initialized the solver, telling it that the variables 
are numbered from 2 to N −1. In this answer, ‘l_1 , ... , l_k ’ denotes a clause 
to be sent to the solver, containing the literals numbered l_1 , ... , l_k .

Send ‘4j +1, 4j +3’ for 1 ≤ j < N/2 (the asymmetry clauses).   For 0 ≤ v < n, 
let v_1 , ... , v_d be the vertices such that v → v_j . Send 
‘2ADJ[v][v_1], ... , 2ADJ[v][v_d]’ and  ‘2ADJ[v1 ][v], ... , 2ADJ[v_d][v]’ (the 
at-least-one clauses). Also, for 1 ≤ i < j ≤ d, send 
‘2ADJ[v][v_i]+1, 2ADJ[v][v_j]+1’ and ‘2ADJ[v_i ][v]+1, 2ADJ[v_j][v]+1’
(the at-most-one clauses).

Here and below, the exact details of 'sending' the clauses depend on the
incremental API of the SAT solver being used.

#### Step C8: Cut clauses

Set j = 0, and do the following while j < t: “Set c = CYC[j], k = 0, and
v = HEAD[c]. For each u with v → u and CID[u] !̸= c, set k = k+1 and l_k ←
2ADJ[v][u]. Then set v = SUCC[v], and repeat the loop on u if v != HEAD[j]. 
Finally send ‘l_1 , ... , l_k ’ and ‘l_1 xor 2, . . . , l_k xor 2’ to the
solver. If t > 2, set j = j +1; otherwise set j = 2.” Here, we’ve used the 
fact that v ∈ CYC[0] ⇐⇒ v / ∈ CYC[1] when t = 2.

Two implementation notes on this step, both of which are easy to get wrong:

- The second clause, ‘l_1 ⊕ 2, ..., l_k ⊕ 2’, is exactly Cut(complement C_j): it
  requires at least one arc *entering* the cycle, where the first clause requires
  at least one arc *leaving* it.  So each cycle contributes two clauses, and the
  ‘otherwise set j = 2’ jump when t = 2 exists only to avoid emitting Cut(C_1),
  which is the clause just emitted as Cut(complement C_0).
- ‘Stop unsuccessfully if any of those clauses have size less than 2’ is not a
  degenerate case to be smoothed over.  Fewer than two edges cross the cut means
  a cycle cannot both leave the vertex set and return to it, so the graph has no
  Hamiltonian cycle and the answer is conclusive.  A size-1 cut clause is **not**
  a legitimate unit clause here.

#### Step C6: Cycle merging

During this routine we have fully merged CYC[i] for 0 ≤ i < j.

C6.1. [Begin loop on j.] Set j - 0.
C6.2. [Choose c.] Set c = CYC[j]. (We’ll try to absorb other cycles into c.)
C6.3. [Begin loop on v.] Set v = HEAD[c] and w = SUCC[v].
C6.4. [Begin loop on v′.] Set v′ to the first vertex such that v → v′.
C6.5. [Is v′ in c?] Set c′ = CID[v′ ], and go to C6.11 if c′ == c.
C6.6. [Check PRED[v′ ].] Set w′ = PRED[v′]. Go to C6.9 if ADJ[w′][w] != 0.
C6.7. [Check SUCC[v′ ].] Set w′ = SUCC[v′]. Go to C6.11 if ADJ[w′][w] = 0.
C6.8. [Reverse subpath.] Set u = w′, u′ = SUCC[u]. While u != v′ , set 
       u′′ = SUCC[u′], SUCC[u′] = u, PRED[u] = u′ , u = u′ , u′ = u′′ .
C6.9. [Merge.] Set SUCC[v] = v′, SUCC[w′] = w, PRED[v′] = v, PRED[w] = w′,
       u = v′ . Repeatedly set CID[u] = c and u = SUCC[u] until u == w.
C6.10. [Delete c′ .] Set t = t−1; go to C7 if t == 1. Otherwise set 
       k = CLOC[c′]. If k > j, set CYC[k] = CYC[t] and CLOC[CYC[k]] = k.
       Otherwise set j = j − 1, and while k < t set CYC[k] = CYC[k+1], 
       CLOC[CYC[k]] = k, k = k+1.
C6.11. [Advance v′ .] Set v′ to the next vertex such that v → v′ and go to C6.5,
        unless v′ was the last neighbor of v.
C6.12. [Advance v.] If w != HEAD[c], set v = w and w = SUCC[w]. Return to C6.4.
C6.13. [Advance j.] Set j = j + 1, and return to C6.2 if j < t, otherwise
       the merging process is complete.

Two facts that make this sound, so they do not have to be rederived:

- Every branch above is guarded by an ADJ lookup, so merging only ever joins
  vertices along **real edges** of the graph.  A merged cycle is therefore a
  genuine cycle, and if merging reaches t = 1 the result is a valid Hamiltonian
  cycle even though it is not the model the solver returned.
- Cut clauses computed from *merged* cycles still exclude the current model.  A
  merged cycle is a union of whole model cycles, so no model arc crosses it, so
  the model violates the cut clause.  Progress in step C8 is still guaranteed.

Working the example above through one merge pass gives, for j = 0, c = 1:
v = HEAD[1] = 0 and w = SUCC[0] = 2.  Neighbours 2 and 3 of v are already in c,
so C6.5 skips them; neighbour 1 has c′ = CID[1] = 2, and w′ = PRED[1] = 8 with
edge 8–2 present, so C6.6 goes straight to the merge without needing the subpath
reversal at C6.8.  That yields SUCC[0] = 1, SUCC[8] = 2, PRED[1] = 0,
PRED[2] = 8, every CID of 2 rewritten to 1, t = 2, CYC = [1, 3], CLOC[1] = 0 and
CLOC[3] = 1 — matching the values stated above.  Walking SUCC from 0 then gives
the nine-vertex cycle 0 1 11 9 4 8 2 6 3.

Note that this example never exercises C6.8, so a test built on it must be
accompanied by one where ADJ[PRED[v′]][w] = 0 but ADJ[SUCC[v′]][w] ≠ 0.