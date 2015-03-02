package sedgewick.sort

// The first question is -- do this with a trait and a bunch of implementations
// of that trait, or use functions?  Sorts seem to be more naturally functions
// to me, so use that
//
// The second question is: sort in place, or make a copy?  It's more efficient
// to sort in place, but a bit more functional to not modify the original, so
// go with the latter

object SortMethods {
  // This is done in a straightforward imperative style on an Array --
  //  it can be done with Lists and more functionally easily using span, etc.,
  // but that implementation is --slow--, which is a good example of why
  // people don't insertion sort Linked lists
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
}
