#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "nqueens_bitwise_loop.h"

using namespace backtracking;

long count_bitwise_loop_solutions(int n) {
  NQueensBitwiseLoop nq(n);
  long nsol = 0;
  for (NQueensBitwiseLoop::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++nsol;
  }
  return nsol;
}

//////////////////////
TEST(NQueensBitwiseLoopTest, CountN1) {
  EXPECT_EQ(count_bitwise_loop_solutions(1), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensBitwiseLoopTest, CountN2) {
  EXPECT_EQ(count_bitwise_loop_solutions(2), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensBitwiseLoopTest, CountN4) {
  EXPECT_EQ(count_bitwise_loop_solutions(4), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensBitwiseLoopTest, CountN8) {
  EXPECT_EQ(count_bitwise_loop_solutions(8), 92)
    << "Got unexpected number of permutations for 8 queens";
}

TEST(NQueensBitwiseLoopTest, CountN9) {
  EXPECT_EQ(count_bitwise_loop_solutions(9), 352)
    << "Got unexpected number of permutations for 9 queens";
}

TEST(NQueensBitwiseLoopTest, VisitN4) {
  std::vector<std::uint8_t> expected0 = {1, 3, 0, 2};
  std::vector<std::uint8_t> expected1 = {2, 0, 3, 1};
 
  NQueensBitwiseLoop nq(4);
  NQueensBitwiseLoop::iterator it = nq.begin();
  EXPECT_EQ(*it, expected0) << "Unexpected 0th permutation";
  EXPECT_EQ(*(++it), expected1) << "Unexpected 1st permutation";
  EXPECT_TRUE(++it == nq.end()) << "Expected last iteration after 2";
}