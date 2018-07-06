
#include<exception>
#include<limits>
#include "nqueens_walker.h"
#include "nqueens_constants.h"
  
using namespace backtracking;

const std::array<std::uint8_t, 32> 
  NQueensWalker::iterator::MultiplyDeBruijnBitPosition =
    {{
      0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
      31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
    }};

NQueensWalker::NQueensWalker(int _n) : n(_n) {}

NQueensWalker::iterator NQueensWalker::begin() const { 
  return iterator(n, false); 
}

NQueensWalker::iterator NQueensWalker::end() const { 
  return iterator(n, true); 
}

std::uint8_t 
  NQueensWalker::iterator::getPositionOfLeastSetBit(std::uint32_t v) const {
    std::uint32_t idx = ((uint32_t)((v & -v) * 0x077CB531U)) >> 27;
    return MultiplyDeBruijnBitPosition[idx];
}

NQueensWalker::iterator::iterator(int _n, bool _done) : 
  done(_done), n(_n), l(2) {
        
  if (n < 1 || n > max_n)
    throw std::invalid_argument("n must be in range [1, 24].");
  if (!done) {
    a = std::vector<uint32_t>(n + 1, 0u);
    b = std::vector<uint32_t>(n + 1, 0u);
    c = std::vector<uint32_t>(n + 1, 0u);
    s = std::vector<uint32_t>(n + 1, 0u);
    mu = n == 32 ? (~0u) : ((1 << n) - 1);
    s[1] = mu;
    ctr = 0;
  } else {
    ctr = n_solutions[n];
  }
}

NQueensWalker::iterator& NQueensWalker::iterator::operator++() {
  if (done) {
    return *this;
  }

  W4: // backtrack
  if (l > 0) {
    --l;
    goto W3;
  } else {
    done = true;
    return *this;
  }

  W2: // Enter level l
  if (l > n) {
    ++ctr;
    return *this;
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
  } else {
    goto W4;
  }
}

NQueensWalker::iterator::value_type 
  NQueensWalker::iterator::operator*() const {

  std::vector<std::uint8_t> retval(n);
  for (int i = 0; i < n; ++i) {
    retval[i] = getPositionOfLeastSetBit(a[i + 1] - a[i]);
  }
  return retval;
}

NQueensWalker::iterator NQueensWalker::iterator::operator++(int) {
  iterator retval = *this; 
  ++(*this); 
  return retval;
}

bool NQueensWalker::iterator::operator==(const iterator& other) {
  if (n != other.n) return false;
  return done ? other.done : ctr == other.ctr;
}

bool NQueensWalker::iterator::operator!=(const iterator& other) {
  return !(*this == other);
}

NQueensWalker::iterator::difference_type 
  NQueensWalker::iterator::operator-(const iterator& other) {
  return ctr - other.ctr;
}