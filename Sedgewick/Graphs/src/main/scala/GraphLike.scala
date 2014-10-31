package sedgewick.graphs

// Adjacency list representation
trait GraphLike {
  def V: Int // Number of vertices
  def E: Int // Number of Edges
  def adj(v: Int): Seq[Int] // Adjacency list
}

trait UndirectedGraph extends GraphLike {
  def degree(u: Int): Int // Degree of vertex
}

trait DirectedGraph extends GraphLike {
  def indegree(u: Int): Int // In Degree of vertex
  def outdegree(u: Int): Int // Out Degree of vertex
  def reverse: DirectedGraph // Provide reversed graph
}