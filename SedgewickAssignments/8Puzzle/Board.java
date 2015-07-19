import static java.lang.Math.abs;
import java.util.List;
import java.util.ArrayList;
import java.util.Iterator;

public class Board {
  //  row major order representation of board state
  private final int[][] blocks;

  // For efficiency, keep track of where the zeros
  //  are.  Note we are allowing multiple zeros!
  private final List<Integer> zeroRows;
  private final List<Integer> zeroCols;

  public Board(int[][] blocks) {
    if (blocks == null || blocks.length == 0)
      throw new IllegalArgumentException("Input blocks is empty");
    zeroRows = new ArrayList<Integer>(1);
    zeroCols = new ArrayList<Integer>(1);
    this.blocks = new int[blocks.length][blocks.length];
    int blockValue;
    for (int i = 0; i < blocks.length; ++i) {
      if (blocks[i].length != blocks.length)
        throw new IllegalArgumentException("blocks not square");
      for (int j = 0; j < blocks.length; ++j) {
        blockValue = blocks[i][j];
        if (blockValue == 0) {
          zeroRows.add(i);
          zeroCols.add(j);
        }
        this.blocks[i][j] = blockValue;
      }
    }
    if (zeroRows.size() == 0)
      throw new IllegalArgumentException("Must have at least one empty");
  }

  public int dimension() {
    return blocks.length;
  }

  // Number of blocks out of place
  public int hamming() {
    int nOutOfPlace = 0;
    int blockValue;
    for (int i = 0; i < blocks.length; ++i) {
      for (int j = 0; j < blocks.length; ++j) {
        blockValue = blocks[i][j];
        // Don't counts blockValue == 0 (empty)
        if ((blockValue != 0) &&
          (blocks[i][j] != (i * blocks.length + j + 1))) {
          nOutOfPlace += 1;
        }
      }
    }
    return nOutOfPlace;
  }

  // Manhattan distance between current state
  //  and solution
  public int manhattan() {
    int sumManhattan = 0;
    int blockValue, expectedRow, expectedColumn;
    for (int i = 0; i < blocks.length; ++i) {
      for (int j = 0; j < blocks.length; ++j) {
        blockValue = blocks[i][j];
        if (blockValue == 0)
          continue; // Blank, ignore
        expectedRow = (blockValue - 1) / blocks.length;
        expectedColumn = (blockValue - 1) % blocks.length;
        sumManhattan += abs(expectedRow - i) +
            abs(expectedColumn - j);
      }
    }
    return sumManhattan;
  }

  public boolean isGoal() {
    return hamming() == 0;
  }

  // A Board created by exchanging two adjacent
  //  non-empty blocks in the same row
  // a) this is terrible name
  public Board twin() {
    int[][] newBlocks = new int[blocks.length][blocks.length];
    for (int i = 0; i < blocks.length; ++i) {
      for (int j = 0; j < blocks.length; ++j) {
        newBlocks[i][j] = blocks[i][j];
      }
    }
    // Swap the first non-empty we run into
    for (int i = 0; i < blocks.length; ++i) {
      for (int j = 0; j < blocks.length - 1; ++j) {
        if ((newBlocks[i][j] != 0) &&
            (newBlocks[i][j + 1] != 0)) {
              int t = newBlocks[i][j];
              newBlocks[i][j] = newBlocks[i][j + 1];
              newBlocks[i][j + 1] = t;
              return new Board(newBlocks);
        }
      }
    }
    throw new IllegalStateException("Unable to find blocks to swap");
  }

  private Board getSwappedCopy(int i1, int i2, int j1, int j2) {
    int[][] newBlocks = new int[blocks.length][blocks.length];
    for (int i = 0; i < blocks.length; ++i)
      for (int j = 0; j < blocks.length; ++j)
        newBlocks[i][j] = blocks[i][j];
    newBlocks[i1][j1] = blocks[i2][j2];
    newBlocks[i2][j2] = blocks[i1][j1];
    return new Board(newBlocks);
  }

  // Get all valid neighbors
  public Iterable<Board> neighbors() {
    // We just look at the zeros
    List<Board> retval = new ArrayList<Board>(zeroRows.size()*4);
    for (int i = 0; i < zeroRows.size(); ++i) {
      int rowIdx = zeroRows.get(i);
      int colIdx = zeroCols.get(i);
      if (rowIdx > 0)
        retval.add(getSwappedCopy(rowIdx, rowIdx - 1,
          colIdx, colIdx));
      if (colIdx > 0)
        retval.add(getSwappedCopy(rowIdx, rowIdx,
            colIdx, colIdx - 1));
      if (rowIdx < blocks.length - 1)
        retval.add(getSwappedCopy(rowIdx, rowIdx + 1,
          colIdx, colIdx));
      if (colIdx < blocks.length - 1)
        retval.add(getSwappedCopy(rowIdx, rowIdx,
          colIdx, colIdx + 1));
    }
    return retval;
  }

  @Override
  public boolean equals(Object y) {
    if (this == y) return true;
    if (y == null) return false;
    if (y.getClass() != this.getClass()) return false;
    Board that = (Board) y;
    if (that.blocks.length != this.blocks.length) return false;
    for (int i = 0; i < blocks.length; ++i) {
      for (int j = 0; j < blocks[i].length; ++j) {
        if (blocks[i][j] != that.blocks[i][j]) return false;
      }
    }
    return true;
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append(blocks.length + "\n");
    for (int i = 0; i < blocks.length; ++i) {
      sb.append(String.format("%2d", blocks[i][0]));
      for (int j = 1; j < blocks[i].length; ++j)
        sb.append(String.format(" %2d", blocks[i][j]));
      sb.append("\n");
    }
    return sb.toString();
  }

  // Tests
  public static void main(String[] args) {
    int[][] blocks =
      new int[][] {{1, 2, 3}, {4, 5, 6}, {7, 8, 0}};
    Board board1 = new Board(blocks);
    if (board1.dimension() != 3)
      throw new IllegalStateException("Board should have been size 3");
    if (board1.hamming() != 0)
      throw new IllegalStateException("Board should have hamming 0");
    if (board1.manhattan() != 0)
      throw new IllegalStateException("Board should have manhattan 0");
    if (!board1.isGoal())
      throw new IllegalStateException("Board has reached goal");

    blocks = new int[][] {{8, 1, 3}, {4, 0, 2}, {7, 6, 5}};
    Board board2 = new Board(blocks);
    if (board2.dimension() != 3)
      throw new IllegalStateException("Board should have been size 3");
    if (board2.hamming() != 5)
      throw new IllegalStateException("Board should have hamming 5");
    if (board2.manhattan() != 10)
      throw new IllegalStateException("Board should have manhattan 10");
    if (board2.isGoal())
      throw new IllegalStateException("Board has not reached goal");

    Board board3 = board1.twin();
    if (board3.hamming() != 2)
      throw new IllegalStateException("Swapped board should have hamming 2");
    if (board3.manhattan() != 2)
      throw new IllegalStateException("Swapped board should have manhattan 2");

    // Generate new test
    Iterator<Board> board2nit = board2.neighbors().iterator();
    Board board2exp =
      new Board(new int[][] {{8, 0, 3}, {4, 1, 2}, {7, 6, 5}});
    Board board2got = board2nit.next();
    if (!board2got.equals(board2exp))
      throw new IllegalStateException("Got unexpected 1st neighbor: " +
          board2got + " expected " + board2exp + " from " + board2);
    board2exp = new Board(new int[][] {{8, 1, 3}, {0, 4, 2}, {7, 6, 5}});
    if (!board2nit.next().equals(board2exp))
      throw new IllegalStateException("Got unexpected 2nd neighbor");
    board2exp = new Board(new int[][] {{8, 1, 3}, {4, 6, 2}, {7, 0, 5}});
    if (!board2nit.next().equals(board2exp))
      throw new IllegalStateException("Got unexpected 3rd neighbor");
    board2exp = new Board(new int[][] {{8, 1, 3}, {4, 2, 0}, {7, 6, 5}});
    if (!board2nit.next().equals(board2exp))
      throw new IllegalStateException("Got unexpected 4th neighbor");
    if (board2nit.hasNext())
      throw new IllegalStateException("Shouldn't have next");
  }

}
