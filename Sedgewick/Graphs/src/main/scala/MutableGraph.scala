package sedgewick.graphs

import collection.mutable.ListBuffer

/** Basic mutable undirected graph class
  *
  * @constructor Create a new [[Graph]]
  * @param V Number of vertices
  * @param E Number of edges
  * @param adj_list Edge adjacency lists
  *
  * Repeated edges are not allowed
  */
class MutableGraph(val V: Int, private var _E: Int,
                   private val adj_list: IndexedSeq[ListBuffer[UndirectedEdge]])
  extends UndirectedGraph[UndirectedEdge] with GraphMutable[UndirectedEdge] {

  // Don't allow setting of E directly, only through add/remove
  def E: Int = _E
  def E_=(v: Int) = {}

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
    f"Undirected mutable graph with $V%d vertices and $E%d edges"

  def addEdge(e: UndirectedEdge):Unit = {
    // We don't allow repeated edges, so...
    if (adj_list(e.u) contains e) return
    adj_list(e.u) += e
    if (!e.isSelf) adj_list(e.v) += UndirectedEdge(e.v, e.u)
    _E += 1
  }

  def removeEdge(e: UndirectedEdge): Unit = {
    if (adj_list(e.u) contains e) {
      adj_list(e.u) -= e
      if (!e.isSelf) adj_list(e.v) -= e
      _E -= 1
    }
  }

  override def clone: MutableGraph =
    new MutableGraph(V, _E, adj_list map (_.clone))
}

object MutableGraph {
  /** Build new mutable Graph from a list of edges
    *
    * @param edgeList List of edges specified as tuples of ints
    * @return A new [[MutableGraph]]
    *
    * Duplicate edges not allowed, self edges are
    */
  def apply(edgeList: List[(Int, Int)],
            allowSelf: Boolean=false): MutableGraph = {

    // Count number of vertices
    val V = edgeList.map(t => t._1 max t._2).max + 1

    // Build up adjacency list, removing duplicates
    //  and self loops if needed
    val adj_init = Array.fill(V)(ListBuffer.empty[UndirectedEdge])

    // Remove duplicates; sort edges so that 0,1 and 1,0 count as a dup
    val edgeSet = edgeList.map(t => (t._1 min t._2, t._1 max t._2)).toSet

    // Add to adjacency list
    edgeSet foreach {
      t => {
        adj_init(t._1) += UndirectedEdge(t._1, t._2)
        if (t._1 != t._2) adj_init(t._2) += UndirectedEdge(t._2, t._1)
      }
    }
    new MutableGraph(V, edgeSet.size, adj_init.toIndexedSeq)
  }
}