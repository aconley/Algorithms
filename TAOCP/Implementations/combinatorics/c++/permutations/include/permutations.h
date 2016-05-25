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
    // Quick return cases
    if (n == 0) {
      return;
    }
    if (n == 1) {
      vis.visit(start, end);
      return;
    }
    if (n == 2) {
      if (vis.visit(start, end) && iter_lt(start, start+1)) {
         std::iter_swap(start, start+1);
         vis.visit(start, end);
      }
      return;
    }

    // Now we know n >= 3, and can use the optimized version
    //  of problem 7.2.1.2.(1)
    RandomIt x, y, z;
    while (true) {
      if (!vis.visit(start, end)) {
        return;
      }

      // Easiest case
      z = end - 1;
      y = z - 1;
      if (iter_lt(y, z)) {
        std::iter_swap(y, z);
        continue;
      }

      // Next easiest case
      x = y - 1;
      if (iter_lt(x, y)) {
        if (iter_lt(x, z)) {
          std::iter_swap(x, z);
          std::iter_swap(y, z);
        } else {
          std::iter_swap(x, z);
          std::iter_swap(x, y);
        }
        continue;
      }

      for (; y >= start && !iter_lt(y, y + 1); --y) {}

      // termination test
      if (y < start) {
        return;
      }

      for (z = end - 1; !iter_lt(y, z); --z) {}
      std::iter_swap(y, z);

      std::reverse(y+1, end);
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
  if (n == 1) {
      vis.visit(start, end);
      return;
  }
  if (vis.visit(start, end) && n == 2) {
    if (vis.visit(start, end)) {
      std::iter_swap(start, start+1);
      vis.visit(start, end);
    }
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

// Visit all the permutations using Heaps method
//
// This is Algorithm G of Knuth Volum 4A 7.2.1.2
//  using the permutation of 7.2.1.2.(27)
//
// Visitor must implement a method
//     bool visit(const RandomIt& start, const RandomIt& end)
//  where RandomIt satisfies RandomAccessIterator
//  and ValueSwappable
template<class RandomIt, class Visitor>
void heap(RandomIt start, RandomIt end, Visitor& vis) {
  auto n = std::distance(start, end);
  if (n == 0) {
    return;
  }
  if (n == 1) {
      vis.visit(start, end);
      return;
  }
  if (vis.visit(start, end) && n == 2) {
    if (vis.visit(start, end)) {
      std::iter_swap(start, start+1);
      vis.visit(start, end);
    }
    return;
  }

  int* c = new int[n + 1];
  int k;
  for (k = 1; k < n; ++k)
    c[k] = 0;

  while (true) {
    k = 1;
    while (c[k] == k)
      c[k++] = 0;
    if (k == n) {
      delete[] c;
      return;
    }
    ++c[k];
    if ((k & 1) == 0) {
      std::iter_swap(start, start + k);
    } else {
      std::iter_swap(start + k, start + c[k] - 1);
    }
    if (!vis.visit(start, end)) {
      delete[] c;
      return;
    }
  }

}

} // namespace

#endif