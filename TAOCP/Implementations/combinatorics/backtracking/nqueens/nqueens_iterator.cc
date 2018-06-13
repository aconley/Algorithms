
#include<exception>
#include<limits>
#include "nqueens_iterator.h"
  
const std::array<std::uint8_t, 32> 
  NQueensIterator::iterator::MultiplyDeBruijnBitPosition =
    {{
      0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
      31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
    }};

const std::array<long, NQueensIterator::iterator::max_n + 1>
  NQueensIterator::iterator::n_solutions =
    {{
          1, 1, 0, 0, 2, 10, 4, 40, 92, 352, 724, 2680, 14200, 73712,
          365596, 2279184, 14772512, 95815104, 666090624,
          4968057848, 39029188884, 314666222712, 2691008701644,
          24233937684440, 227514171973736
    }};

NQueensIterator::NQueensIterator(int _n) : n(_n) {}

NQueensIterator::iterator NQueensIterator::begin() const { 
  return iterator(n, false); 
}

NQueensIterator::iterator NQueensIterator::end() const { 
  return iterator(n, true); 
}

std::uint8_t 
  NQueensIterator::iterator::getPositionOfLeastSetBit(std::uint32_t v) const {
    std::uint32_t idx = ((uint32_t)((v & -v) * 0x077CB531U)) >> 27;
    return MultiplyDeBruijnBitPosition[idx];
}

NQueensIterator::iterator::iterator(int _n, bool _done) : 
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

NQueensIterator::iterator& NQueensIterator::iterator::operator++() {
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

NQueensIterator::iterator::value_type 
  NQueensIterator::iterator::operator*() const {

  std::vector<std::uint8_t> retval(n);
  for (int i = 0; i < n; ++i) {
    retval[i] = getPositionOfLeastSetBit(a[i + 1] - a[i]);
  }
  return retval;
}

NQueensIterator::iterator NQueensIterator::iterator::operator++(int) {
  iterator retval = *this; 
  ++(*this); 
  return retval;
}

bool NQueensIterator::iterator::operator==(const iterator& other) {
  if (n != other.n) return false;
  return done ? other.done : ctr == other.ctr;
}

bool NQueensIterator::iterator::operator!=(const iterator& other) {
  return !(*this == other);
}

NQueensIterator::iterator::difference_type 
  NQueensIterator::iterator::operator-(const iterator& other) {
  return ctr - other.ctr;
}