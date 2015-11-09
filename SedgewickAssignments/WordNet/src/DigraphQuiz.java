import edu.princeton.cs.algs4.DepthFirstOrder;
import edu.princeton.cs.algs4.Digraph;
import edu.princeton.cs.algs4.KosarajuSharirSCC;
import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.Topological;

import java.util.HashMap;
import java.util.Map;

/**
 * Doing the digraph quiz
 */
public class DigraphQuiz {

  private static char A = (char) 'A';

  private static Map<Integer, Character> createVertexToChar(int n) {
    Map<Integer, Character> map = new HashMap<>();
    for (int i = 0; i < n; ++i) {
      map.put(i, (char) (A + i));
    }
    return map;
  }

  private static Map<Character, Integer> createCharToVertex(int n) {
    Map<Character, Integer> map = new HashMap<>();
    for (int i = 0; i < n; ++i) {
      map.put((char) (A + i), i);
    }
    return map;
  }

  public static void topological() {
    int nVert = 8;
    Map<Integer, Character> vToC = createVertexToChar(nVert);
    Map<Character, Integer> cToV = createCharToVertex(nVert);
    Digraph G = new Digraph(nVert);

    // Note: you have to put them on in reverse order
    G.addEdge(cToV.get('A'), cToV.get('B'));
    G.addEdge(cToV.get('B'), cToV.get('C'));
    G.addEdge(cToV.get('C'), cToV.get('H'));
    G.addEdge(cToV.get('C'), cToV.get('D'));
    G.addEdge(cToV.get('E'), cToV.get('F'));
    G.addEdge(cToV.get('E'), cToV.get('A'));
    G.addEdge(cToV.get('F'), cToV.get('A'));
    G.addEdge(cToV.get('F'), cToV.get('C'));
    G.addEdge(cToV.get('F'), cToV.get('B'));
    G.addEdge(cToV.get('G'), cToV.get('H'));
    G.addEdge(cToV.get('G'), cToV.get('F'));
    G.addEdge(cToV.get('G'), cToV.get('C'));
    G.addEdge(cToV.get('H'), cToV.get('D'));

    Topological topological = new Topological(G);
    StdOut.print("Topological order: ");
    for (int v : topological.order()) {
      StdOut.print(vToC.get(v) + " ");
    }
    StdOut.println();
  }

  public static void stronglyConnected() {
    int nVert = 10;
    Map<Integer, Character> vToC = createVertexToChar(nVert);
    Map<Character, Integer> cToV = createCharToVertex(nVert);
    Digraph G = new Digraph(nVert);

    // Note: you have to put them on in reverse order
    G.addEdge(cToV.get('A'), cToV.get('B'));
    G.addEdge(cToV.get('B'), cToV.get('F'));
    G.addEdge(cToV.get('B'), cToV.get('H'));
    G.addEdge(cToV.get('C'), cToV.get('B'));
    G.addEdge(cToV.get('D'), cToV.get('H'));
    G.addEdge(cToV.get('D'), cToV.get('C'));
    G.addEdge(cToV.get('E'), cToV.get('D'));
    G.addEdge(cToV.get('E'), cToV.get('I'));
    G.addEdge(cToV.get('F'), cToV.get('A'));
    G.addEdge(cToV.get('F'), cToV.get('G'));
    G.addEdge(cToV.get('G'), cToV.get('H'));
    G.addEdge(cToV.get('G'), cToV.get('B'));
    G.addEdge(cToV.get('H'), cToV.get('C'));
    G.addEdge(cToV.get('I'), cToV.get('H'));
    G.addEdge(cToV.get('I'), cToV.get('D'));
    G.addEdge(cToV.get('I'), cToV.get('J'));
    G.addEdge(cToV.get('J'), cToV.get('E'));

    // Check the reverse postorder just to be sure
    DepthFirstOrder dfs = new DepthFirstOrder(G.reverse());
    StdOut.println("Kosaraju:");
    StdOut.print("  ReversePost: ");
    for (int v : dfs.reversePost()) {
      StdOut.print(vToC.get(v) + " ");
    }
    StdOut.println();

    // And do the scc
    KosarajuSharirSCC scc = new KosarajuSharirSCC(G);
    // compute list of vertices in each strong component
    StdOut.print("   Components:");
    for (int i = 0; i < nVert; i++) {
      StdOut.print(" " + scc.id(i));
    }
    StdOut.println();
  }

  public static void main(String[] args) {
    topological();
    stronglyConnected();
  }

}
