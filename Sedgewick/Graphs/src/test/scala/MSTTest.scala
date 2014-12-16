import org.scalatest._
import sedgewick.graphs._

class MSTTest extends FlatSpec with Matchers {

  val tinyEWG = WeightedGraph(List((4, 5, 0.35), (4, 7, 0.37), (5, 7, 0.28),
    (0, 7, 0.16), (1, 5, 0.32), (0, 4, 0.38), (2, 3, 0.17), (1, 7, 0.19),
    (0, 2, 0.26), (1, 2, 0.36), (1, 3, 0.29), (2, 7, 0.34), (6, 2, 0.4),
    (3, 6, 0.52), (6, 0, 0.58), (6, 5, 0.93)))

  "A WeightedGraph" should "have the right number of vertices and edges" in {
    tinyEWG.V should be (8)
    tinyEWG.E should be (16)
  }

  "LazyPrimMST" should "find the minimum spanning tree" in {
    val (wt, lpmst) = MST.LazyPrimMST(tinyEWG)

    wt should equal (1.81 +- 0.01)
    lpmst.V should be (8)
    lpmst.E should be (7)

    lpmst.degree(0) should be (2)
    lpmst.adj(0).contains(WeightedEdge(0, 7, 0.16)) should be (true)
    lpmst.adj(0).contains(WeightedEdge(0, 2, 0.26)) should be (true)
    lpmst.adj(0).contains(WeightedEdge(0, 4, 0.38)) should be (false)

    lpmst.degree(1) should be (1)
    lpmst.adj(1).contains(WeightedEdge(1, 7, 0.19)) should be (true)

    lpmst.degree(2) should be (3)
    lpmst.adj(2).contains(WeightedEdge(0, 2, 0.26)) should be (true)
    lpmst.adj(2).contains(WeightedEdge(2, 3, 0.17)) should be (true)
    lpmst.adj(2).contains(WeightedEdge(2, 6, 0.42)) should be (true)

    lpmst.degree(3) should be (1)
    lpmst.degree(4) should be (1)
    lpmst.degree(5) should be (2)
    lpmst.degree(6) should be (1)
    lpmst.degree(7) should be (3)
  }
}
