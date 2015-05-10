package sedgewick.assignments;

import sedgewick.assignments.UnionFind;
import java.lang.IndexOutOfBoundsException;

// Model of a percolation system
public class Percolation {
    private int n; // Size along one dimension
    private boolean[][] sites; // Site status; true if open

    // We need two union find structures -- one to handle
    //  percolation, one to handle being full.  Why?
    // Because being connected to the bottom is important
    //  for percolation, but not fullness.  So if we care
    //  about ansering the fullness question correctly,
    //  we need a seperate data structre
    private UnionFind ufP; // Percolation
    private UnionFind ufF; // Fullness

    /**
     * Constructor
     * @param N grid size (grid is N x N)
     */
    public Percolation(int N) {
        n = N;
        // Grids are [1, N] x [1, N], just keep
        //  empty row/col rather than playing index tricks
        sites = new boolean[n+1][n+1];
        // Use two 'shadow' sites, one at the top,
        //  one at the bottom (indices 0 and n*n + 1)
        ufP = new UnionFind(n * n + 2);
        ufF = new UnionFind(n * n + 1); // Ignore bottom node
    }

    public int getN() { return n; }

    /**
     * Gets internal ID into union find structure of locatin
     * @param i Row index [1, N]
     * @param j Col index [1, N]
     */
    private int getUFID(int i, int j) {
        //return (i - 1) * n + (j - 1) + 1;  // 1 for top site
        return (i - 1) * n + j;
    }

    public boolean isOpen(int i, int j) {
        if (i < 1 || i > n)
            throw new IndexOutOfBoundsException("Row index out of bound");
        if (j < 1 || j > n)
            throw new IndexOutOfBoundsException("Column index out of bound");
        return sites[i][j];
    }

    /**
     * Opens the specified site if it isn't already open
     * @param i Row index [1, N]
     * @param j Column index [1, N]
     * @throw IndexOutOfBoundsException if invalid i, j
     */
    public void open(int i, int j) {
        if (i < 1 || i > n)
            throw new IndexOutOfBoundsException("Row index out of bound");
        if (j < 1 || j > n)
            throw new IndexOutOfBoundsException("Column index out of bound");
        if (sites[i][j]) return;  // Already open

        // We have to connect to neighbors if they are open
        int cmpsite;
        int id = getUFID(i, j);
        // Connect above
        if (i == 1) { // First row is special -- connects to initial site
            ufP.union(id, 0);
            ufF.union(id, 0);
        } else if (sites[i - 1][j]) {
            cmpsite = getUFID(i - 1, j);
            ufP.union(id, cmpsite);
            ufF.union(id, cmpsite);
        }

        // Connect below -- now last one is special
        if (i == n)  // Note -- we don't connect Full!
            ufP.union(id, n * n + 1);
        else if (sites[i + 1][j]) {
            cmpsite = getUFID(i + 1, j);
            ufP.union(id, cmpsite);
            ufF.union(id, cmpsite);
        }

        // Connect to the left
        if ((j > 1) && sites[i][j - 1]) {
            cmpsite = getUFID(i, j - 1);
            ufP.union(id, cmpsite);
            ufF.union(id, cmpsite);
        }

        // Connect to right
        if ((j < n) && sites[i][j + 1]) {
            cmpsite = getUFID(i, j + 1);
            ufP.union(id, cmpsite);
            ufF.union(id, cmpsite);
        }
    }

    /**
     * Is the site full (connected to an open site at the top)?
     * @param i Row of site
     * @param j Col of site
     * @return True if full
     * @throws IndexOutOfBoundsException if invalid i, j
     */
    public boolean isFull(int i, int j) {
        if (i < 1 || i > n)
            throw new IndexOutOfBoundsException("Row index out of bound");
        if (j < 1 || j > n)
            throw new IndexOutOfBoundsException("Column index out of bound");
        return ufF.connected(0, getUFID(i, j));
    }

    /**
     * Does the system percolate?
     * @return True if the system percolates, false otherwise
     */
    public boolean percolates() {
        return ufP.connected(0, n * n + 1);
    }

}