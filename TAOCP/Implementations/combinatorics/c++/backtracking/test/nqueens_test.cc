#include<vector>
#include<array>

#include "gtest/gtest.h"
#include "nqueens.h"

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

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}