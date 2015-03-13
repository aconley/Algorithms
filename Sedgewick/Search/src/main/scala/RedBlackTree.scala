// Red Black Tree.  Based on the Okasaki functional implementation.
package sedgewick.search

class RedBlackTree[K, V](implicit ord: Ordering[K]) extends SymbolTable[K, V] {
  // Internal representation
  private sealed trait Color
  private case object Red extends Color
  private case object Black extends Color

  private sealed trait RBT[K, V]
  private case object Tip extends RBT[K, V]  // Black by convention
  private case class Branch[K, V](color: Color, left: RBT[K, V],
                                  key: K, value: V, right: RBT[K,V])
    extends RBT[K,V]

  private var tree: RBT[K, V] = Tip

  override def isEmpty: Boolean = tree match {
    case Tip => true
    case _ => false
  }

  private def count(t: RBT[K, V]): Int = t match {
    case Tip => 0
    case Branch(_, l, _, _, r) => 1 + count(l) + count(r)
  }
  def size: Int = count(tree)

  @annotation.tailrec
  private def search(key: K, t: RBT[K, V]): Option[V] = t match {
    case Tip => None
    case Branch(_, _, k, v, _) if (ord.equiv(key, k)) => Some(v)
    case Branch(_, l, k, _, _) if (ord.lt(key, k)) => search(key, l)
    case Branch(_, _, _, _, r)  => search(key, r)
  }
  def apply(k: K): Option[V] = search(k, tree)

  @annotation.tailrec
  private def findMin(t: RBT[K, V]): Option[(K, V)] = t match {
    case Tip => None
    case Branch(_, Tip, k, v, _) => Some((k, v))
    case Branch(_, l, _, _, _) => findMin(l)
  }
  def findMin: Option[(K, V)] = findMin(tree)

  def clear(): Unit = tree = Tip

  // This is the hard part
  private def balance(color: Color, left: RBT[K, V], key: K, value: V, right: RBT[K, V]): RBT[K, V] =
    if (color == Red)
      Branch(Red, left, key, value ,right)
    else {
      (left, key, value, right) match {
        case (Branch(Red, Branch(Red, a, xk, xv, b), yk, yv, c), zk, zv, d) =>
          Branch(Red, Branch(Black, a, xk, xv, b), yk, yv, Branch(Black, c, zk, zv, d))
        case (Branch(Red, a, xk, xv, Branch(Red, b, yk, yv, c)), zk, zv, d) =>
          Branch(Red, Branch(Black, a, xk, xv, b), yk, yv, Branch(Black, c, zk, zv, d))
        case (a, xk, xv, Branch(Red, Branch(Red, b, yk, yv, c), zk, zv, d)) =>
          Branch(Red, Branch(Black, a, xk, xv, b), yk, yv, Branch(Black, c, zk, zv, d))
        case (a, xk, xv, Branch(Red, b, yk, yv, Branch(Red, c, zk, zv, d))) =>
          Branch(Red, Branch(Black, a, xk, xv, b), yk, yv, Branch(Black, c, zk, zv, d))
        case _ => Branch(color, left, key ,value, right)
      }
    }

  private def ins(key: K, value: V, t: RBT[K, V]): RBT[K, V] = t match {
    case Tip => Branch(Red, Tip, key, value, Tip)
    case Branch(c, l, k, v, r) if (ord.lt(key, k)) => balance(c, ins(key, value, l), k, v, r)
    case Branch(c, l, k, v, r) if (ord.gt(key, k)) => balance(c, l, k, v, ins(key, value, r))
    case Branch(c, l, k, v, r) => Branch(c, l, k, value, r) // Replace!
  }

  def put(key: K, value: V): Unit = {
    tree = ins(key, value, tree) match {
      case Branch(_, l, k, v, r) => Branch(Black, l, k, v, r)
      case Tip => sys.error("Shouldn't happen")
    }
  }

  // Join to RedBlack Trees -- tricky!
  // This is from Stephan Kars
  private def append(tl: RBT[K, V], tr: RBT[K, V]): RBT[K, V] = (tl, tr) match {
    case (Tip, t) => t
    case (t, Tip) => t
    case (Branch(Red, a, xk, xv, b), Branch(Red, c, yk, yv, d)) =>
      append(b, c) match {
        case Branch(Red, bb, zk, zv, cc) => Branch(Red, Branch(Red, a, xk, xv, bb), zk, zv, Branch(Red, cc, yk, yv, d))
        case bc => Branch(Red, a, xk, xv, Branch(Red, bc, yk, yv, d))
      }
    case (Branch(Black, a, xk, xv, b), Branch(Black, c, yk, yv, d)) =>
      append(b, c) match {
        case Branch(Red, bb, zk, zv, cc) =>
          Branch(Red, Branch(Black, a, xk, xv, bb), zk, zv, Branch(Black, cc, yk, yv, d))
        case bc => balance(Red, a, xk, xv, Branch(Black, bc, yk, yv, d))
      }
    case (a, Branch(Red, b, xk, xv, c)) => Branch(Red, append(a, b), xk, xv, c)
    case (Branch(Red, a, xk, xv, b), c) => Branch(Red, a, xk, xv, append(b, c))
  }

  // Rather tricky!
  private def del(key: K, t: RBT[K, V]): RBT[K, V] = t match {
    case Tip => t
    case Branch(c, l, k, v, r) if (ord.lt(key, k)) => balance(c, del(key, l), k, v, r)
    case Branch(c, l, k, v, r) if (ord.gt(key, k)) => balance(c, l, k, v, del(key, r))
    case Branch(c, l, k, v, r) => append(l, r)
  }

  def delete(k: K): Unit = tree = del(k, tree)

  private def foreach(f: ((K, V)) => Unit, t: RBT[K, V]): Unit = t match {
    case Tip => ()
    case Branch(_, l, k, v, r) =>
      foreach(f, l)
      f((k, v))
      foreach(f, r)
  }
  def foreach(f: ((K, V)) => Unit): Unit = foreach(f, tree)

  private def stringify(t: RBT[K, V]): String = t match {
    case Tip => ""
    case Branch(_, Tip, k, v, Tip) => s"$k -> $v"
    case Branch(_, Tip, k, v, r) => s"$k -> $v, " + stringify(r)
    case Branch(_, l, k, v, Tip) => stringify(l) + s", $k -> $v"
    case Branch(_, l, k, v, r) => stringify(l) + s", $k -> $v, " + stringify(r)
  }
  override def toString: String = "BST(" + stringify(tree) + ")"
}

object RedBlackTree {
  def empty[K, V](implicit ord: Ordering[K]): RedBlackTree[K, V] = new RedBlackTree[K, V]
  def apply[K, V](args: (K, V)*)(implicit ord: Ordering[K]): RedBlackTree[K, V] = {
    val e = new RedBlackTree[K, V]
    for ((k, v) <- args)
      e.put(k, v)
    e
  }
}