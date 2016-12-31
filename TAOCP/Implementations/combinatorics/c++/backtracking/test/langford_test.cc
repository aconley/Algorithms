#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "langford.h"
#include "langford_visitors.h"

template <class T, std::size_t N>
std::ostream& operator<<(std::ostream& o, const std::array<T, N>& arr)
{
    std::copy(arr.cbegin(), arr.cend(), std::ostream_iterator<T>(o, " "));
    return o;
}

//////////////////////
// Test langford_basic

// Counting with solutions
TEST(LangfordBasicTest, CountNBad) {

  backtracking::LangfordCountingVisitor<2> vis2;
  backtracking::langford_basic(vis2);
  EXPECT_EQ(vis2.getN(), 0)
    << "Should be 0 solutions for n = 2";

  backtracking::LangfordCountingVisitor<5> vis5;
  backtracking::langford_basic(vis5);
  EXPECT_EQ(vis5.getN(), 0)
    << "Should be 0 solutions for n = 5";

  backtracking::LangfordCountingVisitor<6> vis6;
  backtracking::langford_basic(vis6);
  EXPECT_EQ(vis6.getN(), 0)
    << "Should be 0 solutions for n = 6";
}

// Counting with solutions
TEST(LangfordBasicTest, CountNGood) {

  backtracking::LangfordCountingVisitor<3> vis3;
  backtracking::langford_basic(vis3);
  EXPECT_EQ(vis3.getN(), 2)
    << "Should be 2 solutions for n = 3";

  backtracking::LangfordCountingVisitor<4> vis4;
  backtracking::langford_basic(vis4);
  EXPECT_EQ(vis4.getN(), 2)
    << "Should be 2 solutions for n = 4";

  backtracking::LangfordCountingVisitor<7> vis7;
  backtracking::langford_basic(vis7);
  EXPECT_EQ(vis7.getN(), 52)
    << "Should be 52 solutions for n = 7";

  backtracking::LangfordCountingVisitor<8> vis8;
  backtracking::langford_basic(vis8);
  EXPECT_EQ(vis8.getN(), 300)
    << "Should be 300 solutions for n = 8";
}

TEST(LangfordBasicTest, Record3) {
  backtracking::LangfordRecordingVisitor<3> vis3;
  backtracking::langford_basic(vis3);

  EXPECT_EQ(vis3.getN(), 2)
    << "Should have 2 solutions for n = 3";
  std::vector<std::array<int, 6>> expected =
    {{2, 3, 1, -2, -1, -3}, {3, 1, 2, -1, -3, -2}};
  EXPECT_EQ(vis3.get(0), expected[0])
    << "Got unexpected first langford solution for n = 3";
  EXPECT_EQ(vis3.get(1), expected[1])
    << "Got unexpected second langford solution for n = 3";
}

/// Optimized Langford

// Counting with solutions
TEST(LangfordTest, CountNBad) {

  backtracking::LangfordCountingVisitor<2> vis2;
  backtracking::langford(vis2);
  EXPECT_EQ(vis2.getN(), 0)
    << "Should be 0 solutions for n = 2";

  backtracking::LangfordCountingVisitor<5> vis5;
  backtracking::langford(vis5);
  EXPECT_EQ(vis5.getN(), 0)
    << "Should be 0 solutions for n = 5";

  backtracking::LangfordCountingVisitor<6> vis6;
  backtracking::langford(vis6);
  EXPECT_EQ(vis6.getN(), 0)
    << "Should be 0 solutions for n = 6";
}

// Counting with solutions
TEST(LangfordTest, CountNGood) {

  backtracking::LangfordCountingVisitor<3> vis3;
  backtracking::langford(vis3);
  EXPECT_EQ(vis3.getN(), 1)
    << "Should be 1 solutions for n = 3";

  backtracking::LangfordCountingVisitor<4> vis4;
  backtracking::langford(vis4);
  EXPECT_EQ(vis4.getN(), 1)
    << "Should be 1 solutions for n = 4";

  backtracking::LangfordCountingVisitor<7> vis7;
  backtracking::langford(vis7);
  EXPECT_EQ(vis7.getN(), 26)
    << "Should be 26 solutions for n = 7";

  backtracking::LangfordCountingVisitor<8> vis8;
  backtracking::langford(vis8);
  EXPECT_EQ(vis8.getN(), 150)
    << "Should be 150 solutions for n = 8";
}

// Counting including reversed
TEST(LangfordTest, CountNGoodWithReversed) {
  backtracking::LangfordCountingVisitor<7> vis7;
  backtracking::langford(vis7, true);
  EXPECT_EQ(vis7.getN(), 52)
    << "Should be 52 solutions for n = 7 if reversed solutions are included";
}

TEST(LangfordTest, Record3) {
  backtracking::LangfordRecordingVisitor<3> vis3;
  backtracking::langford(vis3);

  EXPECT_EQ(vis3.getN(), 1)
    << "Should have 1 solution for n = 3";
  std::vector<std::array<int, 6>> expected =
    {{3, 1, 2, -1, -3, -2}};
  EXPECT_EQ(vis3.get(0), expected[0])
    << "Got unexpected langford solution for n = 3";
}

TEST(LangfordTest, Record3WithReversed) {
  backtracking::LangfordRecordingVisitor<3> vis3;
  backtracking::langford(vis3, true);

  EXPECT_EQ(vis3.getN(), 2)
    << "Should have 2 solutions for n = 3";
  std::vector<std::array<int, 6>> expected =
    {{3, 1, 2, -1, -3, -2}, {2, 3, 1, -2, -1, -3}};
  EXPECT_EQ(vis3.get(0), expected[0])
    << "Got unexpected first langford solution for n = 3";
  EXPECT_EQ(vis3.get(1), expected[1])
    << "Got unexpected second langford solution for n = 3";
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
