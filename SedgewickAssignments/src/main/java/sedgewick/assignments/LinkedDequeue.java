package sedgewick.assignments;

import java.util.Iterator;
import java.util.NoSuchElementException;

// Dequeue implementation using Doubly Linked List
// You can't efficiently remove items at the tail with
//  a singly linked list
public class LinkedDequeue<Item> implements Dequeue<Item> {
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
    public LinkedDequeue() {
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
        Node newnode = new Node();
        newnode.item = item;
        newnode.prev = null;
        newnode.next = head;
        if (head != null) head.prev = newnode;
        head = newnode;
        if (tail == null)
            tail = newnode;
        ++nelements;
    }

    /**
     * Add an item to the end of the list
     * @param item Item to add
     */
    public void addLast(Item item) {
        Node newnode = new Node();
        newnode.item = item;
        newnode.prev = tail;
        newnode.next = null;
        if (tail != null) tail.next = newnode;
        if (head == null) head = newnode;
        tail = newnode;
        ++nelements;
    }

    /**
     * Remove and return the first item on the dequeue
     * @return The first item
     * @throws NoSuchElementException if empty.
     */
    public Item removeFirst() {
        if (head == null) throw new NoSuchElementException("Empty dequeue");
        Item ret = head.item;
        head = head.next; // Will set to null if 1 element dequeue
        if (head != null) head.prev = null;
        else tail = null;
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
        tail = tail.prev; // Null if now empty
        if (tail != null) tail.next = null;
        else head = null;
        --nelements;
        return ret;
    }

    private class LinkedDequeueIterator implements Iterator<Item> {
        private Node current = head;
        public boolean hasNext() { return current != null; }
        public void remove() { throw new UnsupportedOperationException(); }
        public Item next() {
            if (current.next == null) throw new NoSuchElementException();
            Item item = current.item;
            current = current.next;
            return item;
        }
    }

    public Iterator<Item> iterator() {
        return new LinkedDequeueIterator();
    }
}
