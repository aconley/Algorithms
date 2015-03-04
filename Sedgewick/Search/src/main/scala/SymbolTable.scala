package sedgewick.search

trait SymbolTable[K, V] {
  def put(k: K, v: V): Boolean  // Add an entry
  def apply(k: K): Option[V] // Get an entry
  def delete(k: K): Boolean  // Delete a key
  def contains(k: K): Boolean // Is key present?
  def isEmpty: Boolean
  def size: Int
  def foreach(f: (K, V) => Unit): Unit  // Iterate over table
}
