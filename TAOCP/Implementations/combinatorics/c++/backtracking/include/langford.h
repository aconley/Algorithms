#ifndef __langford_h__
#define __langford_h__

namespace backtracking {

// Visit all Langford pairs
//
// That is, permutations
//  of [1..n] u [-1..-n] that have x spaces between the occurrences of x
//  and -x.  The second occurrence is always negative
//
//  Visitor must implement a method
//     bool visit(const int* const vals, int n)
//  noting that vals has 2 n values

// Algorithm L of Knuth 7.2.2 (Backtrack Programming)
template<class RandomIt, class Visitor>
void langford(unsigned int n, Visitor& vis) {
  if (n == 0)
    return;

  unsigned int n2 = 2 * n;

  // Indices start at 0 in this implementation
  int* x = new int[n2]; // Values we will give to visit
  int* p = new int[n + 1]; // Pointer to unused values
  int *y = new int[n2]; // Backtracking array

  // Initialize (L1)
  int j, k, l;
  std::memset(x, 0, std::static_cast<size_t>(n2 * sizeof(unsigned int)));
  for (k = 0; k < n; ++k) p[k] = k + 1;
  p[n] = 0;
  l = 0;

  // Enter level l
  //  Yes, very goto heavy.  But it's much more efficient that way
L2:
  k = p[0];
  if (k == 0) {
    vis.visit(x, n);
    goto L5;
  }
  j = 0;
  while (x[l] < 0) ++l;

  // Try x_l = k
L3:
  int lpkp1 = l + k + 1;
  if (lpkp1 >= n2) goto L5; // Can't insert -- off edge
  if (x[lpkp1] == 0) {
    x[l] = k;
    x[lpkp1] = -k;
    y[l] = j;
    p[j] = p[k];
    ++l;
  }

  // Try again
L4:
  j = k;
  k = p[j];
  if (k != 0) goto L3;

  // Backtrack
L5:
  --l;
  if (l >= 0) {
    while (x[l] < 0) --l;
    k = x[l];
    x[l] = 0;
    x[l + k + 1] = 0;
    j = y[l];
    p[j] = k;
    goto L4;
  }

  // We're done; clean up
  delete[] x;
  delete[] p;
  delete[] y;
}

}
#endif