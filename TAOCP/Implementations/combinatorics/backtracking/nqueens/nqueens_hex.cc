// A modified version of nqueens that uses a hexagonal board.
#include<vector>
#include<exception>
#include<cstdint>
#include<iostream>

static const int MultiplyDeBruijnBitPosition[32] =
{
  0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
  31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9
};

int getPositionOfLeastSetBit(std::uint32_t v) {
  return MultiplyDeBruijnBitPosition[((uint32_t)((v & -v) * 0x077CB531U)) >> 27];
}

long hex_nqueens(int n) {

  if (n == 0) return 0;
  if (n > 32) {
    throw std::invalid_argument("n must be <= 32");
  }
  if (n == 1) {
    return 1;
  }

  // State vectors a_l, b_l, etc.
  //  Note: in this algorithm we use 1 based indexing for a, c, s
  std::vector<std::uint32_t> a(n + 1, 0u), c(n + 1, 0u), s(n + 1, 0u);

  // Mask 2^n - 1
  std::uint32_t mu = n == 32 ? (~0u) : ((1 << n) - 1);

  // W1
  std::uint32_t t;
  int l = 1;
  long n_solutions = 0;

  W2: // Enter level l
  if (l > n) {
    ++n_solutions;
    goto W4;
  }
  s[l] = mu & (~a[l - 1]) & (~c[l - 1]);

  W3: // Try t
  if (s[l] != 0) {
    t = s[l] & (-s[l]);
    a[l] = a[l - 1] + t;
    c[l] = ((c[l - 1] + t) << 1) & mu;
    s[l] -= t;
    ++l;
    goto W2;
  }

  W4: // backtrack
  if (l > 0) {
    --l;
    goto W3;
  }
  // Otherwise we're done
  return n_solutions;
}

int main(int argc, char **argv) {
  for (int n = 1; n < 15; ++n) {
    std::cout << "n = " << n << " " << hex_nqueens(n) << std::endl;
  }
}