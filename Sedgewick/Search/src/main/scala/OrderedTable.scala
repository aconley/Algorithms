// Ordered List implementation of SymbolTable
package sedgewick.search

class OrderedTable[K, V](implicit ord: Ordering[K]) extends SymbolTable[K, V] {
  import collection.mutable.ArrayBuffer  // Mutable so we can replace elements

  private var elements: ArrayBuffer[(K, V)] = ArrayBuffer.empty // Contains actual table

  def size: Int = elements.size
  override def isEmpty: Boolean = elements.isEmpty

  // Binary search -- oddly enough, I can't find such a thing in the scala library
  //  Return either Right(index) or Left(index where k could be inserted)
  private def binsearch(k: K, l: Int, r: Int): Either[Int, Int] = {
    @annotation.tailrec
    def bsearch(l: Int, r: Int): Either[Int, Int] =
      if (l > r)
        Left(l) // Didn't find
      else {
        val m = (l + r) / 2
        val e = elements(m)._1
        if (ord.equiv(k, e))
          Right(m)
        else {
          if (ord.lt(e, k))
            bsearch(m + 1, r)
          else
            bsearch(l, m - 1)
        }
      }

    if (size == 0)
      Left(0)
    else
      bsearch(0, size-1)
  }

  /**
   * Get the element corresponding to the key
   * @param k Key to search on
   * @return Some(value) if found, None if not found
   */
  def apply(k: K): Option[V] = binsearch(k, 0, elements.size) match {
    case Left(_) => None
    case Right(i) => Some(elements(i)._2)
  }

  def clear(): Unit = elements.clear()
  
  /**
   * Add an element to the table; note that duplicates are not allowed
   * @param k Key to add
   * @param v Corresponding value
   * @return True if the element is new, false if replaced
   */
  def put(k: K, v: V): Boolean = {
    binsearch(k, 0, elements.size) match {
      case Left(i) =>
        elements.insert(i, (k, v))
        true
      case Right(i) =>
        elements(i) = (k, v)
        false
    }
  }

  /**
   * Remove the element matching the specified key
   * @param k The key
   * @return True if the element was found and removed, false if not found
   */
  def delete(k: K): Boolean = {
    binsearch(k, 0, elements.size) match {
      case Left(_) => false
      case Right(i) => elements.remove(i)
        true
    }
  }

  def foreach(f: ((K, V)) => Unit): Unit = elements.foreach(f(_))
}

object OrderedTable {
  def empty[K, V](implicit ord: Ordering[K]): OrderedTable[K, V] = new OrderedTable[K, V]
  def apply[K, V](args: (K, V)*)(implicit ord: Ordering[K]): OrderedTable[K, V] = {
    val e = new OrderedTable[K, V]
    for ((k, v) <- args)
      e.put(k, v)
    e
  }
}