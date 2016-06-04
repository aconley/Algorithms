#include "gtest/gtest.h"
#include "binarygreyiterator.h"

using namespace ntuples;

////////////////////////////////////////
// Tests of the binary iterator
////////////////////////////////////////
TEST(BinaryGreyTest, Count) {
  int n = 15;
  auto start = BinaryGreyIterator::begin(n);
  auto end = BinaryGreyIterator::end(n);

  unsigned int nfound = 0;
  for (auto it = start; it != end; ++it)
    ++nfound;

  EXPECT_EQ(nfound, 32768) << "Got unexpected number of ntuples for n = 15";
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
