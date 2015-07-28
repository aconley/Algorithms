import java.util.ArrayList;
import java.util.Set;
import java.util.TreeSet;

/**
 * Brute force method
 */
public class PointSET {

  private final Set<Point2D> points;

  public PointSET() {
    points = new TreeSet<Point2D>();
  }

  public boolean isEmpty() {
    return points.isEmpty();
  }

  public int size() {
    return points.size();
  }

  public void insert(Point2D p) {
    if (p == null)
      throw new NullPointerException("Input point was null");
    points.add(p);
  }

  public boolean contains(Point2D p) {
    if (p == null)
      throw new NullPointerException("Input point was null");
    return points.contains(p);
  }

  public void draw() {
    for (Point2D p : points)
      p.draw();
  }

  public Iterable<Point2D> range(RectHV rect) {
    if (rect == null)
      throw new NullPointerException("Input rect was null");
    ArrayList<Point2D> retval = new ArrayList<>();
    for (Point2D p : points) {
      if (rect.contains(p))
        retval.add(p);
    }
    return retval;
  }

  public Point2D nearest(Point2D p) {
    if (p == null)
      throw new NullPointerException("Input point was null");
    if (isEmpty())
      return null;

    Point2D retval = null;
    double minDistSq = Double.MAX_VALUE;
    for (Point2D q : points) {
      double currDistSq = p.distanceSquaredTo(q);
      if (currDistSq < minDistSq) {
        retval = q;
        minDistSq = currDistSq;
      }
    }
    return retval;
  }
}
