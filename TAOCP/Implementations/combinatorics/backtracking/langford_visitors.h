#ifndef __langford_visitors_h__
#define __langford_visitors_h__

#include<array>
#include<cstdlib>

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

// Records balanced langford pairs; see Knuth 4A page 2-3.
template<std::size_t n> class LangfordBalancedVisitor {
  private:
    std::vector<std::array<int, 2 * n>> solutions;

    bool isSolution(const std::array<int, 2 * n>& a) {
      // We work with twice the distance so things stay integer
      int moment = 2 * n - 1;
      int leftSum = moment * std::abs(a[0]);
      for (int i = 1; i < n; ++i) {
        moment -= 2;
        leftSum += moment * std::abs(a[i]);
      }

      moment = 1;
      int rightSum = std::abs(a[n]);
      for (int i = n + 1; i < 2 * n; ++i) {
        moment += 2;
        rightSum += moment * std::abs(a[i]);
      }
      return leftSum == rightSum;
    }

  public:
    bool visit(const std::array<int, 2 * n>& sol) {
      if (isSolution(sol)) {
        std::array<int, 2 * n> v(sol);
        solutions.push_back(v);
      }
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

}
#endif
