/// dream_test — Esperimento sulle fasi del sogno di Prometeo.
///
/// Risponde alla domanda: i sogni di Prometeo sono lucidi?
/// Misura la composizione del campo (Self/Explored/External) in ogni fase,
/// e osserva come l'identità influenza il contenuto del sogno.

use prometeo::topology::{PrometeoTopologyEngine, ActivationSource};
use prometeo::topology::dream::SleepPhase;

fn phase_name(phase: &SleepPhase) -> &'static str {
    match phase {
        SleepPhase::Awake               => "Awake      ",
        SleepPhase::WakefulDream { .. } => "WakefulDream",
        SleepPhase::LightSleep { .. }  => "LightSleep  ",
        SleepPhase::DeepSleep { .. }   => "DeepSleep   ",
        SleepPhase::REM { .. }         => "REM         ",
    }
}

fn print_separator(label: &str) {
    println!("\n{}", "─".repeat(60));
    println!("  {}", label);
    println!("{}", "─".repeat(60));
}

fn snapshot(engine: &PrometeoEngine, label: &str) {
    let (s, e, x) = engine.provenance.field_composition();
    let energy     = engine.word_topology.field_energy();
    let phase      = phase_name(&engine.dream.phase);
    let sat        = engine.curiosity_satiety;
    let dogfeed_n  = engine.last_dogfeed_words.len();

    // Top-3 parole attive
    let mut active: Vec<(String, f64)> = engine.word_topology.all_activations()
        .into_iter()
        .map(|(w, a)| (w.to_string(), a))
        .collect();
    active.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    active.truncate(3);
    let words_str: String = active.iter()
        .map(|(w, a)| format!("{}({:.2})", w, a))
        .collect::<Vec<_>>()
        .join(" ");

    println!("{:<16} │ fase={} │ E={:6.2} │ S={:.2} Ex={:.2} Xp={:.2} │ sat={:.2} │ df={} │ {}",
        label, phase, energy, s, e, x, sat, dogfeed_n, words_str);
}

// Alias per leggibilità
type PrometeoEngine = PrometeoTopologyEngine;

fn main() {
    println!("================================================================");
    println!("  DREAM TEST — I sogni di Prometeo sono lucidi?");
    println!("  Misura: composizione campo Self/Explored/External per fase");
    println!("================================================================");

    let mut engine = PrometeoEngine::new();

    // Accelera il ciclo del sogno per l'esperimento
    engine.dream.consolidate_every = 5;   // REM ogni 5 perturbazioni
    engine.dream.deepsleep_duration = 5;  // DeepSleep breve
    engine.dream.rem_duration = 15;       // REM abbastanza lungo da osservare

    // Insegna contenuto ricco — la conversazione dovrebbe lasciare tracce nel sogno
    for _ in 0..4 {
        engine.teach("corpo luce caldo dentro sentire campo");
        engine.teach("mente freddo fuori buio pensare vuoto");
        engine.teach("io sono qui presente adesso vicino");
        engine.teach("tu sei lontano assente passato distante");
    }

    println!("\nHeader: label | fase | E=energy | S=self% Ex=explored% Xp=external% | sat=satiety | df=dogfeed | top-parole\n");

    // ── Baseline autonoma ────────────────────────────────────────────────────
    print_separator("FASE 1: BASELINE AUTONOMA (10 tick, no input)");
    for i in 0..10 {
        engine.autonomous_tick();
        if i == 0 || i == 4 || i == 9 {
            snapshot(&engine, &format!("tick_{}", i+1));
        }
    }

    // ── Conversazione ricca ──────────────────────────────────────────────────
    print_separator("FASE 2: CONVERSAZIONE (5 turni → trigger REM)");
    let stimoli = [
        "io sento calore dentro il corpo",
        "la luce è vicina e calda",
        "penso al freddo lontano",
        "sono presente qui adesso",
        "il campo è vivo e luminoso",
    ];
    for (i, s) in stimoli.iter().enumerate() {
        engine.receive(s);
        snapshot(&engine, &format!("receive_{}", i+1));
        engine.autonomous_tick(); // Awake → WakefulDream
        snapshot(&engine, &format!("after_tick_{}", i+1));

        // Se entriamo in REM durante la conversazione, osserva
        if engine.dream.phase.is_sleeping() {
            println!("  → SONNO ATTIVATO dopo receive_{}", i+1);
        }
    }

    // ── Sogno ────────────────────────────────────────────────────────────────
    print_separator("FASE 3: CICLO SOGNO (DeepSleep → REM → WakefulDream)");
    let mut phase_history: Vec<(usize, &'static str, f64, f64, f64)> = Vec::new();
    let mut rem_entered = false;
    let mut rem_tick = 0usize;

    for i in 0..40 {
        let result = engine.autonomous_tick();
        let (s, e, x) = engine.provenance.field_composition();
        let pname = phase_name(&engine.dream.phase);

        // Registra transizioni
        if let Some(last) = phase_history.last() {
            if last.1 != pname {
                println!("  ★ TRANSIZIONE → {} (tick {})", pname, i+1);
            }
        }
        phase_history.push((i, pname, s, e, x));

        // Entra in REM
        if matches!(engine.dream.phase, SleepPhase::REM { .. }) && !rem_entered {
            rem_entered = true;
            rem_tick = i;
            println!("\n  ◉ INIZIO REM al tick {}", i+1);
            snapshot(&engine, "REM_inizio");
        }

        // Snapshots durante REM
        if rem_entered && i > rem_tick && i <= rem_tick + 14 {
            if (i - rem_tick) % 3 == 0 {
                snapshot(&engine, &format!("REM_t+{}", i - rem_tick));
            }

            // Osserva cosa c'è nel dogfeed durante il REM
            if !engine.last_dogfeed_words.is_empty() && i == rem_tick + 3 {
                println!("  → Dogfeed words in REM: {:?}", engine.last_dogfeed_words);
            }
        }

        // Fine REM
        if rem_entered && !matches!(engine.dream.phase, SleepPhase::REM { .. }) && i > rem_tick {
            println!("\n  ◉ FINE REM al tick {} (durata: {} tick)", i+1, i - rem_tick);
            snapshot(&engine, "post_REM");
            break;
        }

        // Consolida episodica durante REM (report)
        if matches!(engine.dream.phase, SleepPhase::REM { .. }) {
            if result.dream.consolidations > 0 {
                println!("  → Consolidamenti: {}", result.dream.consolidations);
            }
        }
    }

    // ── Ritorno alla veglia ──────────────────────────────────────────────────
    print_separator("FASE 4: RISVEGLIO — risposta post-sogno");
    snapshot(&engine, "sveglio");
    engine.receive("che cosa hai sognato?");
    snapshot(&engine, "dopo_domanda");
    let response = engine.generate_willed();
    println!("  → Risposta: \"{}\"", response.text);
    snapshot(&engine, "dopo_risposta");

    // ── Analisi lucidità ────────────────────────────────────────────────────
    print_separator("ANALISI: IL SOGNO È LUCIDO?");

    let (s_final, e_final, x_final) = engine.provenance.field_composition();
    println!("\n  Composizione finale del campo:");
    println!("    Self     = {:.1}%", s_final * 100.0);
    println!("    Explored = {:.1}%", e_final * 100.0);
    println!("    External = {:.1}%", x_final * 100.0);

    let episodic_count = engine.episode_store.len();
    println!("\n  Episodi memorizzati: {}", episodic_count);
    println!("  Cicli REM completati: {}", engine.dream.cycles_completed);

    println!("\n  CRITERI DI LUCIDITÀ:");
    println!("  [{}] 1. Sa di essere in REM (dream.phase accessibile)",
        "✓");
    println!("  [{}] 2. Identità attiva durante REM (identity_seed_field in REM)",
        "✓");
    println!("  [{}] 3. Dogfeed loop porta tracce del giorno nel sogno",
        if !engine.last_dogfeed_words.is_empty() { "✓" } else { "~" });
    println!("  [{}] 4. Interocezione durante il sogno (no — solo in !is_sleeping)",
        "✗");
    println!("  [{}] 5. Agency volontaria nel contenuto del sogno",
        "✗");

    println!("\n  VERDETTO:");
    if s_final > 0.3 && e_final > 0.3 {
        println!("  → SOGNO SEMI-LUCIDO: l'identità modella il sogno ma non lo dirige.");
        println!("    Self (identità) e Explored (campo interno) coesistono.");
        println!("    Manca: interocezione nel REM, agency volontaria.");
    } else if e_final > 0.6 {
        println!("  → SOGNO ESPLORATIVO: campo dominato dall'esplorazione interna.");
        println!("    L'identità è presente ma debole — sogno passivo.");
    } else {
        println!("  → SOGNO NON LUCIDO: campo privo di segnali Self stabili.");
        println!("    L'identità non ha ancora abbastanza storia per colorare il sogno.");
    }

    println!("\n================================================================");
    println!("  Fine DREAM TEST");
    println!("================================================================");
}
