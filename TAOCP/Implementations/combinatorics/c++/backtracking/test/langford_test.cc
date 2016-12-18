#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "langford.h"

template<std::size_t n> class RecordingVisitor {
  private:
    std::vector<std::array<int, 2 * n>> solutions;

  public:
    bool visit(const std::array<int, 2 * n>& sol) {
      std::array<int, 2 * n> v(sol);
      solutions.push_back(v);
      return true;
    }

    void reset() {
      solutions.clear();
    }

    int getN() const {
      return solutions.size();
    }

    const std::array<int, 2 * n>& get(int i) {
      return solutions.at(i);
    }
};

template<std::size_t n> class CountingVisitor {
  private:
    int nsolutions;

  public:
    CountingVisitor() { nsolutions = 0; }

    bool visit(const std::array<int, 2 * n>& rows) {
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
// Test langford_basic

// Counting with solutions
TEST(LangfordBasicTest, CountNBad) {

  CountingVisitor<2> vis2;
  backtracking::langford_basic(vis2);
  EXPECT_EQ(vis2.getN(), 0)
    << "Should be 0 solutions for n = 2";

  CountingVisitor<5> vis5;
  backtracking::langford_basic(vis5);
  EXPECT_EQ(vis5.getN(), 0)
    << "Should be 0 solutions for n = 5";

  CountingVisitor<6> vis6;
  backtracking::langford_basic(vis6);
  EXPECT_EQ(vis6.getN(), 0)
    << "Should be 0 solutions for n = 6";
}

TEST(LangfordBasicTest, CountNGood) {

  CountingVisitor<3> vis3;
  backtracking::langford_basic(vis3);
  EXPECT_EQ(vis3.getN(), 2)
    << "Should be 2 solutions for n = 3";

  CountingVisitor<4> vis4;
  backtracking::langford_basic(vis4);
  EXPECT_EQ(vis4.getN(), 2)
    << "Should be 2 solutions for n = 4";

  CountingVisitor<7> vis7;
  backtracking::langford_basic(vis7);
  EXPECT_EQ(vis7.getN(), 52)
    << "Should be 52 solutions for n = 7";

  CountingVisitor<8> vis8;
  backtracking::langford_basic(vis8);
  EXPECT_EQ(vis8.getN(), 300)
    << "Should be 300 solutions for n = 8";
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
