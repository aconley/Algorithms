package sedgewick.assignments;

/**
 * Created by aconley on 5/17/15.
 */
public interface Connected {
    /**
     * Get the number of connected components
     * @return The number of connected components
     */
    public int getNComponents();

    /**
     * Return the component index for the point p
     * @param p The point identifier
     * @return The component identifier
     */
    public int getComponent(int p);

    /**
     * Are p and q connected?
     * @param p The first point p
     * @param q The second point q
     * @return True if p and q are connected, false otherwise
     */
    public boolean areConnected(int p, int q);

    /**
     * Connect p to q
     * @param p The first point
     * @param q The second point
     */
    public void connect(int p, int q);
}
