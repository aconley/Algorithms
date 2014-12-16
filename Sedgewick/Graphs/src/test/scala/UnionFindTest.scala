package sedgewick.graphs

import org.scalatest._

class UnionFindTest extends FlatSpec with Matchers {
  val edg = List((4, 3), (3, 8), (6, 5), (9, 4), (2, 1),
    (8, 9), (5, 0), (7, 2), (6, 1), (1, 0), (6, 7))
  val tinyUF = UnionFind(edg)
  val tinyUFC = UnionFind(edg, compress=true)

  "UnionFind" should "find the right number of components" in {
    tinyUF.V should be (10)
    tinyUF.nComponents should be (2)
    tinyUFC.V should be (10)
    tinyUFC.nComponents should be (2)
  }

  "UnionFind" should "test if two vertices are connected" in {
    // Non-compressed version
    tinyUF.connected(4, 9) should be (true)
    tinyUF.connected(9, 4) should be (true)
    tinyUF.connected(0, 7) should be (true)
    tinyUF.connected(0, 4) should be (false)
    tinyUF.connected(0, 9) should be (false)
    tinyUF.connected(4, 9) should be (true)  // Make sure half-compression didn't break

    // compressed version
    tinyUFC.connected(4, 9) should be (true)
    tinyUFC.connected(9, 4) should be (true)
    tinyUFC.connected(0, 7) should be (true)
    tinyUFC.connected(0, 4) should be (false)
    tinyUFC.connected(0, 9) should be (false)
    tinyUFC.connected(4, 9) should be (true)
    tinyUFC.connected(3, 4) should be (true)
  }


}
