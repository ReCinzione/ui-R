/// Pulisce i nomi dei vicini nelle co-occorrenze di ogni WordSnapshot.
/// Dopo clean_lexicon, le parole principali sono pulite ma i loro vicini
/// co-occorrenti potrebbero ancora avere nomi malformati ("fisica:" invece di "fisica").
/// Questo binary li rinomina e aggrega i duplicati.

use prometeo::topology::lexicon::clean_token;
use prometeo::topology::persistence::PrometeoState;

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

    let mut total_cleaned = 0usize;
    let mut total_removed = 0usize;

    for ws in &mut state.lexicon.words {
        let mut co   = std::mem::take(&mut ws.co_occurrences);
        let mut cneg = std::mem::take(&mut ws.co_negated);
        let mut caff = std::mem::take(&mut ws.co_affirmed);

        ws.co_occurrences = clean_colist(co, &mut total_cleaned, &mut total_removed);
        ws.co_negated     = clean_colist(cneg, &mut total_cleaned, &mut total_removed);
        ws.co_affirmed    = clean_colist(caff, &mut total_cleaned, &mut total_removed);
    }

    println!("Co-occorrenze normalizzate: {}", total_cleaned);
    println!("Co-occorrenze rimosse (non-alfa): {}", total_removed);

    let backup = std::path::Path::new("prometeo_topology_state.bin.bak3");
    if let Err(e) = std::fs::copy(path, backup) {
        eprintln!("Warning: impossibile creare backup: {}", e);
    } else {
        println!("Backup salvato in: prometeo_topology_state.bin.bak3");
    }

    match state.save_to_binary(path) {
        Ok(()) => println!("Stato salvato in: prometeo_topology_state.bin"),
        Err(e) => { eprintln!("Errore salvataggio: {}", e); std::process::exit(1); }
    }
}

/// Pulisce una lista (neighbor, count): rinomina vicini malformati, aggrega duplicati.
fn clean_colist(
    raw: Vec<(String, u64)>,
    total_cleaned: &mut usize,
    total_removed: &mut usize,
) -> Vec<(String, u64)> {
    let mut out: Vec<(String, u64)> = Vec::with_capacity(raw.len());
    for (neighbor, count) in raw {
        match clean_token(&neighbor) {
            None => {
                *total_removed += 1;
            }
            Some(clean) if clean == neighbor => {
                // già pulito: aggiungi o fondi
                merge_coentry(&mut out, clean, count);
            }
            Some(clean) => {
                *total_cleaned += 1;
                merge_coentry(&mut out, clean, count);
            }
        }
    }
    out
}

fn merge_coentry(list: &mut Vec<(String, u64)>, key: String, count: u64) {
    if let Some(entry) = list.iter_mut().find(|(w, _)| w == &key) {
        entry.1 = entry.1.saturating_add(count);
    } else {
        list.push((key, count));
    }
}
