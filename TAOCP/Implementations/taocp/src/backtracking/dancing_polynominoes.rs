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
    cells: Vec<Cell>,
    max_x: u8,
    max_y: u8,
}

#[derive(Debug)]
struct CellWithLabel {
    label: char,
    cell: Cell,
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

pub struct PolynominoIterator {
    inner: DancingLinksIterator<PolynominoItem, PolynominoOption>,
}

#[derive(Debug)]
pub struct PolynominoSolution {
    cells: Vec<CellWithLabel>,
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

impl fmt::Display for PolynominoSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cells.is_empty() {
            return write!(f, "<empty>");
        }
        let mut max_x = self.cells[0].cell.x;
        let mut max_y = self.cells[0].cell.y;
        for cell in self.cells.iter().skip(1) {
            max_x = std::cmp::max(max_x, cell.cell.x);
            max_y = std::cmp::max(max_y, cell.cell.y);
        }

        let mut shape = vec![vec!['.'; max_x as usize]; max_y as usize];
        for cell in &self.cells {
            shape[cell.cell.y as usize][cell.cell.x as usize] = cell.label;
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
}
