#include "benchmark/benchmark.h"
#include "nqueens.h"

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

static void BM_Nqueens_Basic8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_basic(v);
  }
}
BENCHMARK(BM_Nqueens_Basic8);

static void BM_Nqueens_Basic12(benchmark::State& state) {
  CountingVisitor<12> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_basic(v);
  }
}
BENCHMARK(BM_Nqueens_Basic12);

static void BM_Nqueens_Array8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array8);

static void BM_Nqueens_Array12(benchmark::State& state) {
  CountingVisitor<12> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array12);

BENCHMARK_MAIN();