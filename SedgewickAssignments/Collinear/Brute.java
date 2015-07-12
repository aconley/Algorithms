import java.util.Arrays;

/**
 * Class to find sets of 4 collinear points using brute
 * force methods
 */
public class Brute {
  private int n; // Number of points
  private Point[] points; // Array of points

  public Brute(String inFile) {
    In inputFile = new In(inFile);
    n = inputFile.readInt();
    points = new Point[n];

    int x, y;
    for (int i = 0; i < n; ++i) {
      x = inputFile.readInt();
      y = inputFile.readInt();
      points[i] = new Point(x, y);
    }

    Arrays.sort(points);
  }

  public void drawPoints() {
    StdDraw.setXscale(0, 32768);
    StdDraw.setYscale(0, 32768);
    for (Point p : points)
      p.draw();
  }

  public void findFourCollinear() {
    findFourCollinear(false);
  }

  // Assumes no points are at the same coordinates
  public void findFourCollinear(boolean draw) {
    Point initPoint;
    for (int i = 0; i < n; ++i) {
      initPoint = points[i];
      for (int j = i + 1; j < n; ++j) {
        double currSlope = initPoint.slopeTo(points[j]);
        for (int k = j + 1; k < n; ++k) {
          if (initPoint.slopeTo(points[k]) == currSlope) {
            for (int m = k + 1; m < n; ++m) {
              if (initPoint.slopeTo(points[m]) == currSlope) {
                if (draw)
                  points[i].drawTo(points[m]);

                StdOut.println(points[i] + " -> " + points[j] + " -> " +
                  points[k] + " -> " + points[m]);
              }
            }
          }
        }
      }
    }
  }

  public static void main(String[] args) {
    if (args.length < 1)
      throw new IllegalArgumentException("No input file provided");

    Brute colFinder = new Brute(args[0]);
    colFinder.drawPoints();
    colFinder.findFourCollinear(true);
  }
}
