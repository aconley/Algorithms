#ifndef __nqueens_iterator_h__
#define __nqueens_iterator_h__

#include<array>
#include<cstdint>
#include<iterator>
#include<vector>

// Visits all NQueens solutions using Walkers method.
namespace backtracking {
class NQueensWalker {
  public:
    class iterator: public std::iterator<
      std::input_iterator_tag,  // Iterator category
      const std::vector<std::uint8_t>, // value type
      long, // distance type
      const std::vector<std::uint8_t>*, // pointer type
      const std::vector<std::uint8_t>&> { // reference type

      public:
        explicit iterator(int, bool);
        iterator& operator++();
        iterator operator++(int);
        value_type operator*() const;
        bool operator==(const iterator&);
        bool operator!=(const iterator&);
        difference_type operator-(const iterator&);
      private:
        static const std::array<std::uint8_t, 32> MultiplyDeBruijnBitPosition;

        std::uint8_t getPositionOfLeastSetBit(std::uint32_t v) const;

        bool done;
        long ctr;
        int n, l;
        std::vector<std::uint32_t> a, b, c, s;
        std::uint32_t mu, t;
      };

    explicit NQueensWalker(int);
    
    iterator begin() const;
    iterator end() const;
  
  private:
    int n;
};
}
#endif