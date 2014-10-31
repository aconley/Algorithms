package sedgewick.graphs

import collection.mutable.ListBuffer

// Basic immutable directed graph class
class Digraph(val V: Int, val E: Int,
              private val indeg: IndexedSeq[Int],
              private val adj_list: IndexedSeq[List[Int]])
  extends GraphLike with DirectedGraph {

  def outdegree(v: Int): Int = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range")
    adj_list(v).length
  }

  def indegree(v: Int): Int = indeg(v)

  def adj(v: Int) = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range")
    adj_list(v)
  }

  override def reverse: Digraph = {
    val ideg = adj_list.map(_.length)
    val adj_init = Array.fill(V)(ListBuffer.empty[Int])
    for (u <- 0 until V; v <- adj_list(u)) adj_init(v) += u
    new Digraph(V, E, ideg, adj_init.map(_.toList).toIndexedSeq)
  }

  override def toString: String = f"Directed graph with $V%d vertices"
}

object Digraph {
  // Build new immutable Graph from a list of edges,
  //  where the edges are specified as a list of tuple-s
  // Self loops are allowed by default, but not duplicate
  // edges
  def apply(edgeList: List[(Int, Int)], allowDup: Boolean=false,
            allowSelf: Boolean=true): Digraph = {

    // Count number of vertices
    val V = edgeList.map(t => t._1 max t._2).max + 1

    // Build up adjacency list, removing duplicates
    //  and self loops if needed
    val adj_init = Array.fill(V)(ListBuffer.empty[Int])
    val ideg = Array.fill(V)(0)
    var nedge = 0
    if (allowDup) {
      if (allowSelf) {
        // Simple case -- just insert
        edgeList foreach {
          t => {
            adj_init(t._1) += t._2
            ideg(t._2) += 1
            nedge += 1
          }
        }
      } else {
        // Remove self edges
        edgeList foreach {
          t => if (t._1 != t._2) {
            adj_init(t._1) += t._2
            ideg(t._2) += 1
            nedge += 1
          }
        }
      }
      new Digraph(V, nedge, ideg, adj_init.map(_.toList).toIndexedSeq)
    } else {
      // Remove duplicates
      val edgeSet =
        if (allowSelf)
          edgeList.toSet
        else
          edgeList.filter {
            t => t._1 != t._2
          }.toSet

      edgeSet foreach {
        t => {
          adj_init(t._1) += t._2
          ideg(t._2) += 1
        }
      }
      new Digraph(V, edgeSet.size, ideg, adj_init.map(_.toList).toIndexedSeq)
    }
  }
}