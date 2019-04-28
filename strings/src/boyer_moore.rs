
//! Boyer-Moore logic
use std::cmp;
use std::collections::HashMap;

// The logic for applying shifts on a mismatch.
trait ShiftLogic {
  /// Return the shift to apply if characters s = P[i+1..n] match T[k+1..n-i+k],
  /// but P[i] != T[k], if s = T[k..n-i+k]
  fn shift_on_mismatch(&self, i: usize, s: &[u8]) -> usize;
}

// Encapsulates the logic to apply the basic bad character rule for the
// Boyer-Moore algorithm.
struct BadCharacterRule {
  // The rightmost position of every byte that occurs in the input.
  rightmost: [usize; 256],
}

impl BadCharacterRule {
  fn new(s: &[u8]) -> BadCharacterRule {
    let mut v = [0_usize; 256];
    for (i, &b) in s.iter().enumerate() {
      v[b as usize] = i;
    }
    BadCharacterRule { rightmost: v }
  }
}

impl ShiftLogic for BadCharacterRule {
  fn shift_on_mismatch(&self, i: usize, s: &[u8]) -> usize {
    assert!(!s.is_empty());
    // If the rightmost occurance of the mismatching character T[k] = s[0]
    // is at position j < i, shift so that P[j] is below T[k].  Otherwise,
    // shift by one position.
    std::cmp::max(1, i - self.rightmost[s[0] as usize])
  }
}

// Encapsulates the logic to apply the extended bad character rule for
// the Boyer-Moore algorithm.
struct ExtendedBadCharacterRule {
  // For each character in the input string, holds the positions that character
  // occurs in descending order.
  positions: HashMap<u8, Vec<usize>>,
}

impl ExtendedBadCharacterRule {
  fn new(s: &str) -> ExtendedBadCharacterRule {
    // Count the number of occurances.
    let mut v = [0_usize; 256];
    for &b in s.as_bytes().iter() {
      v[b as usize] += 1;
    }
    let mut m = HashMap::new();
    for (i, &c) in v.iter().enumerate() {
      if c > 0 {
        m.insert(i as u8, Vec::with_capacity(c));
      }
    }

    ExtendedBadCharacterRule { positions: m }
  }
}

impl ShiftLogic for ExtendedBadCharacterRule {
  fn shift_on_mismatch(&self, i: usize, s: &[u8]) -> usize {
    assert!(!s.is_empty());
    // Shift so that the rightmost occurance of the mismatching character
    // T[k] (= s[0]) to the left of i is j, shift so that P[j] is below T[k].
    let rightmost_to_left = match self.positions.get(&s[0]) {
      None => 0_usize, // T[k] does not occur in the pattern.
      Some(ref v) => *v.iter().find(|&&j| j < i).unwrap_or(&0_usize)
    };
    std::cmp::max(1, i - rightmost_to_left)
  }
}