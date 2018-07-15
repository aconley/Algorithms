#ifndef __nqueens_constants_h__
#define __nqueens_constants_h__

#include<array>

namespace backtracking {
  static constexpr int max_n = 24;
  extern const std::array<long, max_n + 1> n_solutions;
}

#endif