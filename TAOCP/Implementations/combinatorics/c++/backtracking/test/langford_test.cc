#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "langford.h"

template<std::size_t n> class RecordingVisitor {
  private:
    std::vector<std::array<int, 2 * n>> solutions;

  public:
    bool visit(const std::array<int, 2 * n>& sol) {
      std::array<int, 2 * n> v(sol);
      solutions.push_back(v);
      return true;
    }

    void reset() {
      solutions.clear();
    }

    int getN() const {
      return solutions.size();
    }

    const std::array<int, 2 * n>& get(int i) {
      return solutions.at(i);
    }
};

template<std::size_t n> class CountingVisitor {
  private:
    int nsolutions;

  public:
    CountingVisitor() { nsolutions = 0; }

    bool visit(const std::array<int, 2 * n>& rows) {
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
