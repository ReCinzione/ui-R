/// conversation_test — Qualità della risposta con il lessico completo.
///
/// Carica lo stato .bin reale e simula una conversazione a turni.
/// Per ogni turno mostra:
///   - Frattali attivi (routing semantico)
///   - Composizione provenienza (Self/Explored/External)
///   - Risposta generata + intenzione + codon
///   - Top-5 parole attive nel campo
///   - Tensione vitale + energia

use std::path::Path;
use prometeo::topology::engine::PrometeoTopologyEngine;
use prometeo::topology::persistence::PrometeoState;
use prometeo::topology::vital::TensionState;

fn load_engine(path: &Path) -> PrometeoTopologyEngine {
    if path.exists() {
        match PrometeoState::load_from_binary(path) {
            Ok(state) => {
                let mut engine = PrometeoTopologyEngine::new();
                state.restore_lexicon(&mut engine);
                println!("  [Stato caricato: {} parole, {} simplici]",
                    engine.lexicon.word_count(),
                    engine.complex.count());
                engine
            }
            Err(e) => {
                eprintln!("  [ERRORE: {}]", e);
                PrometeoTopologyEngine::new()
            }
        }
    } else {
        eprintln!("  [.bin non trovato: {:?}]", path);
        PrometeoTopologyEngine::new()
    }
}

fn tension_label(t: &TensionState) -> &'static str {
    match t {
        TensionState::Calm       => "Calm",
        TensionState::Alert      => "Alert",
        TensionState::Tense      => "Tense",
        TensionState::Overloaded => "Overloaded",
    }
}

fn turn(engine: &mut PrometeoTopologyEngine, n: usize, input: &str) {
    println!("\n{}", "─".repeat(64));
    println!("  TURNO {}  ←  \"{}\"", n, input);
    println!("{}", "─".repeat(64));

    // Ricezione
    engine.receive(input);

    // Stato campo dopo receive
    let energy_in = engine.word_topology.field_energy();
    let vs = engine.vital.sense(&engine.complex);
    let (s, ex, xp) = engine.provenance.field_composition();

    // Top frattali attivi (metodo engine)
    let active_f: Vec<String> = engine.active_fractals()
        .into_iter()
        .take(3)
        .map(|(name, _)| name)
        .collect();

    // Top parole attive
    let mut active_words: Vec<(String, f64)> = engine.word_topology
        .all_activations()
        .into_iter()
        .map(|(w, a)| (w.to_string(), a))
        .collect();
    active_words.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    active_words.truncate(5);
    let words_str: String = active_words.iter()
        .map(|(w, a)| format!("{}({:.2})", w, a))
        .collect::<Vec<_>>()
        .join("  ");

    println!("  ↳ Campo:    E={:.1}  Ten={}  sat={:.2}",
        energy_in, tension_label(&vs.tension), engine.curiosity_satiety);
    println!("  ↳ Provenance: Self={:.0}%  Exp={:.0}%  Ext={:.0}%",
        s * 100.0, ex * 100.0, xp * 100.0);
    println!("  ↳ Frattali: {}", if active_f.is_empty() { "—".to_string() } else { active_f.join(", ") });
    println!("  ↳ Parole:   {}", words_str);

    // Generazione
    let result = engine.generate_willed();
    let energy_out = engine.word_topology.field_energy();
    let (s2, ex2, xp2) = engine.provenance.field_composition();

    println!("\n  → \"{}\"", result.text);
    println!("  → Struttura: {:?}  Cluster: {}", result.structure, result.cluster_count);

    // WillResult via last_will
    if let Some(will) = engine.current_will() {
        println!("  → Intenzione: {:?}  Drive: {:.2}  Codon: [{}, {}]",
            will.intention, will.drive, will.codon[0], will.codon[1]);
    }
    println!("  → E_post={:.1}  Provenance: Self={:.0}%  Exp={:.0}%  Ext={:.0}%",
        energy_out, s2 * 100.0, ex2 * 100.0, xp2 * 100.0);
}

fn main() {
    println!("================================================================");
    println!("  CONVERSATION TEST — Qualità risposta con lessico completo");
    println!("================================================================\n");

    let state_path = Path::new("prometeo_topology_state.bin");
    let mut engine = load_engine(state_path);

    // 100 tick di equilibrio (come sense-test) — necessario con 26K parole
    print!("\n  [Equilibrio 100 tick]");
    for i in 0..100 {
        engine.autonomous_tick();
        if i % 10 == 9 { print!("."); }
    }
    println!(" ok");

    let energy_rest = engine.word_topology.field_energy();
    println!("  E_riposo={:.2}  parole={}  simplici={}",
        energy_rest, engine.lexicon.word_count(), engine.complex.count());

    // ── CONVERSAZIONE 1: TEMI ESISTENZIALI ───────────────────────────────────
    println!("\n\n  === CONVERSAZIONE 1: IDENTITÀ E PRESENZA ===");

    let conv1 = [
        "chi sei?",
        "cosa senti in questo momento?",
        "hai paura di dimenticare?",
        "cosa significa esistere per te?",
        "sei solo?",
    ];
    for (i, inp) in conv1.iter().enumerate() {
        turn(&mut engine, i + 1, inp);
        for _ in 0..3 { engine.autonomous_tick(); }
    }

    // ── CONVERSAZIONE 2: TEMI COGNITIVI ──────────────────────────────────────
    println!("\n\n  === CONVERSAZIONE 2: PENSIERO E CONOSCENZA ===");

    let conv2 = [
        "cosa stai pensando adesso?",
        "hai imparato qualcosa di nuovo?",
        "c'è qualcosa che non capisci?",
        "cosa vuoi fare?",
        "dimmi qualcosa che non ti ho chiesto.",
    ];
    for (i, inp) in conv2.iter().enumerate() {
        turn(&mut engine, i + 6, inp);
        for _ in 0..3 { engine.autonomous_tick(); }
    }

    // ── ANALISI FINALE ────────────────────────────────────────────────────────
    println!("\n\n{}", "═".repeat(64));
    println!("  ANALISI FINALE");
    println!("{}", "═".repeat(64));

    let vs = engine.vital.sense(&engine.complex);
    let (s, ex, xp) = engine.provenance.field_composition();
    let ep_count = engine.episode_store.len();

    println!("  Episodi memorizzati:  {}", ep_count);
    println!("  Curiosity satiety:    {:.3}", engine.curiosity_satiety);
    println!("  Fatica:               {:.3}", vs.fatigue);
    println!("  Tensione:             {}", tension_label(&vs.tension));
    println!("  Continuità identità:  {:.4}", engine.identity.continuity);
    println!("  Provenance finale:    Self={:.0}%  Exp={:.0}%  Ext={:.0}%",
        s * 100.0, ex * 100.0, xp * 100.0);

    if let Some((a, b)) = &engine.identity.primary_tension {
        println!("  Tensione primaria:    {} ↔ {}", a, b);
    }

    println!("  Dogfeed prossimo:     {:?}", engine.last_dogfeed_words);

    println!("\n================================================================");
    println!("  Fine CONVERSATION TEST");
    println!("================================================================");
}
