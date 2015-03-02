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
  property("produces a sorted vector") = forAll{(a1: Vector[Double]) => isSortedAscending(insertionSort(a1))}
  property("produces a sorted array") = forAll{(a1: Array[Double]) => isSortedAscending(insertionSort(a1))}
}