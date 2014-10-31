package sedgewick.graphs

// This enumeration is used to keep track of what point
// we are in a graph search.
object VertexSearchStatus extends Enumeration {
  type VertexSearchStatus = Value
  val Undiscovered = Value // Not discovered; initialize to this
  val Discovered = Value // Discovered, not all neighbors done
  val Finished = Value // Discovered, all neighbors searched
}

object GraphSearch {
  // Visit all vertices connected to v in the graph g
  //  using a depth first search calling the visitor
  // in pre-order

  import VertexSearchStatus._
  import collection.mutable.{Stack => MStack, Queue => MQueue}

  // dfs visitor function from a given vertex for digraphs
  // Relies on caller to set up visitor on initial call
  private def dfsInnerDi(g: DirectedGraph, u: Int, visitor: dfsVisitor,
                         visited: Array[VertexSearchStatus]): Unit = {

    visited(u) = Discovered
    visitor.discoverVertex(u, g) // Pre-order
    for (v <- g.adj(u)) {
      if (visited(v) == Undiscovered) {
        visitor.treeEdge(u, v, g)
        dfsInnerDi(g, v, visitor, visited)
      } else if (visited(v) == Discovered)
        visitor.backEdge(u, v, g)
      else
        visitor.crossEdge(u, v, g)
    }
    visitor.finalizeVertex(u, g)
    visited(u) = Finished
  }

  // dfs visitor function from a given vertex for undirected graphs
  // Relies on caller to set up visitor on initial call
  private def dfsInnerUn(g: UndirectedGraph, u: Int, w: Int,
                         visitor: dfsVisitor,
                         visited: Array[VertexSearchStatus]): Unit = {

    visited(u) = Discovered
    visitor.discoverVertex(u, g) // Pre-order
    for (v <- g.adj(u)) {
      if (visited(v) == Undiscovered) {
        visitor.treeEdge(u, v, g)
        dfsInnerUn(g, v, u, visitor, visited)
      } else if (v != w & visited(v) == Discovered)
        visitor.backEdge(u, v, g)
    }
    visitor.finalizeVertex(u, g)
    visited(u) = Finished
  }

  // Visit all vertices connected to a start vertex u using dfs
  def dfsVisitVertex(g: GraphLike, u: Int, visitor: dfsVisitor): Unit = {
    require(g.V > 0, "Empty graph")
    require(u < g.V & u >= 0, s"Invalid start vertex $u")

    val visited = Array.fill(g.V)(Undiscovered)
    visitor.startVertex(u, g)
    g match {
      case dig : DirectedGraph => dfsInnerDi(dig, u, visitor, visited)
      case undig : UndirectedGraph => dfsInnerUn(undig, u, u, visitor, visited)
    }
  }

  // Visit all vertices in a graph using dfs
  def dfsVisitAll(g: GraphLike, visitor: dfsVisitor): Unit = {
    require(g.V > 0, "Empty graph")
    val visited = Array.fill(g.V)(Undiscovered)
    for (u <- 0 until g.V)
      if (visited(u) == Undiscovered) {
        visitor.startVertex(u, g)
        g match {
          case dig : DirectedGraph => dfsInnerDi(dig, u, visitor, visited)
          case undig : UndirectedGraph => dfsInnerUn(undig, u, u, visitor, visited)
        }
      }
  }

  // bfs visitor function from a given vertex
  // Relies on caller to set up visited on initial call
  private def bfsInner(g: GraphLike, s: Int,
                       visitor: bfsVisitor,
                       visited: Array[VertexSearchStatus]): Unit = {

    val q = new MQueue[Int]

    visited(s) = Discovered
    visitor.discoverVertex(s, g) // Pre-order
    q += s // Enqueue start vertex or else this exits immediately

    // Main loop
    while (!q.isEmpty) {
      val u = q.dequeue()
      for (v <- g.adj(u)) {
        if (visited(v) == Undiscovered) {
          visitor.treeEdge(u, v, g)
          visited(v) = Discovered
          visitor.discoverVertex(v, g)
          q += v
        }
        else
          visitor.nonTreeEdge(u, v, g)
      }
      visited(u) = Finished
    }
  }

  // Visit all vertices connected to a start vertex v using bfs
  def bfsVisitVertex(g: GraphLike, u: Int, visitor: bfsVisitor): Unit = {
    require(g.V > 0, "Empty graph")
    require(u < g.V & u >= 0, s"Invalid start vertex $u")

    val visited = Array.fill(g.V)(Undiscovered)
    visitor.startVertex(u, g)
    bfsInner(g, u, visitor, visited)
  }

  // Visit all vertices in a graph using bfs
  def bfsVisitAll(g: GraphLike, visitor: bfsVisitor): Unit = {
    require(g.V > 0, "Empty graph")
    val visited = Array.fill(g.V)(Undiscovered)
    for (u <- 0 until g.V)
      if (visited(u) == Undiscovered) {
        visitor.startVertex(u, g)
        bfsInner(g, u, visitor, visited)
      }
  }

  // Get list of vertices connected to specified one
  def connectedToVertex(u: Int, g: GraphLike): List[Int] = {
    val vdet = new VertexVisited(g) with dfsVisitor

    dfsVisitVertex(g, u, vdet)
    vdet.visitList
  }

  // Mark connected components
  private class ConnectedComponents(g: GraphLike) extends VertexVisitor {
    private[this] var idx: Int = -1;
    private[this] val comps = Array.fill[Int](g.V)(idx)

    override def startVertex(u: Int, g: GraphLike) = idx += 1
    override def discoverVertex(u: Int, g: GraphLike) = comps(u) = idx
    def components: IndexedSeq[Int] = comps.toIndexedSeq
  }

  // Labels all connected components with increasing index
  // Any two vertices with the same value are connected and
  // in the same component
  def findConnectedComponents(g: GraphLike): IndexedSeq[Int] = {
    val vis = new ConnectedComponents(g) with dfsVisitor
    dfsVisitAll(g, vis)
    vis.components
  }

  // Cycle visitor
  private class CycleDetector extends dfsVisitor {
    private[this] var cycle = false;

    def reset() = cycle = false
    override def backEdge(u: Int, v: Int, g: GraphLike) = cycle = true
    def hasCycle: Boolean = cycle
  }

  // Detects the presence of a cycle using a dfs
  def detectCycle(g: GraphLike): Boolean = {
    val cdet = new CycleDetector
    dfsVisitAll(g, cdet)
    cdet.hasCycle
  }

  // Visitor for finding paths from initial node
  //  to all nodes reachable from that vertex.  Works for
  //  BFS or DFS
  private class Path(g: GraphLike, initVertex: Int) extends VertexVisitor {
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
        val s = new MStack[Int]
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

  // Find DFS path between u and v, returning an empty
  //  list if there is none
  def findDFSPathBetween(u: Int, v: Int, g: GraphLike): List[Int] = {
    val vis = new Path(g, u) with dfsVisitor
    dfsVisitVertex(g, u, vis)
    vis.pathTo(v)
  }

  // Find path between u and all reachable vertices as
  // a map using dfs
  def findDFSPathsFrom(u: Int, g: GraphLike): Map[Int, List[Int]] = {
    val vis = new Path(g, u) with dfsVisitor
    dfsVisitVertex(g, u, vis)

    val ret = collection.mutable.Map.empty[Int, List[Int]]
    for (v <- 0 until g.V)
      if (vis.hasPathTo(v)) ret += (v -> vis.pathTo(v))
    ret.toMap
  }

  // Find BFS path between u and v, returning an empty
  //  list if there is none.  This is the shortest
  // path between the two vertices, although it may not
  // be unique
  def findBFSPathBetween(u: Int, v: Int, g: GraphLike): List[Int] = {
    val vis = new Path(g, u) with bfsVisitor
    bfsVisitVertex(g, u, vis)
    vis.pathTo(v)
  }

  // Find BFS path between u and all reachable vertices as
  // a map
  def findBFSPathsFrom(u: Int, g: GraphLike): Map[Int, List[Int]] = {
    val vis = new Path(g, u) with bfsVisitor
    bfsVisitVertex(g, u, vis)

    val ret = collection.mutable.Map.empty[Int, List[Int]]
    for (v <- 0 until g.V)
      if (vis.hasPathTo(v)) ret += (v -> vis.pathTo(v))
    ret.toMap
  }
}