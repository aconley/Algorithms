import org.scalatest._
import sedgewick.graphs._

class WeightedGraphTest extends FlatSpec with Matchers {

  val g1 = WeightedGraph(List((0, 1, 0.5), (3, 4, 0.2), (3, 5, 0.3)))
  val g2 = WeightedGraph(List((0, 5, 0.3), (4, 3, 0.1), (0, 1, 0.1), (9, 12, 0.3),
    (6, 4, 0.0), (5, 4, 0.2), (0, 2, 0.7), (11, 12, 0.9), (9, 10, 0.4), (0, 6, 0.3),
    (7, 8, 0.2), (9, 11, 0.1), (5, 3, 1.0)))

  "A Graph" should "have the right number of vertices and edges" in {
    g1.V should be(6)
    g1.E should be(3)

    // More complex example
    g2.V should be(13)
    g2.E should be(13)
  }

  it should "support querying degree of vertices" in {
    g1.degree(0) should be(1)
    g1.degree(2) should be(0)
    g1.degree(3) should be(2)
    g2.degree(0) should be(4)
    g2.degree(1) should be(1)
    g2.degree(3) should be(2)
    g2.degree(7) should be(1)
    g2.degree(9) should be(3)
    g2.degree(11) should be(2)
  }

  it should "support querying of edges" in {
    g1.adj(0).contains(WeightedEdge(0, 1, 0.5)) should be(true)
    g1.adj(0).contains(WeightedEdge(0, 2, 0.1)) should be(false)
    g1.adj(1).contains(WeightedEdge(1, 0, 0.5)) should be(true)
    g1.adj(1).contains(WeightedEdge(1, 2, 0.1)) should be(false)
    g1.adj(2).isEmpty should be(true)
    g1.adj(3).contains(WeightedEdge(3, 4, 0.2)) should be(true)

    // Undirected edges don't care about order
    g1.adj(0).contains(WeightedEdge(1, 0, 0.5)) should be(true)
    g1.adj(3).contains(WeightedEdge(4, 3, 0.2)) should be(true)
  }

  it should "have edges with weights" in {
    g1.adj(0).head.weight should equal (0.5 +- 0.01)
    g2.adj(7).head.weight should equal (0.2 +- 0.01)
  }
}
