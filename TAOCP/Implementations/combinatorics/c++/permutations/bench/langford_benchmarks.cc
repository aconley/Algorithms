#include "benchmark/benchmark.h"
#include "langford.h"

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

// Basic
static void BM_Langford_Basic11(benchmark::State& state) {
  CountingVisitor<11> vis;
  while (state.KeepRunning()) {
    backtracking::langford_basic(v);
  }
}
BENCHMARK(BM_Langford_Basic11);

BENCHMARK_MAIN();
