#ifndef __nqueens_visitors_h__
#define __nqueens_visitors_h__

#include<array>
#include<vector>

namespace backtracking {
template<std::size_t n> class RecordingVisitor {
  private:
    std::vector<std::array<int, n>> solutions;

  public:
    bool visit(const std::array<int, n>& rows) {
      std::array<int, n> v(rows);
      solutions.push_back(v);
      return true;
    }

    void reset() {
      solutions.clear();
    }

    int getN() const {
      return solutions.size();
    }

    const std::array<int, n>& get(int i) {
      return solutions.at(i);
    }
};

template<std::size_t n> class CountingVisitor {
  private:
    int nsolutions;

  public:
    CountingVisitor() { nsolutions = 0; }

    bool visit(const std::array<int, n>& rows) {
      ++nsolutions;
      return true;
    }

    void reset() {
      nsolutions = 0;
    }

    int getN() const {
      return nsolutions;
    }
};
}
#endif