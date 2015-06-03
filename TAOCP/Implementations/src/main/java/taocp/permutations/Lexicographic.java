/**
 * Implementation of Knuth vol 4a algorithm L from 7.2.1.2, p 319
 *
 * Generates all permutations of an input array Lexicographically
 */

package taocp.permutations;

import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;
import java.util.Arrays;

public class Lexicographic<E extends Comparable<E>> implements Iterable<List<E>> {
    private E[] arr;  // Holds elements of original list

    public Lexicographic(E[] orig) {
        this.arr = orig.clone(); // Defensive copy
    }

    private class LexIterator implements Iterator<List<E>> {

        private int n; // Number of elements in arrayList
        private E[] a; // Holds current permutation
        private boolean done; // True if we are done; worth caching

        public LexIterator() {
            n = arr.length;
            a = arr.clone();  // Another defensive copy
            done = false;
        }

        @Override
        public boolean hasNext() {
            return !done;
        }

        @Override
        public List<E> next() {

            // Step L1 -- make a copy of the current
            //  permutation to return
            List<E> r = Arrays.asList(a.clone()); // The clone is critical!

            // Next iterate a forward; this is the complicated bit!
            // Tricky part: we index from 0 rather than 1!
            // Knuth L2
            // Note: a0 (which isn't touched) makes this safe
            int j = n - 2;
            while ((j >= 0) && (a[j].compareTo(a[j + 1]) >= 0))  // a[j] >= a[j+1]
                --j;

            if (j >= 0) {
                // Knuth L3
                int l = n - 1;
                E tmp = a[j];
                while (tmp.compareTo(a[l]) >= 0) // a[j] >= a[l]
                    --l;
                a[j] = a[l];
                a[l] = tmp;

                // Knuth L4
                int k = j + 1;
                l = n - 1;
                while (k < l) {
                    tmp = a[k];
                    a[k] = a[l];
                    a[l] = tmp;
                    ++k;
                    --l;
                }
            } else {
                done = true;
            }

            return r;
        }
    }

    @Override
    public Iterator<List<E>> iterator() {
        return new LexIterator();
    }
}
