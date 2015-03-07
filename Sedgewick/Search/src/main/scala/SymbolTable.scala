package sedgewick.search

trait SymbolTable[K, V] {
  def put(k: K, v: V): Unit  // Add an entry
  def apply(k: K): Option[V] // Get an entry
  def clear(): Unit // Empty table
  def contains(k: K): Boolean = this(k) match {
      case Some(_) => true
      case None => false
    }// Is key present?
  def delete(k: K): Unit  // Delete a key
  def foreach(f: ((K, V)) => Unit): Unit  // Iterate over table
  def isEmpty: Boolean = this.size == 0
  def size: Int

}