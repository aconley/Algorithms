package sedgewick.assignments;

import org.junit.BeforeClass;
import org.junit.Test;
import static org.junit.Assert.*;

public class TestPercolation {

    private static Percolation pc;

    @BeforeClass
    public static void setupPercolation() {
        pc = new Percolation(3);
    }

    @Test
    public void testN() {
        assertEquals("Should be 3 by 3", pc.getN(), 3);
    }

    @Test
    public void testOpen() {
        pc.clear();
        for (int i = 1; i <= pc.getN(); ++i)
            for (int j = 1; j <= pc.getN(); ++j)
                assertFalse("All sites should be open", pc.isOpen(i, j));

        pc.open(2, 1);
        assertFalse("1, 2 should be closed", pc.isOpen(1, 2));
        assertTrue("2, 1 should be open", pc.isOpen(2, 1));
    }

    @Test
    public void testFull() {
        pc.clear();
        for (int i = 1; i <= pc.getN(); ++i)
            for (int j = 1; j <= pc.getN(); ++j)
                assertFalse("No sites should be full", pc.isFull(i, j));
        pc.open(1, 2);
        assertTrue("1, 2 should now be open", pc.isOpen(1, 2));
        assertTrue("1, 2 should now be full", pc.isFull(1, 2));
        assertFalse("1, 3 should not be open", pc.isOpen(1, 3));
        assertFalse("1, 3 should not be full", pc.isFull(1, 3));
        pc.open(1, 3);
        assertTrue("1, 3 should now be open", pc.isOpen(1, 3));
        assertTrue("1, 3 should now be full", pc.isFull(1, 3));
    }

    @Test
    public void testPercolates() {
        pc.clear();
        assertFalse("Should not percolate on initialization", pc.doesPercolate());
        pc.open(1, 3);
        pc.open(2, 3);
        pc.open(2, 2);
        assertFalse("Should not percolate after first opens", pc.doesPercolate());
        pc.open(3, 1);
        assertFalse("Should not percolate yet", pc.doesPercolate());
        pc.open(3, 2);
        assertTrue("Should now percolate", pc.doesPercolate());
        assertTrue("3, 2 should be full", pc.isFull(3, 2));
        assertFalse("3, 3 should not be full", pc.isFull(3, 3));
    }
}
