#include "benchmark/benchmark.h"
#include "nqueens_basic.h"
#include "nqueens_array.h"
#include "nqueens_bitwise.h"
#include "nqueens_iterative.h"
#include "nqueens_walker.h"
#include "nqueens_iterator.h"

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

// Basic
static void BM_Nqueens_Basic8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_basic(v);
  }
}
BENCHMARK(BM_Nqueens_Basic8);

static void BM_Nqueens_Basic10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_basic(v);
  }
}
BENCHMARK(BM_Nqueens_Basic10);

// Array
static void BM_Nqueens_Array8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array8);

static void BM_Nqueens_Array10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array10);

static void BM_Nqueens_Array13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array13);

// Bitwise
static void BM_Nqueens_Bitwise8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_bitwise(v);
  }
}
BENCHMARK(BM_Nqueens_Bitwise8);

static void BM_Nqueens_Bitwise10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_bitwise(v);
  }
}
BENCHMARK(BM_Nqueens_Bitwise10);

static void BM_Nqueens_Bitwise13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_bitwise(v);
  }
}
BENCHMARK(BM_Nqueens_Bitwise13);

// Bitwise Iterative
static void BM_Nqueens_BitwiseIterative8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_iterative(v);
  }
}
BENCHMARK(BM_Nqueens_BitwiseIterative8);

static void BM_Nqueens_BitwiseIterative10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_iterative(v);
  }
}
BENCHMARK(BM_Nqueens_BitwiseIterative10);

static void BM_Nqueens_BitwiseIterative13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_iterative(v);
  }
}
BENCHMARK(BM_Nqueens_BitwiseIterative13);

// Walker
static void BM_Nqueens_Walker8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_walker(v);
  }
}
BENCHMARK(BM_Nqueens_Walker8);

static void BM_Nqueens_Walker10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_walker(v);
  }
}
BENCHMARK(BM_Nqueens_Walker10);

static void BM_Nqueens_Walker13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    backtracking::nqueens_walker(v);
  }
}
BENCHMARK(BM_Nqueens_Walker13);

// Iterator
long count_solutions(const NQueensIterator& nq) {
  long n_solutions = 0;
  for (NQueensIterator::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

static void BM_Nqueens_Iterator8(benchmark::State& state) {
  NQueensIterator nq(8);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Iterator8);

static void BM_Nqueens_Iterator10(benchmark::State& state) {
  NQueensIterator nq(10);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Iterator10);

static void BM_Nqueens_Iterator13(benchmark::State& state) {
  NQueensIterator nq(13);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Iterator13);

BENCHMARK_MAIN();