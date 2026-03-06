/// Rilegge i libri già insegnati a Prometeo.
/// Con il lessico espanso a 26K parole (BigBang), le stesse frasi
/// creano connessioni molto più dense tra i cluster semantici.
///
/// Uso: cargo run --release --bin read_books

use prometeo::topology::persistence::PrometeoState;
use prometeo::PrometeoTopologyEngine;
use std::collections::HashSet;
use std::path::Path;
use std::time::Instant;

fn split_sentences(text: &str) -> Vec<&str> {
    // Divide per punteggiatura terminale
    let mut result = Vec::new();
    let mut start = 0;
    for (i, c) in text.char_indices() {
        if matches!(c, '.' | '!' | '?' | ';') {
            let s = text[start..=i].trim();
            if s.len() >= 5 {
                result.push(s);
            }
            start = i + c.len_utf8();
        }
    }
    let tail = text[start..].trim();
    if tail.len() >= 5 {
        result.push(tail);
    }
    result
}

fn read_book(engine: &mut PrometeoTopologyEngine, path: &Path) -> (usize, usize) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("  Errore: {}", e); return (0, 0); }
    };

    let words_before = engine.lexicon.word_count();
    let normalized = content.replace("\r\n", "\n").replace('\r', "\n");
    let paragraphs: Vec<&str> = normalized
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    let mut sentence_count = 0usize;
    for paragraph in &paragraphs {
        let sentences = split_sentences(paragraph);
        for sentence in &sentences {
            engine.teach(sentence);
            sentence_count += 1;
            // Ogni 15 frasi: receive() per riflessione profonda
            if sentence_count % 15 == 0 {
                engine.receive(sentence);
            }
        }
    }

    let words_after = engine.lexicon.word_count();
    (sentence_count, words_after.saturating_sub(words_before))
}

fn main() {
    let bin_path  = Path::new("prometeo_topology_state.bin");
    let books_dir = Path::new("books");

    if !bin_path.exists() {
        eprintln!("Stato .bin non trovato"); std::process::exit(1);
    }
    if !books_dir.exists() {
        eprintln!("Directory books/ non trovata"); std::process::exit(1);
    }

    // ── Carica ────────────────────────────────────────────────────────────
    println!("Caricamento stato Prometeo...");
    let state = match PrometeoState::load_from_binary(bin_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Errore: {}", e); std::process::exit(1); }
    };
    let words_start = state.lexicon.words.len();
    println!("Parole nel lessico: {}", words_start);

    let mut engine = PrometeoTopologyEngine::new();
    state.restore_lexicon(&mut engine);

    // ── Leggi tutti i libri in books/ ─────────────────────────────────────
    let mut book_files: Vec<_> = std::fs::read_dir(books_dir)
        .expect("Errore apertura books/")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("txt"))
        .collect();
    book_files.sort();

    let t0 = Instant::now();
    let mut total_sentences = 0usize;
    let mut total_new_words = 0usize;

    for book in &book_files {
        let name = book.file_name().unwrap().to_string_lossy();
        println!("\nLettura: {}", name);
        let t_book = Instant::now();
        let (sentences, new_words) = read_book(&mut engine, book);
        println!("  {} frasi, {} nuove parole, {:.1}s",
            sentences, new_words, t_book.elapsed().as_secs_f64());
        total_sentences += sentences;
        total_new_words += new_words;
    }

    let words_end = engine.lexicon.word_count();
    println!("\n--- RISULTATI ---");
    println!("Libri letti:      {}", book_files.len());
    println!("Frasi totali:     {}", total_sentences);
    println!("Parole prima:     {}", words_start);
    println!("Parole dopo:      {}", words_end);
    println!("Nuove parole:     {}", total_new_words);
    println!("Tempo totale:     {:.1}s", t0.elapsed().as_secs_f64());

    // ── Backup + Salva ─────────────────────────────────────────────────────
    let backup = Path::new("prometeo_topology_state.bin.pre_rebooks");
    let _ = std::fs::copy(bin_path, backup);
    println!("\nBackup: prometeo_topology_state.bin.pre_rebooks");

    let state_out = PrometeoState::capture(&engine);
    match state_out.save_to_binary(bin_path) {
        Ok(()) => println!("Stato salvato: prometeo_topology_state.bin"),
        Err(e) => { eprintln!("Errore salvataggio: {}", e); std::process::exit(1); }
    }
}
