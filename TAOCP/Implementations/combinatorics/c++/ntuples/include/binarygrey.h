#ifndef __binarygreyiterator__
#define __binarygreyiterator__

#include<iterator>
#include<cstdint>

namespace ntuples {

class BinaryGreyIterator;

class BinaryGrey {
  public:
    explicit BinaryGrey(int nbits);
    BinaryGrey(const BinaryGrey& other) = default;
    BinaryGrey(BinaryGrey&& other) = default;

    typedef BinaryGreyIterator iterator;

    BinaryGreyIterator begin() const;
    BinaryGreyIterator end() const;

    int getNBits() const;

    // Get the successor pattern to the provided one.
    static std::uint32_t getNext(std::uint32_t g);
  private:
    int _nbits;
};

/**
  BinaryGreyIterator is an iterator over Grey Codes with a specified
  number of bits [1, 31].

  For example:
  for ()
*/
class BinaryGreyIterator :
    public std::iterator<std::forward_iterator_tag, const std::uint32_t> {

  public:
    explicit BinaryGreyIterator(int nbits);
    BinaryGreyIterator(const BinaryGreyIterator& other) = default;
    BinaryGreyIterator(BinaryGreyIterator&& other) = default;

    BinaryGreyIterator& operator++();
    BinaryGreyIterator operator++(int);
    bool operator==(const BinaryGreyIterator& rhs) const;
    bool operator!=(const BinaryGreyIterator& rhs) const;
    std::uint32_t operator*() const;

    static BinaryGreyIterator begin(int nbits);
    static BinaryGreyIterator end(int nbits);
  private:
    std::uint32_t maxN; // Number allowed
    std::uint32_t n;    // current counter

    inline unsigned int getState() const;
};

}
#endif
