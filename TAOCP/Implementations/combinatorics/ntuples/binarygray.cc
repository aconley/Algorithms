#include<stdexcept>

#include "binarygray.h"

using namespace ntuples;

////////// BinaryGray //////////////

BinaryGray::BinaryGray(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = nbits;
}

int BinaryGray::getNBits() const {
  return _nbits;
}

BinaryGrayIterator BinaryGray::begin() const {
  return BinaryGrayIterator::begin(_nbits);
}

BinaryGrayIterator BinaryGray::end() const {
  return BinaryGrayIterator::end(_nbits);
}

std::uint32_t BinaryGray::getNext(std::uint32_t g) {
  // Convert back to place, increment, re-convert
  std::uint32_t b = g^(g >> 1);
  b = b^(b >> 2);
  b = b^(b >> 4);
  b = b^(b >> 8);
  b = b^(b >> 16);
  b += 1;
  return b^(b >> 1);
}

////////// BinaryGrayIterator //////////////

BinaryGrayIterator::BinaryGrayIterator(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  maxN = 1u << nbits;
  n = 0u;
}

BinaryGrayIterator& BinaryGrayIterator::operator++() {
  ++n;
  return *this;
}

BinaryGrayIterator BinaryGrayIterator::operator++(int) {
  BinaryGrayIterator tmp(*this);
  operator++();
  return tmp;
}

bool BinaryGrayIterator::operator==(const BinaryGrayIterator& rhs) const {
  return maxN == rhs.maxN && n == rhs.n;
}

bool BinaryGrayIterator::operator!=(const BinaryGrayIterator& rhs) const {
  return maxN != rhs.maxN || n != rhs.n;
}

std::uint32_t BinaryGrayIterator::operator*() const {
  return n^(n >> 1);
}

BinaryGrayIterator BinaryGrayIterator::begin(int nbits) {
  return BinaryGrayIterator(nbits);
}

BinaryGrayIterator BinaryGrayIterator::end(int nbits) {
  BinaryGrayIterator tmp(nbits);
  tmp.n = tmp.maxN;
  return tmp;
}
