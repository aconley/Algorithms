#include<algorithm>
#include<iterator>

template<class Iterator> bool iter_lt(const Iterator& a, const Iterator& b) {
  return (*a) < (*b);
}


// Visit all the permutations lexicographically
//  Visitor must implement a method
//     bool visit(const RandomIt& start, const RandomIt& end)
//  where RandomIt satisfies RandomAccessIterator
//  and ValueSwappable
template<class RandomIt, class Visitor>
  void lexicographic(RandomIt start, RandomIt end, Visitor& vis) {

    int n = std::distance(start, end);
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
