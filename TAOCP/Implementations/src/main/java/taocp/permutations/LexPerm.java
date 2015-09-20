package taocp.permutations;

import java.util.*;

/**
 * Generate the permutations of a sequence lexicographically.
 *
 * Implementation of Knuth vol 4a Algorithm L from 7.2.1.2, p 319
 *
 * Note that only distinct permutations are produced -- so the
 * permutations of 1 2 2 are : 122 212 221, and there are 3
 * instead of 3! = 6.
 */
public class LexPerm<E extends Comparable<E>> implements Iterable<List<E>> {
  // Our strategy here is to hold a copy of the original list
  //  in this class, and have the Iterator just play with the indices
  private final E[] arr;  // Holds elements of original list

  public LexPerm(E[] orig) {
    arr = orig.clone(); // Defensive copy
    // The algorithm expects them to be sorted
    Arrays.sort(arr);
  }

  private class LexIterator implements Iterator<List<E>> {

    // The idea is that arr[idx[i]] is like a_i in Knuth - notation
    //  (although we use 0 based indexing here)
    private int[] idx; // Holds current permutation as indices into arr
    private boolean done; // True if we are done; worth caching

    public LexIterator() {
      idx = new int[arr.length];
      for (int i = 0; i < arr.length; ++i) idx[i] = i;
      done = false;
    }

    @Override
    public boolean hasNext() {
      return !done;
    }

    @Override
    public List<E> next() {

      if (done)
        throw new NoSuchElementException();

      // Step L1 -- make a copy of the current
      //  permutation to return
      List<E> r = new ArrayList<>(arr.length); // Note: no elements, just with given capacity
      for (int i : idx) r.add(arr[i]);

      // Next iterate a forward; this is the complicated bit!
      // Tricky part: we index from 0 rather than 1!
      // Knuth L2:
      //  Find the index j such that we have visited all permutations
      //   beginning with a[0] ... a[j]
      int j = arr.length - 2;
      while ((j >= 0) && (arr[idx[j]].compareTo(arr[idx[j + 1]]) >= 0))  // a[j] >= a[j+1]
        --j;

      if (j < 0) {
        done = true;
      } else {
        // Knuth L3
        //  Increase a[j]
        //  First find the smallest element a[l] that can legitimately
        //   follow a[0]...a[j-1]
        int l = arr.length - 1;
        E tmp = arr[idx[j]];
        while (tmp.compareTo(arr[idx[l]]) >= 0) // a[j] >= a[l]
          --l;
        // Then interchange a[j] and a[l]
        int t = idx[j];
        idx[j] = idx[l];
        idx[l] = t;

        // Knuth L4
        //  Finish the permutation in the lexicographically least way we can,
        //   which turns out to be by reversing a[j+1] to a[n-1].
        int k = j + 1;
        l = arr.length - 1;
        while (k < l) {
          t = idx[k];
          idx[k] = idx[l];
          idx[l] = t;
          ++k;
          --l;
        }
      }

      return r;
    }
  }

  @Override
  public Iterator<List<E>> iterator() {
    return new LexIterator();
  }
}
