/**
 * Union find structure for finding out if vertices are connected
 *
 * @param V Maximum number of vertices
 */
class UnionFind(val V: Int, val compress: Boolean = false) {
  require(V > 0, s"Number of vertices $V must be positive")

  private[this] val comp = Range(0, V).toArray[Int]
  private[this] val compSize = Array.fill[Int](V)(1)
  private[this] var ncomp: Int = V

  /**
   * Add an edge between u and v
   * @param u New edge start point to add
   * @param v New edge finish point to add
   */
  def addEdge(u: Int, v: Int): Unit = {
    require(u >= 0 & u < V, s"Specified vertex $u out of range [0, $V)")
    require(v >= 0 & v < V, s"Specified vertex $v out of range [0, $V)")

    val uRoot = component(u)
    val vRoot = component(v)

    // Join components, attaching the smaller one to the larger one
    if (uRoot != vRoot) {
      if (compSize(uRoot) < compSize(vRoot)) {
        comp(uRoot) = vRoot
        compSize(vRoot) += compSize(uRoot)
      } else {
        comp(vRoot) = uRoot
        compSize(uRoot) += compSize(vRoot)
      }
      ncomp -= 1
    }
  }

  /**
   * Find component of vertex
   *
   * @param u Vertex number
   * @return Component vertex is in
   */
  def component(u: Int): Int = {
    require(u >= 0 & u < V, s"Specified vertex $u out of range [0, $V)")
    if (u == comp(u)) {
      // Vertex is at root
      u
    } else {
      var compIdx = u
      if (compress) {
        // Full compression
        // Find component
        while (compIdx != comp(compIdx)) compIdx = comp(compIdx)
        // Re-assign everything along the path to that
        var tmp = u
        while (tmp != comp(tmp)) {
          comp(tmp) = compIdx
          tmp = comp(tmp)
        }
      } else {
        // Halving compression
        while (compIdx != comp(compIdx)) {
          comp(compIdx) = comp(comp(compIdx))
          compIdx = comp(compIdx)
        }
      }
      compIdx
    }
  }

  /**
   * @return Number of components
   */
  def nComponents: Int = ncomp

  /**
   * Determine whether two vertices are connected
   *
   * @param u First vertex
   * @param v Second vertex
   * @return True if there is a path from u to v
   */
  def connected(u: Int, v:Int): Boolean = {
    require(u >= 0 & u < V, s"Specified vertex $u out of range [0, $V)")
    require(v >= 0 & v < V, s"Specified vertex $v out of range [0, $V)")
    component(u) == component(v)
  }
}
