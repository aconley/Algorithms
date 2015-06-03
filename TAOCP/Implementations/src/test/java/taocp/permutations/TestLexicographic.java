package taocp.permutations;

import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import java.util.Iterator;
import static org.junit.Assert.*;

public class TestLexicographic {

    @org.junit.Test
    public void testIteratorHasNext() throws Exception {
        Integer[] test = {1, 2, 3, 4};
        Lexicographic<Integer> l = new Lexicographic<>(test);
        Iterator<List<Integer>> it = l.iterator();
        assertTrue("Should have at least one permutation", it.hasNext());
    }

    @org.junit.Test
    public void testFirstPerm() throws Exception {
        // First permutation should be input one!
        Integer[] test = {1, 2, 2, 4};
        Lexicographic<Integer> l = new Lexicographic<>(test);
        Iterator<List<Integer>> it = l.iterator();
        assertEquals("First permutation should be input",
                     Arrays.asList(test), it.next());
    }

    @org.junit.Test
    public void testIteratorHasExpectedNumber() throws Exception {
        Integer[] test = {1, 2, 3};
        Lexicographic<Integer> l = new Lexicographic<>(test);
        Iterator<List<Integer>> it = l.iterator();
        for (int i = 0; i < 6; ++i) {
            assertTrue("Should have more permutations on " + (i+1), it.hasNext());
            it.next();
        }
        assertFalse("Should not have any more permutations after " + 6, it.hasNext());
    }

    @org.junit.Test
    public void testOrderDistinct() throws Exception {
        // Test they come out in the order we expect
        //  when the elements are distinct
        Character[] test = {'a', 'b', 'c'};
        Lexicographic<Character> l = new Lexicographic<>(test);
        // Expected order -- note you can't seem to do
        //  ArrayList<ArrayList>, at least not and have asList be
        //  at all helpful.
        List<List<Character>> expected = new ArrayList<List<Character>>(6);
        expected.add(Arrays.asList('a', 'b', 'c'));
        expected.add(Arrays.asList('a', 'c', 'b'));
        expected.add(Arrays.asList('b', 'a', 'c'));
        expected.add(Arrays.asList('b', 'c', 'a'));
        expected.add(Arrays.asList('c', 'a', 'b'));
        expected.add(Arrays.asList('c', 'b', 'a'));

        Iterator<List<Character>> itl = l.iterator();
        Iterator<List<Character>> ite = expected.iterator();
        for (int i = 0; i < 6; ++i) {
            List<Character> perm = itl.next();
            List<Character> expct = ite.next();
            assertEquals("On iteration " + (i + 1) + " got unexpected permutation",
                         expct, perm);
        }
        assertFalse("Should not have any more permutations after " + 6, itl.hasNext());
    }

    @org.junit.Test
    public void testOrderRepeats() throws Exception {
        // A more difficult test -- 4 elements -with- a repeat
        Integer[] test4 = {1, 2, 2, 3};
        Lexicographic<Integer> l4 = new Lexicographic<>(test4);
        List<List<Integer>> expected4 = new ArrayList<List<Integer>>(12); // Note -- only 12!
        expected4.add(Arrays.asList(1, 2, 2, 3));
        expected4.add(Arrays.asList(1, 2, 3, 2));
        expected4.add(Arrays.asList(1, 3, 2, 2));
        expected4.add(Arrays.asList(2, 1, 2, 3));
        expected4.add(Arrays.asList(2, 1, 3, 2));
        expected4.add(Arrays.asList(2, 2, 1, 3));
        expected4.add(Arrays.asList(2, 2, 3, 1));
        expected4.add(Arrays.asList(2, 3, 1, 2));
        expected4.add(Arrays.asList(2, 3, 2, 1));
        expected4.add(Arrays.asList(3, 1, 2, 2));
        expected4.add(Arrays.asList(3, 2, 1, 2));
        expected4.add(Arrays.asList(3, 2, 2, 1));
        Iterator<List<Integer>> itl4 = l4.iterator();
        Iterator<List<Integer>> ite4 = expected4.iterator();
        for (int i = 0; i < 12; ++i) {
            List<Integer> perm = itl4.next();
            List<Integer> expct = ite4.next();
            assertEquals("On iteration " + (i+1) + " got unexpected permutation ",
                         expct, perm);
        }
        assertFalse("Should not have any more permutations after " + 12, itl4.hasNext());
    }
}