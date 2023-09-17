// Pentomino problems.

use lazy_static::lazy_static;
use std::env;

use taocp::backtracking::dancing_links::DancingLinksError;
use taocp::backtracking::dancing_polynominoes::{
    Cell, Polynomino, PolynominoIterator, PolynominoSolution, SimpleBox,
};

// The pentominoes using the Conway naming scheme.
lazy_static! {
    static ref O_PENTOMINO: Polynomino =
        Polynomino::new('O', (0..5).map(|x| Cell { x: x, y: 0 }).collect()).unwrap();
    static ref P_PENTOMINO: Polynomino = Polynomino::new(
        'P',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 0, y: 2 },
            Cell { x: 1, y: 1 },
            Cell { x: 1, y: 2 },
        ],
    )
    .unwrap();
    static ref Q_PENTOMINO: Polynomino = Polynomino::new(
        'Q',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
            Cell { x: 3, y: 0 },
        ],
    )
    .unwrap();
    static ref R_PENTOMINO: Polynomino = Polynomino::new(
        'R',
        vec![
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 1, y: 2 },
            Cell { x: 2, y: 2 }
        ]
    )
    .unwrap();
    static ref S_PENTOMINO: Polynomino = Polynomino::new(
        'S',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
            Cell { x: 2, y: 1 },
            Cell { x: 3, y: 1 }
        ]
    )
    .unwrap();
    static ref T_PENTOMINO: Polynomino = Polynomino::new(
        'T',
        vec![
            Cell { x: 0, y: 2 },
            Cell { x: 1, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 1, y: 2 },
            Cell { x: 2, y: 2 }
        ]
    )
    .unwrap();
    static ref U_PENTOMINO: Polynomino = Polynomino::new(
        'U',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
            Cell { x: 2, y: 1 }
        ]
    )
    .unwrap();
    static ref V_PENTOMINO: Polynomino = Polynomino::new(
        'V',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 0, y: 2 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 }
        ]
    )
    .unwrap();
    static ref W_PENTOMINO: Polynomino = Polynomino::new(
        'W',
        vec![
            Cell { x: 0, y: 1 },
            Cell { x: 0, y: 2 },
            Cell { x: 1, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 2, y: 0 }
        ]
    )
    .unwrap();
    static ref X_PENTOMINO: Polynomino = Polynomino::new(
        'X',
        vec![
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 0 },
            Cell { x: 1, y: 1 },
            Cell { x: 1, y: 2 },
            Cell { x: 2, y: 1 }
        ]
    )
    .unwrap();
    static ref Y_PENTOMINO: Polynomino = Polynomino::new(
        'Y',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 1, y: 0 },
            Cell { x: 2, y: 0 },
            Cell { x: 2, y: 1 },
            Cell { x: 3, y: 0 }
        ]
    )
    .unwrap();
    static ref Z_PENTOMINO: Polynomino = Polynomino::new(
        'Z',
        vec![
            Cell { x: 0, y: 0 },
            Cell { x: 0, y: 1 },
            Cell { x: 1, y: 1 },
            Cell { x: 2, y: 1 },
            Cell { x: 2, y: 2 }
        ]
    )
    .unwrap();
}

fn pentominoes() -> Vec<Polynomino> {
    vec![
        O_PENTOMINO.clone(),
        P_PENTOMINO.clone(),
        Q_PENTOMINO.clone(),
        R_PENTOMINO.clone(),
        S_PENTOMINO.clone(),
        T_PENTOMINO.clone(),
        U_PENTOMINO.clone(),
        V_PENTOMINO.clone(),
        W_PENTOMINO.clone(),
        X_PENTOMINO.clone(),
        Y_PENTOMINO.clone(),
        Z_PENTOMINO.clone(),
    ]
}

fn solve_box(
    height: u8,
    width: u8,
) -> Result<(usize, Option<PolynominoSolution>), DancingLinksError> {
    let mut iterator = PolynominoIterator::new(pentominoes(), SimpleBox { height, width })?;
    match iterator.next() {
        Some(sol) => Ok((iterator.count() + 1, Some(sol))),
        None => Ok((0, None)),
    }
}

fn main() -> Result<(), DancingLinksError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: pentominoes_box height width");
        return Ok(());
    }
    let height = args[1].parse().unwrap();
    let width = args[2].parse().unwrap();

    let (num_solutions, solution) = solve_box(height, width)?;
    println!("There were {} solutions", num_solutions);
    match solution {
        Some(solution) => println!("Example:\n{}", solution),
        None => (),
    };
    Ok(())
}
