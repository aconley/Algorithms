#include<array>
#include<vector>

#include "gtest/gtest.h"
#include "langford.h"
#include "langford_visitors.h"

TEST(LangfordCountingVisitorTest, DoesCount) {
  backtracking::LangfordCountingVisitor<3> vis;
  EXPECT_EQ(vis.getN(), 0) << "should start with n = 0";

  vis.visit({{2, 3, 1, -2, -1, -3}});
  EXPECT_EQ(vis.getN(), 1) << "n should be 1 after the first visit";
  vis.visit({{3, 1, 2, -1, -3, -2}});
  EXPECT_EQ(vis.getN(), 2) << "n should be 2 after the second visit";
}

TEST(LangfordCountingVisitorTest, DoesReset) {
  backtracking::LangfordCountingVisitor<3> vis;
  vis.visit({{2, 3, 1, -2, -1, -3}});
  vis.visit({{3, 1, 2, -1, -3, -2}});
  EXPECT_EQ(vis.getN(), 2) << "n should be 2 after the second visit";

  vis.reset();
  EXPECT_EQ(vis.getN(), 0) << "n should be 0 after a reset";

  vis.visit({{3, 1, 2, -1, -3, -2}});
  EXPECT_EQ(vis.getN(), 1)
    << "should be able to start over visiting after reset";
}

TEST(LangfordRecordingVisitorTest, DoesRecord) {
  backtracking::LangfordRecordingVisitor<3> vis;
  std::vector<std::array<int, 6>> vals =
    {{3, 1, 2, -1, -3, -2}, {2, 3, 1, -2, -1, -3}};
  EXPECT_EQ(vis.getN(), 0) << "should start with n = 0";
  vis.visit(vals[0]);
  EXPECT_EQ(vis.getN(), 1) << "n should be 1 after the first visit";
  vis.visit(vals[1]);
  EXPECT_EQ(vis.getN(), 2) << "n should be 2 after the second visit";

  EXPECT_EQ(vis.get(0), vals[0]);
  EXPECT_EQ(vis.get(1), vals[1]);
}

TEST(LangfordRecordingVisitorTest, DoesReset) {
  backtracking::LangfordRecordingVisitor<3> vis;
  std::vector<std::array<int, 6>> vals =
    {{3, 1, 2, -1, -3, -2}, {2, 3, 1, -2, -1, -3}};
  vis.visit(vals[0]);
  vis.visit(vals[1]);
  EXPECT_EQ(vis.getN(), 2) << "n should be 2 after the second visit";

  vis.reset();
  EXPECT_EQ(vis.getN(), 0) << "n should be 0 after a reset";

  vis.visit(vals[1]);
  EXPECT_EQ(vis.getN(), 1)
    << "should be able to start over visiting after reset";
  EXPECT_EQ(vis.get(0), vals[1]);
}

TEST(LangfordRecordingVisitorTest, MakesCopy) {
  backtracking::LangfordRecordingVisitor<3> vis;
  std::vector<std::array<int, 6>> vals =
    {{3, 1, 2, -1, -3, -2}, {2, 3, 1, -2, -1, -3}};
  vis.visit(vals[0]);
  vis.visit(vals[1]);
  EXPECT_EQ(vis.getN(), 2) << "n should be 2 after the second visit";

  vals[0][0] = 100;
  EXPECT_NE(vis.get(0), vals[0]) << "Visitor should have copy";
}
