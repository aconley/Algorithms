//! The Petersen graph: the standard counterexample, and the clearest case for
//! why "no Hamiltonian cycle exists" is an answer rather than a failure.
//!
//! It is 3-regular, vertex-transitive, and has **no** Hamiltonian cycle — yet
//! it does have a Hamiltonian path, and deleting any single vertex leaves a
//! graph that *is* Hamiltonian.  So it separates the two questions this crate
//! answers, and it separates `Ok(None)` (a proof) from `Err(_)` (the search
//! gave up), which is the distinction the whole error design turns on.
//!
//! Also the natural showcase for the SVG renderer: unlike the board
//! generators, `petersen()` has no side table, because the graph has no
//! inherent geometry.  The familiar outer-pentagon / inner-pentagram drawing
//! is a convention, so the coordinates are computed here rather than shipped
//! with the generator.
//!
//! ```text
//! cargo run --example petersen
//! ```

use std::f64::consts::PI;

use taocp_hamiltonian_paths::{
    find_hamiltonian_cycle, find_hamiltonian_path, generators::petersen, render, Config,
};

/// Radius of the outer pentagon, in SVG user units.
const OUTER_RADIUS: f64 = 100.0;
/// Radius of the inner pentagram.  Roughly 45% of the outer radius is what
/// makes the five radii visible without the inner edges crowding the centre.
const INNER_RADIUS: f64 = 45.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = petersen();

    print!("Hamiltonian cycle: ");
    match find_hamiltonian_cycle(&graph, Config::default())? {
        Some(cycle) => println!("{cycle}"),
        None => println!("none exists — proved, not merely unfound"),
    }

    print!("Hamiltonian path:  ");
    let path = match find_hamiltonian_path(&graph, Config::default())? {
        Some(path) => {
            println!("{path}");
            Some(path)
        }
        None => {
            println!("none exists");
            None
        }
    };

    // Draw whichever answer we got, over the conventional layout.
    if let Some(path) = path {
        let coords = pentagon_layout();
        let svg = render::svg(&graph, &path, &coords);
        let out = std::env::temp_dir().join("petersen.svg");
        std::fs::write(&out, svg)?;
        println!("\nwrote {}", out.display());
    }

    Ok(())
}

/// Coordinates for the conventional drawing, indexed by node index exactly as
/// a generator's side table would be.
///
/// `petersen()` numbers the outer pentagon `0..5` and the inner pentagram
/// `5..10`, with vertex `i` joined to vertex `5 + i`, so both rings share an
/// angle and differ only in radius.
fn pentagon_layout() -> Vec<(f64, f64)> {
    let mut coords = Vec::with_capacity(10);
    for radius in [OUTER_RADIUS, INNER_RADIUS] {
        for i in 0..5 {
            // Start at twelve o'clock so the drawing sits the way it is
            // usually printed, then step a fifth of a turn per vertex.
            let angle = -PI / 2.0 + (i as f64) * 2.0 * PI / 5.0;
            coords.push((radius * angle.cos(), radius * angle.sin()));
        }
    }
    coords
}
