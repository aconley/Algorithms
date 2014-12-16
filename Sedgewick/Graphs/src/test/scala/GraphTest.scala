import org.scalatest._
import sedgewick.graphs._

class GraphTest extends FlatSpec with Matchers {

  val g1 = Graph(List((0, 1), (3, 4), (3, 5)))
  val g2 = Graph(List((0, 5), (4, 3), (0, 1), (9, 12), (6, 4),
    (5, 4), (0, 2), (11, 12), (9, 10), (0, 6), (7, 8), (9, 11),
    (5, 3)))

  "A Graph" should "have the right number of vertices and edges" in {
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
    g1.adj(0).contains(UndirectedEdge(0, 1)) should be (true)
    g1.adj(0).contains(UndirectedEdge(0, 2)) should be (false)
    g1.adj(1).contains(UndirectedEdge(1, 0)) should be (true)
    g1.adj(1).contains(UndirectedEdge(1, 2)) should be (false)
    g1.adj(2).isEmpty should be (true)
    g1.adj(3).contains(UndirectedEdge(3, 4)) should be (true)

    // Undirected edges don't care about order
    g1.adj(0).contains(UndirectedEdge(1, 0)) should be (true)
    g1.adj(3).contains(UndirectedEdge(4, 3)) should be (true)
  }

  it should "ignore duplicate edges unless specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (4, 3))
    val g = Graph(edgeList)
    g.V should be (6)
    g.E should be (3)
  }

  it should "ignore self loops unless specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (1, 1))
    val g = Graph(edgeList)
    g.V should be (6)
    g.E should be (3)
  }

  it should "allow duplicate edges if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (4, 3))
    val g = Graph(edgeList, allowDup=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(3).count(_.v == 4) should be (2)
  }

  it should "allow self loops if specified" in {
    val edgeList = List((0, 1), (3, 4), (3, 5), (2, 2))
    val g = Graph(edgeList, allowSelf=true)
    g.V should be (6)
    g.E should be (4)
    g.adj(2).contains(UndirectedEdge(2,2)) should be (true)
  }
}