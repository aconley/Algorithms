#ifndef __permutations_h__
#define __permutations_h__

#include<algorithm>
#include<iterator>

namespace permutations {

template<class Iterator> bool iter_lt(const Iterator& a, const Iterator& b) {
  return (*a) < (*b);
}

// Visit all the permutations lexicographically
//
//  Visitor must implement a method
//     bool visit(const RandomIt& start, const RandomIt& end)
//  where RandomIt satisfies RandomAccessIterator
//  and ValueSwappable.
//
//  Note that exchanges of identical elements do not count
//  as distinct.  So, for example, {2, 2} has only one
//  permutation.
template<class RandomIt, class Visitor>
  void lexicographic(RandomIt start, RandomIt end, Visitor& vis) {

    auto n = std::distance(start, end);
    if (n == 0) {
      return;
    }

    if (n == 1) {
      vis.visit(start, end);
      return;
    }

    RandomIt j, l;
    while (true) {
      if (!vis.visit(start, end)) {
        return;
      }

      l = end - 1;
      j = end - 2;
      // Easy case
      if (iter_lt(j, l)) {
        std::iter_swap(j, l);
        continue;
      }

      for (; j >= start && !iter_lt(j, j + 1); --j) {}

      // termination test
      if (j < start) {
        return;
      }

      for (l = end - 1; !iter_lt(j, l); --l) {}
      std::iter_swap(j, l);

      std::reverse(j+1, end);
    }
}

// Visit all the permutations using plain changes
//
// That is, only swap adjacent elements to permute.
//
// Visitor must implement a method
//     bool visit(const RandomIt& start, const RandomIt& end)
//  where RandomIt satisfies RandomAccessIterator
//  and ValueSwappable
template<class RandomIt, class Visitor>
void plain(RandomIt start, RandomIt end, Visitor& vis) {

  auto n = std::distance(start, end);
  if (n == 0) {
    return;
  }

  if (!vis.visit(start, end) || n == 1) {
    return;
  }

  int* c = new int[n];
  for (int i = 0; i < n; ++i)
    c[i] = 0;
  int* o = new int[n];
  for (int i = 0; i < n; ++i)
    o[i] = 1;

  int j, q, s;
  while (true) {
    j = n - 1;
    s = 0;
    while (true) {
      q = c[j] + o[j];
      if (q < 0) {
        o[j] = -o[j];
        --j;
      } else if (q == j + 1) {
        if (j == 0) {
          // All done
          delete[] o;
          delete[] c;
          return;
        }
        ++s;
        o[j] = -o[j];
        --j;
      } else {
        std::iter_swap(start + j - c[j] + s, start + j - q + s);
        if (!vis.visit(start, end)) {
          delete[] o;
          delete[] c;
          return;
        }
        c[j] = q;
        break;
      }
    }
  }


}

} // namespace

#endif