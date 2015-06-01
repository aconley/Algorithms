package sedgewick.assignments;

import org.junit.Test;
import java.util.Arrays;
import static org.junit.Assert.*;

import java.util.Iterator;

public class TestRandomizedQueue {
  @Test
  public void testIsEmpty() {
    RandomizedQueue<Integer> rq = new RandomizedQueue<Integer>();
    assertTrue(rq.isEmpty());
    rq.enqueue(4);
    assertFalse(rq.isEmpty());
    rq.dequeue();
    assertTrue(rq.isEmpty());
  }

  @Test
  public void testSize() {
    RandomizedQueue<Integer> rq = new RandomizedQueue<Integer>();
    assertEquals("Should have no elements", rq.size(), 0);

    for (int i = 0; i < 10; ++i) {
      rq.enqueue(i);
      assertEquals("Adding elements should increase size", rq.size(), i + 1);
    }
    for (int i = 9; i > 5; --i) {
      rq.dequeue();
      assertEquals("Removing elements should decrease size", rq.size(), i);
    }
  }

  // See if we get back the same things on dequeue
  @Test
  public void testEnqueueDequeue() {
    RandomizedQueue<Integer> rq = new RandomizedQueue<Integer>();
    int[] testarray = {0, 11, 17, 23, 45, 77, 55};
    Arrays.sort(testarray);
    for (int i = 0; i < testarray.length; ++i)
      rq.enqueue(testarray[i]);
    int[] readarray = new int[testarray.length];
    for (int i = 0; i < readarray.length; ++i)
      readarray[i] = (int) rq.dequeue();
    assertTrue("RandomizedQueue should now be empty", rq.isEmpty());
    Arrays.sort(readarray);
    assertArrayEquals("Should get back expected elements on insertion/removal",
        readarray, testarray);
  }

  // See if we get back the same things on iteration
  @Test
  public void testIteration() {
    RandomizedQueue<Integer> rq = new RandomizedQueue<Integer>();
    int[] testarray = {0, 11, 17, 23, 45, 77, 55};
    Arrays.sort(testarray);
    for (int i = 0; i < testarray.length; ++i)
      rq.enqueue(testarray[i]);
    int[] readarray = new int[testarray.length];
    int i = 0;
    for (Integer v : rq) {
      readarray[i] = (int) v;
      i += 1;
    }
    assertFalse("Iteration should not empty", rq.isEmpty());
    assertEquals("Iteration should not affect count", rq.size(), testarray.length);
    Arrays.sort(readarray);
    assertArrayEquals("Should get back expected elements on iteration",
        readarray, testarray);
  }
}

