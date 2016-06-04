#ifndef __binarygreyiterator__
#define __binarygreyiterator__

#include<iterator>

namespace ntuples {

class BinaryGreyIterator : public std::iterator<std::forward_iterator_tag, const unsigned int> {
  public:
    explicit BinaryGreyIterator(int nbits);
    BinaryGreyIterator(const BinaryGreyIterator& other) = default;
    BinaryGreyIterator(BinaryGreyIterator&& other) = default;

    BinaryGreyIterator& operator++();
    BinaryGreyIterator operator++(int);
    bool operator==(const BinaryGreyIterator& rhs) const;
    bool operator!=(const BinaryGreyIterator& rhs) const;
    unsigned int operator*() const;

    static BinaryGreyIterator begin(int nbits);
    static BinaryGreyIterator end(int nbits);
  private:
    unsigned int maxN; // Number allowed
    unsigned int n;    // current counter

    unsigned int getState() const;
};

}
#endif
