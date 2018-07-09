#include<exception>
#include<limits>
#include<iostream>

#include "nqueens_bitwise_loop.h"
#include "nqueens_constants.h"

using namespace backtracking;

NQueensBitwiseLoop::NQueensBitwiseLoop(int _n) : n(_n) {}

NQueensBitwiseLoop::iterator NQueensBitwiseLoop::begin() const { 
  return iterator(n, false); 
}

NQueensBitwiseLoop::iterator NQueensBitwiseLoop::end() const { 
  return iterator(n, true); 
}

NQueensBitwiseLoop::iterator::iterator(int _n, bool _done) : 
  action(down), done(_done), a(0), b(0), c(0),  n(_n), t(0), 
  max_t(_n - 1), l(0), nm1(_n - 1) {
        
  if (n < 1 || n > max_n)
    throw std::invalid_argument("n must be in range [1, 24].");
  ctr = done ? n_solutions[n] : 0;
  rows = std::vector<uint8_t>(n);

  // Move to the first value.
  operator++();
}

NQueensBitwiseLoop::iterator& NQueensBitwiseLoop::iterator::operator++() {
  if (done) {
    return *this;
  }
  
  while (true) {
    if (action == across) {
      // Try to increase x_l
      if (t < max_t) {
        ++t;
        action = down;
      } else {
        // Backtrack
        --l;
        if (l < 0) break;

        t = rows[l];
        a &= ~(1 << t);
        b &= ~(1 << (t + l));
        c &= ~(1 << (t - l + nm1));
      }
    } else {
      // Test P_l(x_0, ..., x_l)
      rows[l] = t;
      if ((a & (1 << t)) == 0
        && (b & (1 << (t + l))) == 0
        && (c & (1 << (t - l + nm1))) == 0) {
        // Good step
        rows[l] = t;
        // Check for solution found
        if (l == nm1) {
          action = across;
          return *this;
        }

        // Update data structures to try l + 1
        a |= (1 << t);
        b |= (1 << (t + l));
        c |= (1 << (t - l + nm1));

        // x_{l+1} = min D_{l+1}
        t = 0;

        ++l;
      } else {
        action = across;
      }
    }
  }
  done = true;
  return *this;
}

NQueensBitwiseLoop::iterator::value_type 
  NQueensBitwiseLoop::iterator::operator*() const {
  return rows;
}

NQueensBitwiseLoop::iterator NQueensBitwiseLoop::iterator::operator++(int) {
  iterator retval = *this; 
  ++(*this); 
  return retval;
}

bool NQueensBitwiseLoop::iterator::operator==(const iterator& other) {
  if (n != other.n) return false;
  if (done != other.done) return false;
  return done ? other.done : ctr == other.ctr;
}

bool NQueensBitwiseLoop::iterator::operator!=(const iterator& other) {
  return !(*this == other);
}

NQueensBitwiseLoop::iterator::difference_type 
  NQueensBitwiseLoop::iterator::operator-(const iterator& other) {
  return ctr - other.ctr;
}