package sedgewick.graphs

import org.scalatest._

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

  }
}
