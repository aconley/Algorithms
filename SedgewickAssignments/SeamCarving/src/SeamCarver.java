import edu.princeton.cs.algs4.Picture;
import edu.princeton.cs.algs4.StdOut;

import java.awt.*;

/**
 *
 */
public class SeamCarver {

  // Masks for extracting individual colors
  private static int BLUE_MASK = 0x000000ff;
  private static int GREEN_MASK = 0x0000ff00;
  private static int RED_MASK = 0x00ff0000;

  private static double ENERGY_BOUNDARY = 1000.0;

  // Internal representation of picture;
  //  a flat array of integers, with 8-bit rgb values bitpacked
  private final int logicalHeight; // array size and indexing
  private final int logicalWidth;
  private final int[] image;

  // Energy array
  private final double[] energy;

  // Current height/width
  private int height;
  private int width;

  // Distance and from arrays
  private final double[] distTo;
  private final int[] edgeTo;

  private static int getRed(int value) {
    return (value & RED_MASK) >> 16;
  }

  private static int getGreen(int value) {
    return (value & GREEN_MASK) >> 8;
  }

  private static int getBlue(int value) {
    return (value & BLUE_MASK);
  }

  private static int packColor(int r, int g, int b) {
    return b | (g << 8) | (r << 16);
  };

  public SeamCarver(Picture picture) {
    if (picture == null) {
      throw new NullPointerException("Picture is null");
    }
    this.logicalHeight = picture.height();
    this.logicalWidth = picture.width();
    this.height = this.logicalHeight;
    this.width = this.logicalWidth;

    image = new int[this.logicalHeight * this.logicalWidth];
    for (int i = 0; i < this.logicalWidth; ++i) {
      for (int j = 0; j < this.logicalHeight; ++j) {
        Color c = picture.get(i, j);
        image[i * height + j] =
            packColor(c.getRed(), c.getGreen(), c.getBlue());
      }
    }

    // Pre-compute energy^2
    energy = new double[this.logicalHeight * this.logicalWidth];
    for (int j = 0; j < this.logicalHeight; ++j) {
      energy[j] = ENERGY_BOUNDARY;
    }
    for (int i = 1; i < this.logicalWidth - 1; ++i) {
      int baseIdx = i * this.logicalHeight;
      energy[baseIdx] = ENERGY_BOUNDARY;
      for (int j = 1; j < this.logicalHeight - 1; ++j) {
        int centIdx = baseIdx + j;
        int energySquared =
            getDeltaSqVals(image[centIdx + logicalHeight],
                           image[centIdx - logicalHeight])
            + getDeltaSqVals(image[centIdx + 1], image[centIdx - 1]);
        energy[centIdx] = Math.sqrt((double) energySquared);

      }
      energy[baseIdx + this.logicalHeight - 1] = ENERGY_BOUNDARY;
    }
    int baseIdx = (this.logicalWidth - 1) * this.logicalHeight;
    for (int j = 0; j < this.logicalHeight; ++j) {
      energy[baseIdx + j] = ENERGY_BOUNDARY;
    }

    distTo = new double[this.logicalWidth * this.logicalHeight];
    edgeTo = new int[this.logicalWidth * this.logicalHeight];
  }

  public int width() {
    return this.width;
  }

  public int height() {
    return this.height;
  }

  public Picture picture() {
    Picture retval = new Picture(width, height);
    for (int i = 0; i < this.width; ++i) {
      for (int j = 0; j < this.height; ++j) {
        retval.set(i, j, new Color(image[i * logicalHeight + j]));
      }
    }
    return retval;
  }

  public double energy(int x, int y) {
    if (x < 0 || x >= this.width) {
      throw new IndexOutOfBoundsException("x out of bounds");
    }
    if (y < 0 || y >= this.height) {
      throw new IndexOutOfBoundsException("y out of bounds");
    }
    return energy[x * logicalHeight + y];
  }

  private static int getDeltaSqVals(int imgp1, int imgm1) {
    int dr = getRed(imgp1) - getRed(imgm1);
    int dg = getGreen(imgp1) - getGreen(imgm1);
    int db = getBlue(imgp1) - getBlue(imgm1);
    return dr * dr + dg * dg + db * db;
  }

  public int[] findHorizontalSeam() {
    // Initialize distances, etc.

    // Just point straight back for first column
    for (int j = this.logicalHeight; j <this.logicalHeight + this.height; ++j) {
      edgeTo[j] = j - this.logicalHeight;
    }

    // We can happily ignore the first column, since they
    //  all have the same values.
    for (int j = this.logicalHeight; j < this.logicalHeight + this.height; ++j) {
      distTo[j] = energy[j];
    }
    // Max all other distances.
    for (int i = 2; i < this.width; ++i) {
      int baseIdx = i * this.logicalHeight;
      for (int j = 0; j < this.height; ++j) {
        distTo[baseIdx + j] = Double.POSITIVE_INFINITY;
      }
    }

    int baseIdx, nextRowIdx, currBase, currNext;
    for (int i = 1; i < this.width - 1; ++i) {
      baseIdx = i * this.logicalHeight;
      nextRowIdx = baseIdx + this.logicalHeight;
      // We don't need to do j = 0 or j = height - 1 because
      //  no seam will ever go through the edge because
      //  the energy is set extremely high there.
      // But j = 1, j = this.height-1 deserve special treatment
      //  since we can ignore going down/up from there
      currBase = baseIdx + 1;
      currNext = nextRowIdx + 1;
      relax(currBase, currNext + 1);
      relax(currBase, currNext);
      for (int j = 2; j < this.height - 2; ++j) {
        currBase = baseIdx + j;
        currNext = nextRowIdx + j;
        relax(currBase, currNext + 1);
        relax(currBase, currNext);
        relax(currBase, currNext - 1);
      }
      currBase = baseIdx + this.height - 2;
      currNext = nextRowIdx + this.height - 2;
      relax(currBase, currNext);
      relax(currBase, currNext - 1);
    }

    // Now find the best one
    int endOfSeam = getBestHorizontalSeam();

    // Extract the seam from that
    return getHorizontalSeam(endOfSeam);
  }

  private void relax(int from, int to) {
    double thisEnergy = distTo[from] + energy[to];
    if (distTo[to] > thisEnergy) {
      distTo[to] = thisEnergy;
      edgeTo[to] = from;
    }
  }

  private int getBestHorizontalSeam() {
    int basePtr = (this.logicalWidth - 1) * this.logicalHeight;
    int endIdx = 1;
    double bestCost = distTo[basePtr + endIdx];
    double thisCost;
    for (int i = 2; i < this.height-1; ++i) {
      thisCost = distTo[basePtr + i];
      if (thisCost < bestCost) {
        endIdx = i;
        bestCost = thisCost;
      }
    }
    return endIdx;
  }

  private int[] getHorizontalSeam(int end) {
    int[] result = new int[width];
    result[width - 1] = end;
    int fullIdx = (this.width - 1) * this.logicalHeight + end;
    for (int i = width - 2; i >= 0; --i) {
      fullIdx = edgeTo[fullIdx];
      result[i] = fullIdx % this.logicalHeight;
    }
    return result;
  }

  public void removeHorizontalSeam(int[] seam) {
    if (seam == null) {
      throw new NullPointerException("Seam is null");
    }
    if (seam.length != width) {
      throw new IllegalArgumentException("Seam is not correct length");
    }
    if (height == 0) {
      throw new IllegalArgumentException("Tried to remove seam from empty image");
    }

    // Adjust the picture and energy
    for (int i = 0; i < this.width; ++i) {
      int seamIdx = seam[i];
      if (seamIdx < 0 || seamIdx >= this.height) {
        throw new IndexOutOfBoundsException("Seam value: " + seamIdx
            + " out of bounds");
      }
      // Picture
      int seamFullIdx = i * this.logicalHeight + seamIdx;
      if (seamIdx < this.height - 1) {
        System.arraycopy(image, seamFullIdx + 1, image, seamFullIdx,
            height - seamIdx - 1);
      }
    }
    // Energy.  Only elements +- 1 in y need to be adjusted
    //  which, after removal, means the element and the one below it
    // Note: we need to do this in a separate loop
    for (int i = 1; i < this.width-1; ++i) {
      int seamIdx = seam[i];
      int seamFullIdx = i * this.logicalHeight + seamIdx;

      // Do the seam position
      if (seamIdx == 0) {
        energy[seamFullIdx] = ENERGY_BOUNDARY;
      } else if (seamIdx < this.height - 1) {
        int energySquared =
            getDeltaSqVals(image[seamFullIdx + logicalHeight],
                image[seamFullIdx - logicalHeight])
            + getDeltaSqVals(image[seamFullIdx + 1],
                  image[seamFullIdx - 1]);
        energy[seamFullIdx] = Math.sqrt((double) energySquared);
      }
      // And the one below
      if (seamIdx > 1) {
        int energySquared =
            getDeltaSqVals(image[seamFullIdx-1 + logicalHeight],
                image[seamFullIdx-1 - logicalHeight])
                + getDeltaSqVals(image[seamFullIdx],
                image[seamFullIdx - 2]);
        energy[seamFullIdx-1] = Math.sqrt((double) energySquared);
      }
    }

    this.height -= 1;
  }

  public int[] findVerticalSeam() {
    // Just point straight up for first row
    int prod;
    for (int i = 0; i < this.width; ++i) {
      prod = i * this.logicalHeight;
      edgeTo[prod + 1] = prod;
    }

    // We can happily ignore the first column, since they
    //  all have the same values.
    for (int i = 0; i < this.width; ++i) {
      prod = i * this.logicalHeight + 1;
      distTo[prod] = energy[prod];
    }

    // Max all other distances.
    for (int i = 0; i < this.width; ++i) {
      int baseIdx = i * this.logicalHeight;
      for (int j = 2; j < this.height; ++j) {
        distTo[baseIdx + j] = Double.POSITIVE_INFINITY;
      }
    }

    int baseIdx, nextRowIdx, prevRowIdx;
    for (int j = 1; j < this.height - 1; ++j) {
      // i = 1 case
      baseIdx = this.logicalHeight + j;
      nextRowIdx = baseIdx + this.logicalHeight;
      prevRowIdx = baseIdx - this.logicalHeight;
      relax(baseIdx, baseIdx + 1);
      relax(baseIdx, nextRowIdx + 1);
      // core
      for (int i = 2; i < this.width - 2; ++i) {
        baseIdx += this.logicalHeight;
        nextRowIdx += this.logicalHeight;
        prevRowIdx += this.logicalHeight;
        relax(baseIdx, prevRowIdx + 1);
        relax(baseIdx, baseIdx + 1);
        relax(baseIdx, nextRowIdx + 1);
      }
      // i = this.width - 1
      baseIdx += this.logicalHeight;
      prevRowIdx += this.logicalHeight;
      relax(baseIdx, prevRowIdx + 1);
      relax(baseIdx, baseIdx + 1);
    }

    // Now find the best one
    int endOfSeam = getBestVerticalSeam();

    // Extract the seam from that
    return getVerticalSeam(endOfSeam);
  }

  private int getBestVerticalSeam() {
    int endIdx = 1;
    int baseIdx = 2 * this.logicalHeight - 1;
    double bestCost = distTo[baseIdx];
    double thisCost;
    for (int i = 2; i < this.width-1; ++i) {
      baseIdx += this.logicalHeight;
      thisCost = distTo[baseIdx];
      if (thisCost < bestCost) {
        endIdx = i;
        bestCost = thisCost;
      }
    }
    return endIdx;
  }

  private int[] getVerticalSeam(int end) {
    int[] result = new int[height];
    result[height - 1] = end;
    int fullIdx = end * this.logicalHeight + this.height - 1; // [end, height-1]
    for (int j = height - 2; j >= 0; --j) {
      fullIdx = edgeTo[fullIdx];
      result[j] = fullIdx / this.logicalHeight;
    }
    return result;
  }

  public void removeVerticalSeam(int[] seam) {
    if (seam == null) {
      throw new NullPointerException("Seam is null");
    }
    if (seam.length != height) {
      throw new IllegalArgumentException("Seam is not correct length");
    }
    if (width == 0) {
      throw new IllegalArgumentException("Tried to remove seam from empty image");
    }

    // Adjust the picture and energy
    for (int j = 0; j < this.height; ++j) {
      int seamIdx = seam[j];
      if (seamIdx < 0 || seamIdx >= this.width) {
        throw new IndexOutOfBoundsException("Seam value: " + seamIdx
            + " out of bounds");
      }
      // Picture
      for (int i = seamIdx; i < this.width - 1; ++i) {
        int baseIdx = i * this.logicalHeight + j;
        image[baseIdx] = image[baseIdx + this.logicalHeight];
      }

    }
    // Energy.  Only elements +- 1 in x need to be adjusted
    //  which, after removal, means the element and the one below it
    // Note: we need to do this in a separate loop
    for (int j = 1; j < this.height-1; ++j) {
      int seamIdx = seam[j];

      // Do the seam position
      int seamFullIdx = seamIdx * this.logicalHeight + j;
      if (seamIdx == 0) {
        energy[seamFullIdx] = ENERGY_BOUNDARY;
      } else if (seamIdx < this.width - 1) {
        int energySquared =
            getDeltaSqVals(image[seamFullIdx + logicalHeight],
                image[seamFullIdx - logicalHeight])
                + getDeltaSqVals(image[seamFullIdx + 1],
                image[seamFullIdx - 1]);
        energy[seamFullIdx] = Math.sqrt((double) energySquared);
      }
      // And the one to the left
      if (seamIdx > 1) {
        int baseIdx = seamFullIdx - this.logicalHeight;
        int energySquared =
            getDeltaSqVals(image[baseIdx + logicalHeight],
                image[baseIdx - logicalHeight])
                + getDeltaSqVals(image[baseIdx + 1],
                image[baseIdx - 1]);
        energy[baseIdx-1] = Math.sqrt((double) energySquared);
      }
    }

    this.width -= 1;
  }

}
