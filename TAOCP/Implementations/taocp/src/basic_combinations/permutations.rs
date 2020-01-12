// Generate all permutations of a vector using cloning.

// Generates the next permutation of the input vector, returning None if the input is
// already the largest.  
//
//Allows for non-distinct elements in the input.  For example, the iterations of [1, 2, 2] are [1, 2, 2], 
// [2, 1, 2], [2, 2, 1].  Based on Knuth TAOCP 4a 7.2.1.2 exercise 1.
pub fn next_permutation<T: Ord>(mut a: Vec<T>) -> Option<Vec<T>> {
  let n = a.len();
  if n <= 2 {
    return next_permutation_small(a);
  }

  // Check easiest case.
  let z = n - 1;
  let mut y = z - 1;
  if a[y] < a[z] {
    a.swap(y, z);
    return Some(a);
  }

  // Check next easiest case.
  let x = y - 1;
  if a[x] < a[y] {
    if a[x] < a[z] {
      a.swap(x, z);
      a.swap(y, z);
    } else {
      a.swap(x, z);
      a.swap(x, y);
    }
    return Some(a);
  }

  // General case
  loop {
    if a[y] < a[y+1] {
      break;
    }
    if y == 0 {
      return None;
    }
    y -= 1;
  }

  let mut zp = n - 1;
  while a[y] >= a[zp] {
    zp -= 1;
  }
  a.swap(y, zp);

  a[(y+1)..].reverse();
  Some(a)
}

fn next_permutation_small<T:Ord>(mut a: Vec<T>) -> Option<Vec<T>> {
  let n = a.len();
  assert!(n <= 2);
  if n < 2 {
    None
  } else if a[0] < a[1] {
    a.swap(0, 1);
    Some(a)
  } else {
    None
  }
}

// An iterator over cloneable elements that visits all permutation.
pub struct PermutationsIterator<'a, T: 'a> {
  values: &'a Vec<T>,
  indices: Vec<usize>,
  init: bool
}

impl<'a, T: Clone> PermutationsIterator<'a, T> {
  pub fn new(v: &'a Vec<T>) -> Self {
    assert!(v.len() > 0, "Empty values.");
    PermutationsIterator{ values: v, indices: (0..v.len()).collect(), init: false }
  }

  fn clone_by_index(&self) -> Vec<T> {
    let mut m = Vec::with_capacity(self.values.len());
    for idx in &self.indices {
      m.push(self.values[*idx].clone());
    }
    m
  }
}

impl<'a, T: Clone> Iterator for  PermutationsIterator<'a, T>{
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if !self.init {
      self.init = true;
      return Some(self.clone_by_index());
    }

    if let Some(k) = next_permutation(self.indices.clone()) {
      self.indices = k;
      return Some(self.clone_by_index());
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_elem() {
    assert_eq!(next_permutation(vec![3]), None);
  }

  #[test]
  fn two_elem_first() {
    assert_eq!(next_permutation(vec![1, 3]), Some(vec![3, 1]));
  }

  #[test]
  fn two_elem_second() {
    assert_eq!(next_permutation(vec![3, 1]), None);
  }

  #[test]
  fn count_3() {
    assert_eq!(count_permutations(vec![1, 2, 3]), 6);
  }

  #[test]
  fn count_4() {
    assert_eq!(count_permutations(vec![1, 2, 3, 4]), 24);
  }

  #[test]
  fn count_4_with_repeat() {
    assert_eq!(count_permutations(vec![1, 2, 2, 4]), 12);
  }

  #[test]
  fn count_4_with_repeat_starting_on_second() {
    assert_eq!(count_permutations(vec![1, 2, 3, 2]), 11);
  }

  #[test]
  fn count_5_identical() {
    assert_eq!(count_permutations(vec![1, 1, 1, 1, 1]), 1);
  }

  #[test]
  fn count_7_identical() {
    assert_eq!(count_permutations(vec![-1, 2, 4, 11, 22, 33, 34]), 5040);
  }
  
  #[test]
  fn values_3_no_repeats() {
    let expected = vec![
      vec![1, 2, 3], vec![1, 3, 2], 
      vec![2, 1, 3], vec![2, 3, 1],
      vec![3, 1, 2], vec![3, 2, 1]
    ];

    check_permutations(vec![1, 2, 3], expected);
  }

  #[test]
  fn values_4_with_repeats() {
    let expected = vec![
      vec![1, 2, 2, 3], vec![1, 2, 3, 2], vec![1, 3, 2, 2],
      vec![2, 1, 2, 3], vec![2, 1, 3, 2], vec![2, 2, 1, 3], vec![2, 2, 3, 1], vec![2, 3, 1, 2], vec![2, 3, 2, 1],
      vec![3, 1, 2, 2], vec![3, 2, 1, 2], vec![3, 2, 2, 1]
    ];

    check_permutations(vec![1, 2, 2, 3], expected);
  }


  #[test]
  fn permutations_iterator_4() {
    let v = vec!['a', 'b', 'c'];
    let expected = vec![
      vec!['a', 'b', 'c'], vec!['a', 'c', 'b'], 
      vec!['b', 'a', 'c'], vec!['b', 'c', 'a'],
      vec!['c', 'a', 'b'], vec!['c', 'b', 'a']
    ];

    let actual : Vec<Vec<char>> = PermutationsIterator::new(&v).collect();
    assert_eq!(actual, expected);
  }


  fn count_permutations(mut a: Vec<i32>)  -> i32{
    let mut niter = 1;
    loop {
      match next_permutation(a) {
        Some(k) => { niter += 1; a = k },
        None => return niter
      }
    }
  }

  fn check_permutations(mut a: Vec<i32>, expected: Vec<Vec<i32>>) {
    assert_eq!(a, expected[0]);
    let mut niter = 1;
    loop {
      match next_permutation(a) {
        Some(k) => { 
          assert_eq!(expected[niter], k);
          niter += 1; 
          a = k },
        None => break
      }
    }

  }
}