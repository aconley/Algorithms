package sedgewick.graphs

import org.scalatest._

class BasicGraphTest extends FlatSpec with Matchers {

  "A BasicGraph" should "have the right number of vertices and edges" in {
    val edgeList = List((0, 1), (3, 4), (3, 5))
    val g = BasicGraph(edgeList)
    g.V should be (6)
    g.E should be (3)

    // More complex example
    val edgeList2 = List((0, 5), (4, 3), (0, 1), (9, 12), (6, 4),
      (5, 4), (0, 2), (11, 12), (9, 10), (0, 6), (7, 8), (9, 11),
      (5, 3))
    val g2 = BasicGraph(edgeList2)
    g2.V should be (13)
    g2.E should be (13)
  }

  it should "support querying of edges" in {
    val edgeList: List[(Int, Int)] = List((0, 1), (3, 4), (3, 5))
    val g = BasicGraph(edgeList)
    g.adj(0).contains(1) should be (true)
    g.adj(0).contains(2) should be (false)
    g.adj(1).contains(0) should be (true)
    g.adj(1).contains(2) should be (false)
    g.adj(2).isEmpty should be (true)
    g.adj(3).contains(4) should be (true)

  }

  it should "ignore duplicate edges unless specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (4, 3))
    val g = BasicGraph(edgeList)
    g.V should be (6)
    g.E should be (3)
  }

  it should "ignore self loops unless specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (1, 1))
    val g = BasicGraph(edgeList)
    g.V should be (6)
    g.E should be (3)
  }

  it should "allow duplicate edges if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (4, 3))
    val g = BasicGraph(edgeList, allowDup=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(3).count(_ == 4) should be (2)
  }

  it should "allow self loops if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (2, 2))
    val g = BasicGraph(edgeList, allowSelf=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(2).contains(2) should be (true)
  }
}