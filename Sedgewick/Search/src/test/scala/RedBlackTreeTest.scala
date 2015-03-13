import org.scalatest._
import sedgewick.search.RedBlackTree

class RedBlackTreeTest extends FlatSpec with Matchers {
  val example = "ABCDEF".toList.zipWithIndex
  val tbl = RedBlackTree(example:_*)

  "A RedBlackTree" should "have the right number of elements" in {
    tbl.isEmpty should be (false)
    tbl.size should be (example.length)
  }
  "it" should "contain the expected keys" in {
    for ((k, v) <- example) tbl.contains(k) should be (true)
  }
  "it" should "have values matching the expected keys" in {
    for ((k, v) <- example) tbl(k) should be (Some(v))
  }
  "it" should "not contain unexpected elements" in {
    tbl.contains('M') should be (false)
    tbl('M') should be (None)
    tbl.contains('!') should be (false)
  }
  "it" should "support adding elements" in {
    tbl.put('X', 11)
    tbl.contains('X') should be (true)
    tbl('X') should be (Some(11))
    tbl.size should be (example.size + 1)
  }
  "it" should "support replacing elements" in {
    tbl.put('A', 5)
    tbl.contains('A') should be (true)
    tbl('A') should be (Some(5))
    tbl.size should be (example.size + 1)
    tbl.put('A', 0)
    tbl.contains('A') should be (true)
    tbl('A') should be (Some(0))
    tbl.size should be (example.size + 1)
  }
  "it" should "support removing elements" in {
    tbl.delete('X')
    tbl.contains('X') should be (false)
    tbl('X') should be (None)
    tbl.size should be (example.size)
  }
  "it" should "collapse duplicates on construction" in {
    val tbl2 = RedBlackTree("SEARCHSEARCHAC".toList.zipWithIndex:_*)
    tbl2.size should be (6)
  }
}