package taocp.trees;

import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * Generate all sets of nested parenthesis.
 * <p>
 * This is equivalent to generating all forests.  This particular
 * implementation uses bit twiddling techniques, and is
 * Knuth 4A problem 7.1.3.23.
 */
class NestedParens implements Iterable<String> {
  final private static long mu0 = 0x5555555555555555L;

  final private int nparens;
  final private long endmask;

  /**
   * @param nparens Number of open (or close) parenthesis.
   */
  public NestedParens(byte nparens) {
    if (nparens > 32) {
      throw new IllegalArgumentException("Too many parentheses; <= 32 required");
    }
    this.nparens = nparens;
    endmask = ~(-1L >>> (64 - 2 * nparens));
  }

  public NestedParens(int nParens) {
    this((byte) nParens);
  }

  public Iterator<String> iterator() {
    return new ParenIterator();
  }

  private class ParenIterator implements Iterator<String> {
    private long state;
    private StringBuilder sb;

    public ParenIterator() {
      state = -1L >>> (64 - nparens); // 000...111
      sb = new StringBuilder(2 * nparens);
    }

    @Override
    public boolean hasNext() {
      return ((state & endmask) == 0);
    }

    @Override
    public String next() {
      if (!hasNext()) {
        throw new NoSuchElementException("Iteration completed");
      }
      String retval = convertToString(state);
      long z = state^mu0;
      long u = z^(z-1);
      long v = state | u;
      long w = v + 1;
      z = v & (~w);
      z = z >>> Long.bitCount(u & mu0);
      state = w + z;
      return retval;
    }

    private String convertToString(long x) {
      sb.setLength(0);
      for (int i = 0; i < 2 * nparens; ++i) {
        if ((x & 0x1L) == 0) {
          sb.append("(");
        } else {
          sb.append(")");
        }
        x >>= 1;
      }
      return sb.reverse().toString();
    }
  }

  public static void main(String[] args) {
    for (String arg : args) {
      byte nparens = Byte.parseByte(arg);
      System.out.println("For nparens: " + nparens);
      NestedParens pars = new NestedParens(nparens);
      for (String par : pars) {
        System.out.println("    " + par);
      }
    }
  }
}