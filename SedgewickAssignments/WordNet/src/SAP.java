import edu.princeton.cs.algs4.Digraph;
import edu.princeton.cs.algs4.In;
import edu.princeton.cs.algs4.Queue;
import edu.princeton.cs.algs4.StdOut;

import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

/**
 * Computes shortest ancenstral path in a digraph.
 * <p>
 * Does not retain paths computed in previous calls to avoid
 * large storage overheads.
 */
// Can't be final because the autograder is stupid
public class SAP {
  private final Digraph G;

  // Cache previous result from single search
  private int prevVertV;
  private int prevVertW;
  private BestAncestor prevSingle;

  // And previous multi-vertex searches
  private Set<Integer> prevVertsV;
  private Set<Integer> prevVertsW;
  private BestAncestor prevMulti;

  /**
   * Constructor.
   *
   * @param G Input digraph.  Non-null.  The caller should not
   *          subsequently modify the digraph.
   */
  public SAP(Digraph G) {
    if (G == null) {
      throw new NullPointerException("Invalid (null) input digraph");
    }
    this.G = new Digraph(G); // Make a copy so this is immutable

    prevVertV = -1;
    prevVertW = -1;
    prevSingle = null;

    prevVertsV = null;
    prevVertsW = null;
    prevMulti = null;
  }

  private boolean isValidVertex(int v) {
    return (v >= 0) && (v < G.V());
  }

  public int length(int v, int w) {
    BestAncestor ancestor = getClosestAncestor(v, w);
    if (ancestor == null) {
      return -1;
    } else {
      return ancestor.getDistance();
    }
  }

  public int length(Iterable<Integer> v, Iterable<Integer> w) {
    BestAncestor ancestor = getClosestAncestor(v, w);
    if (ancestor == null) {
      return -1;
    } else {
      return ancestor.getDistance();
    }
  }

  public int ancestor(int v, int w) {
    BestAncestor ancestor = getClosestAncestor(v, w);
    if (ancestor == null) {
      return -1;
    } else {
      return ancestor.getVertex();
    }
  }

  public int ancestor(Iterable<Integer> v, Iterable<Integer> w) {
    BestAncestor ancestor = getClosestAncestor(v, w);
    if (ancestor == null) {
      return -1;
    } else {
      return ancestor.getVertex();
    }
  }

  private static class BestAncestor {
    private final int vertex;
    private final int distance;

    public BestAncestor(int vertex, int distance) {
      this.vertex = vertex;
      this.distance = distance;
    }

    public int getVertex() {
      return vertex;
    }

    public int getDistance() {
      return distance;
    }
  }

  /**
   * Gets closest ancestor between two vertices.
   *
   * @param v First vertex
   * @param w Second vertex
   * @return Information about closest shared ancestor, null if none
   *         found.
   */
   private BestAncestor getClosestAncestor(int v, int w) {
    if (!isValidVertex(v)) {
      throw new IndexOutOfBoundsException("Vertex v out of bounds");
    }
    if (!isValidVertex(w)) {
      throw new IndexOutOfBoundsException("Vertex w out of bounds");
    }

    // The most special case
    if (v == w) {
      return new BestAncestor(v, 0);
    }

    if (v == prevVertV && w == prevVertW) {
      return prevSingle;
    }

    // We could do this more efficiently by making a special case
    //  version of BFS that grows out from each vertex in turn.
    Map<Integer, Integer> pathsFromV = new BFSMap(G, v).getDistanceMap();
    Map<Integer, Integer> pathsFromW = new BFSMap(G, w).getDistanceMap();

    int closestAncestor = -1;
    int shortestDistance = Integer.MAX_VALUE;
    for (Map.Entry<Integer, Integer> pV : pathsFromV.entrySet()) {
      if (pathsFromW.containsKey(pV.getKey())) {
        int thisDist = pV.getValue() + pathsFromW.get(pV.getKey());
        if (thisDist < shortestDistance) {
          shortestDistance = thisDist;
          closestAncestor = pV.getKey();
        }
      }
    }

    BestAncestor result = (closestAncestor >= 0)
        ? new BestAncestor(closestAncestor, shortestDistance)
        : null;

    prevVertV = v;
    prevVertW = w;
    prevSingle = result;
    return result;
  }

  // Same thing but iterable
  private BestAncestor getClosestAncestor(Iterable<Integer> v,
      Iterable<Integer>  w) {
    if (v == null || w == null) {
      throw new NullPointerException("Invalid (null) input");
    }
    Set<Integer> vSet = new HashSet<>();
    for (Integer elem : v) {
      vSet.add(elem);
    }
    Set<Integer> wSet = new HashSet<>();
    for (Integer elem : w) {
      wSet.add(elem);
    }

    BestAncestor result = null;
    if (vSet.equals(prevVertsV) && wSet.equals(prevVertsW)) {
      result = prevMulti;
    } else {
      Map<Integer, Integer> pathsFromV = new BFSMap(G, vSet).getDistanceMap();
      Map<Integer, Integer> pathsFromW = new BFSMap(G, wSet).getDistanceMap();

      int closestAncestor = -1;
      int shortestDistance = Integer.MAX_VALUE;
      for (Map.Entry<Integer, Integer> pV : pathsFromV.entrySet()) {
        if (pathsFromW.containsKey(pV.getKey())) {
          int thisDist = pV.getValue() + pathsFromW.get(pV.getKey());
          if (thisDist < shortestDistance) {
            shortestDistance = thisDist;
            closestAncestor = pV.getKey();
          }
        }
      }
      result = (closestAncestor >= 0)
          ? new BestAncestor(closestAncestor, shortestDistance)
          : null;
      prevVertsV = vSet;
      prevVertsW = vSet;
      prevMulti = result;
    }
    return result;
  }

  // Specialized version of BFS that uses a Map to keep track
  //  of distances and visited vertices.  More efficient in the
  //  case where most of the graph is not reachable
  private static class BFSMap {
    private static final int INFINITY = Integer.MAX_VALUE;
    private final Map<Integer, Integer> vertexToDist;

    /**
     * Computes the shortest path from <tt>s</tt> and every other vertex
     * in graph <tt>G</tt>.
     *
     * @param G the digraph
     * @param s the source vertex
     */
    public BFSMap(Digraph G, int s) {
      Map<Integer, Integer> working = new HashMap<>();
      bfs(G, s, working);
      this.vertexToDist = Collections.unmodifiableMap(working);
    }

    /**
     * Computes the shortest path from any one of the source
     * vertices in <tt>sources</tt>
     * to every other vertex in graph <tt>G</tt>.
     *
     * @param G       the digraph
     * @param sources the source vertices
     */
    public BFSMap(Digraph G, Set<Integer> sources) {
      Map<Integer, Integer> working = new HashMap<>();
      bfs(G, sources, working);
      this.vertexToDist = Collections.unmodifiableMap(working);
    }

    // BFS from single source
    private static void bfs(Digraph G, int s,
        Map<Integer, Integer> vertexToDist) {
      Queue<Integer> q = new Queue<Integer>();
      vertexToDist.put(s, 0);
      q.enqueue(s);
      while (!q.isEmpty()) {
        int v = q.dequeue();
        int thisDist = vertexToDist.get(v);
        for (int w : G.adj(v)) {
          if (!vertexToDist.containsKey(w)) {
            vertexToDist.put(w, thisDist + 1);
            q.enqueue(w);
          }
        }
      }
    }

    // BFS from multiple sources
    private static void bfs(Digraph G, Set<Integer> sources,
        Map<Integer, Integer> vertexToDist) {
      Queue<Integer> q = new Queue<Integer>();
      for (int s : sources) {
        vertexToDist.put(s, 0);
        q.enqueue(s);
      }
      while (!q.isEmpty()) {
        int v = q.dequeue();
        int thisDist = vertexToDist.get(v);
        for (int w : G.adj(v)) {
          if (!vertexToDist.containsKey(w)) {
            vertexToDist.put(w, thisDist + 1);
            q.enqueue(w);
          }
        }
      }
    }

    /**
     * Is there a directed path from the source <tt>s</tt> (or sources) to vertex <tt>v</tt>?
     * @param v the vertex
     * @return <tt>true</tt> if there is a directed path, <tt>false</tt> otherwise
     */
    public boolean hasPathTo(int v) {
      return vertexToDist.containsKey(v);
    }

    /**
     * Returns the number of edges in a shortest path from the source <tt>s</tt>
     * (or sources) to vertex <tt>v</tt>?
     * @param v the vertex
     * @return the number of edges in a shortest path
     */
    public int distTo(int v) {
      return vertexToDist.get(v);
    }

    /**
     * Access to the underlying map
     */
    public Map<Integer, Integer> getDistanceMap() {
      return vertexToDist;
    }

  }

  public static void main(String[] args) {
    if (args.length < 3) {
      System.out.println("Usage: inputfile v w");
      System.exit(1);
    }
    In in = new In(args[0]);
    Digraph G = new Digraph(in);
    SAP sap = new SAP(G);
    int v = Integer.parseInt(args[1]);
    int w = Integer.parseInt(args[2]);

    int length   = sap.length(v, w);
    int ancestor = sap.ancestor(v, w);
    StdOut.printf("length = %d, ancestor = %d\n", length, ancestor);
  }
}
