#include<vector>

#include "gtest/gtest.h"
#include "combinations.h"
#include "combinations_chase.h"
#include "combinations_gray.h"

class RecordingVisitor {
  private:
    std::vector<std::vector<int>> solutions;
  public:
    bool visit(std::vector<int>::const_iterator begin,
      std::vector<int>::const_iterator end) {

      std::vector<int> v(begin, end);
      solutions.push_back(v);
      return true;
    }

    void reset() {
      solutions.clear();
    }

    int getN() const {
      return solutions.size();
    }

    const std::vector<int>& get(int i) {
      return solutions.at(i);
    }
};

class CountingVisitor {
  private:
    int nsolutions;

  public:
    CountingVisitor() {
      nsolutions = 0;
    }

    bool visit(std::vector<int>::const_iterator begin,
      std::vector<int>::const_iterator end) {
      ++nsolutions;
      return true;
    }

    void reset() {
      nsolutions = 0;
    }

    int getN() {
      return nsolutions;
    }
};

// Tests

// Un-optimized
TEST(CombinationsBasicTest, Test_3_3) {
  RecordingVisitor vis;
  combinations::combinations_lex_basic(3, 3, vis);
  std::vector<int> expected = {{0, 1, 2}};
  EXPECT_EQ(vis.getN(), 1)
    << "Should have 1 visit for 3 objects taken 3 at a time";
  EXPECT_EQ(vis.get(0), expected) << "Didn't get expected indices";
}

TEST(CombinationsBasicTest, Test_3_2) {
  // 3 objects 2 at a time
  std::vector<std::vector<int>> expected =
    {{0, 1}, {0, 2}, {1, 2}};

  RecordingVisitor vis;
  combinations::combinations_lex_basic(3, 2, vis);
  EXPECT_EQ(vis.getN(), 3)
    << "Should have 3 visits for 3 objects taken 2 at a time";
   for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected combination for 3 objects 2 at a time in index " << i;
  }
}

TEST(CombinationsBasicTest, Test_6_3) {
  // 6 objects 3 at a time
  std::vector<std::vector<int>> expected =
    {{0, 1, 2}, {0, 1, 3}, {0, 2, 3}, {1, 2, 3}, {0, 1, 4},
     {0, 2, 4}, {1, 2, 4}, {0, 3, 4}, {1, 3, 4}, {2, 3, 4},
     {0, 1, 5}, {0, 2, 5}, {1, 2, 5}, {0, 3, 5}, {1, 3, 5},
     {2, 3, 5}, {0, 4, 5}, {1, 4, 5}, {2, 4, 5}, {3, 4, 5}};

  RecordingVisitor vis;
  combinations::combinations_lex_basic(6, 3, vis);
  EXPECT_EQ(vis.getN(), 20)
    << "Should have 20 visits for 6 objects taken 3 at a time";
   for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected combination for 3 objects 2 at a time in index " << i;
  }
}

TEST(CombinationsBasicTest, TestCount1) {
  CountingVisitor vis1;
  combinations::combinations_lex_basic(10, 1, vis1);
  EXPECT_EQ(vis1.getN(), 10) << "10 chose 1 is 10";

  vis1.reset();
  combinations::combinations_lex_basic(20, 1, vis1);
  EXPECT_EQ(vis1.getN(), 20) << "20 chose 1 is 20";
}

TEST(CombinationsBasicTest, TestCount4) {
  CountingVisitor vis4;
  combinations::combinations_lex_basic(10, 4, vis4);
  EXPECT_EQ(vis4.getN(), 210) << "10 chose 4 is 210";

  vis4.reset();
  combinations::combinations_lex_basic(20, 4, vis4);
  EXPECT_EQ(vis4.getN(), 4845) << "20 chose 4 is 4845";
}

TEST(CombinationsBasicTest, TestCountBig) {
  CountingVisitor vis10;
  combinations::combinations_lex_basic(20, 10, vis10);
  EXPECT_EQ(vis10.getN(), 184756) << "20 chose 10 is 184756";
}

// Optimized

TEST(CombinationsTest, Test_3_3) {
  RecordingVisitor vis;
  combinations::combinations_lex(3, 3, vis);
  std::vector<int> expected = {{0, 1, 2}};
  EXPECT_EQ(vis.getN(), 1)
    << "Should have 1 visit for 3 objects taken 3 at a time";
  EXPECT_EQ(vis.get(0), expected) << "Didn't get expected indices";
}

TEST(CombinationsTest, Test_3_2) {
  // 3 objects 2 at a time
  std::vector<std::vector<int>> expected =
    {{0, 1}, {0, 2}, {1, 2}};

  RecordingVisitor vis;
  combinations::combinations_lex(3, 2, vis);
  EXPECT_EQ(vis.getN(), 3)
    << "Should have 3 visits for 3 objects taken 2 at a time";
   for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected combination for 3 objects 2 at a time in index " << i;
  }
}

TEST(CombinationsTest, Test_6_3) {
  // 6 objects 3 at a time
  std::vector<std::vector<int>> expected =
    {{0, 1, 2}, {0, 1, 3}, {0, 2, 3}, {1, 2, 3}, {0, 1, 4},
     {0, 2, 4}, {1, 2, 4}, {0, 3, 4}, {1, 3, 4}, {2, 3, 4},
     {0, 1, 5}, {0, 2, 5}, {1, 2, 5}, {0, 3, 5}, {1, 3, 5},
     {2, 3, 5}, {0, 4, 5}, {1, 4, 5}, {2, 4, 5}, {3, 4, 5}};

  RecordingVisitor vis;
  combinations::combinations_lex(6, 3, vis);
  EXPECT_EQ(vis.getN(), 20)
    << "Should have 20 visits for 6 objects taken 3 at a time";
   for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected combination for 3 objects 2 at a time in index " << i;
  }
}

TEST(CombinationsTest, TestCount1) {
  CountingVisitor vis1;
  combinations::combinations_lex(10, 1, vis1);
  EXPECT_EQ(vis1.getN(), 10) << "10 chose 1 is 10";

  vis1.reset();
  combinations::combinations_lex(20, 1, vis1);
  EXPECT_EQ(vis1.getN(), 20) << "20 chose 1 is 20";
}

TEST(CombinationsTest, TestCount4) {
  CountingVisitor vis4;
  combinations::combinations_lex(10, 4, vis4);
  EXPECT_EQ(vis4.getN(), 210) << "10 chose 4 is 210";

  vis4.reset();
  combinations::combinations_lex(20, 4, vis4);
  EXPECT_EQ(vis4.getN(), 4845) << "20 chose 4 is 4845";
}

TEST(CombinationsTest, TestVaryingT) {
  CountingVisitor vis;
  combinations::combinations_lex(7, 1, vis);
  EXPECT_EQ(vis.getN(), 7);
  vis.reset();
  combinations::combinations_lex(7, 2, vis);
  EXPECT_EQ(vis.getN(), 21);
  vis.reset();
  combinations::combinations_lex(7, 3, vis);
  EXPECT_EQ(vis.getN(), 35);
  vis.reset();
  combinations::combinations_lex(7, 4, vis);
  EXPECT_EQ(vis.getN(), 35);
  vis.reset();
  combinations::combinations_lex(7, 6, vis);
  EXPECT_EQ(vis.getN(), 7);
  vis.reset();
  combinations::combinations_lex(7, 7, vis);
  EXPECT_EQ(vis.getN(), 1);
}

TEST(CombinationsTest, TestCountBig) {
  CountingVisitor vis10;
  combinations::combinations_lex(20, 10, vis10);
  EXPECT_EQ(vis10.getN(), 184756) << "20 chose 10 is 184756";
}

// Revolving door visitor
TEST(GrayCombinationsTest, TestCount1) {
  CountingVisitor vis1;
  combinations::combinations_gray(10, 1, vis1);
  EXPECT_EQ(vis1.getN(), 10) << "10 chose 1 is 10";

  vis1.reset();
  combinations::combinations_gray(20, 1, vis1);
  EXPECT_EQ(vis1.getN(), 20) << "20 chose 1 is 20";
}

TEST(GrayCombinationsTest, TestCount3) {
  CountingVisitor vis3;
  combinations::combinations_gray(7, 3, vis3);
  EXPECT_EQ(vis3.getN(), 35) << "7 chose 3 is 35";
}

TEST(GrayCombinationsTest, TestCount4) {
  CountingVisitor vis4;
  combinations::combinations_gray(10, 4, vis4);
  EXPECT_EQ(vis4.getN(), 210) << "10 chose 4 is 210";

  vis4.reset();
  combinations::combinations_gray(20, 4, vis4);
  EXPECT_EQ(vis4.getN(), 4845) << "20 chose 4 is 4845";
}

TEST(GrayCombinationsTest, TestVaryingT) {
  CountingVisitor vis;
  combinations::combinations_gray(7, 1, vis);
  EXPECT_EQ(vis.getN(), 7);
  vis.reset();
  combinations::combinations_gray(7, 2, vis);
  EXPECT_EQ(vis.getN(), 21);
  vis.reset();
  combinations::combinations_gray(7, 3, vis);
  EXPECT_EQ(vis.getN(), 35);
  vis.reset();
  combinations::combinations_gray(7, 4, vis);
  EXPECT_EQ(vis.getN(), 35);
  vis.reset();
  combinations::combinations_gray(7, 6, vis);
  EXPECT_EQ(vis.getN(), 7);
  vis.reset();
  combinations::combinations_gray(7, 7, vis);
  EXPECT_EQ(vis.getN(), 1);
}

TEST(GrayCombinationsTest, TestCountBig) {
  CountingVisitor vis10;
  combinations::combinations_gray(20, 10, vis10);
  EXPECT_EQ(vis10.getN(), 184756) << "20 chose 10 is 184756";
}

TEST(GrayCombinationsBasicTest, Test_6_3) {
  // 6 objects 3 at a time
  std::vector<std::vector<int>> expected
    {{0, 1, 2}, {0, 2, 3}, {1, 2, 3}, {0, 1, 3}, {0, 3, 4},
     {1, 3, 4}, {2, 3, 4}, {0, 2, 4}, {1, 2, 4}, {0, 1, 4},
     {0, 4, 5}, {1, 4, 5}, {2, 4, 5}, {3, 4, 5}, {0, 3, 5},
     {1, 3, 5}, {2, 3, 5}, {0, 2, 5}, {1, 2, 5}, {0, 1, 5}};

  RecordingVisitor vis;
  combinations::combinations_gray(6, 3, vis);
  EXPECT_EQ(vis.getN(), 20)
    << "Should have 20 visits for 6 objects taken 3 at a time";
   for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected combination for 3 objects 2 at a time in index " << i;
  }
}

// Chase sequence generator
TEST(ChaseCombinationsTest, TestCount1) {
  CountingVisitor vis1;
  combinations::combinations_chase(10, 1, vis1);
  EXPECT_EQ(vis1.getN(), 10) << "10 chose 1 is 10";

  vis1.reset();
  combinations::combinations_chase(20, 1, vis1);
  EXPECT_EQ(vis1.getN(), 20) << "20 chose 1 is 20";
}

TEST(ChaseCombinationsTest, TestCount3) {
  CountingVisitor vis3;
  combinations::combinations_chase(7, 3, vis3);
  EXPECT_EQ(vis3.getN(), 35) << "7 chose 3 is 35";
}

TEST(ChaseCombinationsTest, TestCount4) {
  CountingVisitor vis4;
  combinations::combinations_chase(10, 4, vis4);
  EXPECT_EQ(vis4.getN(), 210) << "10 chose 4 is 210";

  vis4.reset();
  combinations::combinations_chase(20, 4, vis4);
  EXPECT_EQ(vis4.getN(), 4845) << "20 chose 4 is 4845";
}

TEST(ChaseCombinationsTest, TestVaryingT) {
  CountingVisitor vis;
  combinations::combinations_chase(7, 1, vis);
  EXPECT_EQ(vis.getN(), 7);
  vis.reset();
  combinations::combinations_chase(7, 2, vis);
  EXPECT_EQ(vis.getN(), 21);
  vis.reset();
  combinations::combinations_chase(7, 3, vis);
  EXPECT_EQ(vis.getN(), 35);
  vis.reset();
  combinations::combinations_chase(7, 4, vis);
  EXPECT_EQ(vis.getN(), 35);
  vis.reset();
  combinations::combinations_chase(7, 6, vis);
  EXPECT_EQ(vis.getN(), 7);
  vis.reset();
  combinations::combinations_chase(7, 7, vis);
  EXPECT_EQ(vis.getN(), 1);
}

TEST(ChaseCombinationsTest, TestCountBig) {
  CountingVisitor vis10;
  combinations::combinations_chase(20, 10, vis10);
  EXPECT_EQ(vis10.getN(), 184756) << "20 chose 10 is 184756";
}

TEST(ChaseCombinationsBasicTest, Test_6_3) {
  // 6 objects 3 at a time.  See 7.2.1.3 Table 2; these
  // are B_{33} left-right reversed
  std::vector<std::vector<int>> expected
    {{3, 4, 5}, {2, 4, 5}, {0, 4, 5}, {1, 4, 5}, {1, 2, 5},
     {0, 2, 5}, {0, 1, 5}, {0, 3, 5}, {1, 3, 5}, {2, 3, 5},
     {2, 3, 4}, {0, 3, 4}, {1, 3, 4}, {1, 2, 4}, {0, 2, 4},
     {0, 1, 4}, {0, 1, 2}, {0, 1, 3}, {0, 2, 3}, {1, 2, 3}};

  RecordingVisitor vis;
  combinations::combinations_chase(6, 3, vis);
  EXPECT_EQ(vis.getN(), 20)
    << "Should have 20 visits for 6 objects taken 3 at a time";
   for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(vis.get(i), expected[i])
      << "Got unexpected combination for 3 objects 2 at a time in index " << i;
  }
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
