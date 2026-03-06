use prometeo::topology::persistence::PrometeoState;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use serde_json::json;

fn main() {
    let bin_path = Path::new("prometeo_topology_state.bin");
    
    println!("=== ANALISI ARCHI PROMETEO ===\n");
    println!("Caricamento stato da {}...", bin_path.display());
    
    let state = match PrometeoState::load_from_binary(bin_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERRORE caricamento: {}", e);
            std::process::exit(1);
        }
    };
    
    let total_words = state.lexicon.words.len();
    let total_simplices = state.complex.simplices.len();
    
    println!("Totale parole: {}", total_words);
    println!("Totale simplessi: {}", total_simplices);
    println!();
    
    // Analizza simplessi
    let mut words_in_simplices = HashSet::new();
    let mut simplices_by_dimension: HashMap<usize, usize> = HashMap::new();
    let mut structure_types: HashMap<String, usize> = HashMap::new();
    let mut words_by_structure_type: HashMap<String, HashSet<String>> = HashMap::new();
    
    for simplex in &state.complex.simplices {
        let dim = simplex.dimension;
        *simplices_by_dimension.entry(dim).or_insert(0) += 1;
        
        // Aggiungi parole coinvolte
        for &vertex_id in &simplex.vertices {
            if let Some(word_data) = state.lexicon.words.get(vertex_id as usize) {
                words_in_simplices.insert(word_data.word.clone());
            }
        }
        
        // Analizza structure_type nelle faces
        for face in &simplex.faces {
            let struct_type = if face.structure_type.is_empty() {
                "unknown".to_string()
            } else {
                face.structure_type.clone()
            };
            *structure_types.entry(struct_type.clone()).or_insert(0) += 1;
            
            // Aggiungi parole per questo structure_type
            for &vertex_id in &simplex.vertices {
                if let Some(word_data) = state.lexicon.words.get(vertex_id as usize) {
                    words_by_structure_type
                        .entry(struct_type.clone())
                        .or_insert_with(HashSet::new)
                        .insert(word_data.word.clone());
                }
            }
        }
    }
    
    // Analizza co-occorrenze
    let mut words_with_cooccurrence = 0;
    let mut words_with_coaffirmed = 0;
    let mut words_with_conegated = 0;
    let mut words_with_affinities = 0;
    
    let mut words_without_cooccurrence = Vec::new();
    let mut words_without_simplices = Vec::new();
    
    for word_data in &state.lexicon.words {
        let word = &word_data.word;
        
        if !word_data.co_occurrences.is_empty() {
            words_with_cooccurrence += 1;
        } else {
            words_without_cooccurrence.push(word.clone());
        }
        
        if !word_data.co_affirmed.is_empty() {
            words_with_coaffirmed += 1;
        }
        
        if !word_data.co_negated.is_empty() {
            words_with_conegated += 1;
        }
        
        if !word_data.fractal_affinities.is_empty() {
            words_with_affinities += 1;
        }
        
        if !words_in_simplices.contains(word) {
            words_without_simplices.push(word.clone());
        }
    }
    
    // Stampa risultati
    println!("================================================================================");
    println!("SIMPLESSI (Relazioni Topologiche)");
    println!("================================================================================");
    println!("Parole coinvolte in simplessi: {}", words_in_simplices.len());
    println!("Parole NON in simplessi: {}", total_words - words_in_simplices.len());
    println!();
    println!("Distribuzione per dimensione:");
    for dim in 0..=3 {
        if let Some(&count) = simplices_by_dimension.get(&dim) {
            println!("  Dimensione {}: {} simplessi", dim, count);
        }
    }
    
    println!();
    println!("================================================================================");
    println!("STRUCTURE TYPES");
    println!("================================================================================");
    for (struct_type, count) in &structure_types {
        let words_count = words_by_structure_type.get(struct_type).map(|s| s.len()).unwrap_or(0);
        println!("  {:30} : {:6} relazioni, {:6} parole", struct_type, count, words_count);
    }
    
    println!();
    println!("================================================================================");
    println!("CO-OCCORRENZE");
    println!("================================================================================");
    println!("Parole con co_occurrences: {}", words_with_cooccurrence);
    println!("Parole SENZA co_occurrences: {}", total_words - words_with_cooccurrence);
    println!();
    println!("Parole con co_affirmed: {}", words_with_coaffirmed);
    println!("Parole SENZA co_affirmed: {}", total_words - words_with_coaffirmed);
    println!();
    println!("Parole con co_negated: {}", words_with_conegated);
    println!("Parole SENZA co_negated: {}", total_words - words_with_conegated);
    
    println!();
    println!("================================================================================");
    println!("AFFINITÀ FRATTALI");
    println!("================================================================================");
    println!("Parole con fractal_affinities: {}", words_with_affinities);
    println!("Parole SENZA fractal_affinities: {}", total_words - words_with_affinities);
    
    // Salva liste
    println!();
    println!("================================================================================");
    println!("SALVATAGGIO LISTE");
    println!("================================================================================");
    
    words_without_simplices.sort();
    words_without_cooccurrence.sort();
    
    std::fs::write("parole_senza_simplessi.txt", words_without_simplices.join("\n"))
        .expect("Errore scrittura parole_senza_simplessi.txt");
    println!("✓ Salvate {} parole in parole_senza_simplessi.txt", words_without_simplices.len());
    
    std::fs::write("parole_senza_cooccurrences.txt", words_without_cooccurrence.join("\n"))
        .expect("Errore scrittura parole_senza_cooccurrences.txt");
    println!("✓ Salvate {} parole in parole_senza_cooccurrences.txt", words_without_cooccurrence.len());
    
    // Salva report JSON dettagliato
    let mut words_by_struct_type_json: HashMap<String, Vec<String>> = HashMap::new();
    for (struct_type, words_set) in &words_by_structure_type {
        let mut words_vec: Vec<String> = words_set.iter().cloned().collect();
        words_vec.sort();
        words_by_struct_type_json.insert(struct_type.clone(), words_vec);
    }
    
    let report = json!({
        "total_words": total_words,
        "total_simplices": total_simplices,
        "simplices": {
            "words_in_simplices": words_in_simplices.len(),
            "words_not_in_simplices": total_words - words_in_simplices.len(),
            "by_dimension": simplices_by_dimension,
        },
        "structure_types": structure_types,
        "words_by_structure_type": words_by_struct_type_json,
        "co_occurrences": {
            "words_with": words_with_cooccurrence,
            "words_without": total_words - words_with_cooccurrence,
        },
        "co_affirmed": {
            "words_with": words_with_coaffirmed,
            "words_without": total_words - words_with_coaffirmed,
        },
        "co_negated": {
            "words_with": words_with_conegated,
            "words_without": total_words - words_with_conegated,
        },
        "fractal_affinities": {
            "words_with": words_with_affinities,
            "words_without": total_words - words_with_affinities,
        },
    });
    
    std::fs::write("edge_analysis_report.json", serde_json::to_string_pretty(&report).unwrap())
        .expect("Errore scrittura edge_analysis_report.json");
    println!("✓ Report salvato in edge_analysis_report.json");
    
    println!();
    println!("================================================================================");
    println!("ANALISI COMPLETATA");
    println!("================================================================================");
}
