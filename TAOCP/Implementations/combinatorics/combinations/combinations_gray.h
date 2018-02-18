#ifndef __combinations_gray_h__
#define __combinations_gray_h__

#include<vector>

// The methods here visit (s, t) combinations --
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
// Grey code revolving door generator: Knuth 4A 7.2.1.3 Algorithm R
template<class Visitor>
  void combinations_gray(int n, int t, Visitor& vis) {

  if (n == 0 && t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }

  // R1: Initialize
  std::vector<int> c(t);
  int j;
  for (j = 0; j < t; ++j)
    c[j] = j;

  // Easy cases
  if (n == t) {
    vis.visit(c.cbegin(), c.cend());
    return;
  } else if (t == 1) {
    if (!vis.visit(c.cbegin(), c.cend())) return;
    for (j = 1; j < n; ++j) {
      c[0] = j;
      if (!vis.visit(c.cbegin(), c.cend())) return;
    }
    return;
  }

  bool is_t_odd = (t & 1) != 0;
  int tm1 = t - 1;

R2:
  if (!vis.visit(c.cbegin(), c.cend())) return;

// R3: Easy case
  if (is_t_odd) {
    if (c[0] + 1 < c[1]) {
      ++c[0];
      goto R2;
    } else {
      j = 1;
      goto R4;
    }
  } else {
    if (c[0] > 0) {
      --c[0];
      goto R2;
    } else {
      j = 1;
      goto R5;
    }
  }

R4: // Try to decrease c_j
  if (c[j] > j) {
    c[j] = c[j - 1];
    c[j - 1] = j - 1;
    goto R2;
  } else {
    ++j;
  }

R5: // Try to increase c_j
  if (j == tm1) {
    if (c[j] + 1 < n) {
      c[j - 1] = c[j];
      ++c[j];
      goto R2;
    } else {
      return;
    }
  } else {
    if (c[j] + 1 < c[j + 1]) {
      c[j - 1] = c[j];
      ++c[j];
      goto R2;
    } else {
      ++j;
      if (j <= tm1) goto R4;
    }
  }
}
}
#endif

