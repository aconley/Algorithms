#ifndef __nqueens_iterator_h__
#define __nqueens_iterator_h__

#include<cstdint>
#include<iterator>
#include<vector>

class NQueensIterator {
  
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
        value_type operator*();
        bool operator==(iterator);
        bool operator!=(iterator);
      private:
        static constexpr std::uint8_t MultiplyDeBruijnBitPosition[32] =
          {
            0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
            31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
          };

        std::uint8_t getPositionOfLeastSetBit(std::uint32_t v);

        bool done;
        long ctr;
        int n, l;
        std::vector<std::uint32_t> a, b, c, s;
        std::uint32_t mu;
      };

    explicit NQueensIterator(int);
    
    iterator begin();
    iterator end();
  
  private:
    int n;
};

#endif