package sedgewick.assignments;

import java.util.Iterator;

public interface Dequeue<Item> extends Iterable {
    public boolean isEmpty();          // is the deque empty?
    public int size();                 // return the number of items on the deque
    public void addFirst(Item item);   // insert the item at the front of the queue
    public void addLast(Item item);    // insert the item at the end of the queue
    public Item removeFirst();         // delete and return the first item in queue
    public Item removeLast();          // delete and return the last item in queue
    public Iterator<Item> iterator();  // return an iterator that examines the
                                       // items in order from front to back
}
