#include<vector>

#include "benchmark/benchmark.h"
#include "permutations.h"

typedef std::vector<int>::iterator vecIt;

class NoActionVisitor {
  public:
    bool visit(const vecIt& start, const vecIt& end) {
      return true;
    }
};

const std::vector<int> testVec{{0, 1, 2, 3, 4, 5, 6, 7}};

static void BM_Lexicographic(benchmark::State& state) {
  std::vector<int> testCopy(testVec.size());
  NoActionVisitor v;
  while (state.KeepRunning()) {
    testCopy = testVec;
    permutations::lexicographic(testCopy.begin(), testCopy.end(), v);
  }
}
BENCHMARK(BM_Lexicographic);

static void BM_Plain(benchmark::State& state) {
  std::vector<int> testCopy(testVec.size());
  NoActionVisitor v;
  while (state.KeepRunning()) {
    testCopy = testVec;
    permutations::plain(testCopy.begin(), testCopy.end(), v);
  }
}
BENCHMARK(BM_Plain);

BENCHMARK_MAIN();
