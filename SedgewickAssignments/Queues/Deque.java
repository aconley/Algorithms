import java.util.Iterator;
import java.util.NoSuchElementException;

/**
* Dequeue: queue supporting insertion and removal at both
* ends.
*
* The implementation is based on a doubly linked list;
* the circular array method doesn't use size proportional
* to the number of current elements.
*/
public class Deque<Item> implements Iterable<Item> {
    private int nelements; // Number of elements
    private Node head; // Front of dequeue
    private Node tail; // End of dequeue

    private class Node {
        Item item;
        Node prev;
        Node next;
    }

    /**
     * Construct an empty dequeue
     */
    public Deque() {
        nelements = 0;
        head = null;
        tail = null;
    }

    /**
     * Is the dequeue empty?
     * @return True if the dequeue is empty, false otherwise
     */
    public boolean isEmpty() {
        return nelements == 0;
    }

    /**
     * Get current size of dequeue
     * @return Size of dequeue
     */
    public int size() {
        return nelements;
    }

    /**
     * Add an item to the front of the list
     * @param item Item to add
     */
    public void addFirst(Item item) {
        if (item == null)
            throw new  NullPointerException("Tried to add null element");
        Node newnode = new Node();
        newnode.item = item;
        newnode.prev = null;
        newnode.next = null;
        if (isEmpty()) {
            head = newnode;
            tail = head;
        } else {
            newnode.next = head;
            head.prev = newnode;
            head = newnode;
        }
        ++nelements;
    }

    /**
     * Add an item to the end of the list
     * @param item Item to add
     */
    public void addLast(Item item) {
        if (item == null)
            throw new NullPointerException("Tried to add null element");
        Node newnode = new Node();
        newnode.item = item;
        newnode.prev = null;
        newnode.next = null;
        if (isEmpty()) {
            head = newnode;
            tail = head;
        } else {
            tail.next = newnode;
            newnode.prev = tail;
            tail = newnode;
        }
        ++nelements;
    }

    /**
     * Remove and return the first item on the dequeue
     * @return The first item
     * @throws NoSuchElementException if empty.
     */
    public Item removeFirst() {
        if (head == null)
            throw new NoSuchElementException("Empty dequeue");
        Item ret = head.item;
        head = head.next; // Will set to null if 1 element dequeue
        if (head != null) {
            head.prev.next = null; // Prevent dangling refs
            head.prev = null;
        } else {
            tail = null;
        }
        --nelements;
        return ret;
    }

    /**
     * Remove and return the last item on the dequeue
     * @return The last item
     * @throws NoSuchElementException if empty
     */
    public Item removeLast() {
        if (tail == null) throw new NoSuchElementException("Empty dequeue");
        Item ret = tail.item;
        if (nelements == 1) {
            head = null;
            tail = null;
        } else {
            tail = tail.prev;
            tail.next.prev = null;
            tail.next = null;
        }
        --nelements;
        return ret;
    }

    private class LinkedDequeueIterator implements Iterator<Item> {
        private Node current = head;
        public boolean hasNext() { return current != null; }
        public Item next() {
            if (current == null)
                throw new NoSuchElementException("No more elements");
            Item item = current.item;
            current = current.next;
            return item;
        }
        public void remove() {
            throw new UnsupportedOperationException("Remove not supported");
        }
    }

    public Iterator<Item> iterator() {
        return new LinkedDequeueIterator();
    }
}
