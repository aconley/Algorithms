package sedgewick.assignments;

// Basic Union Find Algorithm from Sedgewick et al. 4th edition

// This is the unweighted version that does full compression
//  on merge
public class UnionFind implements Connected {
    private int[] id;    // id[i] = component identifier of i
    private int count;   // number of components

    /**
     * Initializes an empty union-find data structure
     *
     * @param N the number of objects
     */
    public UnionFind(int N) {
        count = N;
        id = new int[N];
        for (int i = 0; i < N; i++)
            id[i] = i;
    }

    /**
     * Returns the number of components.
     *
     * @return the number of components (between 1 and N)
     */
    public int getNComponents() {
        return count;
    }

    /**
     * Returns the component identifier for the component containing p.
     *
     * @param p The element we are finding the component for
     * @return the component identifier
     */
    public int getComponent(int p) {
        return id[p];
    }

    /**
     * Finds if p and q are connected
     *
     * @param p The first point
     * @param q the second point
     * @return true if p and q are connected, false otherwise
     */
    public boolean areConnected(int p, int q) {
        return id[p] == id[q];
    }

    /**
     * Merges the component containing p with the one containing q
     *
     * @param p the first point
     * @param q the second point
     */
    public void connect(int p, int q) {
        if (areConnected(p, q)) return;
        int pid = id[p];
        for (int i = 0; i < id.length; i++)
            if (id[i] == pid) id[i] = id[q];
        count--;
    }
}