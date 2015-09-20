package taocp.ntuples;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * Binary Gray Code generation using Knuth 7.2.1.1 Algorithm G
 */
public class GrayCode implements Iterable<Integer> {
  private final int n; // Number of bits

  public GrayCode(int n) {
    if (n <= 0) {
      throw new IllegalArgumentException("Invalid (non-positive) n");
    }
    if (n > 32) {
      throw new IllegalArgumentException("Only up to 32 bits supported");
    }
    this.n = n;
  }

  public GrayIterator iterator() {
    return new GrayIterator(n);
  }

  private static class GrayIterator implements Iterator<Integer> {

    private final int n;
    private int state; // previous number
    private int ainf; // parity bit

    private boolean done;

    public GrayIterator(int n) {
      this.n = n;
      this.state = 0;
      this.done = false;
      this.ainf = 0;
    }

    @Override
    public boolean hasNext() {
      return !done;
    }

    @Override
    public Integer next() {
      if (done) {
        throw new NoSuchElementException();
      }
      int result = state;
      ainf = 1 - ainf;
      int j;
      if (ainf == 1) {
        j = 0;
      } else {
        j = Integer.numberOfTrailingZeros(state) + 1;
      }

      if (j == n) {
        done = true;
      } else {
        state ^= (1 << j);
      }
      return result;
    }

  }

}
