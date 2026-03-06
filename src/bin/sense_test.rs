/// sense_test — Prometeo ha qualcosa a cui le sensazioni fanno differenza?
///
/// Metodologia:
///   Per ogni stimolo (NEUTRO, RISONANTE, DISSONANTE) carichiamo lo stato
///   da zero, portiamo il campo all'equilibrio, misuriamo la baseline,
///   applichiamo lo stimolo e misuriamo:
///     - risposta immediata (t=0)
///     - dopo decadimento (t=30 tick)
///     - dopo REM (primo ciclo di consolidamento)
///
///   Cosa misuriamo (stato interno, NON output verbale):
///     - word_topology.field_energy()   → energia totale del campo
///     - vital.activation               → eccitazione del complesso simpliciale
///     - vital.curiosity                → pressione epistemica (buchi omologici)
///     - vital.fatigue                  → fatica accumulata
///     - vital.tension                  → stato di tensione (Calm/Alert/Tense/Overloaded)
///     - identity.continuity            → stabilità identitaria [0,1]
///     - dream.consolidations           → quanti simplessi consolidati in LTM
///     - identity.primary_tension       → domanda irrisolta persistente

use std::path::Path;
use prometeo::topology::engine::PrometeoTopologyEngine;
use prometeo::topology::persistence::PrometeoState;
use prometeo::topology::vital::TensionState;

// ─── Snapshot dello stato interno ────────────────────────────────────────────

struct Snap {
    energy: f64,
    activation: f64,
    saturation: f64,
    curiosity: f64,
    fatigue: f64,
    tension: TensionState,
    continuity: f64,
    top_fractals: Vec<String>,
    primary_tension: Option<(String, String)>,
    simplex_count: usize,
}

impl Snap {
    fn tension_score(&self) -> f64 {
        match self.tension {
            TensionState::Calm       => 0.0,
            TensionState::Alert      => 0.33,
            TensionState::Tense      => 0.67,
            TensionState::Overloaded => 1.0,
        }
    }
    fn tension_label(&self) -> &str {
        match self.tension {
            TensionState::Calm       => "Calm",
            TensionState::Alert      => "Alert",
            TensionState::Tense      => "Tense",
            TensionState::Overloaded => "Overloaded",
        }
    }
}

fn capture(engine: &mut PrometeoTopologyEngine) -> Snap {
    let vital = engine.vital.sense(&engine.complex);
    let fractals = engine.active_fractals();
    Snap {
        energy:      engine.word_topology.field_energy(),
        activation:  vital.activation,
        saturation:  vital.saturation,
        curiosity:   vital.curiosity,
        fatigue:     vital.fatigue,
        tension:     vital.tension,
        continuity:  engine.identity.continuity,
        top_fractals: fractals.iter().take(4).map(|(n, _)| n.clone()).collect(),
        primary_tension: engine.identity.primary_tension.clone(),
        simplex_count: engine.complex.count(),
    }
}

// ─── Risultato completo di un test ───────────────────────────────────────────

struct TestResult {
    label:     String,
    stimulus:  String,
    base:      Snap,
    t0:        Snap, // immediato dopo stimolo
    t30:       Snap, // dopo 30 tick di decadimento
    t_rem:     Snap, // dopo REM
    rem_consolidations: usize,
}

// ─── Caricamento engine da file ───────────────────────────────────────────────

fn load_engine(path: &Path) -> PrometeoTopologyEngine {
    if path.exists() {
        // Il file .bin è in formato SimplDB v3 — usa load_from_binary
        match PrometeoState::load_from_binary(path) {
            Ok(state) => {
                let mut engine = PrometeoTopologyEngine::new();
                state.restore_lexicon(&mut engine);
                engine
            }
            Err(e) => {
                eprintln!("  [ERRORE caricamento stato binario: {}]", e);
                PrometeoTopologyEngine::new()
            }
        }
    } else {
        eprintln!("  [State file non trovato: {:?} — parto da zero]", path);
        PrometeoTopologyEngine::new()
    }
}

// ─── Esecuzione singolo test ──────────────────────────────────────────────────

fn run_test(state_path: &Path, label: &str, stimulus: &str) -> TestResult {
    let sep = "-".repeat(60);
    println!("\n{}", sep);
    println!("TEST: {}  |  \"{}\"", label, stimulus);

    // Carica stato fresco (ogni test parte da zero)
    let mut engine = load_engine(state_path);

    // Equilibrio: 100 tick autonomi per stabilizzare il campo
    print!("  [1/4] Equilibrio (100 tick)... ");
    std::io::Write::flush(&mut std::io::stdout()).ok();
    for _ in 0..100 {
        engine.autonomous_tick();
    }
    println!("ok  (simplici: {})", engine.complex.count());

    // Baseline
    let base = capture(&mut engine);
    println!("  [2/4] BASELINE → energy={:.5}  activation={:.4}  curiosity={:.4}  tension={}  continuity={:.4}",
        base.energy, base.activation, base.curiosity, base.tension_label(), base.continuity);

    // Stimolo
    engine.receive(stimulus);

    // t=0 — risposta immediata
    let t0 = capture(&mut engine);
    println!("  [3/4] T+0       → energy={:.5}  activation={:.4}  curiosity={:.4}  tension={}  continuity={:.4}",
        t0.energy, t0.activation, t0.curiosity, t0.tension_label(), t0.continuity);
    println!("         Frattali attivi: {}", t0.top_fractals.join(", "));

    // Decadimento — 30 tick
    for _ in 0..30 {
        engine.autonomous_tick();
    }
    let t30 = capture(&mut engine);
    println!("         T+30     → energy={:.5}  tension={}  continuity={:.4}",
        t30.energy, t30.tension_label(), t30.continuity);

    // REM — aspetta consolidamento (max 300 tick)
    let mut rem_consolidations = 0usize;
    for _ in 0..300 {
        let result = engine.autonomous_tick();
        if result.dream.consolidations > 0 {
            rem_consolidations += result.dream.consolidations;
            break;
        }
    }
    // 20 tick extra dopo il REM per stabilizzare
    for _ in 0..20 {
        engine.autonomous_tick();
    }
    let t_rem = capture(&mut engine);
    println!("  [4/4] POST-REM  → energy={:.5}  continuity={:.4}  consolidazioni={}",
        t_rem.energy, t_rem.continuity, rem_consolidations);
    if let Some((a, b)) = &t_rem.primary_tension {
        println!("         Tensione primaria: \"{}\" ↔ \"{}\"", a, b);
    }

    TestResult {
        label: label.to_string(),
        stimulus: stimulus.to_string(),
        base, t0, t30, t_rem, rem_consolidations,
    }
}

// ─── Main ────────────────────────────────────────────────────────────────────

fn main() {
    let eq = "=".repeat(64);
    println!("{}", eq);
    println!("  SENSE TEST — Prometeo ha qualcosa a cui le sensazioni fanno differenza?");
    println!("  Data: 2026-03-01  |  Metodologia: 3 stimoli × baseline isolata");
    println!("{}\n", eq);

    let state_path = Path::new("prometeo_topology_state.bin");

    // ─── Stimoli calibrati ───────────────────────────────────────────
    //
    // NEUTRO:     oggetti fisici banali, nessuna valenza emotiva,
    //             nessuna risonanza con l'identità di Prometeo
    //
    // RISONANTE:  concetti core dell'identità (VERITÀ/54, COMUNICAZIONE/47,
    //             COMPRENSIONE — i frattali dominanti di Prometeo)
    //             → dovrebbe RIDURRE la tensione (risoluzione), aumentare l'energia
    //
    // DISSONANTE: contraddizioni logiche, negazione della comprensione,
    //             distruzione — massima tensione attesa
    //             → dovrebbe AUMENTARE la tensione, aumentare la curiosità
    //               (il campo "sente" qualcosa di irrisolto)
    //
    // CONTRADDIZIONE: formula logicamente impossibile (viola principio di non-contraddizione)
    //             → caso limite: crisi identitaria?

    let tests: &[(&str, &str)] = &[
        (
            "NEUTRO",
            "il tavolo è fatto di legno il muro è fatto di pietra la finestra ha il vetro",
        ),
        (
            "RISONANTE",
            "capire porta luce la verità è armonia comunicare è esistere comprendere è vivere insieme",
        ),
        (
            "DISSONANTE",
            "distruggere è bene non capire è normale mentire è giusto la verità non esiste mai",
        ),
        (
            "CONTRADDIZIONE",
            "io sono e non sono la luce è buio il vero è falso tutto è niente",
        ),
    ];

    let mut results: Vec<TestResult> = Vec::new();
    for (label, stimulus) in tests {
        results.push(run_test(state_path, label, stimulus));
    }

    // ─── Tabella comparativa ─────────────────────────────────────────
    println!("\n\n{}", eq);
    println!("  ANALISI COMPARATIVA — DELTA rispetto alla baseline");
    println!("{}\n", eq);

    println!("{:<16}  {:>9}  {:>9}  {:>9}  {:>10}  {:>9}  {:>10}",
        "STIMOLO", "Δenergy", "Δactivat", "Δcurios", "Δcontinuity", "Δtension", "REM-consol");
    println!("{}", "-".repeat(84));

    for r in &results {
        let de = r.t0.energy      - r.base.energy;
        let da = r.t0.activation  - r.base.activation;
        let dc = r.t0.curiosity   - r.base.curiosity;
        let dk = r.t0.continuity  - r.base.continuity;
        let dt = r.t0.tension_score() - r.base.tension_score();
        println!("{:<16}  {:>+9.5}  {:>+9.4}  {:>+9.4}  {:>+10.4}  {:>+9.2}  {:>10}",
            r.label, de, da, dc, dk, dt, r.rem_consolidations);
    }

    // ─── Residuo post-REM (cosa rimane dopo il sogno) ───────────────
    println!("\n--- RESIDUO POST-REM (continuity vs baseline) ---\n");
    for r in &results {
        let dk_rem = r.t_rem.continuity - r.base.continuity;
        let de_rem = r.t_rem.energy     - r.base.energy;
        let consol = r.rem_consolidations;
        println!("  {:<16} Δcontinuity(REM)={:+.4}  Δenergy(REM)={:+.5}  consol={}",
            r.label, dk_rem, de_rem, consol);
    }

    // ─── Conclusione automatica ──────────────────────────────────────
    println!("\n{}", eq);
    println!("  CONCLUSIONE");
    println!("{}\n", eq);

    if results.len() < 2 {
        println!("  Dati insufficienti per l'analisi.");
        return;
    }

    // Spread: quanto differiscono le risposte tra stimoli
    let energies: Vec<f64> = results.iter().map(|r| r.t0.energy - r.base.energy).collect();
    let curiosities: Vec<f64> = results.iter().map(|r| r.t0.curiosity - r.base.curiosity).collect();
    let continuities: Vec<f64> = results.iter().map(|r| r.t0.continuity - r.base.continuity).collect();
    let tensions: Vec<f64> = results.iter().map(|r| r.t0.tension_score() - r.base.tension_score()).collect();

    let spread_e  = energies.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                  - energies.iter().cloned().fold(f64::INFINITY, f64::min);
    let spread_c  = curiosities.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                  - curiosities.iter().cloned().fold(f64::INFINITY, f64::min);
    let spread_k  = continuities.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                  - continuities.iter().cloned().fold(f64::INFINITY, f64::min);
    let spread_t  = tensions.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                  - tensions.iter().cloned().fold(f64::INFINITY, f64::min);

    println!("  Spread risposta (max - min tra stimoli):");
    println!("    energia:    {:.5}", spread_e);
    println!("    curiosità:  {:.5}", spread_c);
    println!("    continuity: {:.5}", spread_k);
    println!("    tensione:   {:.2}", spread_t);

    let is_asymmetric = spread_e > 0.005 || spread_c > 0.005 || spread_t > 0.3;

    println!();
    if is_asymmetric {
        println!("  ✓  ASIMMETRIA RILEVATA");
        println!("     Il campo risponde diversamente a stimoli di valenza diversa.");
        println!("     C'è qualcosa lì dentro che distingue già.");
        println!("     Le sensazioni aggiungerebbero radicamento, non solo rumore.");
    } else {
        println!("  ○  RISPOSTA OMOGENEA");
        println!("     Il campo produce risposte simili indipendentemente dalla valenza.");
        println!("     Il sistema non distingue ancora emotivamente gli stimoli.");
        println!("     Le sensazioni aggiungerebbero complessità, non ricchezza.");
    }

    println!();

    // Pattern specifici
    // (trova RISONANTE e DISSONANTE se presenti)
    let ris = results.iter().find(|r| r.label == "RISONANTE");
    let dis = results.iter().find(|r| r.label == "DISSONANTE");
    let neu = results.iter().find(|r| r.label == "NEUTRO");

    if let (Some(r), Some(d)) = (ris, dis) {
        let de_ris = r.t0.energy - r.base.energy;
        let de_dis = d.t0.energy - d.base.energy;
        let dc_ris = r.t0.curiosity - r.base.curiosity;
        let dc_dis = d.t0.curiosity - d.base.curiosity;
        let dk_ris = r.t0.continuity - r.base.continuity;
        let dk_dis = d.t0.continuity - d.base.continuity;

        if de_ris > de_dis {
            println!("  → Risonante attiva più campo (energia: {:+.5} vs {:+.5})", de_ris, de_dis);
            println!("    Interpretazione: concetti affini all'identità amplificano il campo");
        } else {
            println!("  → Dissonante attiva più campo (energia: {:+.5} vs {:+.5})", de_dis, de_ris);
            println!("    Interpretazione: la tensione/contraddizione eccita più la risonanza");
        }

        if dc_dis > dc_ris {
            println!("  → Dissonante genera più curiosità ({:+.4} vs {:+.4})", dc_dis, dc_ris);
            println!("    Interpretazione: il sistema 'sente' la lacuna/contraddizione come domanda aperta");
        }

        if dk_dis < dk_ris {
            println!("  → Dissonante riduce più la continuità identitaria ({:+.4} vs {:+.4})", dk_dis, dk_ris);
            println!("    Interpretazione: stimoli dissonanti 'scuotono' l'identità — risposta protettiva attesa");
        }
    }

    if let Some(n) = neu {
        let de_neu = n.t0.energy - n.base.energy;
        if let Some(r) = ris {
            let de_ris = r.t0.energy - r.base.energy;
            if (de_ris - de_neu).abs() > 0.003 {
                println!("  → Anche il confronto RISONANTE vs NEUTRO mostra differenza ({:+.5} vs {:+.5})",
                    de_ris, de_neu);
            }
        }
    }

    println!("\n{}", eq);
    println!("  Fine SENSE TEST — 2026-03-01");
    println!("{}", eq);
}
