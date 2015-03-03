package sedgewick.sort

import scala.collection.mutable.{Buffer, ArrayBuffer}

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
    if (vals.length < 2) return vals // Quick return
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
    if (vals.length < 2) return vals
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

  // Scala, quite annoyingly, does not provide an integer exponentiation function
  //  So add one.  BigInt does have one
  private def pow(b: Int, n: Int): Int = {
    require(n >= 0, "n can't be negative")
    if (b == 0) {
      0
    } else {
      n match {
        case 0 => 1
        case x if x % 2 == 0 =>
          val pv = pow(b, n / 2)
          pv * pv
        case _ => b * pow(b, n - 1)
      }
    }
  }

  // Knuth's simple h <- 3 * h + 1 sequence
  //  starting at h[0]
  def simpleH(i: Int): Int = {
    require(i >= 0, "i must be non-negative")
    var h = 1
    for (j <- 1 until i) h = 3 * h + 1
    h
  }

  // Sedgewicks interleaved sequence
  def sedgewickH(i: Int): Int = {
    require(i >= 0, "i must be non-negative")
    if (i % 2 == 0) {
      9 * pow(4, i/2) - 9 * pow(2, i/2) + 1
    } else {
      pow(4, (i+3)/2) - 3 * pow(2, (i+3)/2) + 1
    }
  }

  /**
   * Shellsort, O(complicated)
   *
   * @param hgen Function to generate shell sort sequence
   * @param vals Array to return a sorted copy of
   * @param ord Defines ordering of elements
   * @tparam A Element type
   * @return A sorted copy of vals
   *
   * There are various strong assumptions about hgen detailed in Algorithms books --
   *  for example it must be ascending, should start with 1, etc.
   */
  def shellSort[A](hgen: Int => Int)(vals: IndexedSeq[A])(implicit ord: Ordering[A]): IndexedSeq[A] = {

    val n = vals.length
    if (n < 2) return vals

    // Figure out how far along the H sequence we have to go
    def getInitHi(maxval: Int): Int = {
      var hi = 0
      var hv = hgen(hi)
      while (hv < maxval) {
        hi += 1
        hv = hgen(hi)
      }
      hi
    }

    val vals_copy = vals.toBuffer

    // Set up max h
    var hidx = getInitHi(n)
    for (hi <- hidx to 0 by -1) {
      val h = hgen(hi)
      // h-insertion sort
      for (i <- h until n) {
        var j = i
        val curr_val = vals_copy(i)
        while (j >= h && ord.lt(curr_val, vals_copy(j - h))) {
          vals_copy(j) = vals_copy(j - h)
          j -= h
          vals_copy(j) = curr_val
        }
      }
    }
    vals_copy.toIndexedSeq
  }

  /**
   * Top down out-of-place merge sort
   *
   * @param vals Array to return a sorted copy of
   * @param ord Defines ordering of elements
   * @tparam A Element type
   * @return A sorted copy of vals
   */
  def mergeSort[A](vals: IndexedSeq[A])(implicit ord: Ordering[A]): IndexedSeq[A] = {
    // Merge operation of [lo, mid], [mid+1, hi] with auxilliary array for storage
    def merge(a: Buffer[A], aux: Buffer[A], lo: Int,
              mid: Int, hi: Int): Unit = {
      var i = lo // index into left part
      var j = mid + 1 // index into right part
      for (k <- lo to hi) aux(k) = a(k)
      for (k <- lo to hi)
        if (i > mid) {
          // Left is exhausted
          a(k) = aux(j)
          j += 1
        } else if (j > hi) {
          // Right is exhausted
          a(k) = aux(i)
          i += 1
        } else if (ord.lt(aux(j), aux(i))) {  //Using this order makes it stable
          a(k) = aux(j)
          j += 1
        } else {
          a(k) = aux(i)
          i += 1
        }
    }

    def mergeSortInner(a: Buffer[A], aux: Buffer[A],
                       lo: Int, hi: Int): Unit = {
      if (hi <= lo) return
      val mid = lo + (hi - lo) / 2
      mergeSortInner(a, aux, lo, mid)
      mergeSortInner(a, aux, mid+1, hi)
      merge(a, aux, lo, mid, hi)
    }

    val n = vals.length
    if (n < 2) return vals
    var aux = vals.toBuffer // Allocate once

    val vals_copy = vals.toBuffer
    mergeSortInner(vals_copy, aux, 0, n-1)
    vals_copy.toIndexedSeq
  }

  /**
   * Quicksort, O(n log n) average, O(n^2) worst
   *
   * @param vals Array to return a sorted copy of
   * @param ord Defines ordering of elements
   * @tparam A Element type
   * @return A sorted copy of vals
   *
   * Median of three partition selection with small size cutoff switch
   * to insertion sort
   */
  def quickSort[A](vals: IndexedSeq[A])(implicit ord: Ordering[A]): IndexedSeq[A] = {
    def exch(a: Buffer[A], i: Int, j: Int): Unit = {
      val v = a(i)
      a(i) = a(j)
      a(j) = v
    }

    // Exchange if a(j) < a(i)
    def compexch(a: Buffer[A], i: Int, j: Int): Unit = {
      if (ord.lt(a(j), a(i))) {
        val v = a(i)
        a(i) = a(j)
        a(j) = v
      }
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

    // Median of 3 with small size cutoff
    def qInner(a: Buffer[A], l: Int, r: Int, m: Int): Unit = {
      if (r - l < m) return  // Stop and switch to insertion sort

      // Median of 3 bit
      exch(a, (l+r)/2, r-1)
      compexch(a, l, r-1)
      compexch(a, l, r)
      compexch(a, r-1, r)

      // Sort bit
      val i = partition(a, l+1, r-1)
      qInner(a, l, i - 1, m)
      qInner(a, i + 1, r, m)
    }

    if (vals.length < 2) return vals
    val vals_copy = vals.toBuffer

    // Partial Quicksort
    val M = 10   // Hardwired switch point to insertion sort
    qInner(vals_copy, 0, vals_copy.length - 1, M)

    // Hardwired insertion sort
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
}
