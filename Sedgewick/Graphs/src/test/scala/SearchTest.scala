package sedgewick.graphs

import Search.dfsVisitPre
import org.scalatest._

// Test of search algorithms
class SearchTest extends FlatSpec with Matchers {

  "dfsSearchPre" should "visit the right number of vertices" in {
    val v1 = new VisitCount
    val edgeList = List((0, 1), (3, 4), (3, 5))
    val g = BasicGraph(edgeList)

    // Visit 0
    dfsVisitPre(g, 0, v1)
    v1.getNVisited should be(2)

    // Visit 1
    v1.resetNVisited()
    dfsVisitPre(g, 1, v1)
    v1.getNVisited should be(2)

    // Visit 2
    v1.resetNVisited()
    dfsVisitPre(g, 2, v1)
    v1.getNVisited should be(1)

    // Visit 3, 4, 5
    v1.resetNVisited()
    dfsVisitPre(g, 3, v1)
    v1.getNVisited should be(3)
    v1.resetNVisited()
    dfsVisitPre(g, 4, v1)
    v1.getNVisited should be(3)
    v1.resetNVisited()
    dfsVisitPre(g, 5, v1)
    v1.getNVisited should be(3)
  }
}
