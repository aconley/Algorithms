#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "nqueens_walker.h"

using namespace backtracking;

long count_walker_solutions(int n) {
  NQueensWalker nq(n);
  long n_solutions = 0;
  for (NQueensWalker::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

//////////////////////
TEST(NQueensWalkerTest, CountN1) {
  EXPECT_EQ(count_walker_solutions(1), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensWalkerTest, CountN2) {
  EXPECT_EQ(count_walker_solutions(2), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensWalkerTest, CountN4) {
  EXPECT_EQ(count_walker_solutions(4), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensWalkerTest, CountN8) {
  EXPECT_EQ(count_walker_solutions(8), 92)
    << "Got unexpected number of permutations for 8 queens";
}

TEST(NQueensWalkerTest, CountN9) {
  EXPECT_EQ(count_walker_solutions(9), 352)
    << "Got unexpected number of permutations for 9 queens";
}

TEST(NQueensWalkerTest, VisitN4) {
  std::vector<std::uint8_t> expected0 = {1, 3, 0, 2};
  std::vector<std::uint8_t> expected1 = {2, 0, 3, 1};
 
  NQueensWalker nq(4);
  NQueensWalker::iterator it = nq.begin();
  EXPECT_EQ(*(++it), expected0) << "Unexpected 0th permutation";
  EXPECT_EQ(*(++it), expected1) << "Unexpected 1st permutation";
  EXPECT_TRUE(it == nq.end()) << "Expected last iteration after 2";
}
