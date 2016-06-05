#ifndef __looplessgray__
#define __looplessgray__

#include<iterator>
#include<cstdint>

namespace ntuples {

class LooplessGrayIterator;

/**
  Generates Gray code values using a loopless algorithm.

  See Knuth v4 7.2.1.1 Algorithm L
*/
class LooplessGray {
  public:
    explicit LooplessGray(int nbits);
    LooplessGray(const LooplessGray& other) = default;
    LooplessGray(LooplessGray&& other) = default;

    typedef LooplessGrayIterator iterator;

    LooplessGrayIterator begin() const;
    LooplessGrayIterator end() const;

    int getNBits() const;

  private:
    int _nbits;
};

class LooplessGrayIterator final :
  public std::iterator<std::forward_iterator_tag, const std::uint32_t> {

  public:
    explicit LooplessGrayIterator(int nbits);
    ~LooplessGrayIterator(); // final class, doesn't need to be virtual
    LooplessGrayIterator(const LooplessGrayIterator& other);
    LooplessGrayIterator(LooplessGrayIterator&& other);

    LooplessGrayIterator& operator++();
    LooplessGrayIterator operator++(int);
    bool operator==(const LooplessGrayIterator& rhs) const;
    bool operator!=(const LooplessGrayIterator& rhs) const;
    std::uint32_t operator*() const;

    static LooplessGrayIterator begin(int nbits);
    static LooplessGrayIterator end(int nbits);
  private:
    std::uint8_t _nbits;
    // We need some way to indicate that we are done
    bool _done;
    // Gray value
    std::uint32_t _state;
    // Focus pointers
    std::uint8_t* _focus;
};

}

#endif