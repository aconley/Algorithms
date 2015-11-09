import edu.princeton.cs.algs4.In;
import edu.princeton.cs.algs4.StdOut;

/**
 * Determine which word is least related to the others
 */
public class Outcast {

  private final WordNet wordNet;

  public Outcast(WordNet wordNet) {
    if (wordNet == null) {
      throw new NullPointerException("Invalid (null) wordNet");
    }
    this.wordNet = wordNet;
  }

  public String outcast(String[] nouns) {

    if (nouns == null) {
      throw new NullPointerException("Invalid (null) nouns");
    }
    if (nouns.length == 0) {
      throw new IllegalArgumentException("Empty nouns");
    }
    if (nouns.length == 1) {
      return nouns[0];
    }

    // We could cut the work in half by saving pairwise distances,
    //  but if wordnet is modified to memoize that won't be helpful
    int maxDist = 0;
    String outcast = null;
    for (String noun1 : nouns) {
      int distance = 0;
      for (String noun2 : nouns) {
        if (!noun1.equals(noun2)) {
          distance += wordNet.distance(noun1, noun2);
        }
      }

      if (distance > maxDist) {
        maxDist = distance;
        outcast = noun1;
      }
    }

    return outcast;
  }

  public static void main(String[] args) {
    WordNet wordnet = new WordNet(args[0], args[1]);
    Outcast outcast = new Outcast(wordnet);
    for (int t = 2; t < args.length; t++) {
      In in = new In(args[t]);
      String[] nouns = in.readAllStrings();
      StdOut.println(args[t] + ": " + outcast.outcast(nouns));
    }
  }
}