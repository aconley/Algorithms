#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "nqueens_basic.h"

using namespace backtracking;

long count_basic_solutions(int n) {
  NQueensBasic nq(n);
  long n_solutions = 0;
  for (NQueensBasic::iterator it = nq.begin(); it != nq.end(); ++it) {
    ++n_solutions;
  }
  return n_solutions;
}

TEST(NQueensBasicTest, CountN1) {
  EXPECT_EQ(count_basic_solutions(1), 1)
    << "Got unexpected number of permutations for 1 queens";
}

TEST(NQueensBasicTest, CountN2) {
  EXPECT_EQ(count_basic_solutions(2), 0)
    << "Got unexpected number of permutations for 2 queens";
}

TEST(NQueensBasicTest, CountN4) {
  EXPECT_EQ(count_basic_solutions(4), 2)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensBasicTest, CountN8) {
  EXPECT_EQ(count_basic_solutions(8), 92)
    << "Got unexpected number of permutations for 4 queens";
}

TEST(NQueensBasicTest, CountN9) {
  EXPECT_EQ(count_basic_solutions(9), 352)
    << "Got unexpected number of permutations for 9 queens";
}


TEST(NQueensBasicTest, VisitN1) {
  std::vector<std::uint8_t> expected0 = {0};
 
  NQueensBasic nq(1);
  NQueensBasic::iterator it = nq.begin();
  EXPECT_EQ(*(++it), expected0) << "Unexpected 0th permutation";
  EXPECT_TRUE(it == nq.end()) << "Expected last iteration after 1";
}

TEST(NQueensBasicTest, VisitN4) {
  std::vector<std::uint8_t> expected0 = {1, 3, 0, 2};
  std::vector<std::uint8_t> expected1 = {2, 0, 3, 1};
 
  NQueensBasic nq(4);
  NQueensBasic::iterator it = nq.begin();
  EXPECT_EQ(*(++it), expected0) << "Unexpected 0th permutation";
  EXPECT_EQ(*(++it), expected1) << "Unexpected 1st permutation";
  EXPECT_TRUE(it == nq.end()) << "Expected last iteration after 2";
}