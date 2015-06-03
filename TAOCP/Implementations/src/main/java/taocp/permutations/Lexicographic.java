/**
 * Implementation of Knuth vol 4a algorithm L from 7.2.1.2, p 319
 *
 * Generates all permutations of an input array Lexicographically
 */

package taocp.permutations;

import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

public class Lexicographic<E> implements Iterable<List<E>> {
    private E[] arr;  // Holds elements of original list

    public Lexicographic(E[] orig) {
        this.arr = orig.clone(); // Defensive copy
    }

    private class LexIterator implements Iterator<List<E>> {

        private int n; // Number of elements in arrayList
        private int[] a; // Holds current permutation indices
        private boolean done; // True if we are done; worth caching

        public LexIterator(int n) {
            this.n = n;
            a = new int[n + 1]; // a[0] is a dummy
            for (int i = 0; i < n + 1; ++i)
                a[i] = i;
            done = false;
        }

        @Override
        public boolean hasNext() {
            return !done;
        }

        @Override
        public List<E> next() {
            // This is the array list we return on each
            // permutation
            // Note this makes an empty list of capacity n
            List<E> r = new ArrayList<E>(n);

            // Knuth L1 -- this copies the current permutation
            //  into r
            for (int i = 0; i < n; ++i) r.add(arr[a[i+1]-1]);

            // Next iterate a forward; this is the complicated bit!
            // Knuth L2
            // Note: a0 (which isn't touched) makes this safe
            int j = n - 1;
            while (a[j] >= a[j + 1])
                --j;

            // Knuth L3
            if (j > 0) {
                int l = n;
                int tmp = a[j];
                while (tmp >= a[l])
                    --l;
                a[j] = a[l];
                a[l] = tmp;

                // Knuth L4
                int k = j + 1;
                l = n;
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
        return new LexIterator(arr.length);
    }
}
