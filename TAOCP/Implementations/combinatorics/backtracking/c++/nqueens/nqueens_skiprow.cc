// This is a modifed version of nqueens on an n by n board
// that ignores the queen in row r.

#include<exception>
#include<cstdint>
#include<iostream>

class NQueensSkipRow {
public:
  NQueensSkipRow(int n) {
    if (n < 2) throw std::invalid_argument("n must be >= 2");
    if (n > 32) throw std::invalid_argument("n must be <= 32");
    
    nm1_ = n - 1;
    int_n_ = static_cast<int>(n);
  }

  long getNSolutions(int r) {
    if (r < 0) throw std::invalid_argument("r must be >= 0");
    if (r >= int_n_) 
      throw std::invalid_argument("r must be < n");

    a_ = b_ = c_ = 0;
    return visitLevels(0, r);
  }

private:
  int nm1_, int_n_;
  long a_, b_, c_;

  long visitLevels(int l, int r) {    
    if (l > nm1_) {
      return 1;
    }

    // Skip the specified row.
    if (l == r) {
      return visitLevels(l + 1, r);
    }

    long n_solutions = 0;
    for (int xl = 0; xl < int_n_; ++xl) {
      if ( (a_ & (1 << xl)) == 0
        && (b_ & (1 << (xl + l))) == 0
        && (c_ & (1 << (xl - l + nm1_))) == 0) {
          // Valid candidate, update vectors
          a_ |= (1 << xl);
          b_ |= (1 << (xl + l));
          c_ |= (1 << (xl - l + nm1_));

          // Visit next level
          n_solutions += visitLevels(l + 1, r);

          // Undo
          a_ &= ~(1 << xl);
          b_ &= ~(1 << (xl + l));
          c_ &= ~(1 << (xl - l + nm1_));
      }
    }

    return n_solutions;
  }
};

int main(int argc, char **argv) {
  auto nqueens = NQueensSkipRow(8);
  std::cout << "For 8x8 nqueens, the number of solutions when skiping row r is:"
    << std::endl;
  for (int r = 0; r < 8; ++r) {
    std::cout << "r = " << r << " " << nqueens.getNSolutions(r)
      << std::endl;
  }
}