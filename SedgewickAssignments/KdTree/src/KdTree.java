import java.util.ArrayList;
import java.util.List;

/**
 * 2D Kd-tree
 */
public class KdTree {
  private static final double MINVAL = 0.0;
  private static final double MAXVAL = 1.0;

  // Which direction each split is
  //  So if horizontal, compare is on x, if VERT on y
  private enum Direction {
    HORIZONTAL, VERTICAL;

    public Direction otherDirection() {
      if (this.equals(HORIZONTAL))
        return VERTICAL;
      else
        return HORIZONTAL;
    }
  }

  private static class Node {
    private Node left;
    private Node right;
    private Point2D point;
    private Direction dir;

    private RectHV rect; // Enclosing rectangle

    public Node(Point2D point, Direction dir, RectHV rect) {
      this(point, dir, rect, null, null);
    }

    public Node(Point2D point, Direction dir, RectHV rect,
                Node left, Node right) {
      if (rect == null)
        throw new NullPointerException("rect was null");
      if (point == null)
        throw new NullPointerException("point was null");
      this.left = left;
      this.right = right;
      this.rect = rect;
      this.point = point;
      this.dir = dir;
    }

    public int compareToPoint(Point2D other) {
      if (other == null)
        throw new NullPointerException("other was null");
      if (dir == Direction.HORIZONTAL) {
        return Double.compare(this.point.x(), other.x());
      } else {
        return Double.compare(this.point.y(), other.y());
      }
    }

    public int compareToRect(RectHV rectHV) {
      if (rectHV == null)
        throw new NullPointerException("rectHV was null");
      if (dir == Direction.HORIZONTAL) {
        double x = this.point.x();
        if (x > rectHV.xmax())
          return 1;
        else if (x < rectHV.xmin())
          return -1;
        else
          return 0;
      } else {
        double y = this.point.y();
        if (y > rectHV.ymax())
          return 1;
        else if (y < rectHV.ymin())
          return -1;
        else
          return 0;
      }
    }
  }

  // Actual representation
  private Node rootNode;
  private int nNodes;

  public KdTree() {
    rootNode = null;
    nNodes = 0;
  }

  public boolean isEmpty() {
    return nNodes == 0;
  }

  public int size() {
    return nNodes;
  }

  // Now the hard stuff...
  public void insert(Point2D p) {
    if (p == null)
      throw new NullPointerException("Point is null");
    if (rootNode == null) {
      RectHV baseRect = new RectHV(MINVAL, MINVAL, MAXVAL, MAXVAL);
      rootNode = new Node(p, Direction.HORIZONTAL, baseRect);
      nNodes = 1;
    } else {
      if (insert(p, rootNode))
        nNodes += 1;
    }
  }

  // Insertion -not- at the root
  //  Returns true if actually inserted
  private boolean insert(Point2D p, Node h) {
    if (h.point.equals(p)) // Point already present
      return false; // Note that we don't use compareTo -- we want equality both dims
    if (h.compareToPoint(p) > 0) {
      // Insert to the left -- smaller x or smaller y, depending on dir (HOR, VERT)
      if (h.left == null) {
        // Special case so we can set enclosing rect
        RectHV newRect;
        if (h.dir == Direction.VERTICAL) {
          // Compared on y
          newRect = new RectHV(h.rect.xmin(), h.rect.ymin(),
              h.rect.xmax(), h.point.y());
        } else {
          // Compared on x
          newRect = new RectHV(h.rect.xmin(), h.rect.ymin(),
              h.point.x(), h.rect.ymax());
        }
        h.left = new Node(p, h.dir.otherDirection(), newRect);
        return true;
      } else {
        return insert(p, h.left);
      }
    } else {
      // And to the right -- larger x or y for HOR, VERT
      if (h.right == null) {
        RectHV newRect;
        if (h.dir == Direction.VERTICAL) {
          newRect = new RectHV(h.rect.xmin(), h.point.y(),
              h.rect.xmax(), h.rect.ymax());
        } else {
          newRect = new RectHV(h.point.x(), h.rect.ymin(),
              h.rect.xmax(), h.rect.ymax());
        }
        h.right = new Node(p, h.dir.otherDirection(), newRect);
        return true;
      } else {
        return insert(p, h.right);
      }
    }
  }

  public boolean contains(Point2D p) {
    if (p == null)
      throw new NullPointerException("Input point is null");
    return contains(p, rootNode);
  }

  private boolean contains(Point2D p, Node x) {
    if (x == null)
      return false;
    if (x.point.equals(p))
      return true;
    if (x.compareToPoint(p) > 0) {
      return contains(p, x.left);
    } else {
      return contains(p, x.right);
    }
  }

  public void draw() {
    if (rootNode == null) {
      StdDraw.setPenColor(StdDraw.BLACK);
      StdDraw.setPenRadius(0.01);
      RectHV rect = new RectHV(MINVAL, MINVAL, MAXVAL, MAXVAL);
      rect.draw();
    } else {
      draw(rootNode);
    }
  }

  private void draw(Node x) {
    if (x == null)
      return;

    // Draw the point
    StdDraw.setPenColor(StdDraw.BLACK);
    StdDraw.setPenRadius(0.01);
    x.point.draw();

    // And the dividing line
    StdDraw.setPenRadius(0.003);
    if (x.dir == Direction.HORIZONTAL) {
      // This is actually a vertical line... the split is on x
      StdDraw.setPenColor(StdDraw.RED);
      StdDraw.line(x.point.x(), x.rect.ymin(), x.point.x(), x.rect.ymax());
    } else {
      StdDraw.setPenColor(StdDraw.BLUE);
      StdDraw.line(x.rect.xmin(), x.point.y(), x.rect.xmax(), x.point.y());
    }
    draw(x.left);
    draw(x.right);
  }

  public Iterable<Point2D> range(RectHV rect) {
    List<Point2D> ret = new ArrayList<>();

    if (!isEmpty()) {
      range(rect, rootNode, ret);
    }
    return ret;
  }

  private void range(RectHV rect, Node x, List<Point2D> ret) {
    if (x == null)
      return;

    if (!rect.intersects(x.rect)) {
      return;
    }

    if (rect.contains(x.point)) {
      ret.add(x.point);
    }

    range(rect, x.left, ret);
    range(rect, x.right, ret);
  }

  private static class PointDist {
    public Point2D point;
    public double distSq;

    public PointDist(Point2D p, double distSq) {
      this.point = p;
      this.distSq = distSq;
    }
  }

  public Point2D nearest(Point2D p) {
    if (p == null)
      throw new NullPointerException("Point was null");
    if (rootNode == null)
      return null;
    PointDist currBest = new PointDist(null, Double.MAX_VALUE);
    nearest(p, rootNode, currBest);
    return currBest.point;
  }

  private void nearest(Point2D p, Node h, PointDist currBest) {
    double distSq = h.point.distanceSquaredTo(p);
    if (distSq < currBest.distSq) {
      currBest.point = h.point;
      currBest.distSq = distSq;
    }

    if (h.left == null) {
      if (h.right == null) {
        return;
      } else {
        // right only
        double rightDistSq = h.right.rect.distanceSquaredTo(p);
        if (rightDistSq < currBest.distSq)
          nearest(p, h.right, currBest);
      }
    } else {
      if (h.right == null) {
        // Left only
        double leftDistSq = h.left.rect.distanceSquaredTo(p);
        if (leftDistSq < currBest.distSq)
          nearest(p, h.left, currBest);
      } else {
        // We have both; search most likely one first
        double leftDistSq = h.left.rect.distanceSquaredTo(p);
        double rightDistSq = h.right.rect.distanceSquaredTo(p);
        if (leftDistSq < rightDistSq) {
          if (leftDistSq < currBest.distSq)
            nearest(p, h.left, currBest);
          if (rightDistSq < currBest.distSq)
            nearest(p, h.right, currBest);
        } else {
          if (rightDistSq < currBest.distSq)
            nearest(p, h.right, currBest);
          if (leftDistSq < currBest.distSq)
            nearest(p, h.left, currBest);
        }
      }
    }
  }
}
