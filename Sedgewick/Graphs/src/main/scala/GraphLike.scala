package sedgewick.graphs

/** Adjacency list representation of graphs */
trait GraphLike {
  /** Number of vertices */
  def V: Int
  /** Number of edges */
  def E: Int

  /** Get list of adjacent vertices
    *
    * @param u Vertex number [0, V)
    * @return Sequence of adjacent vertices
    */
  def adj(u: Int): Seq[Int]
}

/** Undirected graph trait */
trait UndirectedGraph extends GraphLike {
  /** Degree of vertex
    *
    * @param u Vertex number [0, V)
    * @return Degree of that vertex
    */
  def degree(u: Int): Int
}

/** Directed graph trait */
trait DirectedGraph extends GraphLike {
  /** In degree of vertex
    *
    * @param u Vertex number [0, V)
    * @return In degree of that vertex
    */
  def indegree(u: Int): Int

  /** Out degree of vertex
    *
    * @param u Vertex number [0, V)
    * @return Out degree of that vertex
    */
  def outdegree(u: Int): Int

  /** Get reverse graph
    *
    * @return Reversed [[DirectedGraph]]
    */
  def reverse: DirectedGraph
}