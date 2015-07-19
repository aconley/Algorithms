import java.util.Comparator;
import java.util.Deque;
import java.util.ArrayDeque;

public class Solver {
  // Search on primary board
  private final MinPQ<SearchNode> pq;
  // Search on twinned board
  private final MinPQ<SearchNode> pqtwin;

  private boolean solvable;
  private int nMovesSolution;
  private Deque<Board> sol;

  private static class SearchNode {
    private final Board board;
    private final int nMoves;
    private final SearchNode prev;

    public SearchNode(Board b, int m, SearchNode prev) {
      board = b;
      nMoves = m;
      this.prev = prev;
    }

    @Override
    public String toString() {
      StringBuilder sb = new StringBuilder();
      sb.append("Board: " + board.toString());
      sb.append(" Manhattan: " + board.manhattan());
      sb.append("\n");
      sb.append(" nMoves: " + nMoves);
      return sb.toString();
    }
  }

  private class ManhattanComparator
      implements Comparator<SearchNode> {

    public int compare(SearchNode n1, SearchNode n2) {
      return n1.board.manhattan() + n1.nMoves -
        n2.board.manhattan() - n2.nMoves;
    }
  }

  public Solver(Board initial) {
    if (initial == null)
      throw new NullPointerException("Initial board is null");
    solvable = false;

    pq = new MinPQ<SearchNode>(new ManhattanComparator());
    pqtwin = new MinPQ<SearchNode>(new ManhattanComparator());

    pq.insert(new SearchNode(initial, 0, null));
    pqtwin.insert(new SearchNode(initial.twin(), 0, null));

    // This really shouldn't be in the constructor!
    //  But that's the API spec
    SearchNode currNode = solve();

    // Load up solution for either real board or
    //  twin solution
    nMovesSolution = currNode.nMoves;
    sol = new ArrayDeque<Board>(nMovesSolution);
    while (currNode != null) {
      sol.addFirst(currNode.board);
      currNode = currNode.prev;
    }
  }

  private boolean boardEquals(Board b, SearchNode n) {
    if (b == null)
      return (n == null) ? true : false;
    if (n == null)
      return false;
    return b.equals(n.board);
  }

  private SearchNode solve() {
    while (true) { // Always scary...
      // Start by taking a step on the main queue
      SearchNode currNode = pq.delMin();
      if (currNode.board.isGoal()) {
        solvable = true;
        return currNode;
      }
      // Insert neighbors
      for (Board b : currNode.board.neighbors()) {
        if (!boardEquals(b, currNode.prev)) {
          pq.insert(new SearchNode(b, currNode.nMoves + 1,
              currNode));
        }
      }

      // Try twin
      currNode = pqtwin.delMin();
      if (currNode.board.isGoal()) {
        solvable = false;
        return currNode;
      }
      // Insert neighbors
      for (Board b : currNode.board.neighbors()) {
        if (!boardEquals(b, currNode.prev)) {
          pqtwin.insert(new SearchNode(b, currNode.nMoves + 1,
            currNode));
        }
      }
    }
  }

  public boolean isSolvable() {
    return solvable;
  }

  public int moves() {
    return solvable ? nMovesSolution : -1;
  }

  public Iterable<Board> solution() {
    return solvable ? sol : null;
  }

  public static void main(String[] args) {
    In in = new In(args[0]);
    int N = in.readInt();
    int[][] blocks = new int[N][N];
    for (int i = 0; i < N; ++i)
      for (int j = 0; j < N; ++j)
        blocks[i][j] = in.readInt();
    Board initial = new Board(blocks);

    Solver solver = new Solver(initial);
    if (!solver.isSolvable())
      StdOut.println("No solution possible");
    else {
      StdOut.println("Minimum number of moves = " +
        solver.moves());
      StdOut.println("\n");
      for (Board board : solver.solution())
        StdOut.println(board);
    }
  }
}
