package taocp.permutations;

import java.util.Arrays;
import java.util.ArrayList;
import java.util.Iterator;
import static org.junit.Assert.*;

public class TestLexicographic {

    @org.junit.Test
    public void testIterator() throws Exception {
        Integer[] test = {1, 2, 3, 4};
        Lexicographic<Integer> l = new Lexicographic(test);
        Iterator<ArrayList<Integer>> it = l.iterator();
        org.junit.Assert.assertTrue("Should have more permutations", it.hasNext());
    }
}