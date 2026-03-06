/// fix_post_corpus — Recovery dopo crash di teach-corpus.
///
/// Carica lo stato salvato all'ultima checkpoint (95K frasi),
/// esegue reinforce_bridges() (ora sicuro con cap) e
/// recalibrate_emergent_dimensions(), poi salva.
///
/// Uso:
///   cargo run --release --bin fix-post-corpus

use std::path::Path;
use prometeo::topology::engine::PrometeoTopologyEngine;
use prometeo::topology::persistence::PrometeoState;

fn main() {
    println!("================================================================");
    println!("  FIX POST-CORPUS — Recovery dopo crash teach-corpus");
    println!("================================================================\n");

    let bin_path = Path::new("prometeo_topology_state.bin");

    if !bin_path.exists() {
        eprintln!("File .bin non trovato: {:?}", bin_path);
        std::process::exit(1);
    }

    // Carica stato
    println!("Caricamento stato...");
    let state = match PrometeoState::load_from_binary(bin_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("ERRORE: {}", e); std::process::exit(1); }
    };

    let mut engine = PrometeoTopologyEngine::new();
    state.restore_lexicon(&mut engine);

    println!("  Parole:   {}", engine.lexicon.word_count());
    println!("  Simplici: {}", engine.complex.count());
    println!("  Archi:    {}", engine.word_topology.edge_count());

    // Rinforzo ponti (con fix O(N²))
    println!("\n[Rinforzo ponti semantici — MAX_STABLE=400]...");
    let t0 = std::time::Instant::now();
    let result = engine.reinforce_bridges();
    println!("  Ponti trovati:    {}", result.bridges_found);
    println!("  Ponti rinforzati: {}", result.bridges_reinforced);
    println!("  Simplici creati:  {}", result.simplices_created);
    println!("  Affinita latenti: {}", result.latent_found);
    println!("  Tempo: {:.1}s", t0.elapsed().as_secs_f64());

    // Ricalibrazione dimensioni emergenti
    println!("\n[Ricalibrazione dimensioni emergenti]...");
    let t1 = std::time::Instant::now();
    engine.recalibrate_emergent_dimensions();
    println!("  Tempo: {:.1}s", t1.elapsed().as_secs_f64());

    // Statistiche finali
    println!("\n  Parole:   {}", engine.lexicon.word_count());
    println!("  Simplici: {}", engine.complex.count());
    println!("  Archi:    {}", engine.word_topology.edge_count());

    // Salvataggio finale
    println!("\n[Salvataggio finale...]");
    let state = PrometeoState::capture(&engine);
    match state.save_to_binary(bin_path) {
        Ok(()) => println!("  Salvato: {:?}", bin_path),
        Err(e) => eprintln!("  ERRORE salvataggio: {}", e),
    }

    println!("\n================================================================");
    println!("  Fix completato.");
    println!("================================================================");
}
