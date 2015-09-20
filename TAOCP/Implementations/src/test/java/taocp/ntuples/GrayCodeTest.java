package taocp.ntuples;

import org.junit.Test;
import static org.assertj.core.api.Assertions.*;

import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

public class GrayCodeTest {

  @Test
  public void testSequence3() {
    List<Integer> seq = Arrays.asList(0, 1, 3, 2, 6, 7, 5, 4);
    Iterator<Integer> iter = (new GrayCode(3)).iterator();
    for (Integer val : seq) {
      assertThat(iter.hasNext()).isTrue();
      assertThat(iter.next()).isEqualTo(val);
    }
    assertThat(iter.hasNext()).isFalse();
  }

  @Test
  public void testSequence4() {
    List<Integer> seq = Arrays.asList(0b0000, 0b0001, 0b0011, 0b0010,
        0b0110, 0b0111, 0b0101, 0b0100, 0b1100, 0b1101,
        0b1111, 0b1110, 0b1010, 0b1011, 0b1001, 0b1000);
    Iterator<Integer> iter = (new GrayCode(4)).iterator();
    for (Integer val : seq) {
      assertThat(iter.hasNext()).isTrue();
      assertThat(iter.next()).isEqualTo(val);
    }
    assertThat(iter.hasNext()).isFalse();
  }
}
