// Dancing links solution to polnomino packing problems.

use std::collections::BTreeSet;
use std::fmt;

use crate::backtracking::dancing_links::{
    DancingLinksError, DancingLinksIterator, ProblemOption, ProblemOptionBuilder,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cell {
    x: u8,
    y: u8,
}

// A shape that we are attempting to place Polynominoes into.
pub trait Shape {
    type CellIteratorType: Iterator<Item = Cell>;

    fn cells(&self) -> Self::CellIteratorType;

    fn contains(&self, cell: &Cell) -> bool;
}

// A simple box shape.
pub struct SimpleBox {
    pub width: u8,
    pub height: u8,
}

pub struct SimpleBoxIterator {
    width: u8,
    height: u8,
    curr_x: u8,
    curr_y: u8,
}

impl Shape for SimpleBox {
    type CellIteratorType = SimpleBoxIterator;

    fn cells(&self) -> Self::CellIteratorType {
        SimpleBoxIterator {
            width: self.width,
            height: self.height,
            curr_x: 0,
            curr_y: 0,
        }
    }

    fn contains(&self, cell: &Cell) -> bool {
        cell.x < self.width && cell.y < self.height
    }
}

impl Iterator for SimpleBoxIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_y == self.height {
            None
        } else {
            let cell = Cell {
                x: self.curr_x,
                y: self.curr_y,
            };
            self.curr_x = (self.curr_x + 1) % self.width;
            if self.curr_x == 0 {
                self.curr_y += 1;
            }
            Some(cell)
        }
    }
}

// A polynomino.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomino {
    label: char,
    cell_info: CellInfo,
}

// The cells the constitute a polynomino.
//
// The cells are normalized in the sense that the minimum x/y values across
// all cells are always zero.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CellInfo {
    cells: Vec<Cell>, // non-empty
    max_x: u8,
    max_y: u8,
}

impl Polynomino {
    fn new(label: char, mut cells: Vec<Cell>) -> Result<Self, DancingLinksError> {
        if cells.is_empty() {
            return Err(DancingLinksError::new("Cells cannot be empty"));
        }

        let mut min_x = cells[0].x;
        let mut max_x = cells[0].x;
        let mut min_y = cells[0].y;
        let mut max_y = cells[0].y;
        for cell in cells.iter().skip(1) {
            min_x = std::cmp::min(min_x, cell.x);
            max_x = std::cmp::max(max_x, cell.x);
            min_y = std::cmp::min(min_y, cell.y);
            max_y = std::cmp::max(max_y, cell.y);
        }

        if min_x > 0 || min_y > 0 {
            for cell in cells.iter_mut() {
                cell.x -= min_x;
                cell.y -= min_y;
            }
        }
        cells.sort();

        Ok(Polynomino {
            label,
            cell_info: CellInfo {
                cells,
                max_x: max_x - min_x,
                max_y: max_y - min_y,
            },
        })
    }
}

impl CellInfo {
    fn generate_rotations_and_transpositions(&self) -> impl Iterator<Item = CellInfo> {
        let mut results = BTreeSet::<CellInfo>::new();
        let rot1 = self.rotate();
        let rot2 = rot1.rotate();
        let rot3 = rot2.rotate();
        results.insert(self.clone());
        results.insert(self.reflect());
        results.insert(rot1.reflect());
        results.insert(rot1);
        results.insert(rot2.reflect());
        results.insert(rot2);
        results.insert(rot3.reflect());
        results.insert(rot3);
        results.into_iter()
    }

    fn offset(&self, offset_cell: Cell) -> Vec<Cell> {
        self.cells
            .iter()
            .map(|cell| Cell {
                x: cell.x + offset_cell.x,
                y: cell.y + offset_cell.y,
            })
            .collect()
    }

    // Rotates the contents of the CellInfo, preserving normalization and ordering.
    fn rotate(&self) -> Self {
        let mut new_cells: Vec<Cell> = self
            .cells
            .iter()
            .map(|cell| Cell {
                x: cell.y,
                y: self.max_x - cell.x,
            })
            .collect();
        new_cells.sort();
        CellInfo {
            cells: new_cells,
            max_x: self.max_y,
            max_y: self.max_x,
        }
    }

    // Returns the CellInfo reflected around the x axis.
    fn reflect(&self) -> Self {
        let mut new_cells: Vec<Cell> = self
            .cells
            .iter()
            .map(|cell| Cell {
                x: self.max_x - cell.x,
                y: cell.y,
            })
            .collect();
        new_cells.sort();
        CellInfo {
            cells: new_cells,
            ..*self
        }
    }
}

#[derive(Debug)]
pub struct PolynominoIterator {
    inner: DancingLinksIterator<PolynominoItem, PolynominoOption>,
}

#[derive(Debug)]
pub struct PolynominoSolution {
    options: Vec<PolynominoOption>,
}

impl PolynominoIterator {
    pub fn new<S: Shape>(
        polynominoes: Vec<Polynomino>,
        shape: S,
    ) -> Result<Self, DancingLinksError> {
        if polynominoes.is_empty() {
            return Err("Must provide at least some polynominoes".into());
        }
        let mut options = Vec::with_capacity(polynominoes.len());
        for polynomino in polynominoes {
            for polynomino_cells in polynomino.cell_info.generate_rotations_and_transpositions() {
                for shape_cell in shape.cells() {
                    let positions_at_offset = polynomino_cells.offset(shape_cell);
                    if positions_at_offset
                        .iter()
                        .all(|position_cell| shape.contains(position_cell))
                    {
                        options.push(PolynominoOption {
                            label: polynomino.label,
                            cells: positions_at_offset,
                        });
                    }
                }
            }
        }
        Ok(PolynominoIterator {
            inner: DancingLinksIterator::new(options)?,
        })
    }
}

impl Iterator for PolynominoIterator {
    type Item = PolynominoSolution;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|options| PolynominoSolution { options })
    }
}

impl std::iter::FusedIterator for PolynominoIterator {}

impl fmt::Display for PolynominoSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.options.is_empty() {
            return write!(f, "<empty>");
        }
        let mut max_x = 0;
        let mut max_y = 0;
        for option in &self.options {
            max_x = std::cmp::max(
                max_x,
                option.cells.iter().map(|cell| cell.x).max().unwrap_or(0),
            );
            max_y = std::cmp::max(
                max_y,
                option.cells.iter().map(|cell| cell.y).max().unwrap_or(0),
            );
        }

        let mut shape = vec![vec!['.'; max_x as usize]; max_y as usize];
        for option in &self.options {
            for cell in &option.cells {
                shape[cell.y as usize][cell.x as usize] = option.label;
            }
        }

        let mut output = String::with_capacity((shape.len() + 1) * shape[0].len());
        output.extend(&shape[0]);
        for row in shape.into_iter().skip(1) {
            output.push('\n');
            output.extend(row);
        }
        write!(f, "{}", output)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum PolynominoItem {
    Piece(char),
    Position(Cell),
}

#[derive(Debug, PartialEq, Eq)]
struct PolynominoOption {
    label: char,
    cells: Vec<Cell>,
}

impl ProblemOption<PolynominoItem> for PolynominoOption {
    type PrimaryIteratorType = std::vec::IntoIter<PolynominoItem>;
    type SecondaryIteratorType = std::iter::Empty<PolynominoItem>;
    type BuilderType = Self;

    fn primary_items(&self) -> Self::PrimaryIteratorType {
        let mut items = Vec::with_capacity(1 + self.cells.len());
        items.push(PolynominoItem::Piece(self.label));
        for cell in &self.cells {
            items.push(PolynominoItem::Position(*cell));
        }

        items.into_iter()
    }

    fn secondary_items(&self) -> Self::SecondaryIteratorType {
        std::iter::empty()
    }

    fn builder() -> Self::BuilderType {
        PolynominoOption {
            label: ' ',
            cells: vec![],
        }
    }
}

impl ProblemOptionBuilder<PolynominoItem> for PolynominoOption {
    type ProblemOptionType = Self;
    fn add_primary(&mut self, item: &PolynominoItem) -> &mut Self {
        match item {
            PolynominoItem::Piece(label) => self.label = *label,
            PolynominoItem::Position(cell) => self.cells.push(*cell),
        }
        self
    }

    fn add_secondary(&mut self, _item: &PolynominoItem) -> &mut Self {
        self
    }

    fn build(self) -> Self::ProblemOptionType {
        self
    }
}

#[cfg(test)]
mod tests {
    mod polynomino_tests {
        use crate::backtracking::dancing_polynominoes::{Cell, CellInfo, Polynomino};
        use claim::{assert_err, assert_ok_eq};

        #[test]
        fn creates_expected_polynomino() {
            assert_ok_eq!(
                Polynomino::new(
                    'x',
                    vec![
                        Cell { x: 0, y: 0 },
                        Cell { x: 1, y: 0 },
                        Cell { x: 2, y: 0 },
                        Cell { x: 0, y: 1 }
                    ]
                ),
                Polynomino {
                    label: 'x',
                    cell_info: CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 2, y: 0 }
                        ],
                        max_x: 2,
                        max_y: 1
                    }
                }
            );
        }

        #[test]
        fn handles_offset_polynominos() {
            assert_ok_eq!(
                Polynomino::new(
                    'Y',
                    vec![
                        Cell { x: 2, y: 1 },
                        Cell { x: 3, y: 1 },
                        Cell { x: 2, y: 2 },
                    ]
                ),
                Polynomino {
                    label: 'Y',
                    cell_info: CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 0 },
                        ],
                        max_x: 1,
                        max_y: 1
                    }
                }
            );
        }

        #[test]
        fn empty_cells_is_error() {
            assert_err!(Polynomino::new('Z', vec![]));
        }
    }

    mod cell_info {
        use crate::backtracking::dancing_polynominoes::{Cell, CellInfo};

        #[test]
        fn generates_all_variants() {
            // This piece has no symmetries, so all reflections and rotations should be generated.
            // This is the L shape in Tetris.
            let base = CellInfo {
                cells: vec![
                    Cell { x: 0, y: 0 },
                    Cell { x: 0, y: 1 },
                    Cell { x: 0, y: 2 },
                    Cell { x: 1, y: 0 },
                ],
                max_x: 1,
                max_y: 2,
            };

            let positions: Vec<CellInfo> = base.generate_rotations_and_transpositions().collect();
            assert_eq!(
                positions,
                vec![
                    // *
                    // *
                    // * *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 0, y: 2 },
                            Cell { x: 1, y: 0 }
                        ],
                        max_x: 1,
                        max_y: 2
                    },
                    // * *
                    // *
                    // *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 0, y: 2 },
                            Cell { x: 1, y: 2 }
                        ],
                        max_x: 1,
                        max_y: 2
                    },
                    // *
                    // * * *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 2, y: 0 }
                        ],
                        max_x: 2,
                        max_y: 1
                    },
                    // * * *
                    // *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 1 },
                            Cell { x: 2, y: 1 }
                        ],
                        max_x: 2,
                        max_y: 1
                    },
                    //   *
                    //   *
                    // * *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 1, y: 1 },
                            Cell { x: 1, y: 2 }
                        ],
                        max_x: 1,
                        max_y: 2
                    },
                    //     *
                    // * * *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 2, y: 0 },
                            Cell { x: 2, y: 1 }
                        ],
                        max_x: 2,
                        max_y: 1
                    },
                    // * * *
                    //     *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 1 },
                            Cell { x: 2, y: 0 },
                            Cell { x: 2, y: 1 }
                        ],
                        max_x: 2,
                        max_y: 1
                    },
                    // * *
                    //   *
                    //   *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 2 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 1, y: 1 },
                            Cell { x: 1, y: 2 }
                        ],
                        max_x: 1,
                        max_y: 2
                    }
                ]
            );
        }

        #[test]
        fn generates_all_variants_for_fully_symmetric_piece() {
            // The block is fully symmetric.
            let base = CellInfo {
                cells: vec![
                    Cell { x: 0, y: 0 },
                    Cell { x: 0, y: 1 },
                    Cell { x: 1, y: 0 },
                    Cell { x: 1, y: 1 },
                ],
                max_x: 1,
                max_y: 1,
            };

            assert_eq!(
                base.generate_rotations_and_transpositions()
                    .collect::<Vec<_>>(),
                vec![base]
            );
        }

        #[test]
        fn generates_all_variants_for_partially_symmetric_piece() {
            // The three-cell L shape.
            let base = CellInfo {
                cells: vec![
                    Cell { x: 0, y: 0 },
                    Cell { x: 0, y: 1 },
                    Cell { x: 1, y: 0 },
                ],
                max_x: 1,
                max_y: 1,
            };

            assert_eq!(
                base.generate_rotations_and_transpositions()
                    .collect::<Vec<_>>(),
                vec![
                    // *
                    // * *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 0 }
                        ],
                        max_x: 1,
                        max_y: 1
                    },
                    // * *
                    // *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 1 }
                        ],
                        max_x: 1,
                        max_y: 1
                    },
                    //   *
                    // * *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 0 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 1, y: 1 }
                        ],
                        max_x: 1,
                        max_y: 1
                    },
                    // * *
                    //   *
                    CellInfo {
                        cells: vec![
                            Cell { x: 0, y: 1 },
                            Cell { x: 1, y: 0 },
                            Cell { x: 1, y: 1 }
                        ],
                        max_x: 1,
                        max_y: 1
                    }
                ]
            );
        }
    }

    mod iterator {
        use crate::backtracking::dancing_polynominoes::{
            Cell, Polynomino, PolynominoIterator, SimpleBox,
        };
        use claim::assert_ok;

        #[test]
        fn single_item_filling_box() {
            let box_piece = assert_ok!(Polynomino::new(
                'B',
                vec![
                    Cell { x: 0, y: 0 },
                    Cell { x: 0, y: 1 },
                    Cell { x: 1, y: 0 },
                    Cell { x: 1, y: 1 },
                ],
            ));
            let shape = SimpleBox {
                width: 2,
                height: 2,
            };

            let iterator = assert_ok!(PolynominoIterator::new(vec![box_piece], shape));

            assert_eq!(iterator.count(), 1);
        }

        #[test]
        fn two_item_filling_box() {
            // *
            // * *
            let l_piece = assert_ok!(Polynomino::new(
                'L',
                vec![
                    Cell { x: 0, y: 0 },
                    Cell { x: 0, y: 1 },
                    Cell { x: 1, y: 0 },
                ],
            ));
            // * *
            // * * *
            let p_piece = assert_ok!(Polynomino::new(
                'P',
                vec![
                    Cell { x: 0, y: 0 },
                    Cell { x: 0, y: 1 },
                    Cell { x: 0, y: 2 },
                    Cell { x: 1, y: 0 },
                    Cell { x: 1, y: 1 }
                ],
            ));

            let shape_wide = SimpleBox {
                width: 4,
                height: 2,
            };
            let iterator_wide = assert_ok!(PolynominoIterator::new(
                vec![l_piece.clone(), p_piece.clone()],
                shape_wide
            ));
            assert_eq!(iterator_wide.count(), 4);

            // Rotating the box shouldn't affect anything.
            let shape_tall = SimpleBox {
                width: 2,
                height: 4,
            };
            let iterator_tall =
                assert_ok!(PolynominoIterator::new(vec![l_piece, p_piece], shape_tall));
            assert_eq!(iterator_tall.count(), 4);
        }
    }
}
