#include "benchmark/benchmark.h"
#include "gray.h"
#include "binarygray.h"
#include "looplessgray.h"

#include<iterator>

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

static void BM_Gray(benchmark::State& state) {
  Gray gray(n);
  while (state.KeepRunning()) {
    auto nfound = std::distance(gray.begin(), gray.end());
  }
}
BENCHMARK(BM_Gray);

static void BM_BinaryGray(benchmark::State& state) {
  BinaryGray gray(n);
  while (state.KeepRunning()) {
    auto nfound = std::distance(gray.begin(), gray.end());
  }
}
BENCHMARK(BM_BinaryGray);

static void BM_LooplessGray(benchmark::State& state) {
  LooplessGray gray(n);
  while (state.KeepRunning()) {
    auto nfound = std::distance(gray.begin(), gray.end());
  }
}
BENCHMARK(BM_LooplessGray);

BENCHMARK_MAIN();
