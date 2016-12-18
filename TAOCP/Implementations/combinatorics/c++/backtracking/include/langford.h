#ifndef __langford_h__
#define __langford_h__

#include<array>
#include<iostream>
#include<ostream>

namespace backtracking {

template <class T, std::size_t N>
std::ostream& operator<<(std::ostream& o, const std::array<T, N>& arr)
{
    std::copy(arr.cbegin(), arr.cend(), std::ostream_iterator<T>(o, " "));
    return o;
}

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

  if (n == 0)
    return;

  constexpr unsigned int n2 = 2 * n;

  // Indices start at 0 in this implementation
  std::array<int, n2> x{};    // Values we will give to visit
  std::array<int, n + 1> p; // Pointer to unused values
  std::array<int, n2> y{};    // Backtracking array

  // Initialize (L1)
  int j, k, l, lpkp1;
  for (k = 0; k < n; ++k) p[k] = k + 1;
  p[n] = 0;
  l = 1;

  // Enter level l
  //  Yes, very goto heavy.  But it's much more efficient that way
L2:
  k = p[0];
  if (k == 0) {
    // std::cerr << "Visiting solution: " << x << std::endl;
    vis.visit(x);
    goto L5;
  }
  j = 0;
  while (x[l - 1] < 0) ++l;

  // Try x_{l - 1} = k
L3:
  lpkp1 = l + k + 1;
  if (lpkp1 > n2) goto L5; // Can't insert -- off edge
  if (x[lpkp1 - 1] == 0) {
    // std::cerr << "L3: Trying x[" << l << "] = " << k;
    x[l - 1] = k;
    x[lpkp1 - 1] = -k;
    y[l - 1] = j;
    p[j] = p[k];
    ++l;
    // std::cerr << " x: " << x << " p: " << p << " y: " << y << std::endl;
    goto L2;
  }

  // Try again
L4:
  j = k;
  k = p[j];
  if (k != 0) goto L3;

  // Backtrack
L5:
  // std::cerr << "L5: Backtrack" << std::endl;
  --l;
  if (l > 0) {
    while (x[l - 1] < 0) --l;
    k = x[l - 1];
    x[l - 1] = 0;
    x[l + k] = 0;
    j = y[l - 1];
    p[j] = k;
    // std::cerr << "x: " << x << " p: " << p << " y: " << y << std::endl;
    goto L4;
  }
}

}
#endif
