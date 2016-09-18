#ifndef __nqueens_h__
#define __nqueens_h__

#include<array>
#include<exception>
#include<cstdint>

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

// Array based property testing, Knuth algorithm 7.2.2 B*
template<std::size_t n, template<std::size_t> class Visitor>
  void nqueens_array(Visitor<n>& vis) {

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

  // State vectors
  std::array<int, n> a;
  std::array<int, 2 * n - 1> b;
  std::array<int, 2 * n - 1> c;

  // B1
  a.fill(0);
  b.fill(0);
  c.fill(0);

  int l = 0, t = 0;

  B2: // Enter level l
  if (l > nm1) {
    vis.visit(rows);
    goto B5;
  }
  t = 0;

  B3: // Try t
  if (a[t] == 0 && b[t + l] == 0 && c[t - l + nm1] == 0) {
    // Worked
    a[t] = 1;
    b[t + l] = 1;
    c[t - l + nm1] = 1;
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
    a[t] = 0;
    b[t + l] = 0;
    c[t - l + nm1] = 0;
    goto B4;
  }

  // Otherwise we're done
}

// Bitwise based property testing, Knuth algorithm 7.2.2 B*
//  but with bit vectors
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
    vis.visit(rows);
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

// Set based property testing, Knuth algorithm 7.2.2 W
//  but with bit vectors
static const int MultiplyDeBruijnBitPosition[32] =
{
  0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
  31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
};

int getPositionOfLeastSetBit(std::uint32_t v) {
  return MultiplyDeBruijnBitPosition[((uint32_t)((v & -v) * 0x077CB531U)) >> 27];
}

// Actual nqueens bit
template<std::size_t n, template<std::size_t> class Visitor>
  void nqueens_walker(Visitor<n>& vis) {

  if (n == 0) return;
  if (n > 32) {
    throw std::invalid_argument("n must be <= 32");
  }
  int int_n = static_cast<int>(n);
  std::array<int, n> rows;  // For passing to visit
  if (n == 1) {
    rows[0] = 0;
    vis.visit(rows);
    return;
  }

  // State vectors a_l, b_l, etc.
  //  Note: in this algorithm we use 1 based indexing for a, b, c, s
  std::array<std::uint32_t, n + 1> a, b, c, s;

  // Mask 2^n - 1
  std::uint32_t mu = n == 32 ? (~0u) : ((1 << n) - 1);

  // W1
  a.fill(0u);
  b.fill(0u);
  c.fill(0u);
  s.fill(0u);
  std::uint32_t t;
  int l = 1;

  W2: // Enter level l
  if (l > int_n) {
    // Fill rows
    for (int i = 1; i <= int_n; ++i) {
      rows[i - 1] = backtracking::getPositionOfLeastSetBit(a[i] - a[i - 1]);
    }
    vis.visit(rows);
    goto W4;
  }
  s[l] = mu & (~a[l - 1]) & (~b[l - 1]) & (~c[l - 1]);

  W3: // Try t
  if (s[l] != 0) {
    t = s[l] & (-s[l]);
    a[l] = a[l - 1] + t;
    b[l] = (b[l - 1] + t) >> 1;
    c[l] = ((c[l - 1] + t) << 1) & mu;
    s[l] -= t;
    ++l;
    goto W2;
  }

  W4: // backtrack
  if (l > 0) {
    --l;
    goto W3;
  }
  // Otherwise we're done
}
} // namespace
#endif