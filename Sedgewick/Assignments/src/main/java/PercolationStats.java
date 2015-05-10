package sedgewick.assignments;

import java.lang.Math;
import java.util.Random;

public class PercolationStats {
    private Random rand; // Random number generator
    private int n; // Size of grid being tests
    private int nsims; // Number of sims to do
    private double[] perc; // Point at which percolation was achieved in each sim

    /**
     * Perform NSIMS percolation experiments on a N x N grid
     * @param N Size of grid along one dimension
     * @param NSIMS Number of simulatoins to do
     * @throws java.lang.IllegalArgumentException On invalid N or NSIMS
     */
    public PercolationStats(int N, int NSIMS) {
        if (N <= 0)
            throw new IllegalArgumentException("Illegal (non-positive) N");
        if (NSIMS <= 0)
            throw new IllegalArgumentException("Illegal (non-positive) NSIMS");
        n = N;
        nsims = NSIMS;
        rand = new Random();

        perc = new double[nsims];
        for (int i = 0; i < nsims; i++)
            perc[i] = doSim();
    }

    /**
     * Simulate percolation
     * @return Return the number of sites opened before percolation achieved
     */
    private double doSim() {
        Percolation percolator = new Percolation(n);
        int nopen; // Number of sites opened
        int i, j;
        nopen = 0;
        while (!percolator.percolates()) {
            // Loop to find open site; a little scary
            while (true) {
                i = rand.nextInt(n - 1) + 1;  // [1, N]
                j = rand.nextInt(n - 1) + 1;
                if (!percolator.isOpen(i, j)) {
                    percolator.open(i, j);
                    nopen += 1;
                    break;
                }
            }
        }
        return ((double) nopen) / ((double) (n * n));
    }

    /**
     * Get mean percolation threshold
     * @return Mean percolation threshold
     */
    public double mean() {
        double sum = 0;
        for (double v : perc) sum += v;
        return sum / ((double) nsims);
    }

    /**
     * Get standard deviation of percolation threshold
     * @return Sqrt of second central moment
     */
    public double stddev() {
        double sumsq = 0;
        double mn = mean();
        for (double v : perc) sumsq += (v - mn) * (v - mn);
        return Math.sqrt(sumsq);
    }

    public double confidenceLo() {
        // Approximate as Gaussian; should really sort and do this properly
        return mean() - (1.96*stddev() / Math.sqrt(nsims));
    }

    public double confidenceHi() {
        return mean() + (1.96*stddev() / Math.sqrt(nsims));
    }

    public static void main(String[] args) {
        int N = 20;
        int NSIMS = 2000;
        if (args.length == 2) {
            N = Integer.parseInt(args[0]);
            NSIMS = Integer.parseInt(args[1]);
        }
        PercolationStats stats = new PercolationStats(N, NSIMS);
        System.out.println("mean =\t\t\t" + stats.mean());
        System.out.println("stddev = \t\t\t" + stats.stddev());
        System.out.println("95% confidence interval = \t" + stats.confidenceLo()
                + ", " + stats.confidenceHi());
    }
}