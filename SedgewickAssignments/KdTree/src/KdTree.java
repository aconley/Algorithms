import java.util.ArrayList;
import java.util.List;

/**
 * 2D Kd-tree
 */
public class KdTree {
  private static final double MINVAL = 0.0;
  private static final double MAXVAL = 1.0;

  // Which direction each split is
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
    private int size; // Number of points below
    private Direction dir;

    private RectHV rect; // Used purely for drawing

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
      this.size = 1;
      if (this.left != null) this.size += this.left.size;
      if (this.right != null) this.size += this.right.size;
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

  public KdTree() {
    rootNode = null;
  }

  public boolean isEmpty() {
    return rootNode == null;
  }

  public int size() {
    if (isEmpty())
      return 0;
    else
      return rootNode.size;
  }

  // Now the hard stuff...
  public void insert(Point2D p) {
    if (p == null)
      throw new NullPointerException("Point is null");
    if (rootNode == null) {
      RectHV baseRect = new RectHV(MINVAL, MINVAL, MAXVAL, MAXVAL);
      rootNode = new Node(p, Direction.HORIZONTAL, baseRect);
    } else {
      rootNode = insert(p, rootNode);
    }
  }

  // Insertion -not- at the root
  private Node insert(Point2D p, Node x) {
    if (x.point.equals(p)) // Point already present
      return x; // Note that we don't use compareTo -- we want equality both dims
    if (x.compareToPoint(p) > 0) {
      // Insert to the left -- smaller x or smaller y, depending on dir
      if (x.left == null) {
        // Special case so we can set enclosing rect
        RectHV newRect;
        if (x.dir == Direction.VERTICAL) {
          newRect = new RectHV(x.rect.xmin(), x.rect.ymin(),
              x.point.x(), x.rect.ymax());
        } else {
          newRect = new RectHV(x.rect.xmin(), x.rect.ymin(),
              x.rect.xmax(), x.point.y());
        }
        x.left = new Node(p, x.dir.otherDirection(), newRect);
      } else {
        x.left = insert(p, x.left);
      }
    } else {
      // And to the right -- larger x or y
      if (x.right == null) {
        RectHV newRect;
        if (x.dir == Direction.VERTICAL) {
          newRect = new RectHV(x.point.x(), x.rect.ymin(),
              x.rect.xmax(), x.rect.ymax());
        } else {
          newRect = new RectHV(x.rect.xmin(), x.point.y(),
              x.rect.xmax(), x.rect.ymax());
        }
        x.right = new Node(p, x.dir.otherDirection(), newRect);
      } else {
        x.right = insert(p, x.right);
      }
    }
    return x;
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
      StdDraw.setPenColor(StdDraw.BLUE);
      StdDraw.line(x.rect.xmin(), x.point.y(), x.rect.xmax(), x.point.y());
    } else {
      StdDraw.setPenColor(StdDraw.RED);
      StdDraw.line(x.point.x(), x.rect.ymin(), x.point.x(), x.rect.ymax());
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

}
