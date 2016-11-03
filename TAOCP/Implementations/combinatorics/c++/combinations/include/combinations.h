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

}
#endif
