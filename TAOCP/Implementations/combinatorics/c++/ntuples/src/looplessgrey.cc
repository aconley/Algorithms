#include<stdexcept>

#include "looplessgrey.h"

using namespace ntuples;

////////// LooplessGrey //////////////

LooplessGrey::LooplessGrey(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = nbits;
}

int LooplessGrey::getNBits() const {
  return _nbits;
}

LooplessGreyIterator LooplessGrey::begin() const {
  return LooplessGreyIterator::begin(_nbits);
}

LooplessGreyIterator LooplessGrey::end() const {
  return LooplessGreyIterator::end(_nbits);
}

////////// LooplessGreyIterator //////////////

LooplessGreyIterator::LooplessGreyIterator(int nbits) {
  if (nbits > 31 || nbits <= 0)
    throw std::invalid_argument("Invalid nbits -- must be [1, 31]");
  _nbits = static_cast<std::uint8_t>(nbits);
  _state = 0u;
  _done = false;
  _focus = new std::uint8_t[nbits];
  for (std::uint8_t i = 0u; i <= _nbits; ++i)
    _focus[i] = i;
}

LooplessGreyIterator::LooplessGreyIterator(const LooplessGreyIterator& other) {
  _nbits = other._nbits;
  _state = other._state;
  _done = other._done;
  _focus = new std::uint8_t[_nbits];
  for (std::uint8_t i = 0u; i <= _nbits; ++i)
    _focus[i] = other._focus[i];
}

LooplessGreyIterator::LooplessGreyIterator(LooplessGreyIterator&& other) {
  _nbits = other._nbits;
  _state = other._state;
  _done = other._done;
  _focus = std::move(other._focus);
}

LooplessGreyIterator::~LooplessGreyIterator() {
  delete[] _focus;
}

LooplessGreyIterator& LooplessGreyIterator::operator++() {
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

LooplessGreyIterator LooplessGreyIterator::operator++(int) {
  LooplessGreyIterator tmp(*this);
  operator++();
  return tmp;
}

bool LooplessGreyIterator::operator==(const LooplessGreyIterator& rhs) const {
  return _done == rhs._done && _nbits == rhs._nbits && _state == rhs._state;
}

bool LooplessGreyIterator::operator!=(const LooplessGreyIterator& rhs) const {
  return _done != rhs._done || _nbits != rhs._nbits || _state != rhs._state;
}

std::uint32_t LooplessGreyIterator::operator*() const {
  return _state;
}

LooplessGreyIterator LooplessGreyIterator::begin(int nbits) {
  return LooplessGreyIterator(nbits);
}

LooplessGreyIterator LooplessGreyIterator::end(int nbits) {
  LooplessGreyIterator tmp(nbits);
  tmp._done = true;
  tmp._state = 1u << (nbits - 1);
  return tmp;
}
