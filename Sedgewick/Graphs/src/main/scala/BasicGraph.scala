package sedgewick.graphs

import collection.mutable.ListBuffer

// Very basic immutable graph class
// using a List for the adjacency list type
// Self loops and duplicates handled by apply in object,
//  but potentially supported
class BasicGraph(val V: Int, val E: Int,
                 private val adj_list: IndexedSeq[List[Int]])
  extends GraphLike {

  def degree(v: Int): Int = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range")
    adj_list(v).length
  }

  def adj(v: Int) = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range")
    adj_list(v)
  }

  override def toString: String = f"graph with $V%d vertices"
}

object BasicGraph {
  // Build new immutable Graph from a list of edges,
  //  where the edges are specified as a list of tuple-s
  def apply(edgeList: List[(Int, Int)], allowDup: Boolean=false,
            allowSelf: Boolean=false): BasicGraph = {

    // Count number of vertices
	  val V = edgeList.map(t => t._1 max t._2).max + 1

    // Build up adjacency list, removing duplicates
    //  and self loops if needed
    val adj_init = Array.fill(V)(ListBuffer.empty[Int])
    if (allowDup) {
      if (allowSelf) {
        // Simple case -- just insert
        edgeList foreach {
          t => {
            adj_init(t._1) += t._2
            if (t._1 != t._2) adj_init(t._2) += t._1
          }
        }
      } else {
        // Remove self edges
        edgeList foreach {
          t =>
            if (t._1 != t._2) {
              adj_init(t._1) += t._2
              adj_init(t._2) += t._1
            }
        }
      }
      new BasicGraph(V, edgeList.length, adj_init.map(_.toList).toIndexedSeq)
    } else {
      // Remove duplicates; sort edges so that 0,1 and 1,0 count as a dup
      val edgeSet =
        if (allowSelf)
          edgeList.map(t => (t._1 min t._2, t._1 max t._2)).toSet
        else
          edgeList.filter {
            t => t._1 != t._2
          }.map {
            t => (t._1 min t._2, t._1 max t._2)
          }.toSet

      edgeSet foreach {
        t => {
          adj_init(t._2) += t._1
          if (t._1 != t._2) adj_init(t._1) += t._2
        }
      }
      new BasicGraph(V, edgeSet.size, adj_init.map(_.toList).toIndexedSeq)
    }
  }
}