/// Importa il POS lookup esterno (Morph-it + Word Frequency Lists ITA)
/// nel lessico Prometeo. Aggiorna SOLO parole attualmente senza POS.
///
/// Input:  data/external/pos_lookup.tsv  →  lemma TAB POS (V/N/Adj/Adv/Pro)
/// Output: prometeo_topology_state.bin   (aggiornato)

use prometeo::topology::grammar::PartOfSpeech;
use prometeo::topology::persistence::PrometeoState;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    let bin_path   = std::path::Path::new("prometeo_topology_state.bin");
    let tsv_path   = std::path::Path::new("data/external/pos_lookup.tsv");

    if !bin_path.exists() {
        eprintln!("File .bin non trovato");
        std::process::exit(1);
    }
    if !tsv_path.exists() {
        eprintln!("File pos_lookup.tsv non trovato — esegui build_pos_lookup.py prima");
        std::process::exit(1);
    }

    // ── Carica lookup ──────────────────────────────────────────────────────
    println!("Caricamento pos_lookup.tsv...");
    let file = std::fs::File::open(tsv_path).unwrap();
    let mut lookup: HashMap<String, PartOfSpeech> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut parts = line.splitn(2, '\t');
        let word = match parts.next() { Some(w) => w.to_string(), None => continue };
        let pos_str = match parts.next() { Some(p) => p.trim(), None => continue };
        let pos = match pos_str {
            "V"   => PartOfSpeech::Verb,
            "N"   => PartOfSpeech::Noun,
            "Adj" => PartOfSpeech::Adjective,
            "Adv" => PartOfSpeech::Adverb,
            "Pro" => PartOfSpeech::Pronoun,
            _     => continue,
        };
        lookup.insert(word, pos);
    }
    println!("  {} lemmi nel lookup", lookup.len());

    // ── Carica stato ───────────────────────────────────────────────────────
    let mut state = match PrometeoState::load_from_binary(bin_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Errore caricamento: {}", e); std::process::exit(1); }
    };
    let total = state.lexicon.words.len();
    println!("Parole nel lessico: {}", total);

    // ── Applica POS ────────────────────────────────────────────────────────
    let (mut applied, mut skipped_tagged, mut not_found) =
        (0usize, 0usize, 0usize);
    let (mut n_v, mut n_n, mut n_adj, mut n_adv, mut n_pro) =
        (0usize, 0usize, 0usize, 0usize, 0usize);

    for ws in &mut state.lexicon.words {
        if ws.pos.is_some() {
            skipped_tagged += 1;
            continue; // rispetta i tag già presenti (detect_pos_from_word)
        }
        match lookup.get(&ws.word) {
            Some(pos) => {
                match pos {
                    PartOfSpeech::Verb      => n_v   += 1,
                    PartOfSpeech::Noun      => n_n   += 1,
                    PartOfSpeech::Adjective => n_adj += 1,
                    PartOfSpeech::Adverb    => n_adv += 1,
                    PartOfSpeech::Pronoun   => n_pro += 1,
                }
                ws.pos = Some(pos.clone());
                applied += 1;
            }
            None => { not_found += 1; }
        }
    }

    let pct = |n: usize| 100.0 * n as f64 / total as f64;
    let total_tagged = skipped_tagged + applied;

    println!("\n--- RISULTATI IMPORT POS ---");
    println!("Già taggati (mantenuti): {} ({:.1}%)", skipped_tagged, pct(skipped_tagged));
    println!("Nuovi da lookup:         {} ({:.1}%)", applied, pct(applied));
    println!("Non trovati nel lookup:  {} ({:.1}%)", not_found, pct(not_found));
    println!("TOTALE TAGGATI:          {} / {} ({:.1}%)", total_tagged, total, pct(total_tagged));

    println!("\n--- NUOVI TAG PER TIPO ---");
    println!("  V:   {}", n_v);
    println!("  N:   {}", n_n);
    println!("  Adj: {}", n_adj);
    println!("  Adv: {}", n_adv);
    println!("  Pro: {}", n_pro);

    // ── Campione di nuovi tag per quality check ────────────────────────────
    println!("\n--- CAMPIONE NUOVI N/Adj/Adv (30 parole) ---");
    let mut shown = 0;
    for ws in &state.lexicon.words {
        if shown >= 30 { break; }
        if let Some(pos) = &ws.pos {
            let label = match pos {
                PartOfSpeech::Noun      => "N",
                PartOfSpeech::Adjective => "Adj",
                PartOfSpeech::Adverb    => "Adv",
                PartOfSpeech::Pronoun   => "Pro",
                PartOfSpeech::Verb      => continue,
            };
            println!("  {:28} → {}", ws.word, label);
            shown += 1;
        }
    }

    // ── Salva ──────────────────────────────────────────────────────────────
    let backup = std::path::Path::new("prometeo_topology_state.bin.bak4");
    if let Err(e) = std::fs::copy(bin_path, backup) {
        eprintln!("Warning backup: {}", e);
    } else {
        println!("\nBackup in: prometeo_topology_state.bin.bak4");
    }

    match state.save_to_binary(bin_path) {
        Ok(()) => println!("Salvato: prometeo_topology_state.bin"),
        Err(e) => { eprintln!("Errore salvataggio: {}", e); std::process::exit(1); }
    }
}
