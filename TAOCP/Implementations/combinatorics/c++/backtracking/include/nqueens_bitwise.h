#ifndef __nqueens_bitwise_h__
#define __nqueens_bitwise_h__

#include<array>
#include<exception>
#include<cstdint>

// Visits all NQueens solutions.
//
// Bitwise solution, Knuth algorithm 7.2.2. B*
//
//  Visitor must implement a method
//     bool visit(const array<int, n>& rows)
// Where the queen in column i is in row rows[i]
//  in the range [0, n)
// If visit method returns false, it indicates that the
//  algorithm should be terminated immediately.  This
//  is useful if looking for, say, the first solution
//  to satisfy some additional property.

namespace backtracking {
template<std::size_t n, template<std::size_t> class Visitor>
  void nqueens_bitwise(Visitor<n>& vis) {

  if (n == 0) return;
  if (n > 32) {
    throw std::invalid_argument("n must be <= 32");
  }
  int int_n = static_cast<int>(n);
  int nm1 = int_n - 1;
  std::array<int, n> rows;
  if (n == 1) {
    rows[0] = 0;
    vis.visit(rows);
    return;
  }

  // State vectors
  long a, b, c;

  // B1
  a = b = c = 0;
  int l = 0, t = 0;

  B2: // Enter level l
  if (l > nm1) {
    if (!vis.visit(rows)) return;
    goto B5;
  }
  t = 0;

  B3: // Try t
  if ( (a & (1 << t)) == 0
    && (b & (1 << (t+l))) == 0
    && (c & (1 << (t - l + nm1))) == 0)  {
    // Worked
    a |= (1 << t);
    b |= (1 << (t + l));
    c |= (1 << (t - l + nm1));
    rows[l] = t;
    ++l;
    goto B2;
  }

  B4: // Try again
  if (t < nm1) {
    ++t;
    goto B3;
  }

  B5: // backtrack
  --l;
  if (l >= 0) {
    t = rows[l];
    a &= ~(1 << t);
    b &= ~(1 << (t + l));
    c &= ~(1 << (t - l + nm1));
    goto B4;
  }

  // Otherwise we're done
}
}
#endif