package sedgewick.sort

// The question is -- do this with a trait and a bunch of implementations
// of that trait, or use functions?  Sorts seem to be more naturally functions
// to me, so use that

object SortMethods {
  // This is done in a straightforward imperative style on an Array --
  //  it can be done with Lists and more functionally easily using span, etc.,
  // but that implementation is --slow--, which is a good example of why
  // people don't insertion sort Linked lists
  def insertionSort[A](vals: Array[A])(implicit ord: Ordering[A]): Array[A] = {
    val arraycopy = vals.clone()
    for (i <- 1 until arraycopy.length) {
      val current_value = arraycopy(i)
      var j = i - 1
      while (j > 0 && ord.lt(current_value, vals(j))) {
        arraycopy(j+1) = arraycopy(j)
        j = j - 1
      }
      arraycopy(j) = current_value
    }
    arraycopy
  }
}
