import edu.princeton.cs.algs4.Edge;
import edu.princeton.cs.algs4.EdgeWeightedGraph;
import edu.princeton.cs.algs4.IndexMinPQ;
import edu.princeton.cs.algs4.KruskalMST;
import edu.princeton.cs.algs4.Queue;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * Stuff related to MST quiz
 */
public class MSTQuiz {

  private static final Pattern edgePattern =
      Pattern.compile("\\s*([a-zA-Z])\\s*-\\s*([a-zA-Z])\\s+(\\d+)");

  private static Edge toEdge(String line) {
    Matcher matcher = edgePattern.matcher(line);
    if (matcher.find()) {
      int v1 = matcher.group(1).charAt(0) - ((int) 'A');
      int v2 = matcher.group(2).charAt(0) - ((int) 'A');
      int weight;

      try {
        weight = Integer.parseInt(matcher.group(3));
      } catch (NumberFormatException e) {
        throw new IllegalArgumentException("Failed to convert line: " + line);
      }

      return new Edge(v1, v2, weight);
    } else {
      throw new IllegalArgumentException("Invalid line: " + line);
    }
  }

  private static EdgeWeightedGraph readGraph(String filename)
      throws IOException {
    Path path = Paths.get(filename);

    List<Edge> edges;
    try (Stream<String> lines = Files.lines(path)) {
      edges = lines.map(l -> toEdge(l)).collect(Collectors.toList());
    }

    int nEdges = edges.size();
    int maxV = 0;
    for (Edge e : edges) {
      int v = e.either();
      int w = e.other(v);
      maxV = v > maxV ? v : maxV;
      maxV = w > maxV ? w : maxV;
    }
    EdgeWeightedGraph g = new EdgeWeightedGraph(maxV + 1);
    for (Edge e : edges) {
      g.addEdge(e);
    }
    return g;
  }

  private static void doKruskal() throws IOException {
    EdgeWeightedGraph g = readGraph("kruskalInput.txt");
    KruskalMST k = new KruskalMST(g);
    for (Edge e : k.edges()) {
      System.out.print(String.format("%d ", (int) e.weight()));
    }
    System.out.println();
  }

  private static void doPrim() throws IOException {
    int startV = 'H' - ((int) 'A');
    EdgeWeightedGraph g = readGraph("primInput.txt");

    PrimMST k = new PrimMST(g, startV);
    for (Edge e : k.edges()) {
      System.out.print(String.format("%d ", (int) e.weight()));
    }
    System.out.println();
  }

  private static class PrimMST {
    private static final double FLOATING_POINT_EPSILON = 1E-12;

    private Edge[] edgeTo;        // edgeTo[v] = shortest edge from tree vertex to non-tree vertex
    private double[] distTo;      // distTo[v] = weight of shortest such edge
    private boolean[] marked;     // marked[v] = true if v on tree, false otherwise
    private IndexMinPQ<Double> pq;

    /**
     * Compute a minimum spanning tree (or forest) of an edge-weighted graph.
     *
     * @param G the edge-weighted graph
     */
    public PrimMST(EdgeWeightedGraph G, int startIdx) {
      edgeTo = new Edge[G.V()];
      distTo = new double[G.V()];
      marked = new boolean[G.V()];
      pq = new IndexMinPQ<Double>(G.V());
      for (int v = 0; v < G.V(); v++)
        distTo[v] = Double.POSITIVE_INFINITY;

      prim(G, startIdx);
    }

    // run Prim's algorithm in graph G, starting from vertex s
    private void prim(EdgeWeightedGraph G, int s) {
      distTo[s] = 0.0;
      pq.insert(s, distTo[s]);
      while (!pq.isEmpty()) {
        int v = pq.delMin();
        scan(G, v);
      }
    }

    // scan vertex v
    private void scan(EdgeWeightedGraph G, int v) {
      marked[v] = true;
      for (Edge e : G.adj(v)) {
        int w = e.other(v);
        if (marked[w]) continue;         // v-w is obsolete edge
        if (e.weight() < distTo[w]) {
          distTo[w] = e.weight();
          edgeTo[w] = e;
          if (pq.contains(w)) pq.decreaseKey(w, distTo[w]);
          else pq.insert(w, distTo[w]);
        }
      }
    }

    /**
     * Returns the edges in a minimum spanning tree (or forest).
     *
     * @return the edges in a minimum spanning tree (or forest) as
     * an iterable of edges
     */
    public Iterable<Edge> edges() {
      Queue<Edge> mst = new Queue<Edge>();
      for (int v = 0; v < edgeTo.length; v++) {
        Edge e = edgeTo[v];
        if (e != null) {
          mst.enqueue(e);
        }
      }
      return mst;
    }
  }

  public static void main(String[] args) throws IOException {
    System.out.println("Kruskal:");
    doKruskal();
    //System.out.println("Prim:"); // Doesn't output in the desired order
    //doPrim();
  }
}
