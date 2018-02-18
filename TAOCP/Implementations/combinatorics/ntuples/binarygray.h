#ifndef __binarygray__
#define __binarygray__

#include<iterator>
#include<cstdint>

namespace ntuples {

class BinaryGrayIterator;

/**
  Generates Gray code values using bit twiddling.
*/
class BinaryGray {
  public:
    explicit BinaryGray(int nbits);
    BinaryGray(const BinaryGray& other) = default;
    BinaryGray(BinaryGray&& other) = default;

    typedef BinaryGrayIterator iterator;

    BinaryGrayIterator begin() const;
    BinaryGrayIterator end() const;

    int getNBits() const;

    // Get the successor pattern to the provided one.
    static std::uint32_t getNext(std::uint32_t g);
  private:
    int _nbits;
};

/**
  BinaryGrayIterator is an iterator over Gray Codes with a specified
  number of bits [1, 31].

  For example:
  for ()
*/
class BinaryGrayIterator :
    public std::iterator<std::forward_iterator_tag, const std::uint32_t> {

  public:
    explicit BinaryGrayIterator(int nbits);
    BinaryGrayIterator(const BinaryGrayIterator& other) = default;
    BinaryGrayIterator(BinaryGrayIterator&& other) = default;

    BinaryGrayIterator& operator++();
    BinaryGrayIterator operator++(int);
    bool operator==(const BinaryGrayIterator& rhs) const;
    bool operator!=(const BinaryGrayIterator& rhs) const;
    std::uint32_t operator*() const;

    static BinaryGrayIterator begin(int nbits);
    static BinaryGrayIterator end(int nbits);
  private:
    std::uint32_t maxN; // Number allowed
    std::uint32_t n;    // current counter

    inline unsigned int getState() const;
};

}
#endif
