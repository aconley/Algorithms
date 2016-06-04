#include "benchmark/benchmark.h"
#include "binarygreyiterator.h"

using namespace ntuples;

static void BM_BinaryGrey(benchmark::State& state) {
  int n = 20;
  auto start = BinaryGreyIterator::begin(n);
  auto end = BinaryGreyIterator::end(n);

  unsigned int nfound;
  while (state.KeepRunning()) {
    nfound = 0;
    for (auto it = start; it != end; ++it)
      ++nfound;
  }
}
BENCHMARK(BM_BinaryGrey);

BENCHMARK_MAIN();
