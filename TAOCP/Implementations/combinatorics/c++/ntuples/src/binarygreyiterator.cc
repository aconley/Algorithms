#include<stdexcept>

#include "binarygreyiterator.h"

using namespace ntuples;

BinaryGreyIterator::BinaryGreyIterator(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  maxN = 1u << nbits;
  n = 0;
}

BinaryGreyIterator& BinaryGreyIterator::operator++() {
  ++n; return *this;
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
unsigned int BinaryGreyIterator::operator*() const {
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
