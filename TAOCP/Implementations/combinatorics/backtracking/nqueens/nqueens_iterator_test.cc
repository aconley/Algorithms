#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "gmock/gmock.h"
#include "nqueens_iterator.h"

long count_solutions(int n) {
  NQueensIterator nq(n);
  long n_solutions = 0;
  for (NQueensIterator::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

//////////////////////
TEST(NQueensIterativeTest, CountN1) {
  EXPECT_EQ(count_solutions(1), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensIterativeTest, CountN2) {
  EXPECT_EQ(count_solutions(2), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensIterativeTest, CountN4) {
  EXPECT_EQ(count_solutions(4), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensIterativeTest, CountN8) {
  EXPECT_EQ(count_solutions(8), 92)
    << "Got unexpected number of permutations for 8 queens";
}


TEST(NQueensIterativeTest, CountN9) {
  EXPECT_EQ(count_solutions(9), 352)
    << "Got unexpected number of permutations for 9 queens";
}

TEST(NQueensIterativeTest, VisitN4) {
  std::vector<std::uint8_t> expected0 = {1, 3, 0, 2};
  std::vector<std::uint8_t> expected1 = {2, 0, 3, 1};
 
  NQueensIterator nq(4);
  NQueensIterator::iterator it = nq.begin();
  EXPECT_EQ(*(++it), expected0) << "Unexpected 0th permutation";
  EXPECT_EQ(*(++it), expected1) << "Unexpected 1st permutation";
  EXPECT_TRUE(it == nq.end()) << "Expected last iteration after 2";
}
