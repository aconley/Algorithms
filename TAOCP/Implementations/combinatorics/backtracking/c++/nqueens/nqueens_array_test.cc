#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "nqueens_visitors.h"
#include "nqueens_array.h"

TEST(NQueensArrayTest, CountN1) {
  backtracking::CountingVisitor<1> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensArrayTest, CountN2) {
  backtracking::CountingVisitor<2> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensArrayTest, CountN4) {
  backtracking::CountingVisitor<4> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensArrayTest, VisitN4) {
  backtracking::RecordingVisitor<4> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 2)
    << "Got unexpected number of permutations for 4 queens";
  std::vector<std::array<int, 4>> expected = {{ {{1, 3, 0, 2}}, {{2, 0, 3, 1}} }};
  for (unsigned int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected permutation at position for 4 queens " << i;
  }
}

TEST(NQueensArrayTest, CountN8) {
  backtracking::CountingVisitor<8> vis;
  backtracking::nqueens_array(vis);
  EXPECT_EQ(vis.getN(), 92)
    << "Got unexpected number of permutations for 8 queens";
}