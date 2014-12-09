package sedgewick.graphs

/** Edge type */
trait EdgeLike {
  def u: Int
  def v: Int
  def isSelf: Boolean = u == v
  def reverse: EdgeLike
}

class UndirectedEdge(val u: Int, val v: Int) extends EdgeLike {

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

  def reverse: UndirectedEdge = new UndirectedEdge(v, u)
  override def toString = s"$u <-> $v"
}

object UndirectedEdge {
  def apply(e: (Int, Int)) = new UndirectedEdge(e._1, e._2)
  def apply(f: Int, t: Int) = new UndirectedEdge(f, t)
}

class DirectedEdge(val u: Int, val v: Int) extends EdgeLike {

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
  def weight: Double
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
// A double as the weight seems excessive, but declaring float
// literals takes work in scala, so it's easier to use Double
class WeightedEdge(val u: Int, val v: Int, val weight: Double)
  extends WeightedEdgeLike with EdgeLike {

  // Weight not used in equality
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

  def reverse: WeightedEdge = new WeightedEdge(v, u, weight)
  override def toString = s"$u <-> $v (wt: $weight)"
}

object WeightedEdge {
  def apply(e: (Int, Int, Double)) = new WeightedEdge(e._1, e._2, e._3)
  def apply(f: Int, t: Int, w: Double) = new WeightedEdge(f, t, w)
}

object EdgeImplicits {
  implicit def intTupleToUndirectedEdge(e: (Int, Int)) = UndirectedEdge(e)
  implicit def intTupleToDirectedEdge(e: (Int, Int)) = DirectedEdge(e)
  implicit def intwtTupleToWeightedEdge(e: (Int, Int, Double)) = WeightedEdge(e)
}