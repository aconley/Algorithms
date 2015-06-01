package sedgewick.assignments;

import org.junit.Test;
import static org.junit.Assert.*;
import java.util.Iterator;

public class TestLinkedDequeue {
  @Test
  public void TestFront() {
    // Testing add and remove from the front
    Dequeue<Integer> dq = new LinkedDequeue<Integer>();
    assertTrue("Should be empty", dq.isEmpty());
    dq.addFirst(4);
    dq.addFirst(3);
    assertEquals("Should have two elements", dq.size(), 2);
    // Note: we have to force unboxing, because java can be very stupid
    assertEquals("Should pull off 3", (int) dq.removeFirst(), 3);
    assertEquals("Should pull off 4", (int) dq.removeFirst(), 4);
    assertTrue("Should now be empty", dq.isEmpty());
  }

  @Test
  public void TestBack() {
    // Testing add and remove from the back
    Dequeue<Integer> dq = new LinkedDequeue<Integer>();
    assertTrue("Should be empty", dq.isEmpty());
    dq.addLast(4);
    dq.addLast(3);
    assertEquals("Should have two elements", dq.size(), 2);
    assertEquals("Should pull off 3", (int) dq.removeLast(), 3);
    assertEquals("Should pull off 4", (int) dq.removeLast(), 4);
    assertTrue("Should now be empty", dq.isEmpty());
  }

  @Test
  public void TestFrontBack() {
    // Testing add and remove from both ends
    Dequeue<Integer> dq = new LinkedDequeue<Integer>();
    assertTrue("Should be empty", dq.isEmpty());
    dq.addLast(4);
    dq.addLast(3);
    dq.addFirst(2);
    assertEquals("Should have three elements", dq.size(), 3);
    assertEquals("Should pull off 3", (int) dq.removeLast(), 3);
    assertEquals("Should have two elements", dq.size(), 2);
    dq.addLast(6);
    assertEquals("Should have three elements", dq.size(), 3);
    assertEquals("Should pull off 2", (int) dq.removeFirst(), 2);
    assertEquals("Should pull off 4", (int) dq.removeFirst(), 4);
    assertEquals("Should have one element", dq.size(), 1);
    assertFalse("Should not be empty", dq.isEmpty());
    dq.addFirst(1);
    assertEquals("Should have two elements", dq.size(), 2);
    assertEquals("Should pull 6 off the back", (int) dq.removeLast(), 6);
    assertEquals("Should pull 1 off the back", (int) dq.removeLast(), 1);
    assertTrue("Should now be empty", dq.isEmpty());
  }

  @Test
  public void TestSize() {
    Dequeue<Integer> dq = new LinkedDequeue<Integer>();
    assertEquals("Should start with no entries", dq.size(), 0);
    dq.addFirst(3);
    assertEquals("Should now have one entry", dq.size(), 1);
    dq.addLast(2);
    assertEquals("Should now have two entries", dq.size(), 2);
    dq.removeFirst();
    assertEquals("Should have one entry", dq.size(), 1);
    dq.removeLast();
    assertEquals("Should have 0 entries", dq.size(), 0);
  }

  @Test
  public void TestReverse() {
    // Test using a dequeue to reverse
    final int n = 5;
    Dequeue<Integer> dq = new LinkedDequeue<Integer>();
    for (int i = 0; i < n; ++i)
      dq.addLast(i);
    assertEquals("Should have 5 elements", dq.size(), n);
    for (int i = 0; i < n; ++i)
      assertEquals("Should pull off in reverse order from front",
          (int) dq.removeLast(), n - i - 1);
    assertTrue("Should now be empty", dq.isEmpty());
  }

  @Test
  public void TestIterator() {
    final int n = 5;
    Dequeue<Integer> dq = new LinkedDequeue<Integer>();
    for (int i = 0; i < n; ++i)
      dq.addFirst(i);
    Iterator<Integer> it = dq.iterator();
    assertTrue("Iterator should have more elements", it.hasNext());
    for (int i = 0; i < n; ++i)
        assertEquals("Should pull off in reverse order from front",
            (int) it.next(), n - i - 1);
    assertFalse("Iterator should not have more elements", it.hasNext());
    assertEquals("Original dequeue should still have 5 elements", dq.size(), n);
  }
}
