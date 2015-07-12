import java.lang.Math;
import java.util.Arrays;

/**
 * Class to find sets of 4 collinear points using
 * sorting methods
 */
public class Fast {
  private int n; // Number of points
  private Point[] points; // Array of points
  private int[] used; //

  public Fast(String inFile) {
    In inputFile = new In(inFile);
    n = inputFile.readInt();
    points = new Point[n];

    int x, y;
    for (int i = 0; i < n; ++i) {
      x = inputFile.readInt();
      y = inputFile.readInt();
      points[i] = new Point(x, y);
    }

    // Recall -- this puts them in ascending y order
    //  (with ties broken by y).
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
    Point[] angleSortedPoints = new Point[n];

    Point initPoint;
    for (int i = 0; i <= n-3; ++i) {
      initPoint = points[i];
      // In terms of finding collinear points, we only
      //  need to consider points above this one.  However,
      //  if we don't want to allow sub-segments to appear,
      //  we need to consder them all.  If we do that
      //  then the requirement to put out a segment is that
      //  it start with initPoint in addition to being long enough.
      // In order for this to work, we need to
      //  1) recopy from the original to get them back in 'natural' order
      //  2) Then sort all the points by angle w.r.t. p.
      // We are taking advantage of the stability of Arrays.sort here;
      //  this means that all the collinear points will be in the
      //  same order each time, so that to avoid outputting sub-segements
      //  we only need check each purported line starts with initPoint.
      System.arraycopy(points, 0, angleSortedPoints, 0, n);
      Arrays.sort(angleSortedPoints, initPoint.SLOPE_ORDER);

      // Now look for any 3 successive points with the same slope
      //  those plus initPoint are a segment
      int lowIdx = 0;
      int highIdx = 0;
      double startSlope = initPoint.slopeTo(angleSortedPoints[0]);
      for (int j = 1; j < n; ++j) {
        double currSlope = initPoint.slopeTo(angleSortedPoints[j]);
        if (currSlope == startSlope) {
          highIdx += 1;
        } else {
          // Found one with a different slope
          //  First -- see if we have more than 3 collinear points
          //  saved up that have not been previously used.
          //  If so, output that as a line segment.
          //   The previously used test amounts to ensuring that
          //   initPoint is less than the first point in the rest
          //   of the segment.
          //  Note that because Arrays.sort is stable they will
          //  still be in ascending y order (since they all have
          //   the same slope w.r.t. initPoint)
          if ((highIdx - lowIdx >= 2) &&
            (initPoint.compareTo(angleSortedPoints[lowIdx]) < 0)) {

            // Found a line
            // print it
            StdOut.print(initPoint);
            for (int k = lowIdx; k <= highIdx; ++k)
              StdOut.print(" -> " + angleSortedPoints[k]);
            StdOut.println();

            //Draw it
            if (draw)
              initPoint.drawTo(angleSortedPoints[highIdx]);
          }

          // Either way we got here (found and printed a line,
          //  found a different slope before getting 4 collinear)
          //  time to reset the successive count
          lowIdx = j;
          highIdx = j;
          startSlope = currSlope;
        }
      }

      // Have to handle the last points
      if ((highIdx - lowIdx >= 2) &&
        (initPoint.compareTo(angleSortedPoints[lowIdx]) < 0)) {
        StdOut.print(initPoint);
        for (int k = lowIdx; k <= highIdx; ++k)
          StdOut.print(" -> " + angleSortedPoints[k]);
        StdOut.println();
        if (draw)
          initPoint.drawTo(angleSortedPoints[highIdx]);
      }
    }
  }

  public static void main(String[] args) {
    if (args.length < 1)
      throw new IllegalArgumentException("No input file provided");

    Fast colFinder = new Fast(args[0]);
    //colFinder.drawPoints();
    colFinder.findFourCollinear(false);
  }
}
