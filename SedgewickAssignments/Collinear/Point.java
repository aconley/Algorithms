/*************************************************************************
 *
 * Dependencies: StdDraw.java
 *
 * Description: An immutable data type for points in the plane.
 *
 *************************************************************************/

import java.util.Comparator;

public class Point implements Comparable<Point> {

    // compare points by slope
    public final Comparator<Point> SLOPE_ORDER;       // YOUR DEFINITION HERE

    private final int x;                              // x coordinate
    private final int y;                              // y coordinate

    // create the point (x, y)
    public Point(int x, int y) {
        this.x = x;
        this.y = y;
        SLOPE_ORDER = new SlopeOrder();
    }

    // plot this point to standard drawing
    public void draw() {
        StdDraw.point(x, y);
    }

    // draw line between this point and that point to standard drawing
    public void drawTo(Point that) {
        StdDraw.line(this.x, this.y, that.x, that.y);
    }

    // slope between this point and that point
    public double slopeTo(Point that) {
        if (this == that)
            return Double.NEGATIVE_INFINITY;
        if (this.y == that.y) {
            if (this.x == that.x)
                return Double.NEGATIVE_INFINITY;
            else
                return +0.0;
        }
        if (this.x == that.x)
            return Double.POSITIVE_INFINITY;
        return (that.y - this.y) / ((double) (that.x - this.x));
    }

    private int intCmp(int a, int b) {
        return a - b;
    }

    private int dblCmp(double a, double b) {
        if (a > b)
            return 1;
        else if (a < b)
            return -1;
        else
            return 0;
    }

    // is this point lexicographically smaller than that one?
    // comparing y-coordinates and breaking ties by x-coordinates
    public int compareTo(Point that) {
        if (this == that)
            return 0;
        int cmpY = intCmp(this.y, that.y);
        return (cmpY == 0) ? intCmp(this.x, that.x) : cmpY;
    }

    // return string representation of this point
    public String toString() {
        /* DO NOT MODIFY */
        return "(" + x + ", " + y + ")";
    }

    private class SlopeOrder implements Comparator<Point> {
        public int compare(Point p1, Point p2) {
            double s1 = slopeTo(p1);
            double s2 = slopeTo(p2);
            return dblCmp(s1, s2);
        }
    }

    // unit test
    public static void main(String[] args) {
        Point p1, p2;

        System.out.println("Doing point tests");

        // Horizontal test
        p1 = new Point(-5, 20);
        p2 = new Point(15, 20);
        assert p1.compareTo(p2) < 0 : "p1 should be less than p2";
        assert p1.compareTo(p1) == 0 : "p1 should equal p2";
        assert p2.compareTo(p1) > 0 : "p1 should be greater than p2";
        assert p2.compareTo(p2) == 0 : "p2 should equal itself";
        assert p1.slopeTo(p2) == +0.0 : "horizontal slope should be 0";
        assert p2.slopeTo(p1) == +0.0 : "horizontal slope shoudl be 0";
        assert p1.slopeTo(p1) == Double.NEGATIVE_INFINITY : "slope to self == -\\inf";
        assert p2.slopeTo(p2) == Double.NEGATIVE_INFINITY : "slope to self == -\\inf";

        // Vertical
        p1 = new Point(3, 7);
        p2 = new Point(3, 1);
        assert p1.compareTo(p2) > 0 : "p1 should be > p2";
        assert p2.compareTo(p1) < 0 : "p2 should be < p1";
        assert p1.slopeTo(p2) == Double.POSITIVE_INFINITY : "slope(p1, p2) == \\inf";
        assert p2.slopeTo(p1) == Double.POSITIVE_INFINITY : "slope(p2, p1) == \\inf";

        // General
        p1 = new Point(0, 0);
        p2 = new Point(3, 3);
        assert p2.compareTo(p1) > 0 : "p2 should be > p1";
        assert p1.slopeTo(p2) == 1.0 : "slope(p1, p2) should be 1";
        assert p2.slopeTo(p1) == -1.0 : "slope(p2, p1) should be -1";
    }
}
