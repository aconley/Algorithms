package sedgewick.graphs

// Vertex visitor trait -- does something whenever
//  it visits a vertex during a search.  These get
//  passed to dfsVisit or bfsVisit.
/**
 * Base vertex visitor trait common to both dfs and bfs searches
 *
 * Outputs in [[GraphSearch]] are constructed by passing
 * versions of this to dfsSearch or bfsSearch
 */
trait VertexVisitor {
  /** Called once on start vertex of search for each connected component
    *
    * @param u Vertex number [0, V)
    * @param g [[GraphLike]] being visited
    */
  def startVertex(u: Int, g: GraphLike) = {}

  /** Called when vertex is first discovered in search
    *
    * @param u Vertex number [0, V)
    * @param g [[GraphLike]] being visited
    */
  def discoverVertex(u: Int, g: GraphLike) = {}

  /** Called on encountering tree edge u -> v in graph search
    *
    * @param u Start vertex of edge
    * @param v End vertex of edge
    * @param g [[GraphLike]] being searched
    */
  def treeEdge(u: Int, v: Int, g: GraphLike) = {}

  /** Called when a vertex and all it's neighbors have been searched
    *
    * @param u Vertex number [0, V)
    * @param g [[GraphLike]] being searched
    */
  def finalizeVertex(u: Int, g: GraphLike) = {}
}

/** Additional visitor methods relevant to depth-first search */
trait dfsVisitor extends VertexVisitor {
  /** Called when a back edge from u -> v is encountered
    *
    * @param u Start vertex of edge
    * @param v End vertex of edge
    * @param g [[GraphLike]] being searched
    */
  def backEdge(u: Int, v: Int, g: GraphLike) = {}

  /** Called when a cross edge is encountered
    *
    * @param u Start vertex of edge
    * @param v End vertex of edge
    * @param g [[GraphLike]] being searched
    *
    * Can only happen in [[DirectedGraph]]
    */
  def crossEdge(u: Int, v: Int, g: GraphLike) = {}
}

/** Additional visitor methods relevant to breadth-first search */
trait bfsVisitor extends VertexVisitor {
  /** Called when a non-tree edge is encountered
    *
    * @param u Start vertex of edge
    * @param v End vertex of edge
    * @param g [[GraphLike]] being searched
    */
  def nonTreeEdge(u: Int, v: Int, g: GraphLike) = {}
}

// A few simple examples -- more useful ones
// are found in the Search interface
class VisitCount extends VertexVisitor {
  private[this] var n = 0

  override def discoverVertex(u: Int, g: GraphLike) = n += 1
  def resetNVisited() = n = 0
  def getNVisited = n
}

// This simple one simply keeps a list of every
//  vertex it visits in reverse order
class VisitList extends VertexVisitor {
  private[this] val visited = collection.mutable.MutableList[Int]()

  override def discoverVertex(u: Int, g: GraphLike) = visited += u
  def order = visited.toList
}

// Marks which vertices were visited
class VertexVisited(val g: GraphLike) extends VertexVisitor {
  private[this] var marked = Array.fill(g.V)(false)

  override def discoverVertex(u: Int, g: GraphLike) = {
    assert(u < marked.length, "Vertex index %d out of range".format(u))
    marked(u) = true
  }
  def reset(): Unit = marked = Array.fill(g.V)(false)
  def didVisit(u: Int): Boolean = marked(u)
  def getNVisited: Int = marked.count(_ == true)
  def visitList: List[Int] =
    marked.zipWithIndex.filter(_._1 == true).map(_._2).toList
  def allVisited: Boolean = marked forall (_ == true)
}