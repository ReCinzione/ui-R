/// Diagnostica lessicale post-BigBang.
/// Mostra perché certe parole dominano il campo.

use prometeo::topology::engine::PrometeoTopologyEngine;
use prometeo::topology::persistence::PrometeoState;
use std::path::Path;

fn main() {
    println!("Caricamento...");
    let state = PrometeoState::load_from_binary(Path::new("prometeo_topology_state.bin")).unwrap();
    let mut engine = PrometeoTopologyEngine::new();
    state.restore_lexicon(&mut engine);
    println!("OK: {} parole", engine.lexicon.word_count());

    // Parole sospette viste nel conversation test
    let suspects = ["levarsi", "scripta", "ciclistico", "tendenziale", "variato",
                    "decidi", "entri", "evoluto", "istituto", "lontana"];
    println!("\n--- Diagnostica parole sospette ---");
    println!("{:22} {:6} {:6} {:5} {:6}", "word", "stab", "exp", "archi", "seed");
    for w in &suspects {
        let pat = engine.lexicon.get(w);
        let arc_count = if let Some(id) = engine.word_topology.word_id(w) {
            engine.word_topology.adjacency_list(id).len()
        } else { 0 };
        if let Some(p) = pat {
            println!("{:22} {:.3}  {:6}  {:4}  {:.4}", w, p.stability, p.exposure_count, arc_count, p.stability * 0.08);
        } else {
            println!("{:22} NON TROVATA", w);
        }
    }

    // Top-20 per stabilità totale
    println!("\n--- Top-20 parole per stabilità nel lessico ---");
    println!("{:25} {:6} {:6} {:5} {:6}", "word", "stab", "exp", "archi", "seed");
    let mut all: Vec<(&str, f64, u64, usize)> = engine.lexicon.patterns_iter()
        .map(|(w, p)| {
            let arc_count = if let Some(id) = engine.word_topology.word_id(w) {
                engine.word_topology.adjacency_list(id).len()
            } else { 0 };
            (w.as_str(), p.stability, p.exposure_count, arc_count)
        })
        .collect();
    all.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (w, s, e, a) in all.iter().take(20) {
        println!("{:25} {:.3}  {:6}  {:4}  {:.4}", w, s, e, a, s * 0.08);
    }

    // Distribuzione stability
    let total = engine.lexicon.word_count() as f64;
    let gt90 = engine.lexicon.patterns_iter().filter(|(_, p)| p.stability > 0.90).count();
    let gt70 = engine.lexicon.patterns_iter().filter(|(_, p)| p.stability > 0.70).count();
    let gt50 = engine.lexicon.patterns_iter().filter(|(_, p)| p.stability > 0.50).count();
    let gt20 = engine.lexicon.patterns_iter().filter(|(_, p)| p.stability > 0.20).count();
    let seeded = engine.lexicon.patterns_iter()
        .filter(|(w, p)| {
            p.stability > 0.20 && engine.word_topology.word_id(w)
                .map(|id| !engine.word_topology.adjacency_list(id).is_empty())
                .unwrap_or(false)
        })
        .count();
    println!("\n--- Distribuzione stability ---");
    println!("  stab > 0.90: {:5} ({:.1}%)", gt90, gt90 as f64 / total * 100.0);
    println!("  stab > 0.70: {:5} ({:.1}%)", gt70, gt70 as f64 / total * 100.0);
    println!("  stab > 0.50: {:5} ({:.1}%)", gt50, gt50 as f64 / total * 100.0);
    println!("  stab > 0.20: {:5} ({:.1}%)", gt20, gt20 as f64 / total * 100.0);
    println!("  connesse+seeded: {:5} ({:.1}%)", seeded, seeded as f64 / total * 100.0);
}
