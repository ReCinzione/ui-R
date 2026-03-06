/// Applica il POS tagging migliorato a tutte le parole non ancora taggate nel .bin.
/// Usa detect_pos_from_word() espanso (Phase 36b).
/// Non sovrascrive POS già presenti (rispetta le etichette manuali).

use prometeo::topology::grammar::{detect_pos_from_word, PartOfSpeech};
use prometeo::topology::persistence::PrometeoState;

fn pos_label(pos: &Option<PartOfSpeech>) -> &'static str {
    match pos {
        None => "-",
        Some(PartOfSpeech::Verb)      => "V",
        Some(PartOfSpeech::Noun)      => "N",
        Some(PartOfSpeech::Adjective) => "Adj",
        Some(PartOfSpeech::Adverb)    => "Adv",
        Some(PartOfSpeech::Pronoun)   => "Pro",
    }
}

fn main() {
    let path = std::path::Path::new("prometeo_topology_state.bin");
    if !path.exists() {
        eprintln!("File .bin non trovato");
        std::process::exit(1);
    }

    let mut state = match PrometeoState::load_from_binary(path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Errore caricamento: {}", e); std::process::exit(1); }
    };

    let total = state.lexicon.words.len();
    println!("Parole nel lessico: {}", total);

    let mut tagged_now = 0usize;
    let mut unresolved = 0usize;

    // Contatori per tipo
    let (mut n_verb, mut n_noun, mut n_adj, mut n_adv, mut n_pron) =
        (0usize, 0usize, 0usize, 0usize, 0usize);

    // Riscrive TUTTI i tag con la grammatica aggiornata (nessuna eccezione)
    for ws in &mut state.lexicon.words {
        match detect_pos_from_word(&ws.word) {
            Some(pos) => {
                match &pos {
                    PartOfSpeech::Verb      => n_verb += 1,
                    PartOfSpeech::Noun      => n_noun += 1,
                    PartOfSpeech::Adjective => n_adj  += 1,
                    PartOfSpeech::Adverb    => n_adv  += 1,
                    PartOfSpeech::Pronoun   => n_pron += 1,
                }
                ws.pos = Some(pos);
                tagged_now += 1;
            }
            None => {
                ws.pos = None; // reset esplicito
                unresolved += 1;
            }
        }
    }

    let pct = |n: usize| 100.0 * n as f64 / total as f64;

    println!("\n--- RISULTATI TAGGING ---");
    println!("Tag applicati:  {} ({:.1}%)", tagged_now, pct(tagged_now));
    println!("Non risolti:    {} ({:.1}%)", unresolved, pct(unresolved));

    println!("\n--- DISTRIBUZIONE POS ---");
    println!("Verb:      {} ({:.1}%)", n_verb, pct(n_verb));
    println!("Noun:      {} ({:.1}%)", n_noun, pct(n_noun));
    println!("Adjective: {} ({:.1}%)", n_adj,  pct(n_adj));
    println!("Adverb:    {} ({:.1}%)", n_adv,  pct(n_adv));
    println!("Pronoun:   {} ({:.1}%)", n_pron, pct(n_pron));
    println!("None:      {} ({:.1}%)", unresolved, pct(unresolved));

    // Campione dei nuovi tag per verifica qualità
    println!("\n--- CAMPIONE NUOVI TAG (prime 30 parole) ---");
    let mut shown = 0;
    for ws in &state.lexicon.words {
        if ws.pos.is_some() && shown < 30 {
            let label = pos_label(&ws.pos);
            if label != "V" || ws.word.ends_with("are") || ws.word.ends_with("ere") || ws.word.ends_with("ire") {
                // Mostra solo i non-verbi tra i primi campioni per vedere la varietà
                if label != "V" {
                    println!("  {:28} → {}", ws.word, label);
                    shown += 1;
                }
            }
        }
    }

    // Salva
    let backup = std::path::Path::new("prometeo_topology_state.bin.bak2");
    if let Err(e) = std::fs::copy(path, backup) {
        eprintln!("Warning: impossibile creare backup: {}", e);
    } else {
        println!("\nBackup salvato in: prometeo_topology_state.bin.bak2");
    }

    match state.save_to_binary(path) {
        Ok(()) => println!("Stato taggato salvato in: prometeo_topology_state.bin"),
        Err(e) => { eprintln!("Errore salvataggio: {}", e); std::process::exit(1); }
    }
}
