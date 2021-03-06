#ifndef __langford_h__
#define __langford_h__

#include<array>

namespace backtracking {

// Visit all Langford pairs
//
// That is, permutations
//  of [1..n] u [-1..-n] that have x spaces between the occurrences of x
//  and -x.  The second occurrence is always negative
//
//  Visitor must implement a method
//     bool visit(const array<int, two_n> vals)
//  where two_n = 2 * n

// Algorithm L of Knuth 7.2.2 (Backtrack Programming)
template<std::size_t n, template<std::size_t> class Visitor>
  void langford_basic(Visitor<n>& vis) {

  // Quick check returns when there are no solutions
  if (n <= 0)
    return;
  int nm4 = n % 4;
  if (nm4 == 1 || nm4 == 2)
    return;

  constexpr int n2 = 2 * n;

  // Indices start at 0 in this implementation
  std::array<int, n2> x{};    // Values we will give to visit
  std::array<int, n + 1> p; // Pointer to unused values
  std::array<int, n2> y;    // Backtracking array

  // Initialize (L1)
  int j, k, l, lpkp1;
  for (k = 0; k < n; ++k) p[k] = k + 1;
  p[n] = 0;
  l = 0;

  // Enter level l (which is Knuth's level l + 1)
  //  Yes, very goto heavy.  But it's much more efficient that way
L2:
  k = p[0];
  if (k == 0) {
    vis.visit(x);
    goto L5;
  }
  j = 0;
  while (x[l] < 0) ++l;

  // Try x_{l} = k
L3:
  lpkp1 = l + k + 1;
  if (lpkp1 >= n2) goto L5; // Can't insert -- off edge
  if (x[lpkp1] == 0) {
    x[l] = k;
    x[lpkp1] = -k;
    y[l] = j;
    p[j] = p[k];
    ++l;
    goto L2;
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
}

// Algorithm L of Knuth 7.2.2 (Backtrack Programming)
//  with improvements of exercises 20 and 21
// Doesn't visit the reversed solutions unless visitReversed is true,
//  and even if so they are no longer visited in purely lexicographic
//  order
template<std::size_t n, template<std::size_t> class Visitor>
  void langford(Visitor<n>& vis, bool visitReversed=false) {

  // Quick check returns when there are no solutions
  if (n <= 0)
    return;
  int nm4 = n % 4;
  if (nm4 == 1 || nm4 == 2)
    return;

  constexpr int n2 = 2 * n;
  constexpr int np = ((n & 1) == 0) ? (n - 1) : n;
  constexpr int no2m1 = (n >> 1) - 1;

  // Indices start at 0 in this implementation
  std::array<int, n2> x{};     // Values we will give to visit
  std::array<int, n2> xrev;    // Reversed x
  std::array<int, n + 1> p;    // Pointer to unused values
  std::array<int, n2> y;       // Backtracking array
  std::array<bool, n + 1> a{}; // True if k has appeared

  // Initialize (L1)
  int j, k, l, lpkp1;
  for (k = 0; k < n; ++k) p[k] = k + 1;
  p[n] = 0;
  l = 0;

  // Enter level l (which is Knuth's level l + 1)
  //  Yes, very goto heavy.  But it's much more efficient that way
L2:
  k = p[0];
  if (k == 0) {
    // Visit
    vis.visit(x);
    // And the reverse
    if (visitReversed) {
      for (int i = 0; i < n2; ++i) {
        xrev[i] = - x[n2 - i - 1];
      }
      vis.visit(xrev);
    }
    goto L5;
  }
  j = 0;
  while (x[l] < 0) {
    if ((l == no2m1 && !a[np]) ||
        (l >= (n - 2) && !a[n2 - l - 2]))
      goto L5;
    ++l;
  }

  // Try x_{l} = k
L3:
  lpkp1 = l + k + 1;
  if (lpkp1 >= n2) goto L5; // Can't insert -- off edge
  // Now check the rest of the list
  if (l == no2m1 && !a[np]) {
    while (k != np) {
      j = k;
      k = p[k];
    }
    lpkp1 = l + k + 1;
  }
  if (l >= (n - 2) && !a[n2 - l - 2]) {
    while (l + k + 2 != n2) {
      j = k;
      k = p[k];
    }
    lpkp1 = l + k + 1;
  }
  if (x[lpkp1] == 0) {
    x[l] = k;
    x[lpkp1] = -k;
    a[k] = true;
    y[l] = j;
    p[j] = p[k];
    ++l;
    goto L2;
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
    a[k] = false;
    x[l] = 0;
    x[l + k + 1] = 0;
    j = y[l];
    p[j] = k;
    if (l == no2m1 && k == np) {
      goto L5;
    } else {
      goto L4;
    }
  }
}

}
#endif
