#include "benchmark/benchmark.h"
#include "nqueens_basic.h"
#include "nqueens_array.h"
#include "nqueens_bitwise.h"
#include "nqueens_bitwise_loop.h"
#include "nqueens_iterative.h"
#include "nqueens_walker.h"

using namespace backtracking;

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
long count_solutions(const NQueensBasic& nq) {
  long n_solutions = 0;
  for (NQueensBasic::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

static void BM_Nqueens_Basic8(benchmark::State& state) {
  NQueensBasic nq(8);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Basic8);

static void BM_Nqueens_Basic10(benchmark::State& state) {
  NQueensBasic nq(10);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Basic10);

static void BM_Nqueens_Basic13(benchmark::State& state) {
  NQueensBasic nq(13);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Basic13);

// Array
static void BM_Nqueens_Array8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array8);

static void BM_Nqueens_Array10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array10);

static void BM_Nqueens_Array13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    nqueens_array(v);
  }
}
BENCHMARK(BM_Nqueens_Array13);

// Bitwise
static void BM_Nqueens_Bitwise8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    nqueens_bitwise(v);
  }
}
BENCHMARK(BM_Nqueens_Bitwise8);

static void BM_Nqueens_Bitwise10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    nqueens_bitwise(v);
  }
}
BENCHMARK(BM_Nqueens_Bitwise10);

static void BM_Nqueens_Bitwise13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    nqueens_bitwise(v);
  }
}
BENCHMARK(BM_Nqueens_Bitwise13);

// Bitwise Iterative
static void BM_Nqueens_BitwiseIterative8(benchmark::State& state) {
  CountingVisitor<8> v;
  while (state.KeepRunning()) {
    nqueens_iterative(v);
  }
}
BENCHMARK(BM_Nqueens_BitwiseIterative8);

static void BM_Nqueens_BitwiseIterative10(benchmark::State& state) {
  CountingVisitor<10> v;
  while (state.KeepRunning()) {
    nqueens_iterative(v);
  }
}
BENCHMARK(BM_Nqueens_BitwiseIterative10);

static void BM_Nqueens_BitwiseIterative13(benchmark::State& state) {
  CountingVisitor<13> v;
  while (state.KeepRunning()) {
    nqueens_iterative(v);
  }
}
BENCHMARK(BM_Nqueens_BitwiseIterative13);

// Bitwise Loop
long count_solutions(const NQueensBitwiseLoop& nq) {
  long n_solutions = 0;
  for (NQueensBitwiseLoop::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

static void BM_Nqueens_BitwiseLoop8(benchmark::State& state) {
  NQueensBitwiseLoop nq(8);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_BitwiseLoop8);

static void BM_Nqueens_BitwiseLoop10(benchmark::State& state) {
  NQueensBitwiseLoop nq(10);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_BitwiseLoop10);

static void BM_Nqueens_BitwiseLoop13(benchmark::State& state) {
  NQueensBitwiseLoop nq(13);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_BitwiseLoop13);

// Walker
long count_solutions(const NQueensWalker& nq) {
  long n_solutions = 0;
  for (NQueensWalker::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

static void BM_Nqueens_Walker8(benchmark::State& state) {
  NQueensWalker nq(8);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Walker8);

static void BM_Nqueens_Walker10(benchmark::State& state) {
  NQueensWalker nq(10);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Walker10);

static void BM_Nqueens_Walker13(benchmark::State& state) {
  NQueensWalker nq(13);
  while (state.KeepRunning()) {
    count_solutions(nq);
  }
}
BENCHMARK(BM_Nqueens_Walker13);

BENCHMARK_MAIN();