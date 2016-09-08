#ifndef __nqueens_h__
#define __nqueens_h__

#include<array>

namespace backtracking {

// The methods here visit all NQueens solution
//
//  Visitor must implement a method
//     bool visit(const array<int, n>& rows)
// Where the queen in column i is in row rows[i]
//  in the range [0, n)

// Basic visitor
template<std::size_t n, template<std::size_t> class Visitor>
  void nqueens_basic(Visitor<n>& vis) {

  if (n == 0) return;
  int int_n = static_cast<int>(n);
  int nm1 = int_n - 1;

  // Holds current solution
  std::array<int, n> rows;

  if (n == 1) {
    rows[0] = 0;
    vis.visit(rows);
    return;
  }

  int l = 0, x_l = 0;
  bool isSafe = true;

  B2: // Enter level l
  if (l > nm1) {
    vis.visit(rows);
    goto B5;
  }
  x_l = 0;

  B3: // Try x_l
  isSafe = true;
  // Explicitly check all previous values
  for (int j = 0; j < l; ++j) {
    int xl_m_xj = x_l - rows[j];
    if (xl_m_xj == 0 || xl_m_xj == (j - l) || xl_m_xj == (l - j)) {
      // x_l didn't work
      isSafe = false;
      break;
    }
  }
  if (isSafe) {
    rows[l] = x_l;
    ++l;
    goto B2;
  }

  B4: // Try again
  if (x_l < nm1) {
    ++x_l;
    goto B3;
  }

  B5: // backtrack
  --l;
  if (l >= 0) {
    x_l = rows[l];
    goto B4;
  }

  // Otherwise we're done
}

}

#endif