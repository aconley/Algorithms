package sedgewick.graphs

import org.scalatest._

class MutableGraphTest extends FlatSpec with Matchers {

  val g1 = MutableGraph(List((0, 1), (3, 4), (3, 5)))
  val g2 = MutableGraph(List((0, 5), (4, 3), (0, 1), (9, 12), (6, 4),
    (5, 4), (0, 2), (11, 12), (9, 10), (0, 6), (7, 8), (9, 11),
    (5, 3)))

  "A MutableGraph" should "have the right number of vertices and edges" in {
    g1.V should be (6)
    g1.E should be (3)

    // More complex example
    g2.V should be (13)
    g2.E should be (13)
  }

  it should "support querying degree of vertices" in {
    g1.degree(0) should be (1)
    g1.degree(2) should be (0)
    g1.degree(3) should be (2)
    g2.degree(0) should be (4)
    g2.degree(1) should be (1)
    g2.degree(3) should be (2)
    g2.degree(7) should be (1)
    g2.degree(9) should be (3)
    g2.degree(11) should be (2)
  }

  it should "support querying of edges" in {
    g1.adj(0).contains(1) should be (true)
    g1.adj(0).contains(2) should be (false)
    g1.adj(1).contains(0) should be (true)
    g1.adj(1).contains(2) should be (false)
    g1.adj(2).isEmpty should be (true)
    g1.adj(3).contains(4) should be (true)
  }

  it should "ignore duplicate edges" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (4, 3))
    val g = MutableGraph(edgeList)
    g.V should be (6)
    g.E should be (3)
  }

  it should "allow self loops" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (2, 2))
    val g = MutableGraph(edgeList, allowSelf=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(2).contains(2) should be (true)
  }

  it should "allow cloning" in {
    val g2c = g2.clone
    g2c.V should be (13)
    g2c.E should be (13)
    g2c.degree(0) should be (4)
    g2c.degree(1) should be (1)
    g2c.degree(3) should be (2)
    g2c.degree(7) should be (1)
    g2c.degree(9) should be (3)
    g2c.degree(11) should be (2)
    g2c.adj(0) contains 5 should be (true)
  }
  it should "allow edges to be added" in {
    val g1c = g1.clone
    g1c.addEdge((1, 2))
    g1c.V should be (6)
    g1c.E should be (4)
    g1c.degree(1) should be (2)
    g1c.degree(2) should be (1)
    g1c.adj(0) contains 2 should be (false)
    g1c.adj(1) contains 2 should be (true)
    g1c.adj(2) contains 1 should be (true)

    g1c.addEdge((0, 0))  // Add a self loop
    g1c.V should be (6)
    g1c.E should be (5)
    g1c.degree(0) should be (2)
    g1c.adj(0) contains 0 should be (true)
  }

  it should "ignore edges already present when adding" in {
    val g1c = g1.clone
    g1c.addEdge((0, 1)) // Already present
    g1c.V should be (g1.V)
    g1c.E should be (g1.E)
    g1c.degree(0) should be (1)
    g1c.degree(1) should be (1)
  }

  it should "allow edges to be removed" in {
    val g1c = g1.clone
    g1c.removeEdge((1, 2))  // Doesn't exist, so nothing should change
    g1c.V should be (6)
    g1c.E should be (3)

    g1c.removeEdge((3, 4))
    g1c.V should be (6)
    g1c.E should be (2)
    g1c.degree(1) should be (1)
    g1c.degree(2) should be (0)
    g1c.degree(3) should be (1)
    g1c.degree(4) should be (0)
    g1c.adj(3) contains 4 should be (false)
    g1c.adj(4) contains 3 should be (false)

    // Removing again should have no effect
    g1c.removeEdge((3, 4))
    g1c.V should be (6)
    g1c.E should be (2)

    // Try adding it back, just to make sure there is no
    //  funny interaction
    g1c.addEdge((3, 4))  // Add a self loop
    g1c.V should be (6)
    g1c.E should be (3)
    g1c.degree(3) should be (2)
    g1c.degree(4) should be (1)
    g1c.adj(3) contains 4 should be (true)
    g1c.adj(4) contains 3 should be (true)
    g1c.adj(3) contains 5 should be (true)
  }
}