package sedgewick.graphs

// Adjacency list representation
trait GraphLike {
  def V: Int // Number of vertices
  def E: Int // Number of Edges
  def degree(v: Int): Int // Degree of vertex
  def adj(v: Int): Seq[Int] // Adjacency list
}

trait UndirectedGraph extends GraphLike
trait DirectedGraph extends GraphLike {
  def reverse: DirectedGraph
}