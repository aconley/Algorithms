public class Subset {
  /**
   * @param args Command line args
   */
  public static void main(String[] args) {
    RandomizedQueue<String> rqueue = new RandomizedQueue<String>();

    // The small constant extra memory requirement means we
    // have to process one string at a time
    while (!StdIn.isEmpty()) {
        String s = StdIn.readString();
        rqueue.enqueue(s);
    }

    int k = Integer.parseInt(args[0]);
    for (int i = 0; i < k; ++i) {
      StdOut.println(rqueue.dequeue());
    }
  }

}
