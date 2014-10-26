package sedgewick.graphs

// This enumeration is used to keep track of what point
// we are in a graph search.
object VertexSearchStatus extends Enumeration {
  type VertexSearchStatus = Value
  val Undiscovered = Value // Vertex not discovered; initialize to this
  val Discovered = Value // Vertex discovered, not all neighbors done
  val Finished = Value // Vertex discovered, all neighbors searched
}

object GraphSearch {
  // Visit all vertices connected to v in the graph g
  //  using a depth first search calling the visitor
  // in pre-order

  import VertexSearchStatus._

  // dfs visitor function from a given vertex
  // Relies on caller to set up visitor on initial call
  private def dfsInner(g: GraphLike, v: Int, visitor: VertexVisitor,
    visited: Array[VertexSearchStatus]): Unit = {

    visited(v) = Discovered
    visitor.discoverVertex(v, g) // Pre-order
    for (u <- g.adj(v)) {
      if (visited(u) == Undiscovered) {
        visitor.treeEdge(v, u, g)
        dfsInner(g, u, visitor, visited)
      } else if (visited(u) == Discovered)
        visitor.backEdge(v, u, g)
      else
        visitor.crossEdge(v, u, g)
    }
    visited(v) = Finished
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
  private class CycleDetector extends VertexVisitor {
    private[this] var cycle = false;

    override def backEdge(v: Int, u: Int, g: GraphLike) = cycle = true
    def hasCycle: Boolean = cycle
  }

  // Detects the presence of a cycle using a dfs
  def detectCycle(g: GraphLike): Boolean = {
    val cdet = new CycleDetector
    dfsVisitAll(g, cdet)
    cdet.hasCycle
  }

  // Visitor for finding paths from initial node
  //  to all nodes reachable from that vertex
  private class DFSPath(g: GraphLike, initVertex: Int) extends VertexVisitor {
    val V = g.V
    val startVertex = initVertex
    val edgeTo = Array.fill[Int](V)(V)
    edgeTo(initVertex) = initVertex

    override def treeEdge(v: Int, u: Int, g: GraphLike) = edgeTo(u) = v
    def hasPathTo(u: Int): Boolean = edgeTo(u) < V
    def pathTo(u: Int): List[Int] = {
      if (u == startVertex) {
        List(u)
      } else if (!hasPathTo(u)) {
        List()
      } else {
        // Use stack to back it out
        val s = new collection.mutable.Stack[Int]
        var currVertex = u
        while (currVertex != startVertex) {
          s.push(currVertex)
          currVertex = edgeTo(currVertex)
        }
        s.push(startVertex)  // Path ends on start vertex
        s.toList
      }
    }
  }

  // Find DFS path between v and u, returning an empty
  //  list if there is none
  def findDFSPathBetween(v: Int, u: Int, g: GraphLike): List[Int] = {
    val vis = new DFSPath(g, v)
    dfsVisitVertex(g, v, vis)
    vis.pathTo(u)
  }

  // Find DFS path between v and all reachable vertices as
  // a map
  def findDFSPathsFrom(v: Int, g: GraphLike): Map[Int, List[Int]] = {
    val vis = new DFSPath(g, v)
    dfsVisitVertex(g, v, vis)

    val ret = collection.mutable.Map.empty[Int, List[Int]]
    for (u <- 0 to g.V filter (_ != v))
      if (vis.hasPathTo(u)) ret += (u -> vis.pathTo(u))
    ret.toMap
  }
}