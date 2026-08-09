//! Knight's tours, the classic instance of both questions this crate answers.
//!
//! A *closed* tour is a Hamiltonian cycle of the knight's-move graph; an
//! *open* tour is a Hamiltonian path.  The 6x6 board has both.  The 5x5 board
//! has an open tour but provably no closed one — its squares split 13/12
//! between the two colours, and a closed tour must alternate colours, so it
//! would need them equal.  That makes one board a witness and the other a
//! proof, which is the distinction `Ok(Some(_))` and `Ok(None)` carry.
//!
//! ```text
//! cargo run --example knights_tour
//! ```

use taocp_hamiltonian_paths::{
    find_hamiltonian_cycle, find_hamiltonian_path, generators::knight_graph, render,
    Config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    closed_tour(6, 6)?;
    open_tour(5, 5)?;
    no_closed_tour(5, 5)?;
    Ok(())
}

/// Finds a closed tour and prints the board with each square's visit number.
fn closed_tour(ranks: usize, files: usize) -> Result<(), Box<dyn std::error::Error>> {
    let (graph, squares) = knight_graph(ranks, files);

    println!("{ranks}x{files} closed tour (Hamiltonian cycle):\n");
    match find_hamiltonian_cycle(&graph, Config::default())? {
        Some(cycle) => {
            print!("{}", render::board_ascii(&cycle, &squares, ranks, files));
            println!("\n  {cycle}\n");
        }
        None => println!("  none exists\n"),
    }
    Ok(())
}

/// Finds an open tour and prints the board, plus the two squares it ends on.
fn open_tour(ranks: usize, files: usize) -> Result<(), Box<dyn std::error::Error>> {
    let (graph, squares) = knight_graph(ranks, files);

    println!("{ranks}x{files} open tour (Hamiltonian path):\n");
    match find_hamiltonian_path(&graph, Config::default())? {
        Some(path) => {
            print!("{}", render::board_ascii(&path, &squares, ranks, files));
            let (start, end) = path.endpoints();
            let (from, to) = (squares[start.index()], squares[end.index()]);
            println!(
                "\n  from rank {} file {} to rank {} file {}\n",
                from.rank, from.file, to.rank, to.file
            );
        }
        None => println!("  none exists\n"),
    }
    Ok(())
}

/// Shows the other outcome: `Ok(None)` is a proof, not a failure to search.
fn no_closed_tour(ranks: usize, files: usize) -> Result<(), Box<dyn std::error::Error>> {
    let (graph, _) = knight_graph(ranks, files);

    print!("{ranks}x{files} closed tour: ");
    match find_hamiltonian_cycle(&graph, Config::default())? {
        Some(cycle) => println!("found {cycle}"),
        None => println!("none exists — proved, not merely unfound"),
    }
    Ok(())
}
