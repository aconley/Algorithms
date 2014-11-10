package sedgewick.graphs

import org.scalatest._

class DigraphTest extends FlatSpec with Matchers {

  val g1 = Digraph(List((0, 1), (3, 4), (3, 5))) // Simple example
  // More complex example from Sedgewick
  val g2 = Digraph(List((4, 2), (2, 3), (3, 2), (6, 0), (0, 1),
      (2, 0), (11, 12), (12, 9), (9, 10), (9, 11), (8, 9), (10, 12),
      (11, 4), (4, 3), (3, 5), (7, 8), (8, 7), (5, 4), (0, 5),
      (6, 4), (6, 9), (7, 6)))

  "A Digraph" should "have the right number of vertices and edges" in {
    g1.V should be (6)
    g1.E should be (3)

    // More complex example
    g2.V should be (13)
    g2.E should be (22)
  }

  it should "support querying outdegree of vertices" in {
    g1.outdegree(0) should be (1)
    g1.outdegree(2) should be (0)
    g1.outdegree(3) should be (2)
    g2.outdegree(0) should be (2)
    g2.outdegree(1) should be (0)
    g2.outdegree(3) should be (2)
    g2.outdegree(7) should be (2)
    g2.outdegree(9) should be (2)
    g2.outdegree(11) should be (2)
  }

  it should "support querying indegree of vertices" in {
    g1.indegree(0) should be (0)
    g1.indegree(1) should be (1)
    g1.indegree(2) should be (0)
    g1.indegree(3) should be (0)

    g2.indegree(0) should be (2)
    g2.indegree(1) should be (1)
    g2.indegree(5) should be (2)
    g2.indegree(10) should be (1)
  }

  it should "support querying of edges" in {
    g1.adj(0).contains(DirectedEdge(0, 1)) should be (true)
    g1.adj(0).contains(DirectedEdge(0, 2)) should be (false)
    g1.adj(1).contains(DirectedEdge(1, 0)) should be (false)
    g1.adj(1).contains(DirectedEdge(1, 2)) should be (false)
    g1.adj(2).isEmpty should be (true)
    g1.adj(3).contains(DirectedEdge(3, 4)) should be (true)

    g2.adj(4).contains(DirectedEdge(4, 3)) should be (true)
    g2.adj(4).contains(DirectedEdge(4, 5)) should be (false)
    g2.adj(9).contains(DirectedEdge(9, 6)) should be (false)
    g2.adj(9).contains(DirectedEdge(9, 7)) should be (false)
    g2.adj(9).contains(DirectedEdge(9, 10)) should be (true)

    // Directed edges -do- care about order in equality
    g2.adj(9).contains(DirectedEdge(10, 9)) should be (false)
  }

  it should "ignore duplicate edges unless specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (3, 4))
    val g = Digraph(edgeList)
    g.V should be (6)
    g.E should be (3)
  }

  it should "ignore self loops if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (1, 1))
    val g = Digraph(edgeList, allowSelf = false)
    g.V should be (6)
    g.E should be (3)
  }

  it should "allow duplicate edges if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (3, 4))
    val g = Digraph(edgeList, allowDup=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(3).count(_.v == 4) should be (2)
  }

  it should "allow self loops if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (2, 2))
    val g = Digraph(edgeList, allowSelf=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(2).contains(DirectedEdge(2, 2)) should be (true)
  }

  it should "support the reversal" in {
    val g1r = g1.reverse
    g1r.V should be (6)
    g1r.E should be (3)
    g1r.outdegree(0) should be (0)
    g1r.outdegree(1) should be (1)
    g1r.outdegree(2) should be (0)
    g1r.outdegree(3) should be (0)
    g1r.indegree(0) should be (1)
    g1r.indegree(1) should be (0)
    g1r.indegree(2) should be (0)
    g1r.indegree(3) should be (2)
    g1r.adj(0).isEmpty should be (true)
    g1r.adj(1).contains(DirectedEdge(1, 0)) should be (true)
    g1r.adj(3).contains(DirectedEdge(3, 4)) should be (false)
    g1r.adj(5).contains(DirectedEdge(5, 3)) should be (true)
  }
}