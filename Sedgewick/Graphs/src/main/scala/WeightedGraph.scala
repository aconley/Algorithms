package sedgewick.graphs

import collection.mutable.ListBuffer

import EdgeImplicits.intwtTupleToWeightedEdge

/** Basic immutable undirected graph class with weights
  *
  * @constructor Create a new [[Graph]]
  * @param V Number of vertices
  * @param E Number of edges
  * @param adj_list Edge adjacency lists
  */
class WeightedGraph(val V: Int, val E: Int,
                    private val adj_list: IndexedSeq[List[WeightedEdge]])
  extends UndirectedGraph[WeightedEdge] {

  def degree(v: Int): Int = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range [0, $V)")
    adj_list(v).length
  }

  def adj(v: Int) = {
    require(v >= 0 & v < V, s"Specified vertex $v out of range [0, $V)")
    adj_list(v)
  }

  /** Basic string representation */
  override def toString: String =
    f"Undirected weighted graph with $V%d vertices and $E%d edges"
}

object WeightedGraph {
  /** Build new immutable Graph from a list of edges
    *
    * @param edgeList List of edges specified as tuples of ints and weights
    * @param allowDup Allow duplicate edges
    * @return A new [[Graph]]
    *
    * Self edges not allowed.
    */
  def apply(edgeList: List[(Int, Int, Double)],
            allowDup: Boolean = false): WeightedGraph = {

    // Count number of vertices
    val V = edgeList.map(t => t._1 max t._2).max + 1

    // Build up adjacency list, removing duplicates
    //  and self loops if needed
    val adj_init = Array.fill(V)(ListBuffer.empty[WeightedEdge])
    var nedge = edgeList.length
    if (allowDup) {
      // Remove self edges
      edgeList foreach {
        t =>
          if (t._1 != t._2) {
            adj_init(t._1) += WeightedEdge(t._1, t._2, t._3)
            adj_init(t._2) += WeightedEdge(t._2, t._1, t._3)
          } else nedge -= 1
      }
    } else {
      // Remove self edges and duplicate edges (ignoring weight in dup check)
      val edgeSet = scala.collection.mutable.Set[(Int, Int)]()
      for (edg <- edgeList) {
        if (edg._1 != edg._2) {
          val edgeTup = (edg._1 min edg._2, edg._1 max edg._2)
          if (!edgeSet.contains(edgeTup)) {
            edgeSet += edgeTup
            adj_init(edg._1) += WeightedEdge(edg._1, edg._2, edg._3)
            adj_init(edg._2) += WeightedEdge(edg._2, edg._1, edg._3)
          }
        }
      }
      nedge = edgeSet.size
    }
    new WeightedGraph(V, nedge, adj_init.map(_.toList).toIndexedSeq)
  }
}
