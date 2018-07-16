use std::fmt;

pub struct Solution {
    // The number of queens.
    n: u8,
    // For each of the n rows, the colum number [0, n)
    rows: Vec<u8>,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = vec![vec![false; self.n as usize]; self.n as usize];

        for (i, col) in self.rows.iter().enumerate() {
            board[i as usize][* col as usize] = true;
        }

        let mut output = String::from("");
        for row in board {
            for val in row {
                if val {
                    output += "Q ";
                } else {
                    output += ". ";
                }
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatter_works() {
        let sol = Solution { n: 4, rows: vec![2, 0, 3, 1]};
        assert_eq!(format!("{}", sol), 
            ". . Q . \nQ . . . \n. . . Q \n. Q . . \n");
    }
}
