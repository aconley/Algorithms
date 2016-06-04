#include "benchmark/benchmark.h"
#include "binarygrey.h"

using namespace ntuples;

static void BM_BinaryGrey(benchmark::State& state) {
  BinaryGrey grey(20);
  int n = 20;

  unsigned int nfound;
  while (state.KeepRunning()) {
    nfound = 0;
    for (auto g : grey)
      ++nfound;
  }
}
BENCHMARK(BM_BinaryGrey);

BENCHMARK_MAIN();
