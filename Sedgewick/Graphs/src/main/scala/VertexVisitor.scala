package sedgewick.graphs

// Vertex visitor trait -- does something whenever
//  it visits a vertex during a search.  These get
//  passed to dfsVisit or bfsVisit.
trait VertexVisitor {
  def startVertex(u: Int, g: GraphLike) = {} // Called once on start vertex u
  def discoverVertex(u: Int, g: GraphLike) = {} // On visiting u
  def treeEdge(u: Int, v: Int, g: GraphLike) = {} // Going from u to unvisited v
  def finalizeVertex(u: Int, g: GraphLike) = {} // Done with vertex u
}

trait dfsVisitor extends VertexVisitor {
  def backEdge(u: Int, v: Int, g: GraphLike) = {} // Encounter back edge from u -> v
  def crossEdge(u: Int, v: Int, g: GraphLike) = {} // Encounter cross edge u -> v
}

trait bfsVisitor extends VertexVisitor {
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