import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;
import java.util.Collections;

import edu.princeton.cs.algs4.Digraph;
import edu.princeton.cs.algs4.DirectedCycle;
import edu.princeton.cs.algs4.In;

public class WordNet {

  // Note that nouns can appear in multiple Synsets
  private final Map<String, Set<Integer>> nounsToSynsetID;
  private final Map<Integer, String> synsetIDToSynset;
  private final SAP sap;


  public WordNet(String synsets, String hypernyms) {

    this.synsetIDToSynset = readSynsets(synsets);
    if (this.synsetIDToSynset.isEmpty()) {
      throw new IllegalArgumentException("Found no data in input " + synsets);
    }

    this.nounsToSynsetID = new HashMap<>();
    for (Map.Entry<Integer, String> entry : this.synsetIDToSynset.entrySet()) {
        String[] nouns = entry.getValue().split(" ");
        for (String noun : nouns) {
          Set<Integer> thisSynset = nounsToSynsetID.get(noun);
          if (thisSynset == null) {
            thisSynset = new HashSet<>();
            nounsToSynsetID.put(noun, thisSynset);
          }
          thisSynset.add(entry.getKey());
      }
    }

    Map<Integer, Set<Integer>> edges = readHypernyms(hypernyms);

    int maxID = Collections.max(synsetIDToSynset.keySet());
    Digraph G = new Digraph(maxID + 1);
    for (Map.Entry<Integer, Set<Integer>> hyper : edges.entrySet()) {
      int v = hyper.getKey();
      if (v > maxID) {
        throw new IllegalArgumentException("Found illegal start vertex " + v);
      }
      for (int w : hyper.getValue()) {
        if (w > maxID) {
          throw new IllegalArgumentException("Found illegal end vertex " + 2);
        }
        G.addEdge(v, w);
      }
    }

    // Check the graph for validity
    if (!isGraphDag(G)) {
      throw new IllegalArgumentException("Input graph not a DAG");
    }
    if (!isGraphRooted(G)) {
      throw new IllegalArgumentException("Input graph not Rooted");
    }

    this.sap = new SAP(G);
  }

  /**
   * Get an iterator over all nouns
   * @return The nouns
   */
  public Iterable<String> nouns() {
    return this.nounsToSynsetID.keySet();
  }

  /**
   * Tests whether a given word is a synset word
   * @param word The word to test. Non-null
   * @return True if the word is a recognized noun.
   */
  public boolean isNoun(String word) {
    if (word == null) {
      throw new NullPointerException("word was null");
    }
    return this.nounsToSynsetID.containsKey(word);
  }

  /**
   * Shortest ancestor distance between two nouns.
   * @param nounA First noun
   * @param nounB Second noun
   * @return The ancestor distance between them
   * @throws IllegalArgumentException if the nouns are not known.
   */
  public int distance(String nounA, String nounB) {
    if (nounA == null || nounB == null) {
      throw new NullPointerException("Invalid (null) noun");
    }

    Set<Integer> vertexA = nounsToSynsetID.get(nounA);
    if (vertexA == null) {
      throw new IllegalArgumentException("Unknown nounA: " + nounA);
    }
    Set<Integer> vertexB = nounsToSynsetID.get(nounB);
    if (vertexB == null) {
      throw new IllegalArgumentException("Unknown nounB: " + nounA);
    }
    return sap.length(vertexA, vertexB);
  }

  /**
   * The synset of the closest common ancestor of two nouns
   * @param nounA First noun
   * @param nounB Second noun
   * @return The synset for their common ancestor.
   * @throws IllegalArgumentException if the nouns are not known.
   */
  public String sap(String nounA, String nounB) {
    if (nounA == null || nounB == null) {
      throw new NullPointerException("Invalid (null) noun");
    }

    Set<Integer> vertexA = nounsToSynsetID.get(nounA);
    if (vertexA == null) {
      throw new IllegalArgumentException("Unknown nounA: " + nounA);
    }
    Set<Integer> vertexB = nounsToSynsetID.get(nounB);
    if (vertexB == null) {
      throw new IllegalArgumentException("Unknown nounB: " + nounA);
    }

    return this.synsetIDToSynset.get(sap.ancestor(vertexA, vertexB));
  }

  /**
   * Read in the synsets input file
   * @param synsets Name of input file
   * @return Map of synset ID to synset nouns separated by " "
   */
  private static Map<Integer, String> readSynsets(String synsets) {
    if (synsets == null) {
      throw new NullPointerException("Invalid (null) synsets");
    }

    Map<Integer, String> output = new HashMap<>();

    String line = null;
    In in = new In(synsets); // not auto closable... wha?
    while ((line = in.readLine()) != null) {
      if (line.equals("")) {
        continue;
      }

      String[] lineElements = line.split(",");
      if (lineElements.length < 2) {
        continue;
      }
      Integer synsetID;
      try {
        synsetID = Integer.parseInt(lineElements[0]);
      } catch (NumberFormatException e) {
        throw new IllegalArgumentException("Unable to parse synsetID from "
            + line);
      }
      if (lineElements[1].length() == 0) {
        continue;
      }
      output.put(synsetID, lineElements[1]);
    }
    return output;
  }

  private static Map<Integer, Set<Integer>> readHypernyms(String hypernym) {
    In in = new In(hypernym);
    String line = null;
    Map<Integer, Set<Integer>> output = new HashMap<>();

    while ((line = in.readLine()) != null) {
      if (line.equals("")) {
        continue;
      }
      String[] lineElements = line.split(",");
      if (lineElements.length < 2) {
        continue;
      }
      try {
        int start = Integer.parseInt(lineElements[0]);
        Set<Integer> ends = output.get(start);
        if (ends == null) {
          ends = new HashSet<>();
          output.put(start, ends);
        }
        for (int i = 1; i < lineElements.length; ++i) {
          ends.add(Integer.parseInt(lineElements[i]));
        }
      } catch (NumberFormatException e) {
        throw new IllegalArgumentException("Failed to parse line: " + line);
      }
    }
    return output;
  }

  private static boolean isGraphDag(Digraph G) {
    return !(new DirectedCycle(G).hasCycle());
  }

  private static boolean isGraphRooted(Digraph G) {
    // For a digraph to be rooted, it must have
    //  exactly one vertex with outdegree zero.
    boolean already_found_outdeg0 = false;
    for (int i = 0; i < G.V(); i++) {
      if (G.outdegree(i) == 0) {
        if (already_found_outdeg0) {
          return false;
        } else {
          already_found_outdeg0 = true;
        }
      }
    }
    return already_found_outdeg0;
  }

  public static void main(String[] args) {
    if (args.length < 4) {
      System.out.println("Usage: synsetFile hypernymFile wordA wordB");
      System.exit(1);
    }

    WordNet w = new WordNet(args[0], args[1]);
    System.out.println("Synset: " + w.sap(args[2], args[3]));
    System.out.println("Distance: " + w.distance(args[2], args[3]));
  }

}

