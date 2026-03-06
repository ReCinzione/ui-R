/// Test Percezione Visiva — Prometeo può "vedere" SVG?
///
/// Questo esempio testa se Prometeo, con il suo vocabolario geometrico,
/// può percepire e descrivere immagini SVG semplici.

use prometeo::topology::engine::PrometeoTopologyEngine;
use prometeo::topology::persistence::PrometeoState;
use std::path::PathBuf;

fn main() {
    println!("\n=== TEST PERCEZIONE VISIVA SVG ===\n");
    
    // Carica stato esistente
    let state_path = PathBuf::from("prometeo_topology_state.bin");
    let mut engine = if state_path.exists() {
        println!("Caricamento stato esistente...");
        match PrometeoState::load_from_binary(&state_path) {
            Ok(state) => {
                let words = state.lexicon.words.len();
                let mut eng = PrometeoTopologyEngine::new();
                state.restore_lexicon(&mut eng);
                eng.lexicon.apply_curated_signatures();
                eng.recompute_all_word_affinities();
                println!("✓ Stato caricato: {} parole\n", words);
                eng
            }
            Err(e) => {
                println!("✗ Errore caricamento: {}", e);
                println!("Uso engine vuoto (solo test parser)\n");
                PrometeoTopologyEngine::new()
            }
        }
    } else {
        println!("Nessuno stato trovato, uso engine vuoto\n");
        PrometeoTopologyEngine::new()
    };
    
    // Test 1: Cerchio rosso semplice
    println!("--- TEST 1: Cerchio Rosso ---");
    let svg1 = r#"<svg><circle cx="50" cy="50" r="20" fill="red"/></svg>"#;
    test_svg(&mut engine, svg1, "cerchio rosso al centro");
    
    // Test 2: Quadrato blu
    println!("\n--- TEST 2: Quadrato Blu ---");
    let svg2 = r#"<svg><rect x="10" y="10" width="30" height="30" fill="blue"/></svg>"#;
    test_svg(&mut engine, svg2, "quadrato blu");
    
    // Test 3: Due cerchi (relazione spaziale)
    println!("\n--- TEST 3: Due Cerchi (sopra/sotto) ---");
    let svg3 = r#"<svg>
        <circle cx="50" cy="20" r="10" fill="red"/>
        <circle cx="50" cy="80" r="10" fill="blue"/>
    </svg>"#;
    test_svg(&mut engine, svg3, "cerchio rosso sopra cerchio blu");
    
    // Test 4: Composizione complessa
    println!("\n--- TEST 4: Composizione Complessa ---");
    let svg4 = r#"<svg>
        <circle cx="50" cy="50" r="30" fill="yellow"/>
        <circle cx="40" cy="40" r="5" fill="black"/>
        <circle cx="60" cy="40" r="5" fill="black"/>
        <rect x="35" y="60" width="30" height="5" fill="red"/>
    </svg>"#;
    test_svg(&mut engine, svg4, "faccia sorridente (cerchio giallo con occhi e bocca)");
    
    // Test 5: Linee
    println!("\n--- TEST 5: Linee ---");
    let svg5 = r#"<svg>
        <line x1="0" y1="0" x2="100" y2="100" stroke="black"/>
        <line x1="100" y1="0" x2="0" y2="100" stroke="black"/>
    </svg>"#;
    test_svg(&mut engine, svg5, "due linee incrociate (X)");
    
    println!("\n=== RIEPILOGO ===");
    println!("\nQuesto test verifica se Prometeo:");
    println!("1. Ha vocabolario geometrico (cerchio, quadrato, linea, colori)");
    println!("2. Può attivare parole da strutture SVG");
    println!("3. Genera descrizioni coerenti con le forme percepite");
    println!("\nSe le descrizioni sono vuote o incoerenti:");
    println!("→ Manca vocabolario geometrico (serve lezione compact)");
    println!("\nSe le descrizioni usano parole geometriche:");
    println!("→ La percezione visiva funziona! ✓");
    println!();
}

fn test_svg(engine: &mut PrometeoTopologyEngine, svg: &str, expected: &str) {
    println!("SVG: {}", svg.chars().take(80).collect::<String>());
    println!("Atteso: {}", expected);
    
    // Percepisce SVG
    let response = engine.perceive_svg(svg);
    
    println!("\nRisultati:");
    println!("  Concetti rilevati: {}", response.concepts_detected);
    println!("  Parole attivate: {}", response.words_activated.join(", "));
    println!("  Frattali dominanti: {}", response.dominant_fractals.join(", "));
    println!("  Energia campo: {:.3}", response.field_energy);
    println!("\nDescrizione emergente:");
    println!("  \"{}\"", response.description);
    
    // Analisi
    if response.description.is_empty() || response.description == "..." {
        println!("\n⚠ Descrizione vuota - possibile mancanza vocabolario");
    } else {
        // Conta quante parole geometriche sono nella descrizione
        let geo_words = ["cerchio", "quadrato", "linea", "rosso", "blu", "verde", 
                         "giallo", "sopra", "sotto", "vicino", "grande", "piccolo"];
        let geo_count = geo_words.iter()
            .filter(|w| response.description.to_lowercase().contains(*w))
            .count();
        
        if geo_count > 0 {
            println!("\n✓ Descrizione usa {} parole geometriche", geo_count);
        } else {
            println!("\n⚠ Descrizione non usa parole geometriche");
        }
    }
}
