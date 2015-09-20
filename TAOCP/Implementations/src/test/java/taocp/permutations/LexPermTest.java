package taocp.permutations;

import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import java.util.Iterator;

import static org.assertj.core.api.Assertions.*;

public class LexPermTest {

  @org.junit.Test
  public void testIteratorHasNext() throws Exception {
    List<Integer> test = Arrays.asList(1, 2, 3, 4);
    LexPerm<Integer> l = new LexPerm<>(test);
    Iterator<List<Integer>> it = l.iterator();
    assertThat(it.hasNext())
        .as("Should have at least one permutation")
        .isTrue();
  }

  @org.junit.Test
  public void testFirstPerm() throws Exception {
    // First permutation should be input one!
    List<Integer> test = Arrays.asList(1, 2, 2, 4);
    LexPerm<Integer> l = new LexPerm<>(test);
    Iterator<List<Integer>> it = l.iterator();
    assertThat(it.next()).isEqualTo(test);
  }

  @org.junit.Test
  public void testIteratorHasExpectedNumber() throws Exception {
    List<Integer> test = Arrays.asList(1, 2, 3);
    LexPerm<Integer> l = new LexPerm<>(test);
    Iterator<List<Integer>> it = l.iterator();
    for (int i = 0; i < 6; ++i) {
      assertThat(it.hasNext())
          .as("Should have more permutations on " + (i+1))
          .isTrue();
      it.next();
    }
    assertThat(it.hasNext()).isFalse();
  }

  @org.junit.Test
  public void testOrderDistinct() throws Exception {
    // Test they come out in the order we expect
    //  when the elements are distinct
    List<Character> test = Arrays.asList('a', 'b', 'c');
    LexPerm<Character> l = new LexPerm<>(test);
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
      assertThat(perm)
          .as("On iteration " + (i + 1) + " got unexpected permutation")
          .isEqualTo(expct);
    }
    assertThat(itl.hasNext()).isFalse();
  }

  @org.junit.Test
  public void testOrderDistinctUnsorted() throws Exception {
    // Exactly the same as the previous test, but start
    //  them -out of order-.
    List<Character> test = Arrays.asList('b', 'a', 'c');
    LexPerm<Character> l = new LexPerm<>(test);
    // Expected order -- note you can't seem to do
    //  ArrayList<ArrayList>, at least not and have asList be
    //  at all helpful.
    List<List<Character>> expected = new ArrayList<>(6);
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
      assertThat(perm)
          .as("On iteration " + (i + 1) + " got unexpected permutation")
          .isEqualTo(expct);
    }
    assertThat(itl.hasNext()).isFalse();
  }

  @org.junit.Test
  public void testOrderRepeats() throws Exception {
    // A more difficult test -- 4 elements -with- a repeat
    List<Integer> test4 = Arrays.asList(1, 2, 2, 3);
    LexPerm<Integer> l4 = new LexPerm<>(test4);
    List<List<Integer>> expected4 = new ArrayList<>(12); // Note -- only 12!
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
      assertThat(perm)
          .as("On iteration " + (i+1) + " got unexpected permutation ")
          .isEqualTo(expct);
    }
    assertThat(itl4.hasNext()).isFalse();
  }
}