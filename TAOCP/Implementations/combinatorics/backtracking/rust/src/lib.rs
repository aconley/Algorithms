use std::fmt;

pub struct Solution {
    // The number of queens.
    n: u8,
    // For each of the n rows, the colum number [0, n)
    rows: Vec<u8>,
}

// A function which visits a solution, returning false if
// no further iterations are desired. 
pub type Visitor = Fn(&Solution) -> bool;

pub trait NQueensSolver {
    fn visit(&mut self, n: u8, v: &Visitor);
}

pub struct BitwiseNQueensSolver {
    n: usize,
    nm1: usize, 
    a: u64,
    b: u64, 
    c: u64,
    s: Solution,
    done: bool
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

impl BitwiseNQueensSolver {
    fn init(&mut self, n: u8) -> () {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.n = n as usize;
        self.nm1 = (n - 1) as usize;
        self.done = false;
        self.s = Solution {n : n, rows: vec![0; n as usize]};
    }

    fn visit_levels(&mut self, l: usize, v: &Visitor) {
        if self.done {
            return;
        }
        if l > self.nm1 {
            if !v(&self.s) {
                self.done = true;
                return;
            }
        }

        for xl in 0..self.n {
            let xla : u64 = 1u64 << xl;
            let xlb : u64 = xla << l;
            let xlc : u64 = 1u64 << (xl - l + self.nm1);
            if (self.a & xla == 0) 
                && (self.b & xlb == 0) 
                && (self.c & xlc == 0) {

                // Valid candidate x_l
                self.s.rows[l] = xl as u8;
                self.a |= xla;
                self.b |= xlb;
                self.c |= xlc;

                self.visit_levels(l + 1, v);

                // Undo. 
                self.a &= !xla;
                self.b &= !xlb;
                self.c &= !xlc;
            }
        }
    }
}

impl NQueensSolver for BitwiseNQueensSolver {
    fn visit(&mut self, n: u8, v: &Visitor) {
        self.init(n);
        self.visit_levels(0, v);
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
