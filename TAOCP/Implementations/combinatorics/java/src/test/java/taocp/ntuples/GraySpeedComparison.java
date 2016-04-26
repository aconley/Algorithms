package taocp.ntuples;

import org.openjdk.jmh.annotations.Benchmark;

/**
 * See which implementation is faster
 * <p>
 *   Note that this doesn't count setup time, which is larger
 *   for the loopless algorithm.
 * </p>
 */
public class GraySpeedComparison {
  private static final int NTEST = 16;

  @Benchmark
  public static void binaryGrayTest() {
    Iterable<Integer> binaryGray = new GrayCode(NTEST);
    int cnt = 0;
    for (Integer val : binaryGray) {
      cnt += 1;
    }
  }

  @Benchmark
  public static void looplessGrayTest() {
    Iterable<Integer> looplessGray = new LooplessGrayCode(NTEST);

    int cnt = 0;
    for (Integer val : looplessGray) {
      cnt += 1;
    }
  }
}
