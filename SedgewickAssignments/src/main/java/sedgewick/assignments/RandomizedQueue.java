package sedgewick.assignments;

import java.util.NoSuchElementException;
import java.util.Random;
import java.util.ArrayList;
import java.util.Iterator;

public class RandomizedQueue<Item> implements Iterable<Item> {
  // The method here is to use an array, repack as needed,
  //  always insert on the end, but remove randomly.
  // The iterator takes extra room to set up the random
  //  access.
  // Use an ArrayList for the internal representation, since type
  //  erasure makes using an array complicated
  // always insert at the end.  The randomness comes on removal
  private Random rand; // Random number generator
  private ArrayList<Item> elements; // Holds actual data

  /**
   * Swap the elements in positions i and j
   * @param i First index
   * @param j Second index
   */
  private void swapElements(int i, int j) {
    if (i == j) return;
    Item t = elements.get(i);
    elements.set(i, elements.get(j));
    elements.set(j, t);
  }

  /**
   * Construct empty queue
   */
  public RandomizedQueue() {
    elements = new ArrayList<Item>();
    rand = new Random();
  }

  /**
   * Is the queue empty?
   * @return Returns true if the queue is empty, false otherwise
   */
  public boolean isEmpty() { return elements.isEmpty(); }

  /**
   * Get the size of the queue
   * @return the number of elements in the queue
   */
  public int size() { return elements.size(); }

  /**
   * Add an item to the queue
   * @param item Item to add
   */
  public void enqueue(Item item) {
    // Add in a random position by inserting at the end
    //  then swapping randomly
    elements.add(item);
    int nelem = elements.size();
    if (nelem > 1) {
      int idx = rand.nextInt(nelem);
      swapElements(idx, nelem - 1);
    }
  }

  /**
   * Get a random elements, removing it from the queue
   * @return A random element
   */
  public Item dequeue() {
    // Choose a random element, swap it to the back, remove it
    int nelem = elements.size();
    if (nelem == 1) return elements.remove(0);
    int idx = rand.nextInt(nelem);
    swapElements(idx, nelem-1);
    return elements.remove(nelem-1);
  }

  /**
   * Get a random element without removing it from the queue
   * @return The element
   */
  public Item sample() {
    int idx = rand.nextInt(elements.size());
    return elements.get(idx);
  }

  // This is a bit trickier -- we want to have multiple calls
  //  to iterator give a different ordering.  So construct an
  //  array of indices that we move through.  That is, this
  //  iterator takes linear extra space.
  private class RandomQueueIterator implements Iterator<Item> {
    int indices[];
    int curridx;

    public RandomQueueIterator(int n, Random rnd) {
      indices = new int[n];
      for (int i = 0; i < n; ++i) indices[i] = i;
      // Fisher-Yates shuffle
      for (int i = 0; i < n; ++i) {
        // Swap element i and a random element
        //  must be able to swap i with itself
        int j = rnd.nextInt(i + 1);  // [0, i]
        int t = indices[j];
        indices[j] = indices[i];
        indices[i] = t;
      }
      curridx = 0;
    }

    public boolean hasNext() { return curridx < indices.length; }
    public void remove() { throw new UnsupportedOperationException(); }
    public Item next() {
      if (curridx >= indices.length) throw new NoSuchElementException();
      Item item = elements.get(indices[curridx]);
      curridx += 1;
      return item;
    }
  }

  public Iterator<Item> iterator() {
    return new RandomQueueIterator(elements.size(), rand);
  }
}