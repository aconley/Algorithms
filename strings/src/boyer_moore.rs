
//! Boyer-Moore logic
use std::cmp;
use std::collections::HashMap;

// The logic for applying shifts on a mismatch.
trait ShiftLogic {
  /// Return the shift to apply if the characters P[i+1..n] match T[k+1..n-i+k],
  /// but P[i] != T[k], where n = |P|.
  fn shift_on_mismatch(&self, i: usize, tk: u8) -> usize;
}

// Encapsulates the logic to apply the basic bad character rule for the
// Boyer-Moore algorithm.
struct BadCharacterRule {
  // The rightmost position of every byte that occurs in the input.
  // -1 indicates the byte is not present in the input.
  rightmost: [isize; 256],
}

impl BadCharacterRule {
  fn new(s: &str) -> BadCharacterRule {
    assert!(!s.is_empty());

    let mut v = [-1_isize; 256];
    for (i, &b) in s.as_bytes().iter().enumerate() {
      v[b as usize] = i as isize;
    }
    BadCharacterRule { rightmost: v }
  }
}

impl ShiftLogic for BadCharacterRule {
  fn shift_on_mismatch(&self, i: usize, tk: u8) -> usize {
    // If the rightmost occurance of the mismatching character T[k] = s[0]
    // is at position j < i, shift so that P[j] is below T[k].  Otherwise,
    // shift by one position.
    cmp::max(1, (i as isize) - self.rightmost[tk as usize]) as usize
  }
}

// Encapsulates the logic to apply the extended bad character rule for
// the Boyer-Moore algorithm.
struct ExtendedBadCharacterRule {
  n: usize, // Length of input pattern
  // For each character in the input string, holds the positions that character
  // occurs in descending order.
  positions: HashMap<u8, Vec<usize>>,
}

impl ExtendedBadCharacterRule {
  fn new(s: &str) -> ExtendedBadCharacterRule {
    assert!(!s.is_empty());

    let sb = s.as_bytes();
    let mut m = HashMap::new();
    // Insert in reversed order, so rightmost position comes first.
    for (i, &b) in sb.iter().enumerate().rev() {
      m.entry(b).or_insert_with(Vec::new).push(i);
    }

    ExtendedBadCharacterRule { n: sb.len(), positions: m }
  }
}

impl ShiftLogic for ExtendedBadCharacterRule {
  fn shift_on_mismatch(&self, i: usize, tk: u8) -> usize {
    // Shift so that the rightmost occurance of the mismatching character
    // T[k] (= s[0]) to the left of i is j, shift so that P[j] is below T[k].
    match self.positions.get(&tk) {
      None => self.n - 1, // T[k] does not occur in pattern
      Some(ref v) => v.iter().find(|&&k| k < i).map_or(self.n - 1, |k| i - k)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bad_character_rule_for_character_not_in_pattern() {
    // T = b o w l i n g         k = 5 T[k] = n
    // P =       f r o g         i = 2 P[i] = o
    // P'=             f r o g   shift = 3
    let b = BadCharacterRule::new("frog");
    assert_eq!(b.shift_on_mismatch(2, 'n' as u8), 3);
  }

  #[test]
  fn bad_character_rule_for_character_in_pattern_one_step_left() {
    // T = b o w l i n g         k = 5 T[k] = n
    // P =         n a g         i = 1 P[i] = a
    // P'=           n a g       shift = 1
    let b = BadCharacterRule::new("nag");
    assert_eq!(b.shift_on_mismatch(1, 'n' as u8), 1);
  }

  #[test]
  fn bad_character_rule_for_character_in_pattern_three_steps_left() {
    // T = b o w l i n g         k = 5 T[k] = n
    // P =   n n o a a g         i = 4 P[i] = a
    // P'=         n n o a a g   shift = 3
    let b = BadCharacterRule::new("nnoaag");
    assert_eq!(b.shift_on_mismatch(4, 'n' as u8), 3);
  }

  #[test]
  fn bad_character_rule_for_character_in_pattern_to_right() {
    // T = a b c d e f f         k = 5 T[k] = f
    // P =       d e g f         i = 2 P[i] = g
    // P'=         d e g f       shift = 1
    let b = BadCharacterRule::new("degf");
    assert_eq!(b.shift_on_mismatch(2, 'f' as u8), 1);
  }

  #[test]
  fn bad_character_rule_for_character_in_pattern_to_left_and_right() {
    // T = a b c d e f f         k = 5 T[k] = f
    // P =       f e g f         i = 2 P[i] = g
    // P'=         f e g f       shift = 1
    let b = BadCharacterRule::new("fegf");
    assert_eq!(b.shift_on_mismatch(2, 'f' as u8), 1);
  }

   #[test]
  fn extended_bad_character_rule_for_character_not_in_pattern() {
    // T = b o w l i n g         k = 5 T[k] = n
    // P =       f r o g         i = 2 P[i] = o
    // P'=             f r o g   shift = 3
    let b = ExtendedBadCharacterRule::new("frog");
    assert_eq!(b.shift_on_mismatch(2, 'n' as u8), 3);
  }

  #[test]
  fn extended_bad_character_rule_for_character_in_pattern_one_step_left() {
    // T = b o w l i n g         k = 5 T[k] = n
    // P =         n a g         i = 1 P[i] = a
    // P'=           n a g       shift = 1
    let b = ExtendedBadCharacterRule::new("nag");
    assert_eq!(b.shift_on_mismatch(1, 'n' as u8), 1);
  }

  #[test]
  fn extended_bad_character_rule_for_character_in_pattern_three_steps_left() {
    // T = b o w l i n g         k = 5 T[k] = n
    // P =   n n o a a g         i = 4 P[i] = a
    // P'=         n n o a a g   shift = 3
    let b = ExtendedBadCharacterRule::new("nnoaag");
    assert_eq!(b.shift_on_mismatch(4, 'n' as u8), 3);
  }

  #[test]
  fn extended_bad_character_rule_for_character_in_pattern_to_right() {
    // This one is different than the simple bad character rule
    // T = a b c d e f d         k = 5 T[k] = f
    // P =       d e g f         i = 2 P[i] = g
    // P'=             d e g f   shift = 3
    let b = ExtendedBadCharacterRule::new("degf");
    assert_eq!(b.shift_on_mismatch(2, 'f' as u8), 3);
  }

  #[test]
  fn extended_bad_character_rule_for_character_in_pattern_to_left_and_right() {
    // This one is different than the simple bad character rule
    // T = a b c d e f f         k = 5 T[k] = f
    // P =       f e g f         i = 2 P[i] = g
    // P'=           f e g f     shift = 2
    let b = ExtendedBadCharacterRule::new("fegf");
    assert_eq!(b.shift_on_mismatch(2, 'f' as u8), 2);
  }

  #[test]
  fn extended_bad_character_rule_for_character_in_pattern_multiple_times_left() {
    // This one is different than the simple bad character rule
    // T = a b c d e f f         k = 5 T[k] = f
    // P =     f f e g f         i = 3 P[i] = g
    // P'=         f f e g f     shift = 2
    let b = ExtendedBadCharacterRule::new("ffegf");
    assert_eq!(b.shift_on_mismatch(3, 'f' as u8), 2);
  }
}