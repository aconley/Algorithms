import java.util.NoSuchElementException;
import java.util.Iterator;

/**
 * RandomizedQueue implementation
 *
 * The implementation is to use an array and repack
 * as needed.  We always insert at the end, but
 * remove randomly.  Iterators keep an index array
 * into the original array for returning elements.
 */
public class RandomizedQueue<Item> implements Iterable<Item> {
  private static final int MINCAP = 5; // Minimum capacity
  private int size; // Number of elements actually present
  private Item[] elements;  // Can't use built ins

  /**
   * Construct empty queue
   */
  public RandomizedQueue() {
    size = 0;
    elements = (Item[]) new Object[MINCAP];
  }

  /**
   * Is the queue empty?
   * @return Returns true if the queue is empty, false otherwise
   */
  public boolean isEmpty() { return size == 0; }

  /**
   * Get the size of the queue
   * @return the number of elements in the queue
   */
  public int size() { return size; }

  /**
   * Resize internal storage
   */
  private void resize(int newCapacity) {
    Item[] newElements = (Item[]) new Object[newCapacity];
    for (int i = 0; i < size; ++i)
      newElements[i] = elements[i];
    elements = newElements;
  }

  /**
   * Add an item to the queue
   * @param item Item to add
   */
  public void enqueue(Item item) {
    if (item == null)
      throw new NullPointerException("Tried to add null elements");

    if (size == elements.length)
      resize(2 * size);

    elements[size++] = item;
  }

  /**
   * Get a random elements, removing it from the queue
   * @return A random element
   */
  public Item dequeue() {
    if (size == 0)
      throw new NoSuchElementException("RandomizedQueue is empty");
    // Chose a random element, save it, swap the element
    // at the end in, and null the reference in elements.
    int idx = StdRandom.uniform(size);
    Item retval = elements[idx];
    elements[idx] = elements[--size];
    elements[size] = null;

    // Resize if needed
    if (elements.length > MINCAP && size < elements.length / 4)
      resize(elements.length / 2);

    return retval;
  }

  /**
   * Get a random element without removing it from the queue
   * @return The element
   */
  public Item sample() {
    if (size == 0)
      throw new NoSuchElementException("RandomizedQueue is empty");
    return elements[StdRandom.uniform(size)];
  }

  // This is a bit trickier -- we want to have multiple calls
  //  to iterator give a different ordering.  So construct an
  //  array of indices that we move through.  That is, this
  //  iterator takes linear extra space.  But we avoid
  //  storing a full copy of each Item, since it could be large.
  private class RandomQueueIterator implements Iterator<Item> {
    private int[] indices;
    private int curridx;

    public RandomQueueIterator() {
      indices = new int[size];
      for (int i = 0; i < size; ++i)
        indices[i] = i;
      // Fisher-Yates shuffle
      for (int i = 1; i < size; ++i) {
        // Swap element i and a random element
        //  must be able to swap i with itself
        int j = StdRandom.uniform(i + 1);  // [0, i]
        int t = indices[i];
        indices[i] = indices[j];
        indices[j] = t;
      }
      curridx = 0;
    }

    public boolean hasNext() { return curridx < indices.length; }
    public Item next() {
      if (curridx >= indices.length) throw new NoSuchElementException();
      Item item = elements[indices[curridx++]];
      return item;
    }
    public void remove() {
      throw new UnsupportedOperationException("Removal not supported");
    }
  }

  /**
   * Get a randomized iterator.  Two calls to this will produce
   * iterators that give back the elements in the RandomizedQueue
   * in an independent random order.
   */
  public Iterator<Item> iterator() {
    return new RandomQueueIterator();
  }
}
