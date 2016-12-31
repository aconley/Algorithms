#ifndef __langford_visitors_h__
#define __langford_visitors_h__

#include<array>

namespace backtracking {

// Records all solutions
template<std::size_t n> class LangfordRecordingVisitor {
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

// Counts the solutions
template<std::size_t n> class LangfordCountingVisitor {
  private:
    int nsolutions;

  public:
    LangfordCountingVisitor() { nsolutions = 0; }

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

}
#endif
