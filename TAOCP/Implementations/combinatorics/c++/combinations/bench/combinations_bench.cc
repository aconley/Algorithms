#include "benchmark/benchmark.h"
#include "combinations.h"

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

// Lex
static void BM_Combinations_Basic_14_4(benchmark::State& state) {
  CountingVisitor<4> vis4;
  while (state.KeepRunning()) {
    combinations::combinations_lex_basic(14, vis4);
  }
}
BENCHMARK(BM_Combinations_Basic_14_4);

static void BM_Combinations_Basic_16_5(benchmark::State& state) {
  CountingVisitor<5> vis5;
  while (state.KeepRunning()) {
    combinations::combinations_lex_basic(16, vis5);
  }
}
BENCHMARK(BM_Combinations_Basic_16_5);

// Optimizied
static void BM_Combinations_14_4(benchmark::State& state) {
  CountingVisitor<4> vis4;
  while (state.KeepRunning()) {
    combinations::combinations_lex(14, vis4);
  }
}
BENCHMARK(BM_Combinations_14_4);

static void BM_Combinations_16_5(benchmark::State& state) {
  CountingVisitor<5> vis5;
  while (state.KeepRunning()) {
    combinations::combinations_lex(16, vis5);
  }
}
BENCHMARK(BM_Combinations_16_5);

// Grey code / Revolving door
static void BM_Combinations_Grey_14_4(benchmark::State& state) {
  CountingVisitor<4> vis4;
  while (state.KeepRunning()) {
    combinations::combinations_grey(14, vis4);
  }
}
BENCHMARK(BM_Combinations_Grey_14_4);

static void BM_Combinations_Grey_16_5(benchmark::State& state) {
  CountingVisitor<5> vis5;
  while (state.KeepRunning()) {
    combinations::combinations_grey(16, vis5);
  }
}
BENCHMARK(BM_Combinations_Grey_16_5);

BENCHMARK_MAIN();
