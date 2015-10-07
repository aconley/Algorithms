package taocp.trees;

import com.google.common.collect.ImmutableList;
import com.google.common.collect.Iterables;
import org.junit.Test;

import java.util.Iterator;

import static org.assertj.core.api.Assertions.*;

/**
 * Test for nested parens
 */
public class NestedParensTest {
  @Test
  public void test3() {
    ImmutableList<String> expectedValues =
        ImmutableList.of("((()))", "(()())", "(())()", "()(())", "()()()");
    Iterator<String> par3 = new NestedParens(3).iterator();
    assertThat(par3.hasNext()).isTrue();
    Iterator<String> expectedIterator = expectedValues.iterator();
    while (par3.hasNext()) {
      assertThat(expectedIterator.hasNext())
          .as("Mismatch in expected number of values")
          .isTrue();
      assertThat(par3.next()).isEqualTo(expectedIterator.next());
    }
    assertThat(par3.hasNext()).isFalse();
  }

  @Test
  public void testNElements() {
    for (int i = 2; i < 8; ++i) {
      NestedParens par = new NestedParens(i);
      assertThat((long) Iterables.size(par))
          .as("Unexpected size for m = " + i)
          .isEqualTo(catalan(i));
    }
  }

  /**
   * Get nth catalan number
   * @param n
   * @return
   */
  private static long catalan(int n) {
    if (n < 0) {
      throw new IllegalArgumentException("Invalid (non-positive) n");
    }
    if (n < 2) {
      return 1;
    }
    long upperProd = 1;
    long lowerProd = 1;
    for (long k = 2; k <= n; ++k) {
      upperProd *= (n + k);
      lowerProd *= k;
    }
    return upperProd / lowerProd;
  }
}
