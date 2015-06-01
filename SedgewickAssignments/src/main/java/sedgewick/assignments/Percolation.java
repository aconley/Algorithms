package sedgewick.assignments;

import java.lang.IndexOutOfBoundsException;

// Model of a percolation system
public class Percolation {
    private final int n; // Size along one dimension
    private boolean[][] sites; // Site status; true if open

    // We need two union find structures -- one to handle
    //  percolation, one to handle being full.  Why?
    // Because being connected to the bottom is important
    //  for percolation, but not fullness.  So if we care
    //  about answering the fullness question correctly,
    //  we need a separate data structure
    private Connected ufP; // Percolation
    private Connected ufF; // Fullness

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
        ufP = new QuickUnion(n * n + 2);
        ufF = new QuickUnion(n * n + 1); // Ignore bottom node
    }

    public void clear() {
        ufP.clear();
        ufF.clear();
        for (int i = 0; i < n+1; ++i)
            for (int j = 0; j < n+1; ++j)
                sites[i][j] = false;
    }

    public int getN() { return n; }

    /**
     * Gets internal ID into union find structure of location
     * @param i Row index [1, N]
     * @param j Col index [1, N]
     */
    private int getUnionFindID(int i, int j) {
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
     * @throws IndexOutOfBoundsException if invalid i, j
     */
    public void open(int i, int j) {
        if (i < 1 || i > n)
            throw new IndexOutOfBoundsException("Row index out of bound");
        if (j < 1 || j > n)
            throw new IndexOutOfBoundsException("Column index out of bound");
        if (sites[i][j]) return;  // Already open

        sites[i][j] = true;

        // We have to connect to neighbors if they are open
        int compID;
        int id = getUnionFindID(i, j);
        // Connect above
        if (i == 1) { // First row is special -- connects to initial site
            ufP.connect(id, 0);
            ufF.connect(id, 0);
        } else if (sites[i - 1][j]) {
            compID = getUnionFindID(i - 1, j);
            ufP.connect(id, compID);
            ufF.connect(id, compID);
        }

        // Connect below -- now last one is special
        if (i == n)  // Note -- we don't connect Full!
            ufP.connect(id, n * n + 1);
        else if (sites[i + 1][j]) {
            compID = getUnionFindID(i + 1, j);
            ufP.connect(id, compID);
            ufF.connect(id, compID);
        }

        // Connect to the left
        if ((j > 1) && sites[i][j - 1]) {
            compID = getUnionFindID(i, j - 1);
            ufP.connect(id, compID);
            ufF.connect(id, compID);
        }

        // Connect to right
        if ((j < n) && sites[i][j + 1]) {
            compID = getUnionFindID(i, j + 1);
            ufP.connect(id, compID);
            ufF.connect(id, compID);
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
        // isOpen also does bounds checking for us
        if (!isOpen(i, j)) return false;
        return ufF.areConnected(0, getUnionFindID(i, j));
    }

    /**
     * Does the system percolate?
     * @return True if the system percolates, false otherwise
     */
    public boolean doesPercolate() {
        return ufP.areConnected(0, n * n + 1);
    }

}
