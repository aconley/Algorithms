package sedgewick.graphs

// Vertex visitor trait -- does something whenever
//  it visits a vertex during a search.  These get
//  passed to dfsVisit or bfsVisit
trait VertexVisitor {
  def discoverVertex(v: Int, g: GraphLike) = {}
  def treeEdge(v: Int, g: GraphLike) = {}
  def backEdge(v: Int, g: GraphLike) = {}
  def crossEdge(v: Int, g: GraphLike) = {}
}

// A few simple examples -- more useful ones
// are found in the Search interface
class VisitCount extends VertexVisitor {
  private[this] var n = 0

  override def discoverVertex(v: Int, g: GraphLike) = n += 1
  def resetNVisited() = n = 0
  def getNVisited = n
}

// This simple one simply keeps a list of every
//  vertex it visits in reverse order
class VisitList extends VertexVisitor {
  private[this] val visited = collection.mutable.MutableList[Int]()

  override def discoverVertex(v: Int, g: GraphLike) = visited += v
  def order = visited.toList
}

// Marks which vertices were visited
class VertexVisited(val g: GraphLike) extends VertexVisitor {
  private[this] var marked = Array.fill(g.V)(false)

  override def discoverVertex(v: Int, g: GraphLike) = {
    assert(v < marked.length, "Vertex index %d out of range".format(v))
    marked(v) = true
  }
  def reset(): Unit = marked = Array.fill(g.V)(false)
  def didVisit(v: Int): Boolean = marked(v)
  def getNVisited: Int = marked.count(_ == true)
  def visitList: List[Int] =
    marked.zipWithIndex.filter(_._1 == true).map(_._2).toList
  def allVisited: Boolean = marked forall (_ == true)
}