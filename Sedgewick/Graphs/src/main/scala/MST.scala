package sedgewick.graphs

/**
 * Routines for creating a minimum spanning tree for undirected, weighted graphs
 */
object MST {
  import collection.mutable.{PriorityQueue => MPQueue}
  import collection.mutable.ListBuffer
  import scala.math.Ordering

  // Ordering for minimum weight edge
  val ord = new Ordering[WeightedEdge] {
    def compare(x: WeightedEdge, y: WeightedEdge): Int = y.weight compare x.weight
  }

  /** Construct a MST using a Lazy version of Prim's algorithm
    *
    * @param G A [[WeightedGraph]] that is assumed to be connected
    * @return A [[WeightedGraph]] giving the Minimum Spanning Tree
    *
    * If G is not connected, then the MST for the connected component
    * starting at vertex 0 will be returned
    */
  def LazyPrimMST(G: WeightedGraph): (Double, WeightedGraph) = {
    require(G.V > 0, "G has no vertices")

    def visit(G: WeightedGraph, u: Int, m: Array[Boolean],
              pq: MPQueue[WeightedEdge]): Unit = {
      m(u) = true
      for (e <- G.adj(u))
        if (!m(e.v)) pq += e
    }

    val edges = new MPQueue[WeightedEdge]()(ord)
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

    // Build return values
    val totwt = mst.foldLeft(0.0)(_ + _._3)
    (totwt, WeightedGraph(mst.toList))
  }

  /** Construct a MST using a Lazy version of Prim's algorithm
    *
    * @param G A [[WeightedGraph]] that is assumed to be connected
    * @return A [[WeightedGraph]] giving the Minimum Spanning Tree
    *
    * If G is not connected, then the MST for the connected component
    * starting at vertex 0 will be returned
    */
  def KruskalMST(G: WeightedGraph): (Double, WeightedGraph) = {
    require(G.V > 0, "G has no vertices")

    val edges = new MPQueue[WeightedEdge]()(ord)
    val mst = new ListBuffer[(Int, Int, Double)]()
    val uf = new UnionFind(G.V, true)
    var nadded = 0

    // Insert all edges
    edges ++= G.edges

    // Empty the queue, adding lowest weight edges that don't
    // form a cycle
    while (!edges.isEmpty && (nadded < G.V - 1)) {
      val e = edges.dequeue
      if (!uf.connected(e.u, e.v)) {
        uf.addEdge(e.u, e.v)
        mst += ((e.u, e.v, e.weight))
        nadded += 1
      }
    }

    // Build return values
    val totwt = mst.foldLeft(0.0)(_ + _._3)
    (totwt, WeightedGraph(mst.toList))
  }
}