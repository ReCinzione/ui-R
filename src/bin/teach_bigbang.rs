/// Insegna il Big Bang lessicale a Prometeo.
/// Legge data/external/bigbang_lessons.txt (generato da build_bigbang.py)
/// e invoca teach_compact_file() per espandere il lessico da ~7K a ~20K parole.
///
/// Uso: cargo run --release --bin teach_bigbang

use prometeo::topology::persistence::PrometeoState;
use prometeo::PrometeoTopologyEngine;
use std::path::Path;
use std::time::Instant;

fn main() {
    let bin_path     = Path::new("prometeo_topology_state.bin");
    let lessons_path = Path::new("data/external/bigbang_lessons.txt");

    if !bin_path.exists() {
        eprintln!("File .bin non trovato");
        std::process::exit(1);
    }
    if !lessons_path.exists() {
        eprintln!("File bigbang_lessons.txt non trovato");
        eprintln!("Esegui prima: cd data/external && py -3 build_bigbang.py");
        std::process::exit(1);
    }

    // ── Carica stato ───────────────────────────────────────────────────────
    println!("Caricamento stato Prometeo...");
    let state = match PrometeoState::load_from_binary(bin_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Errore caricamento: {}", e); std::process::exit(1); }
    };

    let words_before = state.lexicon.words.len();
    println!("Parole prima del BigBang: {}", words_before);

    let mut engine = PrometeoTopologyEngine::new();
    state.restore_lexicon(&mut engine);

    // ── Insegna BigBang ────────────────────────────────────────────────────
    println!("\nAvvio BigBang lessicale ({})...", lessons_path.display());
    println!("(può richiedere alcuni minuti)\n");

    let t0 = Instant::now();

    match engine.teach_compact_file(lessons_path) {
        Ok((result, _sentences)) => {
            let elapsed = t0.elapsed();
            let words_after = engine.lexicon.word_count();

            println!("--- RISULTATI BIGBANG ---");
            println!("Parole prima:       {}", words_before);
            println!("Parole dopo:        {}", words_after);
            println!("Nuove parole:       {}", words_after.saturating_sub(words_before));
            println!("Parole processate:  {}", result.words_processed.len());
            println!("Parole già note:    {}", result.known_count);
            println!("Parole create:      {}", result.new_count);
            println!("Tempo:              {:.1}s", elapsed.as_secs_f64());
        }
        Err(e) => {
            eprintln!("Errore insegnamento: {}", e);
            std::process::exit(1);
        }
    }

    // ── Backup + Salvataggio ───────────────────────────────────────────────
    let backup = Path::new("prometeo_topology_state.bin.pre_bigbang");
    if let Err(e) = std::fs::copy(bin_path, backup) {
        eprintln!("Warning backup: {}", e);
    } else {
        println!("\nBackup in: prometeo_topology_state.bin.pre_bigbang");
    }

    let state_out = PrometeoState::capture(&engine);
    match state_out.save_to_binary(bin_path) {
        Ok(()) => println!("Stato salvato: prometeo_topology_state.bin"),
        Err(e) => { eprintln!("Errore salvataggio: {}", e); std::process::exit(1); }
    }
}
