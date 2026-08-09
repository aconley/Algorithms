//! One image per refinement round: a flipbook of the abstraction tightening.
//!
//! This is what [`Search`] exists for.  The abstraction the solver starts from
//! is "every vertex has exactly one predecessor and one successor", which a
//! *set of disjoint cycles* satisfies just as well as a single spanning one.
//! When the solver returns such a cover, a round adds clauses ruling that
//! particular fragmentation out, and the search goes again — until either one
//! spanning cycle is left, or the abstraction goes unsatisfiable and no
//! Hamiltonian cycle exists.
//!
//! Watching that happen is more illuminating than the answer alone, and it is
//! why the renderers take a whole `Decomposition` rather than a single tour:
//! the final answer is the degenerate one-segment case of the same picture.
//!
//! **Why the Petersen graph and not a knight's board.**  Measured across this
//! crate's generators, refinement almost never engages: every satisfiable
//! knight, grid and random instance tried — including boards up to 10x10 —
//! was settled by the *first* solve, with zero refinement rounds, and only 3
//! of ~210 near-threshold random graphs needed even one.  CaDiCaL finds a
//! spanning cycle directly.  The Petersen graph is the one instance here that
//! reliably fragments: it has no Hamiltonian cycle, so the loop runs until the
//! abstraction is exhausted, giving a flipbook with something in it.
//!
//! ```text
//! cargo run --example refinement_flipbook [output-dir]
//! ```
//!
//! Writes `round-00.svg`, `round-01.svg`, … to `output-dir`, defaulting to a
//! directory under the system temp dir.

use std::f64::consts::PI;
use std::path::PathBuf;

use taocp_hamiltonian_paths::{generators::petersen, render, Config, Progress, Search};

/// Radius of the outer pentagon, in SVG user units.
const OUTER_RADIUS: f64 = 100.0;
/// Radius of the inner pentagram.
const INNER_RADIUS: f64 = 45.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = match std::env::args().nth(1) {
        Some(dir) => PathBuf::from(dir),
        None => std::env::temp_dir().join("taocp-flipbook"),
    };
    std::fs::create_dir_all(&out_dir)?;

    let graph = petersen();
    let coords = pentagon_layout();

    let mut search = Search::new(&graph, Config::default())?;
    let mut round = 0;

    let outcome = loop {
        let progress = search.step()?;

        // Draw the cover this round proposed, spurious or final.  `cover` is
        // None only before the first step, or when the precondition checks
        // settled the question without ever consulting the solver.
        if let Some(cover) = search.cover() {
            let cover = cover?;
            let path = out_dir.join(format!("round-{round:02}.svg"));
            std::fs::write(&path, render::svg(&graph, &cover, &coords))?;
            println!(
                "round {round:2}: {} cycle(s) covering {} vertices -> {}",
                cover.len(),
                cover.covered_vertices(),
                path.display()
            );
            round += 1;
        }

        match progress {
            Progress::Refining { .. } => continue,
            settled => break settled,
        }
    };

    println!();
    match outcome {
        Progress::Found(cycle) => println!("found a Hamiltonian cycle: {cycle}"),
        Progress::NoCycle => {
            println!("no Hamiltonian cycle exists — the abstraction was refined");
            println!("until it went unsatisfiable, which is a proof, not a timeout.");
        }
        Progress::LimitReached => println!("gave up: resource limit reached"),
        Progress::Refining { .. } => unreachable!("the loop breaks on settling"),
    }

    let stats = search.stats();
    println!(
        "\n{} refinement rounds, {} initial clauses, {} added by refinement",
        stats.rounds, stats.initial_clauses, stats.refinement_clauses
    );
    println!("cycles per round: {:?}", stats.segments_per_round);

    Ok(())
}

/// Coordinates for the conventional Petersen drawing, indexed by node index.
///
/// `petersen()` numbers the outer pentagon `0..5` and the inner pentagram
/// `5..10`, with vertex `i` joined to vertex `5 + i`, so both rings share an
/// angle and differ only in radius.
fn pentagon_layout() -> Vec<(f64, f64)> {
    let mut coords = Vec::with_capacity(10);
    for radius in [OUTER_RADIUS, INNER_RADIUS] {
        for i in 0..5 {
            let angle = -PI / 2.0 + (i as f64) * 2.0 * PI / 5.0;
            coords.push((radius * angle.cos(), radius * angle.sin()));
        }
    }
    coords
}
