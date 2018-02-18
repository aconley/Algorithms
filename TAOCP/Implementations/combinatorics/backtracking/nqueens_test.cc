#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "nqueens_basic.h"
#include "nqueens_array.h"
#include "nqueens_bitwise.h"
#include "nqueens_iterative.h"
#include "nqueens_walker.h"

template<std::size_t n> class RecordingVisitor {
  private:
    std::vector<std::array<int, n>> solutions;

  public:
    bool visit(const std::array<int, n>& rows) {
      std::array<int, n> v(rows);
      solutions.push_back(v);
      return true;
    }

    void reset() {
      solutions.clear();
    }

    int getN() const {
      return solutions.size();
    }

    const std::array<int, n>& get(int i) {
      return solutions.at(i);
    }
};

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

//////////////////////
// Test nqueens_basic
TEST(NQueensBasicTest, CountN1) {

  CountingVisitor<1> vis;
  backtracking::nqueens_basic(vis);

  EXPECT_EQ(vis.getN(), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensBasicTest, CountN2) {

  CountingVisitor<2> vis;
  backtracking::nqueens_basic(vis);

  EXPECT_EQ(vis.getN(), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensBasicTest, CountN4) {
  CountingVisitor<4> vis;
  backtracking::nqueens_basic(vis);

  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensBasicTest, VisitN4) {
  RecordingVisitor<4> vis;
  backtracking::nqueens_basic(vis);

  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";

  std::vector<std::array<int, 4>> expected = {{ {{1, 3, 0, 2}}, {{2, 0, 3, 1}} }};
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected permutation at position for 4 queens " << i;
  }
}

TEST(NQueensBasicTest, CountN8) {
  CountingVisitor<8> vis;
  backtracking::nqueens_basic(vis);

  EXPECT_EQ(vis.getN(), 92)
    << "Got unexpected number of permutations for 8 queens";
}

//////////////////////
// Exact same tests but array
TEST(NQueensArrayTest, CountN1) {
  CountingVisitor<1> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensArrayTest, CountN2) {
  CountingVisitor<2> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensArrayTest, CountN4) {
  CountingVisitor<4> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensArrayTest, VisitN4) {
  RecordingVisitor<4> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
  std::vector<std::array<int, 4>> expected = {{ {{1, 3, 0, 2}}, {{2, 0, 3, 1}} }};
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected permutation at position for 4 queens " << i;
  }
}

TEST(NQueensArrayTest, CountN8) {
  CountingVisitor<8> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 92)
    << "Got unexpected number of permutations for 8 queens";
}

//////////////////////
// Exact same tests but bit twiddling
TEST(NQueensBitwiseTest, CountN1) {
  CountingVisitor<1> vis;
  backtracking::nqueens_bitwise(vis);
  EXPECT_EQ(vis.getN(), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensBitwiseTest, CountN2) {
  CountingVisitor<2> vis;
  backtracking::nqueens_bitwise(vis);
  EXPECT_EQ(vis.getN(), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensBitwiseTest, CountN4) {
  CountingVisitor<4> vis;
  backtracking::nqueens_bitwise(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensBitwiseTest, VisitN4) {
  RecordingVisitor<4> vis;
  backtracking::nqueens_bitwise(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
  std::vector<std::array<int, 4>> expected = {{ {{1, 3, 0, 2}}, {{2, 0, 3, 1}} }};
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected permutation at position for 4 queens " << i;
  }
}

TEST(NQueensBitwiseTest, CountN8) {
  CountingVisitor<8> vis;
  backtracking::nqueens_bitwise(vis);
  EXPECT_EQ(vis.getN(), 92)
    << "Got unexpected number of permutations for 8 queens";
}

//////////////////////
// Exact same tests but iterative bit twiddling
TEST(NQueensIterativeBitwiseTest, CountN1) {
  CountingVisitor<1> vis;
  backtracking::nqueens_iterative(vis);
  EXPECT_EQ(vis.getN(), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensIterativeBitwiseTest, CountN2) {
  CountingVisitor<2> vis;
  backtracking::nqueens_iterative(vis);
  EXPECT_EQ(vis.getN(), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensIterativeBitwiseTest, CountN4) {
  CountingVisitor<4> vis;
  backtracking::nqueens_iterative(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensIterativeBitwiseTest, VisitN4) {
  RecordingVisitor<4> vis;
  backtracking::nqueens_iterative(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
  std::vector<std::array<int, 4>> expected = {{ {{1, 3, 0, 2}}, {{2, 0, 3, 1}} }};
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected permutation at position for 4 queens " << i;
  }
}

TEST(NQueensIterativeBitwiseTest, CountN8) {
  CountingVisitor<8> vis;
  backtracking::nqueens_iterative(vis);
  EXPECT_EQ(vis.getN(), 92)
    << "Got unexpected number of permutations for 8 queens";
}

//////////////////////
// Exact same tests but walkers method
TEST(NQueensWalkerTest, CountN1) {
  CountingVisitor<1> vis;
  backtracking::nqueens_walker(vis);
  EXPECT_EQ(vis.getN(), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensWalkerTest, CountN2) {
  CountingVisitor<2> vis;
  backtracking::nqueens_walker(vis);
  EXPECT_EQ(vis.getN(), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensWalkerTest, CountN4) {
  CountingVisitor<4> vis;
  backtracking::nqueens_walker(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensWalkerTest, VisitN4) {
  RecordingVisitor<4> vis;
  backtracking::nqueens_walker(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
  std::vector<std::array<int, 4>> expected = {{ {{1, 3, 0, 2}}, {{2, 0, 3, 1}} }};
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected permutation at position for 4 queens " << i;
  }
}

TEST(NQueensWalkerTest, CountN8) {
  CountingVisitor<8> vis;
  backtracking::nqueens_walker(vis);
  EXPECT_EQ(vis.getN(), 92)
    << "Got unexpected number of permutations for 8 queens";
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}