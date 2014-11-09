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

/** Graphs where we can add and remove edges.
  *
  * Don't mix with GraphMutableWeighted */
trait GraphMutable extends Mutable with Cloneable {
  /** Add an edge
    *
    * @param edge Edge to add
    * @return
    */
  def addEdge(edge: (Int, Int)): Unit

  /** Remove edge if present
    *
    * @param edge Edge to remove
    * @return
    */
  def removeEdge(edge: (Int, Int)): Unit
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

/** Trait for graphs with edge weights */
trait GraphWeighted {
  /** Get the weight for a specified edge
    *
    * @param edge Edge to get weight for
    * @return Some(weight) if edge is present, None if edge is not present
    */
  def getEdgeWeight(edge: (Int, Int)): Option[Float]
}

/** Trait for mutable graphs with edge weights.
  *
  * This can't be mixed with GraphMutable because
  * addEdge needs a weight as well
  */
trait GraphMutableWeighted extends Mutable with Cloneable {
  /** Set the weight for a specified edge
    *
    * @param edge Edge to set weight for
    * @param weight New weight
    */
  def setEdgeWeight(edge: (Int, Int), weight: Float): Unit

  /** Add an edge
    *
    * @param edge Edge to add
    * @param weight Weight of new edge
    */
  def addEdge(edge: (Int, Int), weight: Float): Unit

  /** Remove edge if present
    *
    * @param edge Edge to remove
    */
  def removeEdge(edge: (Int, Int)): Unit
}