package sedgewick.assignments;

// Full Quick Union Algorithm from Sedgewick et al. 4th edition

// This uses both weighting and path compression.
public class QuickUnion implements Connected {
    private int[] id;    // id[i] = component identifier of i
    private int[] sz;    // Size of each component
    private int count;   // number of components

    /**
     * Initializes an empty union-find data structure
     *
     * @param N the number of objects
     */
    public QuickUnion(int N) {
        count = N;
        id = new int[N];
        for (int i = 0; i < N; ++i) id[i] = i;
        sz = new int[N];
        for (int i = 0; i < N; ++i) sz[i] = 1;
    }

    /**
     * Reset all entries to unconnected
     */
    public void clear() {
        for (int i = 0; i < id.length; ++i) id[i] = i;
        for (int i = 0; i < id.length; ++i) sz[i] = 1;
        count = id.length;
    }

    /**
     * Query number of sites
     * @return Number of sites
     */
    public int getN() { return id.length; }

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
        int comp = p;
        // Search
        while (comp != id[comp]) comp = id[comp];
        // Path compression -- rewalk the tree,
        //  making each node we hit point at the root
        //  we just found
        int oldidx;
        while (p != id[p]) {
            oldidx = p;
            p = id[p];
            id[oldidx] = comp;
        }
        return comp;
    }

    /**
     * Finds if p and q are connected
     *
     * @param p The first point
     * @param q the second point
     * @return true if p and q are connected, false otherwise
     */
    public boolean areConnected(int p, int q) {
        return getComponent(p) == getComponent(q);
    }

    /**
     * Merges the component containing p with the one containing q
     *
     * @param p the first point
     * @param q the second point
     */
    public void connect(int p, int q) {
        int pComp = getComponent(p);
        int qComp = getComponent(q);
        if (pComp == qComp) return; // Already connected

        // Always join smaller one to larger one
        if (sz[pComp] < sz[qComp]) {
            id[pComp] = qComp;
            sz[qComp] += sz[pComp];
        } else {
            id[qComp] = pComp;
            sz[pComp] += sz[qComp];
        }

        count--;
    }
}