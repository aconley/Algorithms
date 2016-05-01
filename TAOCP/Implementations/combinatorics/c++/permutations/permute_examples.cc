#include<iostream>
#include<vector>
#include "permutations/permutations.h"

typedef std::vector<int>::iterator vecIt;
class CountingVisitor {
  private:
    int n;
  public:
    CountingVisitor() {
      n = 0;
    }

    bool visit(const vecIt& start, const vecIt& end) {
      ++n;
      return true;
    }

    void reset() {
      n = 0;
    }

    int getN() const {
      return n;
    }
};

int main() {

  std::vector<int> testVec = {1, 2, 2, 4};
  CountingVisitor v;

  permutations::lexicographic(testVec.begin(), testVec.end(), v);
  std::cout << "There were " << v.getN() << " lexicographic permutations\n";

  v.reset();
  permutations::plain(testVec.begin(), testVec.end(), v);
  std::cout << "There were " << v.getN() << " plain permutations\n";


  return 0;
}