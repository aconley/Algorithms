//! Common utility methods

// Returns the length of the longest common prefix of two byte arrays.
fn longest_prefix(v1: &[u8], v2: &[u8]) -> usize {
  let mut l = 0_usize;
  for (a, b) in v1.iter().zip(v2) {
    if a != b {
      return l;
    } else {
      l += 1;
    }
  }
  l
}

/// The fundamental preprocessing algorithm.
/// 
/// Given a string s, returns a vector v such that v[i] is the length
/// of the longest substring of s that starts a i and matches a prefix of s,
/// where all indexing is bytewise.
///
/// Algorithms on Strings, Trees, and Sequences, Gusfield, Section 1.4
fn z_algorithm(s: &str) -> Vec<usize> {
  if s.is_empty() {
    return Vec::new();
  }
  let b = s.as_bytes();
  let n = b.len();

  let mut result = Vec::with_capacity(b.len());
  result.push(b.len());

  let mut l = 0_usize;
  let mut r = 0_usize;

  for k in 1_usize..n {
    // Compute z[k]
    if k > r {
      // Case 1; not in a current z box.
      let zk = longest_prefix(&b, &b[k..]);
      if zk > 0 {
        l = k;
        r = k + zk - 1;
      }
      result.push(zk);
    } else {
      let kp = k - l;
      let zkp = result[kp];
      let beta = r - k + 1;
      if zkp < beta {
        // Case 2a.
        result.push(zkp);
      } else if zkp > beta {
        // We must have zk = beta; See Exercise 1.6.6
        result.push(beta);
      } else {
        // Case 2b.
        let pos_mismatch = longest_prefix(&b[r + 1..], &b[beta..]) + r + 1;
        result.push(pos_mismatch - k);
        l = k;
        r = pos_mismatch - 1;
      }
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn longest_prefix_for_same_string_returns_full_length() {
    let s = "Ask not what your country can do for you".as_bytes();
    assert_eq!(longest_prefix(s, s), s.len());
  }

  #[test]
  fn longest_prefix_for_unmatched_strings_returns_zero() {
    let s1 = "abc".as_bytes();
    let s2 = "bcd".as_bytes();
    assert_eq!(longest_prefix(s1, s2), 0_usize);
  }

  #[test]
  fn longest_prefix_for_prefix_returns_prefix_length() {
    let s1 = "abcdef".as_bytes();
    let s2 = "abcd".as_bytes();
    assert_eq!(longest_prefix(s1, s2), 4_usize);
    assert_eq!(longest_prefix(s2, s1), 4_usize);
  }

  #[test]
  fn longest_prefix_for_partial_match_returns_match_length() {
    let s1 = "abcdef".as_bytes();
    let s2 = "abcabc".as_bytes();
    assert_eq!(longest_prefix(s1, s2), 3_usize);
    assert_eq!(longest_prefix(s2, s1), 3_usize);
  }

  #[test]
  fn z_for_empty_input_returns_empty_result() {
    assert!(z_algorithm("").is_empty());
  }

  #[test]
  fn z_for_single_character_returns_one() {
    assert_eq!(z_algorithm("a"), vec![1]);
  }

  #[test]
  fn z_for_aa_returns_expected_value() {
    assert_eq!(z_algorithm("aa"), vec![2, 1]);
  }

  #[test]
  fn z_for_aaa_returns_expected_value() {
    assert_eq!(z_algorithm("aaa"), vec![3, 2, 1]);
  }

  #[test]
  fn z_for_aaaa_returns_expected_value() {
    assert_eq!(z_algorithm("aaab"), vec![4, 2, 1, 0]);
  }

  #[test]
  fn z_case2a_returns_expected_value() {
    // Tests case 2a.
    assert_eq!(z_algorithm("aabcaabxaaz"), 
      vec![11, 1, 0, 0, 3, 1, 0, 0, 2, 1, 0]);
    assert_eq!(z_algorithm("aabaacdaabaaca"), 
      vec![14, 1, 0, 2, 1, 0, 0, 6, 1, 0, 2, 1, 0, 1]);
  }

  #[test]
  fn z_case2b_off_end_returns_expected_value() {
    // Test case 2b running off end.
    assert_eq!(z_algorithm("aabaabcaabaaba"), 
      vec![14, 1, 0, 3, 1, 0, 0, 6, 1, 0, 4, 1, 0, 1]);
  }

  #[test]
  fn z_case2b_returns_expected_value() {
    // Test case 2b not running off end.
    assert_eq!(z_algorithm("aabaabcaabaabab"), 
      vec![15, 1, 0, 3, 1, 0, 0, 6, 1, 0, 4, 1, 0, 1, 0]);
    
    assert_eq!(z_algorithm("aabaaacdaabaad"),
      vec![14, 1, 0, 2, 2, 1, 0, 0, 5, 1, 0, 2, 1, 0]);
  }
}