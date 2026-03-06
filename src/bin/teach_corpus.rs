/// teach_corpus — Insegna corpus di testo italiano a Prometeo.
///
/// Legge data/external/corpus_italiano.txt (generato da fetch_leipzig.py)
/// e insegna ogni frase con teach(), mostrando progresso e statistiche.
///
/// Uso:
///   cargo run --release --bin teach-corpus
///   cargo run --release --bin teach-corpus -- --max 50000
///   cargo run --release --bin teach-corpus -- --file data/external/altro.txt

use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;
use prometeo::topology::engine::PrometeoTopologyEngine;
use prometeo::topology::persistence::PrometeoState;

// Salva ogni N frasi insegnate
const SAVE_EVERY: usize = 5_000;

// Mostra progresso ogni N frasi
const REPORT_EVERY: usize = 500;

fn load_engine(bin_path: &Path) -> PrometeoTopologyEngine {
    if bin_path.exists() {
        match PrometeoState::load_from_binary(bin_path) {
            Ok(state) => {
                let mut engine = PrometeoTopologyEngine::new();
                state.restore_lexicon(&mut engine);
                println!("  Stato caricato: {} parole, {} simplici, {} archi",
                    engine.lexicon.word_count(),
                    engine.complex.count(),
                    engine.word_topology.edge_count());
                engine
            }
            Err(e) => {
                eprintln!("  ERRORE caricamento: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("  File .bin non trovato: {:?}", bin_path);
        std::process::exit(1);
    }
}

fn save_state(engine: &PrometeoTopologyEngine, bin_path: &Path) {
    let state = PrometeoState::capture(engine);
    match state.save_to_binary(bin_path) {
        Ok(()) => {},
        Err(e) => eprintln!("\n  WARN: errore salvataggio: {}", e),
    }
}

fn main() {
    // Argomenti semplici
    let args: Vec<String> = std::env::args().collect();
    let mut corpus_path = Path::new("data/external/corpus_italiano.txt").to_path_buf();
    let mut max_sentences: Option<usize> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" | "-f" if i + 1 < args.len() => {
                corpus_path = Path::new(&args[i + 1]).to_path_buf();
                i += 2;
            }
            "--max" | "-n" if i + 1 < args.len() => {
                max_sentences = args[i + 1].parse().ok();
                i += 2;
            }
            _ => { i += 1; }
        }
    }

    println!("================================================================");
    println!("  TEACH CORPUS — Integrazione corpus italiano in Prometeo");
    println!("================================================================\n");

    let bin_path = Path::new("prometeo_topology_state.bin");

    // Verifica che il corpus esista
    if !corpus_path.exists() {
        eprintln!("Corpus non trovato: {:?}", corpus_path);
        eprintln!("Esegui prima: cd data/external && py -3 fetch_leipzig.py");
        std::process::exit(1);
    }

    // Leggi il corpus
    println!("Lettura corpus: {:?}", corpus_path);
    let content = match std::fs::read_to_string(&corpus_path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Errore lettura: {}", e); std::process::exit(1); }
    };

    // Conta frasi valide (non commenti, non vuote)
    let sentences: Vec<&str> = content.lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
        .collect();

    let total = max_sentences.unwrap_or(sentences.len()).min(sentences.len());
    println!("Frasi da insegnare: {}/{}", total, sentences.len());

    // Carica lo stato
    println!("\nCaricamento stato Prometeo...");
    let mut engine = load_engine(bin_path);

    let archi_prima = engine.word_topology.edge_count();
    let parole_prima = engine.lexicon.word_count();

    println!("\n  [Inizio insegnamento — salva ogni {} frasi]", SAVE_EVERY);
    println!("{}", "─".repeat(64));

    let t0 = Instant::now();
    let mut known_total: usize = 0;
    let mut new_total: usize = 0;
    let mut frasi_ok: usize = 0;

    for (idx, sentence) in sentences.iter().take(total).enumerate() {
        let sentence = sentence.trim();
        if sentence.is_empty() { continue; }

        let result = engine.teach(sentence);
        known_total += result.known_count;
        new_total += result.new_count;
        frasi_ok += 1;

        // Progress report
        if frasi_ok % REPORT_EVERY == 0 {
            let elapsed = t0.elapsed().as_secs_f64();
            let rate = frasi_ok as f64 / elapsed;
            let remaining = (total - frasi_ok) as f64 / rate;
            let archi_ora = engine.word_topology.edge_count();

            print!("\r  [{:>6}/{:>6}]  {:.0} fr/s  +{} archi  parole note/nuove={}/{}  ~{:.0}s rimasti   ",
                frasi_ok, total, rate,
                archi_ora.saturating_sub(archi_prima),
                known_total, new_total,
                remaining);
            io::stdout().flush().ok();
        }

        // Salvataggio intermedio
        if frasi_ok % SAVE_EVERY == 0 {
            print!("\n  [Salvataggio a {} frasi...]", frasi_ok);
            io::stdout().flush().ok();
            save_state(&engine, bin_path);
            println!(" ok");
        }

        let _ = idx; // suppress warning
    }

    println!();
    println!("{}", "─".repeat(64));

    // Ricalibrazione finale (reinforce_bridges va fatto in processo fresco via fix-post-corpus)
    println!("\n  [Ricalibrazione dimensioni emergenti...]");
    engine.recalibrate_emergent_dimensions();

    // Statistiche finali
    let elapsed = t0.elapsed();
    let archi_dopo = engine.word_topology.edge_count();
    let parole_dopo = engine.lexicon.word_count();

    println!("\n{}", "═".repeat(64));
    println!("  RISULTATI");
    println!("{}", "═".repeat(64));
    println!("  Frasi insegnate:    {:>8}", frasi_ok);
    println!("  Parole note:        {:>8}", known_total);
    println!("  Parole nuove:       {:>8}", new_total);
    println!("  Parole lessico:     {:>8} → {}", parole_prima, parole_dopo);
    println!("  Archi word_topology:{:>8} → {} (+{})",
        archi_prima, archi_dopo,
        archi_dopo.saturating_sub(archi_prima));
    println!("  Simplici:           {:>8}", engine.complex.count());
    println!("  Tempo totale:       {:>8.1}s ({:.0} fr/s)",
        elapsed.as_secs_f64(),
        frasi_ok as f64 / elapsed.as_secs_f64());

    // Salvataggio finale
    println!("\n  [Salvataggio finale...]");
    save_state(&engine, bin_path);
    println!("  Salvato: {:?}", bin_path);

    println!("\n================================================================");
    println!("  Fine TEACH CORPUS");
    println!("================================================================");
}
