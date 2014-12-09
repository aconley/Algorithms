package sedgewick.graphs

/**
 * Routines for creating a minimum spanning tree for undirected, weighted graphs
 */
object MST {
  import collection.mutable.{PriorityQueue => MPQueue}
  import collection.mutable.ListBuffer
  import scala.math.Ordering

  /** Construct a MST using a Lazy version of Prim's algorithm
    *
    * @param G A [[WeightedGraph]] that is assumed to be connected
    * @return A [[WeightedGraph]] giving the Minimum Spanning Tree
    *
    * If G is not connected, then the MST for the connected component
    * starting at vertex 0 will be returned
    */

  def LazyPrimMST(G: WeightedGraph): WeightedGraph = {
    require(G.V > 0, "G has no vertices")

    // Function for priority queue
    def minEdge(e: WeightedEdge) = e.weight

    def visit(G: WeightedGraph, v: Int, m: Array[Boolean],
              pq: MPQueue[WeightedEdge]): Unit = {
      m(v) = true
      for (e <- G.adj(v))
        if (!m(e.v)) pq += e
    }

    val edges = new MPQueue[WeightedEdge]()(Ordering.by(minEdge))
    val marked = Array.fill[Boolean](G.V)(false)
    val mst = new ListBuffer[(Int, Int, Double)]()

    visit(G, 0, marked, edges)
    while (!edges.isEmpty) {
      val e = edges.dequeue
      if (!(marked(e.u) && marked(e.v))) {
        mst += ((e.u, e.v, e.weight))
        if (!marked(e.u)) visit(G, e.u, marked, edges)
        if (!marked(e.v)) visit(G, e.v, marked, edges)
      }
    }

    // Turn this into a graph
    WeightedGraph(mst.toList)
  }
}