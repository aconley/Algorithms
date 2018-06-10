
#include<exception>
#include "nqueens_iterator.h"

NQueensIterator::NQueensIterator(int _n) : n(_n) {}

NQueensIterator::iterator NQueensIterator::begin() { 
  return iterator(n, false); 
}

NQueensIterator::iterator NQueensIterator::end() { 
  return iterator(n, true); 
}

std::uint8_t 
  NQueensIterator::iterator::getPositionOfLeastSetBit(std::uint32_t v) {
    std::uint32_t idx = ((uint32_t)((v & -v) * 0x077CB531U)) >> 27;
    return MultiplyDeBruijnBitPosition[idx];
}

NQueensIterator::iterator::iterator(int _n, bool _done) : 
  done(_done), ctr(0), n(_n), l(1) {
        
  if (n < 1 || n > 32)
    throw std::invalid_argument("n must be in range [1, 32].");
  if (!done) {
    a = std::vector<uint32_t>(n + 1, 0u);
    b = std::vector<uint32_t>(n + 1, 0u);
    c = std::vector<uint32_t>(n + 1, 0u);
    s = std::vector<uint32_t>(n + 1, 0u);
    mu = n == 32 ? (~0u) : ((1 << n) - 1);
  }
}

NQueensIterator::iterator& NQueensIterator::iterator::operator++() {
  return *this;
}

NQueensIterator::iterator::value_type NQueensIterator::iterator::operator*() {
  std::vector<std::uint8_t> retval(n);
  return retval;
}

NQueensIterator::iterator NQueensIterator::iterator::operator++(int) {
  iterator retval = *this; 
  ++(*this); 
  return retval;
}

bool NQueensIterator::iterator::operator==(iterator other) {
  if (n != other.n) return false;
  return done ? other.done : ctr == other.ctr;
}

bool NQueensIterator::iterator::operator!=(iterator other) {
  return !(*this == other);
}