#include "gtest/gtest.h"
#include "gray.h"
#include "binarygray.h"
#include "looplessgray.h"

#include<array>

using namespace ntuples;

////////////////////////////////////////
// Tests of the basic version
////////////////////////////////////////
TEST(GrayTest, Count) {
  Gray gray(15);

  unsigned int nfound = 0;
  for (auto it : gray)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(GrayTest, Pattern4) {
  std::array<std::uint32_t, 16> expected =
    { 0b0000, 0b0001, 0b0011, 0b0010, 0b0110, 0b0111, 0b0101, 0b0100,
      0b1100, 0b1101, 0b1111, 0b1110, 0b1010, 0b1011, 0b1001, 0b1000};
  Gray gray(4);
  int idx = 0;
  for (auto it : gray) {
    ASSERT_LT(idx, 16) << "Got more gray values than expected";
    EXPECT_EQ(it, expected[idx])
      << "Got unexpected iterator value at index " << idx;
    ++idx;
  }
}

////////////////////////////////////////
// Tests of the binary version
////////////////////////////////////////
TEST(BinaryGrayTest, IteratorCount) {
  int n = 15;
  auto start = BinaryGrayIterator::begin(n);
  auto end = BinaryGrayIterator::end(n);

  unsigned int nfound = 0;
  for (auto it = start; it != end; ++it)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(BinaryGrayTest, Count) {
  BinaryGray gray(15);

  unsigned int nfound = 0;
  for (auto it : gray)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(BinaryGrayTest, Pattern4) {
  std::array<std::uint32_t, 16> expected =
    { 0b0000, 0b0001, 0b0011, 0b0010, 0b0110, 0b0111, 0b0101, 0b0100,
      0b1100, 0b1101, 0b1111, 0b1110, 0b1010, 0b1011, 0b1001, 0b1000};
  BinaryGray gray(4);
  int idx = 0;
  for (auto it : gray) {
    ASSERT_LT(idx, 16) << "Got more gray values than expected";
    EXPECT_EQ(it, expected[idx])
      << "Got unexpected iterator value at index " << idx;
    ++idx;
  }
}

TEST(BinaryGrayTest, GetNext) {
  EXPECT_EQ(BinaryGray::getNext(0b0110), 0b0111);
  EXPECT_EQ(BinaryGray::getNext(0b1110), 0b1010);
}

////////////////////////////////////////
// Tests of the loopless version
////////////////////////////////////////
TEST(LooplessGrayTest, Count) {
  LooplessGray gray(15);

  unsigned int nfound = 0;
  for (auto it : gray)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(LooplessGrayTest, Pattern4) {
  std::array<std::uint32_t, 16> expected =
    { 0b0000, 0b0001, 0b0011, 0b0010, 0b0110, 0b0111, 0b0101, 0b0100,
      0b1100, 0b1101, 0b1111, 0b1110, 0b1010, 0b1011, 0b1001, 0b1000};
  LooplessGray gray(4);
  int idx = 0;
  for (auto it : gray) {
    ASSERT_LT(idx, 16) << "Got more gray values than expected";
    EXPECT_EQ(it, expected[idx])
      << "Got unexpected iterator value at index " << idx;
    ++idx;
  }
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
