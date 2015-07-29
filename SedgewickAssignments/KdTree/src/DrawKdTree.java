/**
 * Created by aconley on 7/28/15.
 */
public class DrawKdTree {
  public static void main(String[] args) {
    String filename = args[0];
    In in = new In(filename);

    StdDraw.show(0);

    // initialize the two data structures with point from standard input
    KdTree kdtree = new KdTree();
    while (!in.isEmpty()) {
      double x = in.readDouble();
      double y = in.readDouble();
      Point2D p = new Point2D(x, y);
      kdtree.insert(p);
    }

    StdDraw.clear();
    kdtree.draw();
    StdDraw.show();
  }
}
