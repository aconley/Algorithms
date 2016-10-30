#include<array>

#include "gtest/gtest.h"
#include "combinations.h"

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

TEST(CombinationsTest, TestAll) {
  RecordingVisitor<3> vis;
  combinations::combinations_lex_basic(3, vis);
  std::array<int, 3> expected = {{0, 1, 2}};
  EXPECT_EQ(vis.getN(), 1) << "Should have 1 visit for (0, 3) visitor";
  EXPECT_EQ(vis.get(0), expected) << "Didn't get expected indices";
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
