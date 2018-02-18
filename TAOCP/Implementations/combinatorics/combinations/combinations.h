#ifndef __combinations_h__
#define __combinations_h__

#include<vector>

// The methods here all visit (s, t) combinations --
// that is, a combination of n = s + t things taken t at
// a time

// Visitor must implement one methods:
//   bool visit(std::vector<int>::const_iterator begin,
//              std::vector<int>::const_iterator end) ->
// where the t elements are identified in [begin, end),
// and are unique integers in the range [0, t).
// visit should return false to terminate the algorithm
//  immediately.

namespace combinations {

// Basic, un-optimized generator
//  This is algorithm L of Knuth TAOCP 7.2.1.3
// This visits n objects taken t at a time
template<class Visitor>
  void combinations_lex_basic(int n, int t, Visitor& vis) {

  if (n == 0 || t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }
  int s = n - t;

  // L1: Initialize
  std::vector<int> values(t);
  int j;
  for (j = 0; j < t; ++j)
    values[j] = j;

  if (s == 0) {
    // Quick exit case
    vis.visit(values.cbegin(), values.cend());
    return;
  }

  int tm1 = t - 1;
  int nm1 = n - 1;
  while (true) {
    // L2: visit
    if (!vis.visit(values.cbegin(), values.cend())) return;

    // L3 find j
    for (j = 0; j < tm1 && (values[j] + 1 == values[j + 1]); ++j)
      values[j] = j;
    if (j == tm1 && values[tm1] == nm1) {
      // Done
      break;
    } else {
      ++values[j];
    }
  }
}

// Optimized version of permutations visitor
//  This is algorithm T of Knuth TAOCP 7.2.1.3
// This visits n objects taken t at a time
template<class Visitor>
  void combinations_lex(int n, int t, Visitor& vis) {

  if (n == 0 || t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }

  // L1: Initialize
  std::vector<int> c(t + 2);
  int j, x;
  for (j = 0; j < t; ++j)
    c[j] = j;
  c[t] = n;
  c[t + 1] = 0;

  auto vis_end = c.cend() - 2;

  // Quick exit cases
  if (n == t) {
    vis.visit(c.cbegin(), vis_end);
    return;
  } else if (t == 1) {
    if (!vis.visit(c.cbegin(), vis_end)) return;
    for (j = 1; j < n; ++j) {
      c[0] = j;
      if (!vis.visit(c.cbegin(), vis_end)) return;
    }
    return;
  }
  j = t;

T2: // visit
  if (!vis.visit(c.cbegin(), vis_end)) return;
  if (j > 0) {
    x = j;
    goto T6;
  }

// T3
  if (c[0] + 1 < c[1]) {
    c[0] += 1;
    goto T2;
  }
  j = 2;

T4:
  c[j - 2] = j - 2;
  x = c[j - 1] + 1;
  if (x == c[j]) {
    ++j;
    goto T4;
  }
  if (j > t) return;

T6:
  c[j - 1] = x;
  --j;
  goto T2;
}

}
#endif
