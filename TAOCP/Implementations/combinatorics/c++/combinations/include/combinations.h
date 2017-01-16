#ifndef __combinations_h__
#define __combinations_h__

#include<array>

// The methods here all visit (s, t) combinations --
// that is, a combination of s + t things taken t at
// a time

// Visitor must implement a method
//   bool visit(const array<int, t>& values)
// where the t elements are identified in values[0], ..., values[t-1].
// and are unique integers in the range [0, t).
// visit should return false to terminate the algorithm
//  immediately.

namespace combinations {

// Basic, un-optimized generator
//  This is algorithm L of Knuth TAOCP 7.2.1.3
// This visits n objects taken t at a time
template<std::size_t t,
         template<std::size_t> class Visitor>
  void combinations_lex_basic(std::size_t n, Visitor<t>& vis) {

  if (t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }
  std::size_t s = n - t;

  // L1: Initialize
  std::array<int, t> values;
  std::size_t j;
  for (j = 0; j < t; ++j)
    values[j] = static_cast<int>(j);

  if (s == 0) {
    // Quick exit case
    vis.visit(values);
    return;
  }

  std::size_t tm1 = t - 1;
  std::size_t nm1 = n - 1;
  while (true) {
    // L2: visit
    if (!vis.visit(values)) break;

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
template<std::size_t t,
         template<std::size_t> class Visitor>
  void combinations_lex(std::size_t n, Visitor<t>& vis) {

  if (t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }

  // L1: Initialize
  std::array<int, t> c;
  int j, x;
  for (j = 0; j < t; ++j)
    c[j] = j;

  // Quick exit cases
  if (n == t) {
    vis.visit(c);
    return;
  } else if (t == 1) {
    vis.visit(c);
    for (j = 1; j < n; ++j) {
      c[0] = j;
      if (!vis.visit(c)) return;
    }
    return;
  }
  j = t;

T2: // visit
if (!vis.visit(c)) return;
  if (j > 0) {
    x = j;
    goto T6;
  }

T3:
  if (c[0] + 1 < c[1]) {
    c[0] += 1;
    goto T2;
  }
  j = 2;

T4:
  c[j - 2] = j - 2;
  x = c[j - 1] + 1;
  if (x == n) return;
  if (x == c[j]) {
    ++j;
    goto T4;
  }

T6:
  c[j - 1] = x;
  --j;
  goto T2;
}

// Grey code revolving door generator: Knuth 4A 7.2.1.3 Algorithm R
template<std::size_t t,
         template<std::size_t> class Visitor>
  void combinations_grey(std::size_t n, Visitor<t>& vis) {

  if (t == 0) return;
  if (n < t) {
    throw new std::invalid_argument("n should be >= t");
  }

  // R1: Initialize
  std::array<int, t> c;
  int j;
  for (j = 0; j < t; ++j)
    c[j] = j;

  // Easy cases
  if (n == t) {
    vis.visit(c);
    return;
  } else if (t == 1) {
    vis.visit(c);
    for (j = 1; j < n; ++j) {
      c[0] = j;
      if (!vis.visit(c)) return;
    }
    return;
  }

  bool is_t_odd = (t & 1) != 0;

R2:
  if (!vis.visit(c)) return;

R3: // Easy case
  if (is_t_odd) {
    if (c[0] + 1 < c[1]) {
      ++c[0];
      goto R2;
    } else {
      j = 2;
      goto R4;
    }
  } else {
    if (c[0] > 0) {
      --c[0];
      goto R2;
    } else {
      j = 2;
      goto R5;
    }
  }

R4: // Try to decrease c_j
  if (c[j - 1] >= j) {
    c[j - 1] = c[j - 2];
    c[j - 2] = j - 2;
    goto R2;
  } else {
    ++j;
  }

R5: // Try to increase c_j
  if (j == t) {
    if (c[j - 1] + 1 < n) {
      c[j - 2] = c[j - 1];
      ++c[j - 1];
      goto R2;
    } else {
      return;
    }
  } else {
    if (c[j - 1] + 1 < c[j]) {
      c[j - 2] = c[j - 1];
      ++c[j-1];
      goto R2;
    } else {
      ++j;
      if (j <= t) goto R4;
    }
  }
}

}
#endif
