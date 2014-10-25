package sedgewick.graphs

// Vertex visitor trait -- does something whenever
//  it visits a vertex
trait VertexVisitor {
  def visit(v: Int): Unit // Call on each visit
}

class VisitCount extends VertexVisitor {
  private[this] var n = 0

  def visit(v: Int) = n += 1
  def resetNVisited() = n = 0
  def getNVisited = n
}

// This simple one simply keeps a list of every
//  vertex it visits in reverse order
class VisitList extends VertexVisitor {
  private[this] val visited = collection.mutable.MutableList[Int]()

  def visit(v: Int) = visited += v
  def order = visited.toList
}

// This keeps a boolean array of all vertices visited,
// allowing constant time queries
class VertexVisited(val g: GraphLike) extends VertexVisitor {
  private[this] val marked = Array.fill(g.V)(false)

  def visit(v: Int) = marked(v) = true
  def visited(v: Int): Boolean = marked(v)
  def allVisited(): Boolean = marked forall (_ == true)
}