package sedgewick.assignments;

import org.junit.BeforeClass;
import org.junit.Test;
import static org.junit.Assert.*;

/**
 * Simple tests for Union Find
 */
public class TestUnionFind {

    private static UnionFind uf;

    @BeforeClass
    public static void setupUnion() {
        uf = new UnionFind(10);
        uf.connect(4, 3);
        uf.connect(3, 8);
        uf.connect(6, 5);
        uf.connect(9, 4);
        uf.connect(2, 1);
        uf.connect(8, 9);
        uf.connect(5, 0);
        uf.connect(7, 2);
        uf.connect(6, 1);
        uf.connect(1, 0);
        uf.connect(6, 7);
    }

    @Test
    public void testNComponents() {
        assertEquals("Should be 2 components", uf.getNComponents(), 2);
    }

    @Test
    public void testConnected() {
        assertTrue("0 and 6 should be connected", uf.areConnected(0, 6));
        assertTrue("6 and 7 should be connected", uf.areConnected(6, 7));
        assertTrue("1 and 7 should be connected", uf.areConnected(1, 7));
        assertTrue("8 and 9 should be connected", uf.areConnected(8, 9));
        assertTrue("3 and 9 should be connected", uf.areConnected(3, 9));
        assertFalse("0 and 4 should not be connected", uf.areConnected(0, 4));
        assertFalse("7 and 8 should not be connected", uf.areConnected(7, 8));
    }

    @Test
    public void testClear() {
        UnionFind uf2 = new UnionFind(4);
        assertEquals("Should have 4 sites", uf2.getNComponents(), 4);
        uf2.connect(2, 3);
        assertTrue("2 and 3 should be connected", uf2.areConnected(2, 3));
        uf2.clear();
        assertFalse("2 and 3 should no longer be connected", uf2.areConnected(2, 3));
    }
}
