package taocp.permutations;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

/**
 * Generate the permutations of a sequence containing only unique elements
 * using only adjacent pair interchanges
 *
 * Implementation of Knuth 4a Algorithm P from 7.2.1.2 p. 322
 *
 * Note that, unlike Algorithm L, this doesn't handle repeats -- or,
 * rather, it always treats elements as distinct.  Thus, the inputs do
 * not need to be sorted, nor Comparable
 */
public class PlainPerm<E> implements Iterable<List<E>> {
  // Like LexPerm, we hold the original array here and just
  //  play with indices in the iterators
  private final E[] arr; // Original array

  public PlainPerm(E[] orig) {
    arr = orig.clone(); // Defensive copy
  }

  private class PlainIterator implements Iterator<List<E>> {

    private int[] idx; // Holds current permutation as indices into arr
    private int[] c; // Inversions
    private byte[] sigma; // Direction array
    private boolean done; // True if we are done; worth caching

    public PlainIterator() {
      idx = new int[arr.length];
      for (int i = 0; i < arr.length; ++i) idx[i] = i; // Initial position
      c = new int[arr.length]; // Step P1: set c[i] to 0
      sigma = new byte[arr.length];
      for (int i = 0; i < arr.length; ++i) sigma[i] = +1; // Go up initially
      done = false;
    }

    @Override
    public boolean hasNext() {
      return !done;
    }

    @Override
    public List<E> next() {

      // Step P2 -- visit permutation by making a copy of the current
      //  permutation to return after we update
      List<E> r = new ArrayList<>(arr.length); // Note: no elements, just with given capacity
      for (int i : idx) r.add(arr[i]);

      // Step P3 -- prepare for change
      //   Determine the j for which c[j] is about to change,
      //   s is the number of indices k > j such that c[k] = k-1
      int j = arr.length - 1;
      int s = 0;
      int q = c[j] + sigma[j];
      // Main loop
      while ((q < 0) || (q != j)) {
        if (q < 0) {
          // P7 -- switch direction
          sigma[j] *= -1;
          --j;
        } else {
          // P6
          if (j == 0) {
            // Termination
            done = true;
            return r;
          }
          s += 1;
          sigma[j] *= -1;
          --j;
        }
      }
      // P5 -- interchange and finish permutation
      int idx1 = j - c[j] + s;
      int idx2 = j - q + s;
      int t = idx[idx1];
      idx[idx1] = idx[idx2];
      idx[idx2] = t;
      c[j] = q;

      return r;
    }
  }

  @Override
  public Iterator<List<E>> iterator() {
      return new PlainIterator();
    }
}
