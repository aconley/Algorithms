#include "benchmark/benchmark.h"
#include "langford.h"
#include "langford_visitors.h"

// Basic
static void BM_Langford_Basic7(benchmark::State& state) {
  backtracking::LangfordCountingVisitor<7> vis;
  while (state.KeepRunning()) {
    backtracking::langford_basic(vis);
  }
}
BENCHMARK(BM_Langford_Basic7);

static void BM_Langford_Basic11(benchmark::State& state) {
  backtracking::LangfordCountingVisitor<11> vis;
  while (state.KeepRunning()) {
    backtracking::langford_basic(vis);
  }
}
BENCHMARK(BM_Langford_Basic11);

// Optimized
static void BM_Langford7(benchmark::State& state) {
  backtracking::LangfordCountingVisitor<7> vis;
  while (state.KeepRunning()) {
    backtracking::langford(vis, true);
  }
}
BENCHMARK(BM_Langford7);

static void BM_Langford11(benchmark::State& state) {
  backtracking::LangfordCountingVisitor<11> vis;
  while (state.KeepRunning()) {
    backtracking::langford(vis, true);
  }
}
BENCHMARK(BM_Langford11);

BENCHMARK_MAIN();
