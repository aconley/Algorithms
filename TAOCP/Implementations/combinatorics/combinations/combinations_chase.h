#ifndef __combinations_chase_h__
#define __combinations_chase_h__

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
// Near-perfect generator (Chase's sequence): Knuth 4A 7.2.1.3 Exercise 45
template<class Visitor>
  void combinations_chase(int n, int t, Visitor& vis) {

  if (n == 0 && t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }

  // CC1: Initialize
  std::vector<int> c(t + 1), z(t + 1, 0);
  int j, s, r, x;
  s = n - t;
  for (j = 0; j <= t; ++j) c[j] = s + j;
  auto vis_end = c.cend() - 1;

/*
  std::cout << "Initial";
  std::copy(c.cbegin(), vis_end, std::ostream_iterator<int>(std::cout, " "));
  std::cout << std::endl;
*/
  // Easy case
  if (n == t) {
    vis.visit(c.cbegin(), vis_end);
    return;
  } else if (t == 1) {
    if (!vis.visit(c.cbegin(), vis_end)) return;
    for (j = n - 2; j >= 0; --j) {
      c[0] = j;
      if (!vis.visit(c.cbegin(), vis_end)) return;
    }
    return;
  }

  r = 1;

CC2: // Visit
  if (!vis.visit(c.cbegin(), vis_end)) return;
  j = r;

CC3: // Branch
  if (z[j - 1] != 0) goto CC5;

// CC4: Try to decrease c_j
  x = c[j - 1] + (c[j - 1] & 1) - 2;
  if (x >= j) {
    c[j - 1] = x;
    r = 1;
  } else if (c[j - 1] == j) {
    --c[j - 1];
    z[j - 1] = c[j] - ((c[j] + 1) & 1);
    r = j;
  } else if (c[j - 1] < j) {
    c[j - 1] = j;
    z[j - 1] = c[j] - ((c[j] + 1) & 1);
    r = std::max(1, j - 1);
  } else {
    c[j - 1] = x;
    r = j;
  }
  goto CC2;

CC5: // Try to increase c_j
  x = c[j - 1] + 2;
  if (x < z[j - 1]) {
    c[j - 1] = x;
  } else if (x == z[j - 1] && z[j] != 0) {
    c[j - 1] = x - (c[j] & 1);
  } else {
    z[j - 1] = 0;
    ++j;
    if (j > t) return;
    goto CC3;
  }
  if (c[0] > 0) {
    r = 1;
  } else {
    r = j - 1;
  }
  goto CC2;
}

}
#endif