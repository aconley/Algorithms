#include<stdexcept>
#include<iostream>

#include "gray.h"

using namespace ntuples;

////////// Gray //////////////

Gray::Gray(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = nbits;
}

int Gray::getNBits() const {
  return _nbits;
}

GrayIterator Gray::begin() const {
  return GrayIterator::begin(_nbits);
}

GrayIterator Gray::end() const {
  return GrayIterator::end(_nbits);
}



////////// GrayIterator //////////////

GrayIterator::GrayIterator(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _done = false;
  _nbits = static_cast<std::uint8_t>(nbits);
  _state = 0u;
  _ainf = false;
}

GrayIterator& GrayIterator::operator++() {
  if (_done) return *this;
  _ainf = ! _ainf;
  if (_ainf) {
    _state ^= 1u;
  } else {
    std::uint8_t j = ntz(_state) + 1;
    if (j >= _nbits) {
      _done = true;
      return *this;
    }
    _state ^= (1u << j);
  }
  return *this;
}

GrayIterator GrayIterator::operator++(int) {
  GrayIterator tmp(*this);
  operator++();
  return tmp;
}

bool GrayIterator::operator==(const GrayIterator& rhs) const {
  return _done == rhs._done && _nbits == rhs._nbits && _state == rhs._state;
}

bool GrayIterator::operator!=(const GrayIterator& rhs) const {
  return _done != rhs._done || _nbits != rhs._nbits || _state != rhs._state;
}

std::uint32_t GrayIterator::operator*() const {
  return _state;
}

GrayIterator GrayIterator::begin(int nbits) {
  return GrayIterator(nbits);
}

GrayIterator GrayIterator::end(int nbits) {
  GrayIterator tmp(nbits);
  tmp._done = true;
  tmp._state = 1u << (nbits - 1);
  return tmp;
}

// Number of trailing zeros, 5-21 from Hackers Delight
/*
std::uint8_t GrayIterator::ntz(std::uint32_t x) {
  unsigned y;
  if (x == 0) return 32;
  std::uint8_t n = 31;
  y = x << 16; if (y != 0) { n -= 16; x = y; }
  y = x <<  8; if (y != 0) { n -=  8; x = y; }
  y = x <<  4; if (y != 0) { n -=  4; x = y; }
  y = x <<  2; if (y != 0) { n -=  2; x = y; }
  y = x <<  1; if (y != 0) { n -=  1; }
  return n;
}
*/

// Number of trailing zeros, 5-23 from Hackers Delight
std::uint8_t GrayIterator::ntz(std::uint32_t x) {
  std::uint8_t n = 0;
  x = ~x & (x - 1);
  while (x != 0) {
    ++n;
    x >>= 1;
  }
  return n;
}