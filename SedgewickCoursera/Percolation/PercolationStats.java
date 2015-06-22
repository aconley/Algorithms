public class PercolationStats {
    private int n; // Size of grid being tests
    private int nsims; // Number of sims to do
    private double[] perc; // Point at which percolation was achieved in each sim

    /**
     * Perform NSIMS percolation experiments on a N x N grid
     * @param N Size of grid along one dimension
     * @param T Number of simulations to do
     * @throws java.lang.IllegalArgumentException On invalid N or NSIMS
     */
    public PercolationStats(int N, int T) {
      if (N <= 0)
        throw new IllegalArgumentException("Illegal (non-positive) N");
      if (T <= 0)
        throw new IllegalArgumentException("Illegal (non-positive) T");
      n = N;
      nsims = T;

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
                i = StdRandom.uniform(1, n + 1);
                j = StdRandom.uniform(1, n + 1);
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
      if (nsims == 1) return Double.NaN;
      double mn = mean();
      double sumsq = 0;
      for (double v : perc) sumsq += (v - mn) * (v - mn);
      return Math.sqrt(sumsq / (nsims - 1));
    }

    public double confidenceLo() {
      // Approximate as Gaussian; should really sort and do this properly
      return mean() - (1.96 * stddev() / Math.sqrt(nsims));
    }

    public double confidenceHi() {
      return mean() + (1.96 * stddev() / Math.sqrt(nsims));
    }

    public static void main(String[] args) {
      int n = Integer.parseInt(args[0]);
      int nsims = Integer.parseInt(args[1]);
      PercolationStats ps = new PercolationStats(n, nsims);

      StdOut.println("mean                    = " + ps.mean());
      StdOut.println("stddev                  = " + ps.stddev());
      StdOut.println("95% confidence interval = " + ps.confidenceLo() +
        ", " + ps.confidenceHi());
    }
  }
