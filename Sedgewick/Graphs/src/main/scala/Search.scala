package sedgewick.graphs

object Search {
  // Visit all vertices connected to v in the graph g
  //  using a depth first search calling the visitor
  // in pre-order
  def dfsVisitPre(g: GraphLike, v: Int, visitor: VertexVisitor): Unit = {
    require(v < g.V & v >= 0, s"Invalid start vertex $v")

    val visited = Array.fill(g.V)(false)
    def dfsPre(g: GraphLike, w: Int): Unit = {
      visited(w) = true
      visitor.visit(w) // Pre-order
      for (e <- g.adj(w))
        if (!visited(e)) dfsPre(g, e)
    }
    dfsPre(g, v)
  }

  // Visit all vertices connected to v in the graph g
  //  using a depth first search calling the visitor
  // in post-order
  def dfsVisitPost(g: GraphLike, v: Int, visitor: VertexVisitor): Unit = {
    require(v < g.V & v >= 0, s"Invalid start vertex $v")

    val visited = Array.fill(g.V)(false)
    def dfsPost(g: GraphLike, w: Int): Unit = {
      visited(w) = true
      for (e <- g.adj(w))
        if (!visited(e)) dfsPost(g, e)
      visitor.visit(w) // Post-order
    }
    dfsPost(g, v)
  }
}