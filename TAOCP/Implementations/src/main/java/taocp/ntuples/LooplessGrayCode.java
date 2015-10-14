package taocp.ntuples;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * Binary Gray Code generation using Knuth 7.2.1.1 Algorithm L
 */
public class LooplessGrayCode implements Iterable<Integer> {
  private final int n; // Number of bits

  public LooplessGrayCode(int n) {
    if (n <= 0) {
      throw new IllegalArgumentException("Invalid (non-positive) n");
    }
    if (n > 32) {
      throw new IllegalArgumentException("Only up to 32 bits supported");
    }
    this.n = n;
  }

  public Iterator<Integer> iterator() {
    return new LooplessGrayIterator(n);
  }

  private static class LooplessGrayIterator implements Iterator<Integer> {
    private final int n;
    private int[] focus; // Focus pointers
    private int state; // The tuple a
    private boolean done;

    public LooplessGrayIterator(int n) {
      this.n = n;
      this.state = 0;
      this.focus = new int[n+1];
      this.done = false;

      for (int i = 0; i <= n; ++i) {
        focus[i] = i;
      }
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
      int j = focus[0];
      focus[0] = 0;

      if (j == n) {
        done = true;
      } else {
        focus[j] = focus[j + 1];
        focus[j + 1] = j + 1;
        state ^= (1 << j);
      }

      return result;
    }
  }
}
