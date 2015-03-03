import org.scalacheck.Properties
import org.scalacheck.Prop.{forAll, BooleanOperators}

import sedgewick.sort.SortMethods._

// Convenience methods for sorting tests
object SortConvenience {
  def isSortedAscending[A](as: IndexedSeq[A])(implicit ord: Ordering[A]): Boolean = {
    if (as.length == 0) return true
    for (i <- 1 until as.length)
      if (ord.lt(as(i), as(i-1))) return false
    true
  }
}

object InsertionSortSpecification extends Properties("insertionSort") {
  import SortConvenience.isSortedAscending
  property("preserves length") = forAll{ (a1: Array[Int]) => insertionSort(a1).length == a1.length }
  property("multiple sorts are idempotent") =
    forAll{ (a1: Array[Int]) => insertionSort(a1) == insertionSort(insertionSort(a1))}
  property("head should be minimum") = forAll{ (a1: Array[Int]) =>
    (a1.length > 0) ==> (a1.min == insertionSort(a1).head) }
  property("produces a sorted array") = forAll{(a1: Array[Double]) => isSortedAscending(insertionSort(a1))}
}

object SelectionSortSpecification extends Properties("selectionSort") {
  import SortConvenience.isSortedAscending
  property("preserves length") = forAll{ (a1: Array[Int]) => selectionSort(a1).length == a1.length }
  property("multiple sorts are idempotent") =
    forAll{ (a1: Array[Int]) => selectionSort(a1) == selectionSort(selectionSort(a1))}
  property("head should be minimum") = forAll{ (a1: Array[Int]) =>
    (a1.length > 0) ==> (a1.min == selectionSort(a1).head) }
  property("produces a sorted array") = forAll{(a1: Array[Double]) => isSortedAscending(selectionSort(a1))}
}

object ShellSortSpecification extends Properties("shellSort") {
  import SortConvenience.isSortedAscending

  // simple H sequence
  val shell1 = shellSort[Int](simpleH) _
  property("preserves length") = forAll{ (a1: Array[Int]) => shell1(a1).length == a1.length }
  property("multiple sorts are idempotent") =
    forAll{ (a1: Array[Int]) => shell1(a1) == shell1(shell1(a1))}
  property("head should be minimum") = forAll{ (a1: Array[Int]) =>
    (a1.length > 0) ==> (a1.min == shell1(a1).head) }
  property("produces a sorted array") = forAll{(a1: Array[Int]) => isSortedAscending(shell1(a1))}

  // More complex, but efficient version
  val shell2 = shellSort[Int](sedgewickH) _
  property("preserves length") = forAll{ (a1: Array[Int]) => shell2(a1).length == a1.length }
  property("multiple sorts are idempotent") =
    forAll{ (a1: Array[Int]) => shell2(a1) == shell2(shell2(a1))}
  property("head should be minimum") = forAll{ (a1: Array[Int]) =>
    (a1.length > 0) ==> (a1.min == shell2(a1).head) }
  property("produces a sorted array") = forAll{(a1: Array[Int]) => isSortedAscending(shell2(a1))}

}

object MergeSortSpecification extends Properties("mergeSort") {
  import SortConvenience.isSortedAscending
  property("preserves length") = forAll{ (a1: Array[Int]) => mergeSort(a1).length == a1.length }
  property("multiple sorts are idempotent") =
    forAll{ (a1: Array[Int]) => mergeSort(a1) == mergeSort(mergeSort(a1))}
  property("head should be minimum") = forAll{ (a1: Array[Int]) =>
    (a1.length > 0) ==> (a1.min == mergeSort(a1).head) }
  property("produces a sorted array") = forAll{(a1: Array[Double]) => isSortedAscending(mergeSort(a1))}
}

object QuickSortSpecification extends Properties("quickSort") {
  import SortConvenience.isSortedAscending
  property("preserves length") = forAll{ (a1: Array[Int]) => quickSort(a1).length == a1.length }
  property("multiple sorts are idempotent") =
    forAll{ (a1: Array[Int]) => quickSort(a1) == quickSort(quickSort(a1))}
  property("head should be minimum") = forAll{ (a1: Array[Int]) =>
    (a1.length > 0) ==> (a1.min == quickSort(a1).head) }
  property("produces a sorted array") = forAll{(a1: Array[Double]) => isSortedAscending(quickSort(a1))}
}