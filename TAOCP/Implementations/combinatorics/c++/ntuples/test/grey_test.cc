#include "gtest/gtest.h"
#include "binarygrey.h"
#include "looplessgrey.h"

#include<array>

using namespace ntuples;

////////////////////////////////////////
// Tests of the binary version
////////////////////////////////////////
TEST(BinaryGreyTest, IteratorCount) {
  int n = 15;
  auto start = BinaryGreyIterator::begin(n);
  auto end = BinaryGreyIterator::end(n);

  unsigned int nfound = 0;
  for (auto it = start; it != end; ++it)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(BinaryGreyTest, Count) {
  BinaryGrey grey(15);

  unsigned int nfound = 0;
  for (auto it : grey)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(BinaryGreyTest, Pattern4) {
  std::array<std::uint32_t, 16> expected =
    { 0b0000, 0b0001, 0b0011, 0b0010, 0b0110, 0b0111, 0b0101, 0b0100,
      0b1100, 0b1101, 0b1111, 0b1110, 0b1010, 0b1011, 0b1001, 0b1000};
  BinaryGrey grey(4);
  int idx = 0;
  for (auto it : grey) {
    ASSERT_LT(idx, 16) << "Got more grey values than expected";
    EXPECT_EQ(it, expected[idx])
      << "Got unexpected iterator value at index " << idx;
    ++idx;
  }
}

TEST(BinaryGreyTest, GetNext) {
  EXPECT_EQ(BinaryGrey::getNext(0b0110), 0b0111);
  EXPECT_EQ(BinaryGrey::getNext(0b1110), 0b1010);
}

////////////////////////////////////////
// Tests of the loopless version
////////////////////////////////////////
TEST(LooplessGreyTest, Count) {
  LooplessGrey grey(15);

  unsigned int nfound = 0;
  for (auto it : grey)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

TEST(LooplessGreyTest, Pattern4) {
  std::array<std::uint32_t, 16> expected =
    { 0b0000, 0b0001, 0b0011, 0b0010, 0b0110, 0b0111, 0b0101, 0b0100,
      0b1100, 0b1101, 0b1111, 0b1110, 0b1010, 0b1011, 0b1001, 0b1000};
  LooplessGrey grey(4);
  int idx = 0;
  for (auto it : grey) {
    ASSERT_LT(idx, 16) << "Got more grey values than expected";
    EXPECT_EQ(it, expected[idx])
      << "Got unexpected iterator value at index " << idx;
    ++idx;
  }
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
