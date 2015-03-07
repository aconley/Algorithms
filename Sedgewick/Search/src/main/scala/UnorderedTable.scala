// Unordered List implementation of SymbolTable
package sedgewick.search

class UnorderedTable[K, V](implicit eq: Equiv[K]) extends SymbolTable[K, V] {
  import collection.mutable.MutableList  // Mutable so we can replace elements

  private var N: Int = 0 // Number of elements
  private var elements: MutableList[(K, V)] = MutableList() // Contains actual table

  def size: Int = N
  override def isEmpty: Boolean = N == 0

  /**
   * Get the element corresponding to the key
   * @param k Key to search on
   * @return Some(value) if found, None if not found
   */
  def apply(k: K): Option[V] = elements.find(x => eq.equiv(x._1, k)) match {
    case None => None
    case Some((_, v)) => Some(v)
  }

  def clear(): Unit = {
    N = 0
    elements.clear()
  }

  /**
   * Add an element to the table; note that duplicates are not allowed
   * @param k Key to add
   * @param v Corresponding value
   */
  def put(k: K, v: V): Unit = {
    val idx = elements.indexWhere(x => eq.equiv(x._1, k))
    if (idx == -1) {
      // New element
      elements += ((k, v))
      N += 1
    } else {
      // Replace old element
      elements(idx) = (k, v)
    }
  }

  /**
   * Remove the element matching the specified key
   * @param k The key
   */
  def delete(k: K): Unit = {
    // This is a bit painful, since it sweeps the list twice.  That doesn't
    //  seem like it should be necessary.  But the DoubleLinkedList is deprecated
    //  in scala 2.11, and there doesn't seem to be a replacement that has fast
    //  removals
    if (contains(k)) {
      elements = elements.foldLeft(MutableList[(K, V)]())((b, a) => if (eq.equiv(a._1, k)) b else b += a)
      N -= 1
    }
  }

  def foreach(f: ((K, V)) => Unit): Unit = elements.foreach(f(_))
}

object UnorderedTable {
  def empty[K, V](): UnorderedTable[K, V] = new UnorderedTable[K, V]
  def apply[K, V](args: (K, V)*): UnorderedTable[K, V] = {
    val e = new UnorderedTable[K, V]
    for ((k, v) <- args)
      e.put(k, v)
    e
  }
}