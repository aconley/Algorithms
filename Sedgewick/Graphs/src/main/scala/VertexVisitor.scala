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
trait VertexVisitor[A <: EdgeLike] {
  /** Called once on start vertex of search for each connected component
    *
    * @param u Vertex number [0, V)
    * @param g [[GraphLike]] being visited
    */
  def startVertex(u: Int, g: GraphLike[A]) = {}

  /** Called when vertex is first discovered in search
    *
    * @param u Vertex number [0, V)
    * @param g [[GraphLike]] being visited
    */
  def discoverVertex(u: Int, g: GraphLike[A]) = {}

  /** Called on encountering tree edge u -> v in graph search
    *
    * @param e Edge
    * @param g [[GraphLike]] being searched
    */
  def treeEdge(e: A, g: GraphLike[A]) = {}

  /** Called when a vertex and all it's neighbors have been searched
    *
    * @param u Vertex number [0, V)
    * @param g [[GraphLike]] being searched
    */
  def finalizeVertex(u: Int, g: GraphLike[A]) = {}
}

/** Additional visitor methods relevant to depth-first search */
trait dfsVisitor[A <: EdgeLike] extends VertexVisitor[A] {
  /** Called when a back edge from u -> v is encountered
    *
    * @param e Edge
    * @param g [[GraphLike]] being searched
    */
  def backEdge(e: A, g: GraphLike[A]) = {}

  /** Called when a cross edge is encountered
    *
    * @param e Edge
    * @param g [[GraphLike]] being searched
    *
    * Can only happen in [[DirectedGraph]]
    */
  def crossEdge(e: A, g: GraphLike[A]) = {}
}

/** Additional visitor methods relevant to breadth-first search */
trait bfsVisitor[A <: EdgeLike] extends VertexVisitor[A] {
  /** Called when a non-tree edge is encountered
    *
    * @param e Edge
    * @param g [[GraphLike]] being searched
    */
  def nonTreeEdge(e: A, g: GraphLike[A]) = {}
}

// A few simple examples -- more useful ones
// are found in the Search interface
class VisitCount[A <: EdgeLike] extends VertexVisitor[A] {
  private[this] var n = 0

  override def discoverVertex(u: Int, g: GraphLike[A]) = n += 1
  def resetNVisited() = n = 0
  def getNVisited = n
}

// This simple one simply keeps a list of every
//  vertex it visits in reverse order
class VisitList[A <: EdgeLike] extends VertexVisitor[A] {
  private[this] val visited = collection.mutable.MutableList[Int]()

  override def discoverVertex(u: Int, g: GraphLike[A]) = visited += u
  def order = visited.toList
}

// Marks which vertices were visited
class VertexVisited[A <: EdgeLike](val g: GraphLike[A]) extends VertexVisitor[A] {
  private[this] var marked = Array.fill(g.V)(false)

  override def discoverVertex(u: Int, g: GraphLike[A]) = {
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