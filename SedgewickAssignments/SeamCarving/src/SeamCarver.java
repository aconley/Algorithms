import edu.princeton.cs.algs4.Picture;

import java.awt.*;

/**
 *
 */
public class SeamCarver {

  // Masks for extracting individual colors
  private static int BLUE_MASK = 0x000000ff;
  private static int GREEN_MASK = 0x0000ff00;
  private static int RED_MASK = 0x00ff0000;

  private static float ENERGY_BOUNDARY = 1000.0f;

  // Internal representation of picture;
  //  a flat array of integers, with 8-bit rgb values bitpacked
  private int height;
  private int width;
  private int[] image;

  private boolean isTransposed;

  // Energy array; 32 bits should be enough
  private float[] energy;

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
  }

  public SeamCarver(Picture picture) {
    if (picture == null) {
      throw new NullPointerException("Picture is null");
    }
    this.height = picture.height();
    this.width = picture.width();

    image = new int[this.height * this.width];
    for (int i = 0; i < this.width; ++i) {
      for (int j = 0; j < this.height; ++j) {
        Color c = picture.get(i, j);
        image[i * height + j] =
            packColor(c.getRed(), c.getGreen(), c.getBlue());
      }
    }

    isTransposed = false;

    // Pre-compute energy
    energy = calcEnergy(image, width, height);
  }

  private static float[] calcEnergy(int[] image, int width, int height) {
    float[] energy = new float[width * height];
    for (int j = 0; j < height; ++j) {
      energy[j] = ENERGY_BOUNDARY;
    }
    for (int i = 1; i < width - 1; ++i) {
      int baseIdx = i * height;
      energy[baseIdx] = ENERGY_BOUNDARY;
      for (int j = 1; j < height - 1; ++j) {
        energy[baseIdx + j] = (float) getEnergy(i, j, image, width, height);
      }
      energy[baseIdx + height - 1] = ENERGY_BOUNDARY;
    }
    int baseIdx = (width - 1) * height;
    for (int j = 0; j < height; ++j) {
      energy[baseIdx + j] = ENERGY_BOUNDARY;
    }
    return energy;
  }

  private void transpose() {
    int[] newImage = new int[width * height];
    float[] newEnergy = new float[width * height];

    int baseIdx;
    for (int i = 0; i < width; ++i) {
      baseIdx = i * height;
      for (int j = 0; j < height; ++j) {
        newImage[j * width + i] = image[baseIdx + j];
      }
    }
    for (int i = 0; i < width; ++i) {
      baseIdx = i * height;
      for (int j = 0; j < height; ++j) {
        newEnergy[j * width + i] = energy[baseIdx + j];
      }
    }
    isTransposed = !isTransposed;
    int temp = width;
    width = height;
    height = temp;
    image = newImage;
    energy = newEnergy;
  }

  public int width() {
    return isTransposed ? this.height : this.width;
  }

  public int height() {
    return isTransposed ? this.width : this.height;
  }

  public Picture picture() {
    if (isTransposed) {
      transpose();
    }
    Picture retval = new Picture(width, height);
    for (int i = 0; i < this.width; ++i) {
      for (int j = 0; j < this.height; ++j) {
        retval.set(i, j, new Color(image[i * height + j]));
      }
    }
    return retval;
  }

  public double energy(int x, int y) {
    if (isTransposed) {
      if (x < 0 || x >= this.height) {
        throw new IndexOutOfBoundsException("x out of bounds");
      }
      if (y < 0 || y >= this.width) {
        throw new IndexOutOfBoundsException("y out of bounds");
      }
      // Note we don't just index into the array, which is float
      return getEnergy(y, x, image, width, height);
    } else {
      if (x < 0 || x >= this.width) {
        throw new IndexOutOfBoundsException("x out of bounds");
      }
      if (y < 0 || y >= this.height) {
        throw new IndexOutOfBoundsException("y out of bounds");
      }
      // Note we don't just index into the array, which is float
      return getEnergy(x, y, image, width, height);
    }
  }

  private static double getEnergy(int i, int j, int[] image,
      int width, int height) {
    if (i <= 0 || i >= width - 1 || j <= 0 || j >= height-1) {
      return ENERGY_BOUNDARY;
    }
    int centIdx = i * height + j;
    int energySquared =
        getDeltaSqVals(image[centIdx + height], image[centIdx - height])
            + getDeltaSqVals(image[centIdx + 1], image[centIdx - 1]);
    return Math.sqrt((double) energySquared);
  }

  private static int getDeltaSqVals(int imgp1, int imgm1) {
    int dr = getRed(imgp1) - getRed(imgm1);
    int dg = getGreen(imgp1) - getGreen(imgm1);
    int db = getBlue(imgp1) - getBlue(imgm1);
    return dr * dr + dg * dg + db * db;
  }

  public int[] findHorizontalSeam() {
    // Get some special cases out of the way
    if (isTransposed) {
      if (width == 1) {
        int[] ret = new int[height];
        return ret;
      } else if (height == 1) {
        int[] ret = new int[1];
        ret[0] = 0;
        return ret;
      }
    } else {
      if (height == 1) {
        int[] ret = new int[width];
        return ret;
      } else if (width == 1) {
        int[] ret = new int[1];
        ret[0] = 0;
        return ret;
      }
    }

    if (isTransposed) {
      transpose();
    }

    return findSeam();
  }

  public int[] findVerticalSeam() {
    // Get some special cases out of the way
    if (isTransposed) {
      if (height == 1) {
        int[] ret = new int[width];
        return ret;
      } else if (width == 1) {
        int[] ret = new int[1];
        ret[0] = 0;
        return ret;
      }
    } else {
      if (width == 1) {
        int[] ret = new int[height];
        return ret;
      } else if (height == 1) {
        int[] ret = new int[1];
        ret[0] = 0;
        return ret;
      }
    }

    if (!isTransposed) {
      transpose();
    }

    return findSeam();
  }

  // Works horizontally on current representation
  private int[] findSeam() {

    float[] distTo = new float[width * height];
    int[] edgeTo = new int[width * height];

    for (int j = 0; j < height; ++j) {
      distTo[j] = 0.0f;
    }

    // Max all other distances.
    int baseIdx;
    for (int i = 1; i < width; ++i) {
      baseIdx = i * height;
      for (int j = 0; j < height; ++j) {
        distTo[baseIdx + j] = Float.POSITIVE_INFINITY;
      }
    }

    baseIdx = 0;
    int nextRowIdx = height;
    int thisIdx, thisNextRow;
    for (int i = 0; i < width - 1; ++i) {

      // j = 0
      relax(baseIdx, nextRowIdx, distTo, edgeTo);
      relax(baseIdx, nextRowIdx + 1, distTo, edgeTo);

      for (int j = 1; j < height - 1; ++j) {
        thisIdx = baseIdx + j;
        thisNextRow = nextRowIdx + j;
        relax(thisIdx, thisNextRow - 1, distTo, edgeTo);
        relax(thisIdx, thisNextRow, distTo, edgeTo);
        relax(thisIdx, thisNextRow + 1, distTo, edgeTo);
      }

      // j = height - 1
      thisIdx = baseIdx + height - 1;
      thisNextRow = nextRowIdx + height - 1;
      relax(thisIdx, thisNextRow - 1, distTo, edgeTo);
      relax(thisIdx, thisNextRow, distTo, edgeTo);

      baseIdx += height;
      nextRowIdx += height;
    }

    // Now find the best one
    int endOfSeam = getBestHorizontalSeam(distTo);

    // Extract the seam from that
    return getHorizontalSeam(endOfSeam, edgeTo);
  }

  private void relax(int from, int to, float[] distTo, int[] edgeTo) {
    float thisEnergy = distTo[from] + energy[to];
    if (distTo[to] > thisEnergy) {
      distTo[to] = thisEnergy;
      edgeTo[to] = from;
    }
  }

  private int getBestHorizontalSeam(float[] distTo) {
    int thisIdx = (width - 1) * height;
    int endIdx = 0;
    float bestCost = distTo[thisIdx];
    float thisCost;
    for (int j = 1; j < height ; ++j) {
      thisIdx += 1;
      thisCost = distTo[thisIdx];
      if (thisCost < bestCost) {
        endIdx = j;
        bestCost = thisCost;
      }
    }
    return endIdx;
  }

  private int[] getHorizontalSeam(int end, int[] edgeTo) {
    int[] result = new int[width];
    result[width - 1] = end;
    int fullIdx = (width - 1) * height + end;
    for (int j = width - 2; j >= 0; --j) {
      fullIdx = edgeTo[fullIdx];
      result[j] = fullIdx % height;
    }
    return result;
  }

  public void removeHorizontalSeam(int[] seam) {
    if (isTransposed) {
      if (width == 0) {
        throw new IllegalArgumentException("Tried to remove seam from empty image");
      }
      if (width == 1) {
        throw new IllegalArgumentException("Removing seam would result in empty image");
      }
      validateSeam(seam, height, width);
      transpose();
    } else {
      if (height == 0) {
        throw new IllegalArgumentException("Tried to remove seam from empty image");
      }
      if (height == 1) {
        throw new IllegalArgumentException("Removing seam would result in empty image");
      }
      validateSeam(seam, width, height);
    }

    removeSeam(seam);
  }

  public void removeVerticalSeam(int[] seam) {
    if (isTransposed) {
      if (height == 0) {
        throw new IllegalArgumentException("Tried to remove seam from empty image");
      }
      if (height == 1) {
        throw new IllegalArgumentException("Removing seam would result in empty image");
      }
      validateSeam(seam, width, height);
    } else {
      if (width == 0) {
        throw new IllegalArgumentException("Tried to remove seam from empty image");
      }
      if (width == 1) {
        throw new IllegalArgumentException("Removing seam would result in empty image");
      }
      validateSeam(seam, height, width);
      transpose();
    }

    removeSeam(seam);
  }

  // Horizontally from current representation
  private void removeSeam(int[] seam) {
    // Adjust the image
    int baseIdx, baseNewIdx;
    int[] newImage = new int[width * (height - 1)];
    for (int i = 0; i < this.width; ++i) {
      int seamIdx = seam[i];
      baseIdx = i * height;
      baseNewIdx = i * (height - 1);
      System.arraycopy(image, baseIdx, newImage, baseNewIdx, seamIdx);
      System.arraycopy(image, baseIdx + seamIdx + 1,
          newImage, baseNewIdx + seamIdx, height - seamIdx - 1);
    }

    image = newImage;
    height -= 1;
    energy = calcEnergy(image, width, height);
  }

  private void validateSeam(int[] seam, int expectedLength,
      int expectedRange) {
    if (seam == null) {
      throw new NullPointerException("Null seam");
    }
    if (seam.length == 0) {
      throw new IllegalArgumentException("Empty seam");
    }
    if (seam.length != expectedLength) {
      throw new IllegalArgumentException("Seam wrong length: "
          + seam.length + " but expected " + expectedLength);
    }
    int prevSeam = seam[0];
    if (prevSeam < 0 || prevSeam >= expectedRange) {
      throw new IllegalArgumentException("Invalid seam index");
    }
    for (int i = 1; i < seam.length; ++i) {
      int thisSeam = seam[i];
      if (thisSeam < 0 || thisSeam >= expectedRange) {
        throw new IllegalArgumentException("Invalid seam index");
      }
      if (Math.abs(thisSeam - prevSeam) > 1) {
        throw new IllegalArgumentException("Seam jumps more than 1 index");
      }
      prevSeam = thisSeam;
    }
  }

  private static void testCase1() {
    Picture picture = new Picture(8, 1);
    picture.set(0, 0, new Color(7, 7, 5));
    picture.set(1, 0, new Color(1, 1, 3));
    picture.set(2, 0, new Color(4, 0, 2));
    picture.set(3, 0, new Color(6, 6, 0));
    picture.set(4, 0, new Color(2, 5, 6));
    picture.set(5, 0, new Color(0, 6, 2));
    picture.set(6, 0, new Color(3, 0, 2));
    picture.set(7, 0, new Color(8, 3, 5));

    SeamCarver carver = new SeamCarver(picture);
    carver.removeVerticalSeam(new int[]{3});
    carver.picture();
    carver.energy(3, 0);
    if (carver.findHorizontalSeam().length != 7)
      throw new IllegalArgumentException("Unexpected h seam length");
    carver.energy(0, 0);
    carver.energy(1, 0);
    if (carver.findVerticalSeam().length != 1)
      throw new IllegalArgumentException("Unexpected v seam length");
    if (carver.width() != 7)
      throw new IllegalArgumentException("Unexpected width");
    carver.removeVerticalSeam(new int[]{4});
    if (carver.width() != 6)
      throw new IllegalArgumentException("Unexpected width");
    if (carver.findHorizontalSeam().length != 6)
      throw new IllegalArgumentException("Unexpected h seam length");
  }

  public static void main(String[] args) {
    testCase1();
  }
}
