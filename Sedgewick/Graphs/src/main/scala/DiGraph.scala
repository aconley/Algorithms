package sedgewick.graphs

import collection.mutable.ListBuffer

/** Basic immutable directed graph class
  *
  * @constructor Create a new [[Digraph]]
  * @param V Number of vertices
  * @param E Number of edges
  * @param indeg Indegree of each edge
  * @param adj_list Edge adjacency lists
  */
class Digraph(val V: Int, val E: Int,
              private val indeg: IndexedSeq[Int],
              private val adj_list: IndexedSeq[List[DirectedEdge]])
  extends DirectedGraph[DirectedEdge] {

  def outdegree(v: Int): Int = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range [0, $V)")
    adj_list(v).length
  }

  def indegree(v: Int): Int = indeg(v)

  def adj(v: Int) = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range [0, $V)")
    adj_list(v)
  }

  override def reverse: Digraph = {
    val ideg = adj_list.map(_.length)
    val adj_init = Array.fill(V)(ListBuffer.empty[DirectedEdge])
    for (u <- 0 until V; e <- adj_list(u)) adj_init(e.v) += e.reverse
    new Digraph(V, E, ideg, adj_init.map(_.toList).toIndexedSeq)
  }

  override def toString: String = f"Directed graph with $V%d vertices and $E%d edges"
}

object Digraph {
  /** Create [[Digraph]] from list of edges
    *
    * @param edgeList List of edges where tuples (u, v) represents
    *                 an edge from u to v
    * @param allowDup Allow duplicate edges
    * @param allowSelf Allow self loops
    * @return A new [[Digraph]]
    */
  def apply(edgeList: List[(Int, Int)], allowDup: Boolean=false,
            allowSelf: Boolean=true): Digraph = {

    // Count number of vertices
    val V = edgeList.map(t => t._1 max t._2).max + 1

    // Build up adjacency list, removing duplicates
    //  and self loops if needed
    val adj_init = Array.fill(V)(ListBuffer.empty[DirectedEdge])
    val ideg = Array.fill(V)(0)
    var nedge = 0
    if (allowDup) {
      if (allowSelf) {
        // Simple case -- just insert
        edgeList foreach {
          t => {
            adj_init(t._1) += DirectedEdge(t._1, t._2)
            ideg(t._2) += 1
            nedge += 1
          }
        }
      } else {
        // Remove self edges
        edgeList foreach {
          t => if (t._1 != t._2) {
            adj_init(t._1) += DirectedEdge(t._1, t._2)
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
          adj_init(t._1) += DirectedEdge(t._1, t._2)
          ideg(t._2) += 1
        }
      }
      new Digraph(V, edgeSet.size, ideg, adj_init.map(_.toList).toIndexedSeq)
    }
  }
}