package sedgewick.assignments;

import org.junit.Test;
import static org.junit.Assert.*;

/**
 * Simple tests for Union Find
 */
public class TestUnionFind {

    @Test
    public void testNComponents() {
        UnionFind uf = new UnionFind(10);
        uf.union(4, 3);
        uf.union(3, 8);
        uf.union(6, 5);
        uf.union(9, 4);
        uf.union(2, 1);
        uf.union(8, 9);
        uf.union(5, 0);
        uf.union(7, 2);
        uf.union(6, 1);
        uf.union(1, 0);
        uf.union(6, 7);

        assertEquals("Should be 2 components", uf.count(), 2);
        assertTrue("0 and 6 should be connected", uf.connected(0, 6));
    }

}
