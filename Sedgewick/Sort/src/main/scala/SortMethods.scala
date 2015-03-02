package sedgewick.sort

import scala.collection.mutable.Buffer

// The first question is -- do this with a trait and a bunch of implementations
// of that trait, or use functions?  Sorts seem to be more naturally functions
// to me, so use that
//
// The second question is: sort in place, or make a copy?  It's more efficient
// to sort in place, but a bit more functional to not modify the original, so
// go with the latter

object SortMethods {
  // These are done in a pretty straightforward imperative style
  // They can be done more functionally, and on Seqs, but those are
  // generally rather slow.  Many sort methods depend on O(1) access
  // and modification to be efficient.

  /**
   * Insertion sort, O(n^2)
   * @param vals Array to return a sorted copy of
   * @param ord Defines ordering of elements
   * @tparam A Element type
   * @return A sorted copy of vals
   */
  def insertionSort[A](vals: IndexedSeq[A])(implicit ord: Ordering[A]): IndexedSeq[A] = {
    if (vals.length == 0) return vals // Quick return
    // Get a mutable copy to play with
    val vals_copy = vals.toBuffer
    for (i <- 1 until vals_copy.length) {
      val current_value = vals_copy(i)
      var j = i
      while (j > 0 && ord.lt(current_value, vals_copy(j-1))) {
        vals_copy(j) = vals_copy(j-1)
        j = j - 1
      }
      vals_copy(j) = current_value
    }
    vals_copy.toIndexedSeq
  }

  /**
   * Selection sort, O(n^2)
   * @param vals Array to return a sorted copy of
   * @param ord Defines ordering of elements
   * @tparam A Element type
   * @return A sorted copy of vals
   */
  def selectionSort[A](vals: IndexedSeq[A])(implicit ord: Ordering[A]): IndexedSeq[A] = {
    if (vals.length == 0) return vals
    val vals_copy = vals.toBuffer
    for (i <- 0 until (vals_copy.length-1)) {
      // Find the index of the minimum from [i, vals.length)
      var midx = i
      var mval = vals_copy(midx)
      for (j <- (i+1) until vals_copy.length)
        if (ord.lt(vals_copy(j), mval)) {
          midx = j
          mval = vals_copy(midx)
        }
      // Do the exchange
      if (midx != i) {
        val tmp = vals_copy(i)
        vals_copy(i) = vals_copy(midx)
        vals_copy(midx) = tmp
      }
    }
    vals_copy.toIndexedSeq
  }

  /**
   * Quicksort, O(n log n) average, O(n^2) worst
   * @param vals Array to return a sorted copy of
   * @param ord Defines ordering of elements
   * @tparam A Element type
   * @return A sorted copy of vals
   */
  def quickSort[A](vals: IndexedSeq[A])(implicit ord: Ordering[A]): IndexedSeq[A] = {
    def exch(a: Buffer[A], i: Int, j: Int): Unit = {
      val v = a(i)
      a(i) = a(j)
      a(j) = v
    }

    // Find the index of the first element larger than a(r)
    def firstLarger(a: Buffer[A], l: Int, r: Int): Int = {
      val e = a(r)
      for (i <- l until r)
        if (ord.gt(a(i), e)) return i
      return r
    }

    // Find the index of the last element smaller than a(r)
    def lastSmaller(a: Buffer[A], l: Int, r: Int): Int = {
      val e = a(r)
      for (i <- (r-1) until l by -1)
        if (ord.lt(a(i), e)) return i
      return l
    }

    def partition(a: Buffer[A], l: Int, r: Int): Int = {
      var i = firstLarger(a, l, r)
      var j = lastSmaller(a, l, r)
      while (i < j) {
        exch(a, i, j)
        i = firstLarger(a, l, r)
        j = lastSmaller(a, l, r)
      }
      exch(a, i, r)
      i
    }

    def qInner(a: Buffer[A], l: Int, r: Int): Unit = {
      if (r <= l) return
      val i = partition(a, l, r)
      qInner(a, l, i - 1)
      qInner(a, i + 1, r)
    }

    if (vals.length == 0) return vals
    val vals_copy = vals.toBuffer
    qInner(vals_copy, 0, vals_copy.length - 1)
    vals_copy.toIndexedSeq
  }
}
