import org.scalatest._
import sedgewick.sort.SortMethods._

class InsertionSortTest extends FlatSpec with Matchers {
  val a1 = Array(9, 3, 7, 1, -1)

  "insertionSort" should "sort a simple array" in {
    insertionSort(a1) shouldEqual Array(-1, 1, 3, 7, 9)
  }
}