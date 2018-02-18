#include<vector>
#include<array>
#include<exception>

#include "gtest/gtest.h"
#include "permutations.h"

typedef std::vector<int>::iterator vecIt;

class CountingVisitor {
  private:
    int n;
  public:
    CountingVisitor() {
      n = 0;
    }

    bool visit(const vecIt& start, const vecIt& end) {
      ++n;
      return true;
    }

    void reset() {
      n = 0;
    }

    int getN() const {
      return n;
    }
};

template<class T, std::size_t N> class RecordingVisitor {
  private:
    std::vector< std::array<T, N>> values;

  public:
    bool visit(const vecIt& start, const vecIt& end) {
      std::array<T, N> arr;
      if (std::distance(start, end) != N) {
        throw new std::invalid_argument("Got unexpected number of elements in visitor");
      }
      std::copy_n(start, N, arr.begin());
      values.push_back(arr);
      return true;
    }

    void reset() {
      values.clear();
    }

    int getN() const {
      return values.size();
    }

    std::array<T, N> getNth(int i) {
      return values.at(i);
    }
};

////////////////////////////////////////
// Tests of the lexicographic generator
////////////////////////////////////////
TEST(LexicographicTest, CountWithNoRepeats4) {
  std::vector<int> testVec = {0, 1, 2, 3};

  CountingVisitor v;

  permutations::lexicographic(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 24)
    << "Got unexpected number of permutations for 4 elements";
}

TEST(LexicographicTest, CountWithNoRepeats7) {
  std::vector<int> testVec = {-1, 0, 1, 2, 3, 5, 6};

  CountingVisitor v;

  permutations::lexicographic(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 5040)
    << "Got unexpected number of permutations for 7 elements";
}

TEST(LexicographicTest, CountWithRepeats) {
  std::vector<int> testVec = {1, 2, 2, 4};

  CountingVisitor v;

  permutations::lexicographic(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 12)
    << "Got unexpected number of permutations for {1, 2, 2, 4}";
}

TEST(LexicographicTest, CheckOrderWithNoRepeats3) {
  std::vector<std::array<int, 3>> expected =
    {{{1, 2, 3}}, {{1, 3, 2}}, {{2, 1, 3}},
		 {{2, 3, 1}}, {{3, 1, 2}}, {{3, 2, 1}} };
  std::vector<int> testArr{{1, 2, 3}};

  RecordingVisitor<int, 3> v;

  permutations::lexicographic(testArr.begin(), testArr.end(), v);
  EXPECT_EQ(v.getN(), expected.size())
    << "Got unexpected number of permutations for {1, 2, 3}";
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(v.getNth(i), expected[i])
      << "Got unexpected permutation at position " << i;
  }
}

TEST(LexicographicTest, CheckOrderWithRepeats4) {
  std::vector<std::array<int, 4>> expected =
    {{{1, 2, 2, 3}}, {{1, 2, 3, 2}}, {{1, 3, 2, 2}},
		 {{2, 1, 2, 3}}, {{2, 1, 3, 2}}, {{2, 2, 1, 3}}, {{2, 2, 3, 1}},
		 {{2, 3, 1, 2}}, {{2, 3, 2, 1}}, {{3, 1, 2, 2}}, {{3, 2, 1, 2}},
		 {{3, 2, 2, 1}} };
  std::vector<int> testArr{{1, 2, 2, 3}};

  RecordingVisitor<int, 4> v;

  permutations::lexicographic(testArr.begin(), testArr.end(), v);
  EXPECT_EQ(v.getN(), expected.size())
    << "Got unexpected number of permutations for {1, 2, 2, 3}";
  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(v.getNth(i), expected[i])
      << "Got unexpected permutation at position " << i;
  }
}

////////////////////////////////////////
// Tests of the plain permutations generator
////////////////////////////////////////
TEST(PlainChangesTest, CountWithNoRepeats) {
  std::vector<int> testVec = {-1, 0, 1, 2, 3, 5, 6};

  CountingVisitor v;

  permutations::plain(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 5040)
    << "Got unexpected number of permutations for 7 elements";
}

TEST(PlainChangesTest, CountWithRepeats) {
  std::vector<int> testVec = {-1, 0, 2, 2, 2, 5, 6};

  CountingVisitor v;

  permutations::plain(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 5040)
    << "Got unexpected number of permutations for 7 elements";
}

TEST(PlainChangesTest, CheckOrderWithNoRepeats) {
  std::vector<std::array<int, 4>> expected =
    {{{1, 2, 3, 4}}, {{1, 2, 4, 3}}, {{1, 4, 2, 3}}, {{4, 1, 2, 3}},
     {{4, 1, 3, 2}}, {{1, 4, 3, 2}}, {{1, 3, 4, 2}}, {{1, 3, 2, 4}},
     {{3, 1, 2, 4}}, {{3, 1, 4, 2}}, {{3, 4, 1, 2}}, {{4, 3, 1, 2}},
     {{4, 3, 2, 1}}, {{3, 4, 2, 1}}, {{3, 2, 4, 1}}, {{3, 2, 1, 4}},
     {{2, 3, 1, 4}}, {{2, 3, 4, 1}}, {{2, 4, 3, 1}}, {{4, 2, 3, 1}},
     {{4, 2, 1, 3}}, {{2, 4, 1, 3}}, {{2, 1, 4, 3}}, {{2, 1, 3, 4}}};

  std::vector<int> testArr{{1, 2, 3, 4}};

  RecordingVisitor<int, 4> v;

  permutations::plain(testArr.begin(), testArr.end(), v);
  EXPECT_EQ(v.getN(), expected.size())
    << "Got unexpected number of permutations for {1, 2, 3, 4}";

  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(v.getNth(i), expected[i])
      << "Got unexpected permutation at position " << i;
  }
}

////////////////////////////////////////
// Tests of the heap permutations generator
////////////////////////////////////////
TEST(HeapTest, CountWithNoRepeats) {
  std::vector<int> testVec = {-1, 0, 1, 2, 3, 5, 6};

  CountingVisitor v;

  permutations::heap(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 5040)
    << "Got unexpected number of permutations for 7 elements";
}

TEST(HeapTest, CountWithRepeats) {
  std::vector<int> testVec = {-1, 0, 2, 2, 2, 5, 6};

  CountingVisitor v;

  permutations::heap(testVec.begin(), testVec.end(), v);
  EXPECT_EQ(v.getN(), 5040)
    << "Got unexpected number of permutations for 7 elements";
}

TEST(HeapTest, CheckOrderWithNoRepeats) {
  std::vector<std::array<int, 4>> expected =
  {{{1, 2, 3, 4}}, {{2, 1, 3, 4}}, {{3, 1, 2, 4}}, {{1, 3, 2, 4}},
   {{2, 3, 1, 4}}, {{3, 2, 1, 4}}, {{4, 2, 1, 3}}, {{2, 4, 1, 3}},
   {{1, 4, 2, 3}}, {{4, 1, 2, 3}}, {{2, 1, 4, 3}}, {{1, 2, 4, 3}},
   {{1, 3, 4, 2}}, {{3, 1, 4, 2}}, {{4, 1, 3, 2}}, {{1, 4, 3, 2}},
   {{3, 4, 1, 2}}, {{4, 3, 1, 2}}, {{4, 3, 2, 1}}, {{3, 4, 2, 1}},
   {{2, 4, 3, 1}}, {{4, 2, 3, 1}}, {{3, 2, 4, 1}}, {{2, 3, 4, 1}}};

  std::vector<int> testArr{{1, 2, 3, 4}};

  RecordingVisitor<int, 4> v;

  permutations::heap(testArr.begin(), testArr.end(), v);
  EXPECT_EQ(v.getN(), expected.size())
    << "Got unexpected number of permutations for {1, 2, 3, 4}";

  for (int i = 0; i < expected.size(); ++i) {
    EXPECT_EQ(v.getNth(i), expected[i])
      << "Got unexpected permutation at position " << i;
  }
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}