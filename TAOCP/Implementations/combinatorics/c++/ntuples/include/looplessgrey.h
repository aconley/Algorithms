#ifndef __looplessgrey__
#define __looplessgrey__

#include<iterator>
#include<cstdint>

namespace ntuples {

class LooplessGreyIterator;

/**
  Generates Grey code values using a loopless algorithm.

  See Knuth v4 7.2.1.1 Algorithm L
*/
class LooplessGrey {
  public:
    explicit LooplessGrey(int nbits);
    LooplessGrey(const LooplessGrey& other) = default;
    LooplessGrey(LooplessGrey&& other) = default;

    typedef LooplessGreyIterator iterator;

    LooplessGreyIterator begin() const;
    LooplessGreyIterator end() const;

    int getNBits() const;

  private:
    int _nbits;
};

class LooplessGreyIterator final :
  public std::iterator<std::forward_iterator_tag, const std::uint32_t> {

  public:
    explicit LooplessGreyIterator(int nbits);
    ~LooplessGreyIterator(); // final class, doesn't need to be virtual
    LooplessGreyIterator(const LooplessGreyIterator& other);
    LooplessGreyIterator(LooplessGreyIterator&& other);

    LooplessGreyIterator& operator++();
    LooplessGreyIterator operator++(int);
    bool operator==(const LooplessGreyIterator& rhs) const;
    bool operator!=(const LooplessGreyIterator& rhs) const;
    std::uint32_t operator*() const;

    static LooplessGreyIterator begin(int nbits);
    static LooplessGreyIterator end(int nbits);
  private:
    std::uint8_t _nbits;
    // We need some way to indicate that we are done
    bool _done;
    // Grey value
    std::uint32_t _state;
    // Focus pointers
    std::uint8_t* _focus;
};

}

#endif