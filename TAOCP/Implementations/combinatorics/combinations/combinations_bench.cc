#include "benchmark/benchmark.h"
#include "combinations.h"
#include "combinations_gray.h"
#include "combinations_chase.h"

class CountingVisitor {
  private:
    int nsolutions;

  public:
    CountingVisitor() { nsolutions = 0; }

    bool visit(std::vector<int>::const_iterator begin,
      std::vector<int>::const_iterator end) {
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
  CountingVisitor vis4;
  while (state.KeepRunning()) {
    combinations::combinations_lex_basic(14, 4, vis4);
  }
}
BENCHMARK(BM_Combinations_Basic_14_4);

static void BM_Combinations_Basic_16_5(benchmark::State& state) {
  CountingVisitor vis5;
  while (state.KeepRunning()) {
    combinations::combinations_lex_basic(16, 5, vis5);
  }
}
BENCHMARK(BM_Combinations_Basic_16_5);

// Optimizied
static void BM_Combinations_14_4(benchmark::State& state) {
  CountingVisitor vis4;
  while (state.KeepRunning()) {
    combinations::combinations_lex(14, 4, vis4);
  }
}
BENCHMARK(BM_Combinations_14_4);

static void BM_Combinations_16_5(benchmark::State& state) {
  CountingVisitor vis5;
  while (state.KeepRunning()) {
    combinations::combinations_lex(16, 5, vis5);
  }
}
BENCHMARK(BM_Combinations_16_5);

// Grey code / Revolving door
static void BM_Combinations_Gray_14_4(benchmark::State& state) {
  CountingVisitor vis4;
  while (state.KeepRunning()) {
    combinations::combinations_gray(14, 4, vis4);
  }
}
BENCHMARK(BM_Combinations_Gray_14_4);

static void BM_Combinations_Gray_16_5(benchmark::State& state) {
  CountingVisitor vis5;
  while (state.KeepRunning()) {
    combinations::combinations_gray(16, 5, vis5);
  }
}
BENCHMARK(BM_Combinations_Gray_16_5);

// Chase sequence
static void BM_Combinations_Chase_14_4(benchmark::State& state) {
  CountingVisitor vis4;
  while (state.KeepRunning()) {
    combinations::combinations_chase(14, 4, vis4);
  }
}
BENCHMARK(BM_Combinations_Chase_14_4);

static void BM_Combinations_Chase_16_5(benchmark::State& state) {
  CountingVisitor vis5;
  while (state.KeepRunning()) {
    combinations::combinations_chase(16, 5, vis5);
  }
}
BENCHMARK(BM_Combinations_Chase_16_5);

BENCHMARK_MAIN();
