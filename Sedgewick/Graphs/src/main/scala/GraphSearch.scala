package sedgewick.graphs

import scala.collection.mutable

/** Used for keeping track of search progress */
private object VertexSearchStatus extends Enumeration {
  type VertexSearchStatus = Value
  val Undiscovered = Value // Not discovered; initialize to this
  val Discovered = Value // Discovered, not all neighbors done
  val Finished = Value // Discovered, all neighbors searched
}

/**
 * Routines related to graph searching
 */
object GraphSearch {

  import VertexSearchStatus._
  import collection.mutable.{Stack => MStack, Queue => MQueue}

  /**
   * Inner dfs visitor function for [[DirectedGraph]]
   * @param g [[DirectedGraph]] being searched
   * @param u Start vertex [0, g.V)
   * @param visitor [[dfsVisitor]] implementing actions to take
   * @param visited Keeps track of search progress
   *
   * Relies on caller to set up visitor and visited on initial call
   */
  private def dfsInnerDi[A <: DirectedEdgeLike](g: DirectedGraph[A],
                                                u: Int, visitor: dfsVisitor[A],
                                                visited: Array[VertexSearchStatus]): Unit = {

    visited(u) = Discovered
    visitor.discoverVertex(u, g) // Pre-order
    for (e <- g.adj(u)) {
      if (visited(e.v) == Undiscovered) {
        visitor.treeEdge(e, g)
        dfsInnerDi(g, e.v, visitor, visited)
      } else if (visited(e.v) == Discovered)
        visitor.backEdge(e, g)
      else
        visitor.crossEdge(e, g)
    }
    visitor.finalizeVertex(u, g)
    visited(u) = Finished
  }

  /**
   * Inner dfs visitor function for [[UndirectedGraph]]
   * @param g [[UndirectedGraph]] being searched
   * @param u Start vertex [0, g.V)
   * @param w Vertex search came to u from
   * @param visitor [[dfsVisitor]] implementing actions to take
   * @param visited Keeps track of search progress
   *
   * Relies on caller to set up visitor and visited on initial call
   */
  private def dfsInnerUn[A <: UndirectedEdgeLike](g: UndirectedGraph[A], u: Int, w: Int,
                                                  visitor: dfsVisitor[A],
                                                  visited: Array[VertexSearchStatus]): Unit = {
    visited(u) = Discovered
    visitor.discoverVertex(u, g) // Pre-order
    for (e <- g.adj(u)) {
      if (visited(e.v) == Undiscovered) {
        visitor.treeEdge(e, g)
        dfsInnerUn(g, e.v, e.u, visitor, visited)
      } else if (e.v != w & visited(e.v) == Discovered)
        visitor.backEdge(e, g)
    }
    visitor.finalizeVertex(u, g)
    visited(u) = Finished
  }

  /**
   * dfs visit all vertices connected to a start vertex with a visitor
   *
   * @param g [[GraphLike]] to search
   * @param u Starting vertex [0, g.V)
   * @param visitor [[dfsVisitor]] to use during search
   */
  def dfsVisitVertex[A <: UndirectedEdgeLike](g: UndirectedGraph[A], u: Int,
                                              visitor: dfsVisitor[A]): Unit = {
    require(g.V > 0, "Empty graph")
    require(u < g.V & u >= 0, s"Invalid start vertex $u")

    val visited = Array.fill(g.V)(Undiscovered)
    visitor.startVertex(u, g)
    dfsInnerUn(g, u, u, visitor, visited)
  }

  /**
   * dfs visit all vertices connected to a start vertex with a visitor
   *
   * @param g [[GraphLike]] to search
   * @param u Starting vertex [0, g.V)
   * @param visitor [[dfsVisitor]] to use during search
   */
  def dfsVisitVertex[A <: DirectedEdgeLike](g: DirectedGraph[A], u: Int,
                                            visitor: dfsVisitor[A]): Unit = {
    require(g.V > 0, "Empty graph")
    require(u < g.V & u >= 0, s"Invalid start vertex $u")

    val visited = Array.fill(g.V)(Undiscovered)
    visitor.startVertex(u, g)
    dfsInnerDi(g, u, visitor, visited)
  }

  /**
   * dfs Visit all vertices in a [[UndiredGraph]] with a visitor
   *
   * @param g [[UndirectedGraph]] to search
   * @param visitor [[dfsVisitor]] to use during search
   */
  def dfsVisitAll[A <: UndirectedEdgeLike](g: UndirectedGraph[A],
                                           visitor: dfsVisitor[A]): Unit = {
    require(g.V > 0, "Empty graph")
    val visited = Array.fill(g.V)(Undiscovered)
    for (u <- 0 until g.V)
      if (visited(u) == Undiscovered) {
        visitor.startVertex(u, g)
        dfsInnerUn(g, u, u, visitor, visited)
      }
  }

  /**
   * dfs Visit all vertices in a [[DirectedGraph]] with a visitor
   *
   * @param g [[GraphLike]] to search
   * @param visitor [[dfsVisitor]] to use during search
   */
  def dfsVisitAll[A <: DirectedEdgeLike](g: DirectedGraph[A],
                                         visitor: dfsVisitor[A]): Unit = {
    require(g.V > 0, "Empty graph")
    val visited = Array.fill(g.V)(Undiscovered)
    for (u <- 0 until g.V)
      if (visited(u) == Undiscovered) {
        visitor.startVertex(u, g)
        dfsInnerDi(g, u, visitor, visited)
      }
  }

  /**
   * Breadth-first search inner function
   *
   * @param g [[GraphLike]] to search
   * @param s Start vertex [0, g.V)
   * @param visitor [[bfsVisitor]] to use during search
   * @param visited Keeps track of search progress
   */
  private def bfsInner[A <: EdgeLike](g: GraphLike[A], s: Int,
                                      visitor: bfsVisitor[A],
                                      visited: Array[VertexSearchStatus]): Unit = {

    val q = new MQueue[Int]

    visited(s) = Discovered
    visitor.discoverVertex(s, g) // Pre-order
    q += s // Enqueue start vertex or else this exits immediately

    // Main loop
    while (!q.isEmpty) {
      val u = q.dequeue()
      for (e <- g.adj(u)) {
        if (visited(e.v) == Undiscovered) {
          visitor.treeEdge(e, g)
          visited(e.v) = Discovered
          visitor.discoverVertex(e.v, g)
          q += e.v
        }
        else
          visitor.nonTreeEdge(e, g)
      }
      visited(u) = Finished
    }
  }

  /**
   * Visit all vertices connected to a start vertex using bfs
   *
   * @param g [[GraphLike]] to search
   * @param u start vertex [0, g.V)
   * @param visitor [[bfsVisitor]] to use during search
   */
  def bfsVisitVertex[A <: EdgeLike](g: GraphLike[A], u: Int, visitor: bfsVisitor[A]): Unit = {
    require(g.V > 0, "Empty graph")
    require(u < g.V & u >= 0, s"Invalid start vertex $u")

    val visited = Array.fill(g.V)(Undiscovered)
    visitor.startVertex(u, g)
    bfsInner(g, u, visitor, visited)
  }

  /**
   * Visit all vertices in a [[GraphLike]] using bfs
   *
   * @param g [[GraphLike]] to search
   * @param visitor [[bfsVisitor]] to use during search
   */
  def bfsVisitAll[A <: EdgeLike](g: GraphLike[A], visitor: bfsVisitor[A]): Unit = {
    require(g.V > 0, "Empty graph")
    val visited = Array.fill(g.V)(Undiscovered)
    for (u <- 0 until g.V)
      if (visited(u) == Undiscovered) {
        visitor.startVertex(u, g)
        bfsInner(g, u, visitor, visited)
      }
  }

  /**
   * Get list of vertices connected to a given vertex
   *
   * @param u Vertex to start from [0, g.V)
   * @param g [[GraphLike]] to search
   * @return List of vertices reachable from u
   *
   * Uses dfs internally
   */
  def connectedToVertex[A <: EdgeLike](u: Int, g: GraphLike[A]): List[Int] = {
    val vdet = new VertexVisited[A](g) with dfsVisitor[A]

    dfsVisitVertex(g, u, vdet)
    vdet.visitList
  }

  /**
   * Class for marking connected components
   *
   * @constructor Create marking visitor
   * @param g [[GraphLike]] we will search
   */
  private class ConnectedComponents[A <: EdgeLike](g: GraphLike[A])
    extends VertexVisitor[A] {
    private[this] var idx: Int = -1
    private[this] val comps = Array.fill[Int](g.V)(idx)

    override def startVertex(u: Int, g: GraphLike[A]) = idx += 1
    override def discoverVertex(u: Int, g: GraphLike[A]) = comps(u) = idx
    def components: IndexedSeq[Int] = comps.toIndexedSeq
  }

  /** Labels all connected components with increasing index
    *
    * @param g [[UndirectedGraph]] to search
    * @return A list giving the component each vertex belongs to; any
    *         two vertices with the same value are connected and
    *         in the same component
    *
    * This implementation for undirected graphs only, which are easier
    */
  def findConnectedComponents[A <: EdgeLike](g: UndirectedGraph[A]): IndexedSeq[Int] = {
    val vis = new ConnectedComponents[A](g) with dfsVisitor[A]
    dfsVisitAll(g, vis)
    vis.components
  }

  /** Cycle detection visitor */
  private class CycleDetector[A <: EdgeLike] extends dfsVisitor[A] {
    private[this] var cycle = false

    def reset() = cycle = false
    override def backEdge(u: A, g: GraphLike[A]) = cycle = true
    def hasCycle: Boolean = cycle
  }

  /**
   * Determines whether a cycle is present
   *
   * @param g [[GraphLike]] to search
   * @return True if a cycle is detected, false otherwise
   */
  def detectCycle[A <: EdgeLike](g: GraphLike[A]): Boolean = {
    val cdet = new CycleDetector[A]
    dfsVisitAll(g, cdet)
    cdet.hasCycle
  }

  /**
   * Visitor for finding paths between vertices
   *
   * @constructor Make new path object
   * @param g [[GraphLike]] to search
   * @param initVertex Initial vertex [0, g.V)
   *
   * Works for bfs or dfs
   */
  private class Path[A <: EdgeLike](g: GraphLike[A], initVertex: Int)
    extends VertexVisitor[A] {
    val V = g.V
    val startVertex = initVertex
    val edgeTo = Array.fill[Int](V)(V)
    edgeTo(initVertex) = initVertex

    override def treeEdge(e: A, g: GraphLike[A]) = edgeTo(e.v) = e.u

    /** Is there a path from the start vertex to the specified one?
      *
      * @param u Vertex to search for path to
      * @return True if there is a path from initVertex to u
      */
    def hasPathTo(u: Int): Boolean = edgeTo(u) < V

    /**
     * Get a path from start vertex to specified one
     *
     * @param u Vertex to search for path to
     * @return The path, or None if one is not found
     */
    def pathTo(u: Int): Option[List[Int]] = {
      if (u == startVertex) {
        Some(List(u))
      } else if (!hasPathTo(u)) {
        None
      } else {
        // Use stack to back it out
        val s = new MStack[Int]
        var currVertex = u
        while (currVertex != startVertex) {
          s.push(currVertex)
          currVertex = edgeTo(currVertex)
        }
        s.push(startVertex)  // Path ends on start vertex
        Some(s.toList)
      }
    }
  }

  /**
   * Find a dfs path between two vertices if it exists
   *
   * @param u Start vertex [0, g.V)
   * @param v End vertex [0, g.V)
   * @param g [[GraphLike]] to search
   * @return A dfs path from u to v in g if it exists, or
   *         None if one is not found
   */
  def findDFSPathBetween[A <: EdgeLike](u: Int, v: Int,
                                        g: GraphLike[A]): Option[List[Int]] = {
    val vis = new Path[A](g, u) with dfsVisitor[A]
    dfsVisitVertex(g, u, vis)
    vis.pathTo(v)
  }

  /**
   * Find dfs paths between vertex and all reachable vertices
   *
   * @param u Start vertex [0, g.V)
   * @param g [[GraphLike]] to search
   * @return Map indexed by reachable vertex v of dfs path from u to v
   */
  def findDFSPathsFrom[A <: EdgeLike](u: Int, g: GraphLike[A]): Map[Int, List[Int]] = {
    val vis = new Path[A](g, u) with dfsVisitor[A]
    dfsVisitVertex(g, u, vis)

    val ret = collection.mutable.Map.empty[Int, List[Int]]
    for (v <- 0 until g.V)
      vis.pathTo(v) match {
        case Some(pth) => ret += (v -> pth)
        case None =>
      }
    ret.toMap
  }

  /**
   * Find a bfs path between two vertices if it exists
   *
   * @param u Start vertex [0, g.V)
   * @param v End vertex [0, g.V)
   * @param g [[GraphLike]] to search
   * @return A bfs path from u to v in g if it exists, or
   *         None if one is not found
   *
   * This is the shortest path between vertices, but it may
   * not be unique
   */
  def findBFSPathBetween[A <: EdgeLike](u: Int, v: Int,
                                        g: GraphLike[A]): Option[List[Int]] = {
    val vis = new Path[A](g, u) with bfsVisitor[A]
    bfsVisitVertex(g, u, vis)
    vis.pathTo(v)
  }

  /**
   * Find bfs paths between vertex and all reachable vertices
   *
   * @param u Start vertex [0, g.V)
   * @param g [[GraphLike]] to search
   * @return Map indexed by reachable vertex v of bfs path from u to v
   *
   * Each path is the shortest path between the two vertices, but may
   * not be unique
   */
  def findBFSPathsFrom[A <: EdgeLike](u: Int, g: GraphLike[A]): Map[Int, List[Int]] = {
    val vis = new Path[A](g, u) with bfsVisitor[A]
    bfsVisitVertex(g, u, vis)

    val ret = collection.mutable.Map.empty[Int, List[Int]]
    for (v <- 0 until g.V)
      vis.pathTo(v) match {
        case Some(pth) => ret += (v -> pth)
        case None =>
      }
    ret.toMap
  }

  /** Topological sort (forward) */
  private class TopologicalSortVisitor[A <: EdgeLike] extends dfsVisitor[A] {
    private[this] var cycle = false // Can't do it if there is a cycle
    private[this] val topo = new MStack[Int] // Holds sort

    def reset() = {
      cycle = false
      topo.clear()
    }
    override def backEdge(e: A, g: GraphLike[A]) = cycle = true

    override def finalizeVertex(u: Int, g: GraphLike[A]) =
      if (!cycle) topo.push(u)

    def hasCycle: Boolean = cycle

    def topologicalSort: Option[List[Int]] =
      if (cycle) None else Some(topo.toList)
  }

  /** Topologically sort a directed acyclic graph
    *
    * @param g The DAG to search
    * @return A forward topological ordering of the vertices, or None
    *         if this is not possible because the graph has a cycle
    */
  def topologicalSort[A <: EdgeLike](g: DirectedGraph[A]): Option[List[Int]] = {
    val vis = new TopologicalSortVisitor[A]
    dfsVisitAll(g, vis)
    vis.topologicalSort
  }


  /** Reverse post visitor for Kosaru */
  private class ReversePostVisitor[A <: EdgeLike] extends dfsVisitor[A] {
    private[this] val revpost = new MStack[Int] // Holds sort
    override def finalizeVertex(u: Int, g: GraphLike[A]) = revpost.push(u)
    def reversePost: List[Int] = revpost.toList
  }

  /**
   * Kosaru algorithm for finding connected components in a digraph
   *
   * @param g [[DirectedGraph]] to search
   * @return A sequence giving the component each vertex belongs to; any
   *         two vertices with the same value are connected and
   *         in the same component
   */
  def kosaruComponents[A <: EdgeLike](g: DirectedGraph[A]): IndexedSeq[Int] = {
    require(g.V > 0, "Empty graph")

    val vis = new ReversePostVisitor[A]
    dfsVisitAll(g.reverse, vis)
    val order = vis.reversePost

    // Now a custom dfs search all using order to mark
    //  the connected components
    val visC = new ConnectedComponents[A](g) with dfsVisitor[A]
    val visited = Array.fill(g.V)(Undiscovered)
    for (u <- order)
      if (visited(u) == Undiscovered) {
        visC.startVertex(u, g)
        dfsInnerDi(g, u, visC, visited)
      }
    visC.components
  }

  /** Tarajan's strong components algorithm
    *
    * @param g [[DirectedGraph]] to find strong components of
    * @return A sequence giving the component each vertex belongs to; any
    *         two vertices with the same value are connected and
    *         in the same component
    */
  // This can be implemented as a visitor (see the Boost implementation),
  // but it's a bit easier to use a more specialized recursive search
  def tarajanComponents[A <: EdgeLike](g: DirectedGraph[A]): IndexedSeq[Int] = {
    require(g.V > 0, "Empty graph")

    val preorder = Array.fill(g.V)(-1)
    val low = Array.fill(g.V)(g.V)
    val cc = Array.fill(g.V)(g.V)
    var cnt0 = 0
    var cnt1 = 0
    val stck = new MStack[Int]

    def tarajanInner(u: Int): Unit = {
      preorder(u) = cnt0
      low(u) = cnt0
      var minVal = cnt0
      cnt0 += 1
      stck.push(u)
      for (e <- g.adj(u)) {
        if (preorder(e.v) == -1) tarajanInner(e.v)
        if (low(e.v) < minVal) minVal = low(e.v)
      }
      if (minVal < low(u)) {
        low(u) = minVal
      } else {
        // Fill in connected component
        // by pulling vertices from the stack until we reach
        // u and setting their component number
        var v = g.V
        do {
          v = stck.pop()
          cc(v) = cnt1
          low(v) = g.V
        } while (v != u)
        cnt1 += 1
      }
    }

    for (u <- 0 until g.V)
      if (preorder(u) == -1) tarajanInner(u)

    cc.toIndexedSeq
  }

  /** Labels all connected components of a directed Graph with
    * increasing index
    *
    * @param g [[DirectedGraph]] to search
    * @return A list giving the component each vertex belongs to; any
    *         two vertices with the same value are connected and
    *         in the same component
    *
    * This implementation for directed graphs using the Kosaru
    * algorithm
    */
  def findConnectedComponents[A <: EdgeLike](g: DirectedGraph[A]): IndexedSeq[Int] = {
    tarajanComponents(g)
  }

}