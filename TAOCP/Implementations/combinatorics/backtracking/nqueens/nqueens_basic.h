#ifndef __nqueens_basic_h__
#define __nqueens_basic_h__

#include<array>
#include<cstdint>
#include<iterator>
#include<vector>

// Visits all NQueens solutions.
//
// Basic, unoptimized implementation.
namespace backtracking {
class NQueensBasic {
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
        bool done;
        long ctr;
        std::uint8_t n, x_l;
        int l, nm1;
        std::vector<std::uint8_t> rows;
      };

    explicit NQueensBasic(int);
    
    iterator begin() const;
    iterator end() const;
  
  private:
    int n;
};
}
#endif