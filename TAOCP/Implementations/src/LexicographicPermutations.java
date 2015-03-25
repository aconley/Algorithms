/**
 * Implementation of Knuth vol 4a algorithm L from 7.2.1.2, p 319
 *
 * Generates all permutations of an input array Lexicographically
 */

package knuth.permutations;

import java.util.Iterator;

public class LexicographicPermutations<E> implements Iterable<E[]> {
    private E[] arrayList;  // Holds elements of original list

    public LexicographicPermutations(E[] orig) {
        this.arrayList = orig;  // Should we copy?
    }

    private class LexIterator<E> implements Iterator<E[]> {

        private int n; // Number of elements in arrayList
        private int[] a; // Holds current permutation indices
        private boolean done; // True if we are done; worth caching

        public LexIterator(int n) {
            this.n = n;
            this.a = new int[n + 1]; // a[0] is a dummy
            for (int i = 0; i < n + 1; ++i)
                this.a[i] = i;
            this.done = false;
        }

        @Override
        public boolean hasNext() {
            return !done;
        }

        @Override
        public E[] next() {
            E[] r; // Return array
            r = new E[n];

            // Knuth L1
            for (int i = 0; i < n + 1; ++i) {
                r[i] = arrayList[a[i]-1];
            }

            // Next iterate a forward; this is the complicated bit
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
    public Iterator<E[]> iterator() {
        return new LexIterator<E>(arrayList.length);
    }
}
