#ifndef __gray__
#define __gray__

#include<iterator>
#include<cstdint>

namespace ntuples {

class GrayIterator;

/**
  Generates Gray code values

  See Knuth v4 7.2.1.1 Algorithm G
*/
class Gray {
  public:
    explicit Gray(int nbits);
    Gray(const Gray& other) = default;
    Gray(Gray&& other) = default;

    typedef GrayIterator iterator;

    GrayIterator begin() const;
    GrayIterator end() const;

    int getNBits() const;

  private:
    int _nbits;
};

class GrayIterator final :
  public std::iterator<std::forward_iterator_tag, const std::uint32_t> {

  public:
    explicit GrayIterator(int nbits);
    GrayIterator(const GrayIterator& other) = default;
    GrayIterator(GrayIterator&& other) = default;

    GrayIterator& operator++();
    GrayIterator operator++(int);
    bool operator==(const GrayIterator& rhs) const;
    bool operator!=(const GrayIterator& rhs) const;
    std::uint32_t operator*() const;

    static GrayIterator begin(int nbits);
    static GrayIterator end(int nbits);
  private:
    std::uint8_t _nbits;
    // We need some way to indicate that we are done,
    bool _done;
    // State
    std::uint32_t _state;
    // Parity bit
    bool _ainf;

    static std::uint8_t ntz(std::uint32_t v);
};

}

#endif