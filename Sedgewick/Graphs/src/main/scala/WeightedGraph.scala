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
    * @return A new [[Graph]]
    *
    * Duplicates and self edges not allowed.  Note that duplicates are
    * judged ignoring the weight
    */
  def apply(edgeList: List[(Int, Int, Double)]): WeightedGraph = {

    // Count number of vertices
    val V = edgeList.map(t => t._1 max t._2).max + 1

    // Build up adjacency list, removing duplicates
    //  and self loops if needed
    val adj_init = Array.fill(V)(ListBuffer.empty[WeightedEdge])
      // Remove duplicates; sort edges so that 0,1 and 1,0 count as a dup
      val edgeSet =
          edgeList.filter {
            t => t._1 != t._2
          }.map {
            t => (t._1 min t._2, t._1 max t._2, t._3)
          }.toSet

      edgeSet foreach {
        t => {
          adj_init(t._1) += WeightedEdge(t._1, t._2, t._3)
          adj_init(t._2) += WeightedEdge(t._2, t._1, t._3)
        }
      }
      new WeightedGraph(V, edgeSet.size, adj_init.map(_.toList).toIndexedSeq)
    }
  }
