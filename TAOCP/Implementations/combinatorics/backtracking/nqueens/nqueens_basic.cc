#include<exception>
#include<limits>
#include "nqueens_basic.h"

using namespace backtracking;

const std::array<long, NQueensBasic::iterator::max_n + 1>
  NQueensBasic::iterator::n_solutions =
    {{
          1, 1, 0, 0, 2, 10, 4, 40, 92, 352, 724, 2680, 14200, 73712,
          365596, 2279184, 14772512, 95815104, 666090624,
          4968057848, 39029188884, 314666222712, 2691008701644,
          24233937684440, 227514171973736
    }};

NQueensBasic::NQueensBasic(int _n) : n(_n) {}

NQueensBasic::iterator NQueensBasic::begin() const { 
  return iterator(n, false); 
}

NQueensBasic::iterator NQueensBasic::end() const { 
  return iterator(n, true); 
}

NQueensBasic::iterator::iterator(int _n, bool _done) : 
  done(_done), n(_n), l(1), nm1(_n-1) {
        
  if (n < 1 || n > max_n)
    throw std::invalid_argument("n must be in range [1, 24].");
  ctr = done ? n_solutions[n] : 0;
  if (done) {
    ctr = n_solutions[n];
  } else {
    ctr = 0;
    rows = std::vector<std::uint8_t>(n, 0u);
    rows[0] = 255;
  }
}

NQueensBasic::iterator& 
  NQueensBasic::iterator::operator++() {

  if (done) {
    return *this;
  }

  B5: // backtrack
  --l;
  if (l >= 0) {
    x_l = rows[l];
    goto B4;
  } else {
    done = true;
    return *this;
  }

  B2: // Enter level l
  if (l > nm1) {
    ++ctr;
    return *this;
  }
  x_l = 0;

  B3: // Try x_l
  // Explicitly check all previous values
  for (std::uint8_t j = 0; j < l; ++j) {
    int xl_m_xj = x_l - rows[j];
    if (xl_m_xj == 0 || xl_m_xj == (j - l) || xl_m_xj == (l - j)) {
      // x_l didn't work
      goto B4;
    }
  }
  rows[l] = x_l;
  ++l;
  goto B2;

  B4: // Try again
  ++x_l;
  if (x_l < n) {
    goto B3;
  }
  goto B5;
}

NQueensBasic::iterator::value_type 
  NQueensBasic::iterator::operator*() const {

  return rows;
}

NQueensBasic::iterator 
  NQueensBasic::iterator::operator++(int) {

  iterator retval = *this; 
  ++(*this); 
  return retval;
}

bool NQueensBasic::iterator::operator==(const iterator& other) {
  if (n != other.n) return false;
  return done ? other.done : ctr == other.ctr;
}

bool NQueensBasic::iterator::operator!=(const iterator& other) {
  return !(*this == other);
}

NQueensBasic::iterator::difference_type 
  NQueensBasic::iterator::operator-(const iterator& other) {
  return ctr - other.ctr;
}
