#ifndef __nqueens_h__
#define __nqueens_h__

#include<array>
#include<exception>
#include<cstdint>

namespace backtracking {

// Visit all NQueens solutions.
//
// Bitwise implementation of Walkers method, Knuth 7.2.2 W

// Visitor must implement a method
//     bool visit(const array<int, n>& rows)
// Where the queen in column i is in row rows[i]
//  in the range [0, n)
// If visit method returns false, it indicates that the
//  algorithm should be terminated immediately.  This
//  is useful if looking for, say, the first solution
//  to satisfy some additional property.

static const int MultiplyDeBruijnBitPosition[32] =
{
  0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
  31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
};

int getPositionOfLeastSetBit(std::uint32_t v) {
  return MultiplyDeBruijnBitPosition[((uint32_t)((v & -v) * 0x077CB531U)) >> 27];
}

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
    if (!vis.visit(rows)) return;
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
