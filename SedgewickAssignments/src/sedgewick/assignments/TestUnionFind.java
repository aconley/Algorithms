package sedgewick.assignments;

/**
 * Simple tests for Union Find
 */
public class TestUnionFind {

    public static void main(String[] args) {
        UnionFind uf = new UnionFind(10);
        uf.union(4, 3);
        uf.union(3, 8);
        uf.union(6, 5);
        uf.union(9, 4);
        uf.union(2, 1);
        uf.union(8, 9);
        uf.union(5, 0);
        uf.union(7, 2);
        uf.union(6, 1);
        uf.union(1, 0);
        uf.union(6, 7);

        System.out.format("Number of components %d%n", uf.count());
        if (uf.connected(0, 6))
          System.out.format("0 and 6 are connected%n");
        else
          System.out.format("0 and 6 are not connected%n");
    }

}
