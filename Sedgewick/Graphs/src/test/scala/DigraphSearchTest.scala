package sedgewick.graphs

import GraphSearch._
import org.scalatest._

// Test of search algorithms on directed graphs
class DigraphSearchTest extends FlatSpec with Matchers {

  // Three graphs to play with
  // 1) a very simple one
  // 2) the more complex one used as a preliminary
  //  example in Sedgewick 4th Ed
  // 3) A small tree (no cycles)
  val g1 = Digraph(List((0, 1), (3, 4), (3, 5))) // Simple example
  // More complex example from Sedgewick (tinyDG)
  val g2 = Digraph(List((4, 2), (2, 3), (3, 2), (6, 0), (0, 1),
      (2, 0), (11, 12), (12, 9), (9, 10), (9, 11), (8, 9), (10, 12),
      (11, 4), (4, 3), (3, 5), (7, 8), (8, 7), (5, 4), (0, 5),
      (6, 4), (6, 9), (7, 6)))
  val g3 = Digraph(List((0, 1), (0, 2), (1, 3), (2, 4), (2, 6)))

  "dfsVisitVertex" should "visit the right number of vertices" in {
    // Simple graph
    val v1 = new VisitCount with dfsVisitor

    // Visit 0
    dfsVisitVertex(g1, 0, v1)
    v1.getNVisited should be(2)

    // Visit 1
    v1.resetNVisited()
    dfsVisitVertex(g1, 1, v1)
    v1.getNVisited should be(1)

    // Visit 2
    v1.resetNVisited()
    dfsVisitVertex(g1, 2, v1)
    v1.getNVisited should be(1)

    // Visit 3, 4, 5
    v1.resetNVisited()
    dfsVisitVertex(g1, 3, v1)
    v1.getNVisited should be(3)
    v1.resetNVisited()
    dfsVisitVertex(g1, 4, v1)
    v1.getNVisited should be(1)
    v1.resetNVisited()
    dfsVisitVertex(g1, 5, v1)
    v1.getNVisited should be(1)

    // More complex graph; the example tinyG example in Sedgewick v4
    v1.resetNVisited()
    dfsVisitVertex(g2, 0, v1)
    v1.getNVisited should be (6)
    v1.resetNVisited()
    dfsVisitVertex(g2, 1, v1)
    v1.getNVisited should be (1)
    v1.resetNVisited()
    dfsVisitVertex(g2, 6, v1)
    v1.getNVisited should be (11)
    v1.resetNVisited()
    dfsVisitVertex(g2, 10, v1)
    v1.getNVisited should be (10)
  }

  it should "visit the right vertices" in {
    val vis = new VertexVisited(g2) with dfsVisitor
    dfsVisitVertex(g2, 0, vis)
    vis.getNVisited should be (6)
    vis.allVisited should be (false)
    vis.didVisit(4) should be (true)
    vis.didVisit(9) should be (false)
    vis.visitList shouldEqual List(0, 1, 2, 3, 4, 5)

    vis.reset()
    dfsVisitVertex(g2, 9, vis)
    vis.getNVisited should be (10)
    vis.didVisit(7) should be (false)
    vis.didVisit(9) should be (true)
    vis.didVisit(11) should be (true)
    vis.visitList shouldEqual List(0, 1, 2, 3, 4, 5, 9, 10, 11, 12)

    vis.reset()
    dfsVisitVertex(g2, 8, vis)
    vis.getNVisited should be (13)
    vis.allVisited should be (true)
  }

  "bfsVisitVertex" should "visit the right number of vertices" in {
    // Simple graph
    val v1 = new VisitCount with bfsVisitor

    bfsVisitVertex(g1, 0, v1)
    v1.getNVisited should be(2)
    v1.resetNVisited()
    bfsVisitVertex(g1, 1, v1)
    v1.getNVisited should be(1)
    v1.resetNVisited()
    bfsVisitVertex(g1, 2, v1)
    v1.getNVisited should be(1)
    v1.resetNVisited()
    bfsVisitVertex(g1, 3, v1)
    v1.getNVisited should be(3)
    v1.resetNVisited()
    bfsVisitVertex(g1, 4, v1)
    v1.getNVisited should be(1)
    v1.resetNVisited()
    bfsVisitVertex(g1, 5, v1)
    v1.getNVisited should be(1)

    // More complex graph; the example tinyG example in Sedgewick v4
    v1.resetNVisited()
    bfsVisitVertex(g2, 0, v1)
    v1.getNVisited should be (6)
    v1.resetNVisited()
    bfsVisitVertex(g2, 1, v1)
    v1.getNVisited should be (1)
    v1.resetNVisited()
    bfsVisitVertex(g2, 6, v1)
    v1.getNVisited should be (11)
    v1.resetNVisited()
    bfsVisitVertex(g2, 8, v1)
    v1.getNVisited should be (13)
    v1.resetNVisited()
    bfsVisitVertex(g2, 9, v1)
    v1.getNVisited should be (10)
  }

  it should "visit the right vertices" in {
    val vis = new VertexVisited(g2) with bfsVisitor
    bfsVisitVertex(g2, 0, vis)
    vis.getNVisited should be (6)
    vis.allVisited should be (false)
    vis.didVisit(4) should be (true)
    vis.didVisit(9) should be (false)
    vis.visitList shouldEqual List(0, 1, 2, 3, 4, 5)

    vis.reset()
    bfsVisitVertex(g2, 8, vis)
    vis.getNVisited should be (13)
    vis.didVisit(7) should be (true)
    vis.didVisit(8) should be (true)
    vis.didVisit(9) should be (true)
    vis.allVisited should be (true)

    vis.reset()
    bfsVisitVertex(g2, 9, vis)
    vis.getNVisited should be (10)
    vis.didVisit(7) should be (false)
    vis.didVisit(10) should be (true)
    vis.didVisit(11) should be (true)
    vis.visitList shouldEqual List(0, 1, 2, 3, 4, 5, 9, 10, 11, 12)
  }

  "connectedToVertex" should "find the connected vertices" in {
    connectedToVertex(0, g2) shouldEqual List(0, 1, 2, 3, 4, 5)
    connectedToVertex(3, g2) shouldEqual List(0, 1, 2, 3, 4, 5)
    connectedToVertex(6, g2) shouldEqual
      List(0, 1, 2, 3, 4, 5, 6, 9, 10, 11, 12)
    connectedToVertex(10, g2) shouldEqual
      List(0, 1, 2, 3, 4, 5, 9, 10, 11, 12)
  }

  "findDFSPathBetween" should "find the path between vertices" in {
    findDFSPathBetween(0, 4, g2) shouldEqual Some(List(0, 5, 4))
    findDFSPathBetween(0, 0, g2) shouldEqual Some(List(0))
    findDFSPathBetween(0, 9, g2) shouldEqual None
  }

  "findDFSPathsFrom" should "find the paths between vertices" in {
    val paths = findDFSPathsFrom(0, g2)
    paths get 4 shouldEqual Some(List(0, 5, 4))
    paths get 0 shouldEqual Some(List(0))
    paths get 9  shouldEqual None
    paths get 5 shouldEqual Some(List(0, 5))
  }

  "findBFSPathBetween" should "find the path between vertices" in {
    findBFSPathBetween(0, 3, g2) shouldEqual Some(List(0, 5, 4, 3))
    findBFSPathBetween(0, 0, g2) shouldEqual Some(List(0))
    findBFSPathBetween(0, 9, g2) shouldEqual None
  }

  "findBFSPathsFrom" should "find the paths between vertices" in {
    val paths = findBFSPathsFrom(0, g2)
    paths get 0 shouldEqual Some(List(0))
    paths get 9  shouldEqual None
    paths get 3 shouldEqual Some(List(0, 5, 4, 3))
  }

  "detectCycle" should "detect cycles" in {
    detectCycle(g1) should be (false)
    detectCycle(g2) should be (true)
    detectCycle(g3) should be (false)
  }

  "topologicalSort" should "perform a toplogical sort of a DAG" in {
    // Example from Sedgewick 4.2; simplified version of g2,
    // but without a cycle
    val gt = Digraph(List((0, 1),(0, 5),(0, 6), (2, 0), (2, 3),
      (3, 5), (5, 4), (6, 4), (6, 9), (7, 6), (8, 7),
      (9, 10), (9, 11), (9, 12), (11, 12)))
    topologicalSort(gt) shouldEqual
      Some(List(8, 7, 2, 3, 0, 6, 9, 11, 12, 10, 1, 5, 4))
  }

  "kosaruComponents" should
    "find the connected components of a digraph" in {
    kosaruComponents(g2) shouldEqual
      IndexedSeq(1, 0, 1, 1, 1, 1, 3, 4, 4, 2, 2, 2, 2)
  }

  "tarajanComponents" should
    "find the connected components of a digraph" in {
    tarajanComponents(g2) shouldEqual
      IndexedSeq(1, 0, 1, 1, 1, 1, 3, 4, 4, 2, 2, 2, 2)
  }
}
