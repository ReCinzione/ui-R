/// Pulisce il lessico nel .bin esistente:
/// - Parole con punteggiatura finale/iniziale (`:`, `.`, `,`, ecc.) → strip + merge
/// - Contrazioni apostrofo (`all'acqua`) → prende parte dopo apostrofo + merge
/// - Parole puramente non-alfabetiche → rimosse
///
/// In caso di conflitto (stessa chiave pulita già presente), la WordSnapshot
/// con esposizione maggiore "vince" la firma/stabilità, ma i conteggi si sommano.

use prometeo::topology::lexicon::clean_token;
use prometeo::topology::persistence::{PrometeoState, WordSnapshot};
use std::collections::HashMap;

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

    let before = state.lexicon.words.len();
    println!("Parole prima della pulizia: {}", before);

    // Costruisce una mappa word → WordSnapshot pulita/merged
    // La chiave è la forma normalizzata (post clean_token)
    let mut merged: HashMap<String, WordSnapshot> = HashMap::new();

    let mut removed = 0usize;
    let mut normalized = 0usize;
    let mut kept = 0usize;

    for ws in std::mem::take(&mut state.lexicon.words) {
        let original = ws.word.clone();
        match clean_token(&original) {
            None => {
                // Parola completamente non-alfabetica → scarta
                removed += 1;
            }
            Some(clean) if clean == original => {
                // Parola già pulita → inserisci o fondi se chiave duplicata
                merge_into(&mut merged, clean, ws);
                kept += 1;
            }
            Some(clean) => {
                // Parola normalizzata → ridenomina e fondi
                println!("  NORM  '{}'  →  '{}'", original, clean);
                merge_into(&mut merged, clean, ws);
                normalized += 1;
            }
        }
    }

    state.lexicon.words = merged.into_values().collect();
    // Ordine deterministico per riproducibilità
    state.lexicon.words.sort_by(|a, b| a.word.cmp(&b.word));

    let after = state.lexicon.words.len();

    println!("\n--- RISULTATI PULIZIA ---");
    println!("Prima: {} parole", before);
    println!("Dopo:  {} parole", after);
    println!("Mantenute identiche: {}", kept);
    println!("Normalizzate:        {}", normalized);
    println!("Rimosse (non-alfa):  {}", removed);
    println!("Merge/deduplicate:   {}", (kept + normalized).saturating_sub(after));

    // Salva il backup prima di sovrascrivere
    let backup = std::path::Path::new("prometeo_topology_state.bin.bak");
    if let Err(e) = std::fs::copy(path, backup) {
        eprintln!("Warning: impossibile creare backup: {}", e);
    } else {
        println!("\nBackup salvato in: prometeo_topology_state.bin.bak");
    }

    match state.save_to_binary(path) {
        Ok(()) => println!("Stato pulito salvato in: prometeo_topology_state.bin"),
        Err(e) => { eprintln!("Errore salvataggio: {}", e); std::process::exit(1); }
    }
}

/// Inserisce o fonde una WordSnapshot nella mappa per chiave `key`.
/// In caso di collisione: firma/stabilità dalla snapshot con più esposizioni,
/// exposure_count sommato, co_occurrences aggregate.
fn merge_into(map: &mut HashMap<String, WordSnapshot>, key: String, mut incoming: WordSnapshot) {
    incoming.word = key.clone();

    if let Some(existing) = map.get_mut(&key) {
        // Mantiene firma/stabilità/affinità della snapshot più esposta
        if incoming.exposure_count > existing.exposure_count {
            existing.signature            = incoming.signature;
            existing.fractal_affinities   = incoming.fractal_affinities;
            existing.stability            = incoming.stability;
            existing.pos                  = incoming.pos.or(existing.pos.take());
        } else {
            existing.pos = existing.pos.take().or(incoming.pos);
        }
        // Somma esposizioni
        existing.exposure_count = existing.exposure_count.saturating_add(incoming.exposure_count);

        // Fondi co-occorrenze: accumula i conteggi per ogni parola vicina
        for (neighbor, count) in incoming.co_occurrences {
            let entry = existing.co_occurrences.iter_mut().find(|(w, _)| w == &neighbor);
            if let Some((_, c)) = entry { *c += count; }
            else { existing.co_occurrences.push((neighbor, count)); }
        }
        for (neighbor, count) in incoming.co_negated {
            let entry = existing.co_negated.iter_mut().find(|(w, _)| w == &neighbor);
            if let Some((_, c)) = entry { *c += count; }
            else { existing.co_negated.push((neighbor, count)); }
        }
        for (neighbor, count) in incoming.co_affirmed {
            let entry = existing.co_affirmed.iter_mut().find(|(w, _)| w == &neighbor);
            if let Some((_, c)) = entry { *c += count; }
            else { existing.co_affirmed.push((neighbor, count)); }
        }
    } else {
        map.insert(key, incoming);
    }
}
