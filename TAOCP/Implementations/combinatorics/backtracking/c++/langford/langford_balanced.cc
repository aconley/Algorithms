#include<array>
#include<vector>
#include<iostream>
#include<iterator>
#include<ostream>

#include "langford.h"
#include "langford_visitors.h"

template <class T, std::size_t N>
std::ostream& operator<<(std::ostream& o, const std::array<T, N>& arr)
{
    std::copy(arr.cbegin(), arr.cend(), std::ostream_iterator<T>(o, " "));
    return o;
}

// Finds balanced langford pairs for n = 16
int main(int argc, char* argv[]) {
  const int n = 16;
  backtracking::LangfordBalancedVisitor<n> vis;
  backtracking::langford(vis);
  int nFound = vis.getN();
  std::cout << "Found " << nFound << " solutions for n = " << n << "\n";

  for (int i = 0; i < nFound; ++ i) {
    std::cout << "  (" << (i + 1) << "): " << vis.get(i) << "\n";
  }
  return 0;
}
