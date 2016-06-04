#include<stdexcept>

#include "binarygrey.h"

using namespace ntuples;

////////// BinaryGrey //////////////

BinaryGrey::BinaryGrey(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = nbits;
}

int BinaryGrey::getNBits() const {
  return _nbits;
}

BinaryGreyIterator BinaryGrey::begin() const {
  return BinaryGreyIterator::begin(_nbits);
}

BinaryGreyIterator BinaryGrey::end() const {
  return BinaryGreyIterator::end(_nbits);
}

std::uint32_t BinaryGrey::getNext(std::uint32_t g) {
  // Convert back to place, increment, re-convert
  std::uint32_t b = g^(g >> 1);
  b = b^(b >> 2);
  b = b^(b >> 4);
  b = b^(b >> 8);
  b = b^(b >> 16);
  b += 1;
  return b^(b >> 1);
}

////////// BinaryGreyIterator //////////////

BinaryGreyIterator::BinaryGreyIterator(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  maxN = 1u << nbits;
  n = 0u;
}

BinaryGreyIterator& BinaryGreyIterator::operator++() {
  ++n;
  return *this;
}

BinaryGreyIterator BinaryGreyIterator::operator++(int) {
  BinaryGreyIterator tmp(*this);
  operator++();
  return tmp;
}

bool BinaryGreyIterator::operator==(const BinaryGreyIterator& rhs) const {
  return maxN == rhs.maxN && n == rhs.n;
}

bool BinaryGreyIterator::operator!=(const BinaryGreyIterator& rhs) const {
  return maxN != rhs.maxN || n != rhs.n;
}

std::uint32_t BinaryGreyIterator::operator*() const {
  return n^(n >> 1);
}

BinaryGreyIterator BinaryGreyIterator::begin(int nbits) {
  return BinaryGreyIterator(nbits);
}

BinaryGreyIterator BinaryGreyIterator::end(int nbits) {
  BinaryGreyIterator tmp(nbits);
  tmp.n = tmp.maxN;
  return tmp;
}
