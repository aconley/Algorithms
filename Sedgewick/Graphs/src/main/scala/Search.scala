package sedgewick.graphs

// This enumeration is used to keep track of what point
// we are in a graph search.
object VertexSearchStatus extends Enumeration {
  type VertexSearchStatus = Value
  val Undiscovered = Value // Vertex not discovered; initialize to this
  val Discovered = Value // Vertex discovered, not all neighbors done
  val Finished = Value // Vertex discovered, all neighbors searched
}

object Search {
  // Visit all vertices connected to v in the graph g
  //  using a depth first search calling the visitor
  // in pre-order

  import VertexSearchStatus._

  // dfs visitor function from a given vertex
  // Relies on caller to set up visitor on initial call
  private def dfsInner(g: GraphLike, w: Int, visitor: VertexVisitor,
    visited: Array[VertexSearchStatus]): Unit = {

    visited(w) = Discovered
    visitor.discoverVertex(w, g) // Pre-order
    for (e <- g.adj(w)) {
      if (visited(e) == Undiscovered) {
        visitor.treeEdge(w, g)
        dfsInner(g, e, visitor, visited)
      } else if (visited(e) == Discovered)
        visitor.backEdge(e, g)
      else
        visitor.crossEdge(e, g)
    }
    visited(w) = Finished
  }

  // Visit all vertices connected to a start vertex v using dfs
  def dfsVisitVertex(g: GraphLike, v: Int, visitor: VertexVisitor): Unit = {
    require(g.V > 0, "Empty graph")
    require(v < g.V & v >= 0, s"Invalid start vertex $v")

    val visited = Array.fill(g.V)(Undiscovered)
    dfsInner(g, v, visitor, visited)
  }

  // Visit all vertices in a graph using dfs
  def dfsVisitAll(g: GraphLike, visitor: VertexVisitor): Unit = {
    require(g.V > 0, "Empty graph")
    val visited = Array.fill(g.V)(Undiscovered)
    for (v <- 0 to g.V)
      if (visited(v) == Undiscovered)
        dfsInner(g, v, visitor, visited)
  }

  // Get list of vertices connected to specified one
  def connectedToVertex(v: Int, g: GraphLike): List[Int] = {
    val vdet = new VertexVisited(g)

    dfsVisitVertex(g, v, vdet)
    vdet.visitList
  }

  // Cycle visitor
  class CycleDetector extends VertexVisitor {
    private[this] var cycle = false;

    override def backEdge(v: Int, g: GraphLike) = cycle = true
    def hasCycle: Boolean = cycle
  }

  // Detects the presence of a cycle using a dfs
  def detectCycle(g: GraphLike): Boolean = {
    val cdet = new CycleDetector
    dfsVisitAll(g, cdet)
    cdet.hasCycle
  }
}