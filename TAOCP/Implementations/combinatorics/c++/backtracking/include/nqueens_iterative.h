#ifndef __nqueens_iterative_h__
#define __nqueens_iterative_h__

#include<array>
#include<exception>
#include<cstdint>

namespace backtracking {

template <std::size_t n, template<std::size_t> class Visitor>
  class NQueensIterative {
private:
  long a, b, c;
  std::array<int, n> rows;
  int int_n;
  int nm1;
  bool done;

  void visitLevels(int l, Visitor<n>& vis) {
    if (done) return;
    if (l > nm1) {
      if (!vis.visit(rows)) {
        done = true;
      }
      return;
    }

    for (int xl = 0; xl < int_n; ++xl) {
      if ( (a & (1 << xl)) == 0
        && (b & (1 << (xl + l))) == 0
        && (c & (1 << (xl - l + nm1))) == 0) {
          // Valid candidate, update vectors
          rows[l] = xl;
          a |= (1 << xl);
          b |= (1 << (xl + l));
          c |= (1 << (xl - l + nm1));

          // Visit next level
          visitLevels(l + 1, vis);

          // Undo
          a &= ~(1 << xl);
          b &= ~(1 << (xl + l));
          c &= ~(1 << (xl - l + nm1));
        }
    }
  }

public:
  NQueensIterative() {
    int_n = static_cast<int>(n);
    nm1 = n - 1;
  }

  void visit(Visitor<n>& vis) {
    a = b = c = 0;
    done = false;
    visitLevels(0, vis);
  }
};

template<std::size_t n, template<std::size_t> class Visitor>
  void nqueens_iterative(Visitor<n>& vis) {

  if (n == 0) return;
  if (n > 32) {
    throw std::invalid_argument("n must be <= 32");
  }

  NQueensIterative<n, Visitor> v;
  v.visit(vis);
}
}
#endif