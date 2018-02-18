#include<stdexcept>

#include "looplessgray.h"

using namespace ntuples;

////////// LooplessGray //////////////

LooplessGray::LooplessGray(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = nbits;
}

int LooplessGray::getNBits() const {
  return _nbits;
}

LooplessGrayIterator LooplessGray::begin() const {
  return LooplessGrayIterator::begin(_nbits);
}

LooplessGrayIterator LooplessGray::end() const {
  return LooplessGrayIterator::end(_nbits);
}

////////// LooplessGrayIterator //////////////

LooplessGrayIterator::LooplessGrayIterator(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = static_cast<std::uint8_t>(nbits);
  _state = 0u;
  _done = false;
  _focus = new std::uint8_t[nbits];
  for (std::uint8_t i = 0u; i <= _nbits; ++i)
    _focus[i] = i;
}

LooplessGrayIterator::LooplessGrayIterator(const LooplessGrayIterator& other) {
  _nbits = other._nbits;
  _state = other._state;
  _done = other._done;
  _focus = new std::uint8_t[_nbits];
  for (std::uint8_t i = 0u; i <= _nbits; ++i)
    _focus[i] = other._focus[i];
}

LooplessGrayIterator::LooplessGrayIterator(LooplessGrayIterator&& other) {
  _nbits = other._nbits;
  _state = other._state;
  _done = other._done;
  _focus = std::move(other._focus);
}

LooplessGrayIterator::~LooplessGrayIterator() {
  delete[] _focus;
}

LooplessGrayIterator& LooplessGrayIterator::operator++() {
  if (_done) return *this;
  std::uint8_t j = _focus[0];
  if (j < _nbits) {
    _focus[0] = 0;
    _focus[j] = _focus[j + 1];
    _focus[j + 1] = j + 1;
    _state ^= (1u << j);
  } else {
    _done = true;
  }
  return *this;
}

LooplessGrayIterator LooplessGrayIterator::operator++(int) {
  LooplessGrayIterator tmp(*this);
  operator++();
  return tmp;
}

bool LooplessGrayIterator::operator==(const LooplessGrayIterator& rhs) const {
  return _done == rhs._done && _nbits == rhs._nbits && _state == rhs._state;
}

bool LooplessGrayIterator::operator!=(const LooplessGrayIterator& rhs) const {
  return _done != rhs._done || _nbits != rhs._nbits || _state != rhs._state;
}

std::uint32_t LooplessGrayIterator::operator*() const {
  return _state;
}

LooplessGrayIterator LooplessGrayIterator::begin(int nbits) {
  return LooplessGrayIterator(nbits);
}

LooplessGrayIterator LooplessGrayIterator::end(int nbits) {
  LooplessGrayIterator tmp(nbits);
  tmp._done = true;
  tmp._state = 1u << (nbits - 1);
  return tmp;
}
