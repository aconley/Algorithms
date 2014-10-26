package sedgewick.graphs

import Search.dfsVisitPre
import org.scalatest._

// Test of search algorithms
class SearchTest extends FlatSpec with Matchers {

  // Two graphs to play with -- a very simple
  //  one and the more complex one used as a preliminary
  //  example in Sedgewick 4th Ed
  val edgeList1 = List((0, 1), (3, 4), (3, 5))
  val g1 = BasicGraph(edgeList1)
  val edgeList2 = List((0, 5), (4, 3), (0, 1), (9, 12), (6, 4),
    (5, 4), (0, 2), (11, 12), (9, 10), (0, 6), (7, 8), (9, 11),
    (5, 3))
  val g2 = BasicGraph(edgeList2)

  "dfsSearchPre" should "visit the right number of vertices" in {
    // Simple graph
    val v1 = new VisitCount

    // Visit 0
    dfsVisitPre(g1, 0, v1)
    v1.getNVisited should be(2)

    // Visit 1
    v1.resetNVisited()
    dfsVisitPre(g1, 1, v1)
    v1.getNVisited should be(2)

    // Visit 2
    v1.resetNVisited()
    dfsVisitPre(g1, 2, v1)
    v1.getNVisited should be(1)

    // Visit 3, 4, 5
    v1.resetNVisited()
    dfsVisitPre(g1, 3, v1)
    v1.getNVisited should be(3)
    v1.resetNVisited()
    dfsVisitPre(g1, 4, v1)
    v1.getNVisited should be(3)
    v1.resetNVisited()
    dfsVisitPre(g1, 5, v1)
    v1.getNVisited should be(3)

    // More complex graph; the example tinyG example in Sedgewick v4
    v1.resetNVisited()
    dfsVisitPre(g2, 1, v1)
    v1.getNVisited should be (7)
    v1.resetNVisited()
    dfsVisitPre(g2, 4, v1)
    v1.getNVisited should be (7)
    v1.resetNVisited()
    dfsVisitPre(g2, 8, v1)
    v1.getNVisited should be (2)
    v1.resetNVisited()
    dfsVisitPre(g2, 9, v1)
    v1.getNVisited should be (4)
    v1.resetNVisited()
  }

  it should "visit the right vertices" in {
    val vis = new VertexVisited(g2)
    dfsVisitPre(g2, 0, vis)
    vis.getNVisited should be (7)
    vis.allVisited should be (false)
    vis.didVisit(4) should be (true)
    vis.didVisit(9) should be (false)
    vis.visitList shouldEqual List(0, 1, 2, 3, 4, 5, 6)

    vis.reset()
    dfsVisitPre(g2, 8, vis)
    vis.getNVisited should be (2)
    vis.didVisit(7) should be (true)
    vis.didVisit(8) should be (true)
    vis.didVisit(9) should be (false)
    vis.visitList shouldEqual List(7, 8)

    vis.reset()
    dfsVisitPre(g2, 9, vis)
    vis.getNVisited should be (4)
    vis.didVisit(7) should be (false)
    vis.didVisit(9) should be (true)
    vis.didVisit(11) should be (true)
    vis.visitList shouldEqual List(9, 10, 11, 12)
  }
}
