#include<vector>
#include<array>

#include "gtest/gtest.h"
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
    << "Got unexpected number of permutations for 4 queens";
}

/*
TEST(NQueensIterativeTest, VisitN4) {
  NQueensIterator nq(n);
  NQueensIterator::iterator it = nq.begin();
  ++it;
  EXPECT_EQ(*it, {{1, 3, 0, 2}})
    << "Got unexpected permutation at position for 4 queens at position 0";
  ++it;
  EXPECT_EQ(*it, {{2, 0, 3, 1}})
    << "Got unexpected permutation at position for 4 queens at position 1";
  ++it;
  EXPECT_EQ(it, nq.end()) << "Expected last iteration after 2";
}*/
