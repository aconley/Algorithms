#include "benchmark/benchmark.h"
#include "binarygrey.h"
#include "looplessgrey.h"

using namespace ntuples;

const int n = 18;

// Comparison of pure counting
static void BM_PureCount(benchmark::State& state) {

  unsigned int nmax = 1u << n;
  unsigned int nfound;
  while (state.KeepRunning()) {
    nfound = 0;
    for (unsigned int i = 0; i < nmax; ++i)
      benchmark::DoNotOptimize(++nfound);
  }
}
BENCHMARK(BM_PureCount);

static void BM_BinaryGrey(benchmark::State& state) {
  BinaryGrey grey(n);

  unsigned int nfound;
  while (state.KeepRunning()) {
    nfound = 0;
    for (auto g : grey)
      benchmark::DoNotOptimize(++nfound);
  }
}
BENCHMARK(BM_BinaryGrey);

static void BM_LooplessGrey(benchmark::State& state) {
  LooplessGrey grey(n);

  unsigned int nfound;
  while (state.KeepRunning()) {
    nfound = 0;
    for (auto g : grey)
      benchmark::DoNotOptimize(++nfound);
  }
}
BENCHMARK(BM_LooplessGrey);

BENCHMARK_MAIN();
