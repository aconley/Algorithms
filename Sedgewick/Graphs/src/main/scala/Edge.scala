package sedgewick.graphs

/** Edge type */
trait EdgeLike {
  def u: Int
  def v: Int
  def isSelf: Boolean = u == v
}

trait UndirectedEdgeLike extends EdgeLike

trait DirectedEdgeLike extends EdgeLike {
  def reverse: DirectedEdgeLike
}

class UndirectedEdge(val u: Int, val v: Int) extends UndirectedEdgeLike {

  // See chap 30 of programming in Scala
  // equality independent of order
  override def equals(other: Any) = other match {
    case that: UndirectedEdge => ((u == that.u) && (v == that.v)) ||
      ((u == that.v) && (v == that.u))
    case _ => false
  }
  // Must make this give the same for any objects that match via equals
  override def hashCode = {
    val minV = u min v
    val maxV = u max v
    41 * (minV + 41) + maxV
  }

  override def toString = s"$u <-> $v"
}

object UndirectedEdge {
  def apply(e: (Int, Int)) = new UndirectedEdge(e._1, e._2)
  def apply(f: Int, t: Int) = new UndirectedEdge(f, t)
}

class DirectedEdge(val u: Int, val v: Int) extends DirectedEdgeLike {

  // This time equality does depend on order
  override def equals(other: Any) = other match {
    case that: DirectedEdge => (u == that.u) && (v == that.v)
    case _ => false
  }
  override def hashCode = {
    41 * (u + 41) + v
  }

  def reverse: DirectedEdge = new DirectedEdge(v, u)

  override def toString = s"$u -> $v"
}

object DirectedEdge {
  def apply(e: (Int, Int)) = new DirectedEdge(e._1, e._2)
  def apply(f: Int, t: Int) = new DirectedEdge(f, t)
}

trait WeightedEdgeLike extends EdgeLike {
  def weight: Float
}

/** Edge with weights (undirected)
  *
  * @param u From vertex
  * @param v To vertex
  * @param weight Weight of vertex
  *
  * Note that weight is not considered in equality checks, so using
  * this class disallows self edges
  */
class WeightedEdge(val u: Int, val v: Int, val weight: Float)
  extends WeightedEdgeLike with UndirectedEdgeLike {

  // Weight not used in equ ality
  override def equals(other: Any) = other match {
    case that: WeightedEdge => ((u == that.u) && (v == that.v)) ||
      ((u == that.v) && (v == that.u))
    case _ => false
  }
  // Must make this give the same for any objects that match via equals
  override def hashCode = {
    val minV = u min v
    val maxV = u max v
    41 * (minV + 41) + maxV
  }

  override def toString = s"$u <-> $v (wt: $weight)"
}

object WeightedEdge {
  def apply(e: (Int, Int, Float)) = new WeightedEdge(e._1, e._2, e._3)
  def apply(f: Int, t: Int, w: Float) = new WeightedEdge(f, t, w)
}

object EdgeImplicits {
  implicit def intTupleToUndirectedEdge(e: (Int, Int)) = UndirectedEdge(e)
  implicit def intTupleToDirectedEdge(e: (Int, Int)) = DirectedEdge(e)
  implicit def intwtTupleToWeightedEdge(e: (Int, Int, Float)) = WeightedEdge(e)
}