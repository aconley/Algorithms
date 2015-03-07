// Binary search tree
package sedgewick.search

class BST[K, V](implicit ord: Ordering[K]) extends SymbolTable[K, V] {
  // Internal representation
  private sealed trait BTree[K, V]
  private case object Tip extends BTree[K, V]
  private case class Branch[K, V](key: K, elem: V,
    left: BTree[K, V], right: BTree[K, V]) extends BTree[K, V]

  private var tree: BTree[K, V] = Tip
  override def isEmpty: Boolean = tree match {
    case Tip => true
    case _ => false
  }

  private def count(t: BTree[K, V]): Int = t match {
    case Tip => 0
    case Branch(_, _, l, r) => 1 + count(l) + count(r)
  }
  def size: Int = count(tree)

  @annotation.tailrec
  private def search(key: K, t: BTree[K, V]): Option[V] = t match {
    case Tip => None
    case Branch(k, v, _, _) if (ord.equiv(key, k)) => Some(v)
    case Branch(k, _, l, _) if (ord.lt(key, k)) => search(key, l)
    case Branch(k, _, _, r)  => search(key, r)
  }
  def apply(k: K): Option[V] = search(k, tree)

  @annotation.tailrec
  private def findMin(t: BTree[K, V]): Option[(K, V)] = t match {
    case Tip => None
    case Branch(k, v, Tip, _) => Some((k, v))
    case Branch(k, v, l, _) => findMin(l)
  }
  def findMin: Option[(K, V)] = findMin(tree)

  private def deleteMin(t: BTree[K, V]): BTree[K, V] = t match {
    case Tip => Tip
    case Branch(k, v, Tip, Tip) => Tip
    case Branch(k, v, Tip, r) => r
    case Branch(k, v, l, r) => Branch(k, v, deleteMin(l), r)
  }
  def deleteMin(): Unit = tree = deleteMin(tree)

  def clear(): Unit = tree = Tip

  private def put(key: K, value: V, t: BTree[K, V]): BTree[K, V] = t match {
    case Tip => Branch(key, value, Tip, Tip)
    case Branch(k, v, l, r) if (ord.equiv(k, key)) => Branch(k, value, l, r)
    case Branch(k, v, l, r) if (ord.lt(key, k)) =>
      Branch(k, v, put(key, value, l), r)
    case Branch(k, v, l, r) => Branch(k, v, l, put(key, value, r))
  }
  def put(key: K, value: V): Unit = put(key, value, tree)

  private def foreach(f: ((K, V)) => Unit, t: BTree[K, V]): Unit = t match {
    case Tip => ()
    case Branch(k, v, l, r) =>
      foreach(f, l)
      f((k, v))
      foreach(f, r)
  }
  def foreach(f: ((K, V)) => Unit): Unit = foreach(f, tree)

  def delete(k: K): Unit = sys.error("todo")
}