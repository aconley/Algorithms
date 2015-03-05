// Unordered List implementation of SymbolTable
package sedgewick.search

class UnorderedTable[K, V] extends SymbolTable[K, V] {
  private var N: Int = 0 // Number of elements
  private var elements: List[(K, V)] = Nil // Contains actual table

  def size: Int = N
  override def isEmpty: Boolean = N == 0

  /**
   * Get the element corresponding to the key
   * @param k Key to search on
   * @return Some(value) if found, None if not found
   */
  def apply(k: K): Option[V] = {
    @annotation.tailrec
    def srch(key: K, l: List[(K, V)]): Option[V] = l match {
      case Nil => None
      case x :: xs => if (key == x._1) Some(x._2) else srch(key, l.tail)
    }
    srch(k, elements)
  }


  /**
   * Add an element to the table; note that duplicates are not allowed
   * @param k Key to add
   * @param v Corresponding value
   * @return True if the element was added, false if it was already present
   */
  // TODO: Change this to replace the value if already present!
  def put(k: K, v: V): Boolean = {
    if (!this.contains(k)) {
      elements = (k, v) :: elements
      N += 1
      true
    } else false
  }

  /**
   * Remove the element matching the specified key
   * @param k The key
   * @return True if the element was found and removed, false if not found
   */
  def delete(k: K): Boolean = {
    def del(key: K, l: List[(K, V)]): (Boolean, List[(K, V)]) = l match {
      case Nil => (false, Nil)
      case x :: xs => if (key == x._1) (true, xs) else {
        val tl = del(k, l.tail)
        (tl._1, x :: tl._2)
      }
    }

    val delRes = del(k, elements)
    if (delRes._1) {
      elements = delRes._2
      N -= 1
    }
    delRes._1
  }

  def foreach(f: (K, V) => Unit): Unit = elements foreach (e => f(e._1, e._2))
}