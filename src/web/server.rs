/// Server — Entry point del binario prometeo-web.
///
/// L'engine vive in un thread OS dedicato (non e Send).
/// Comunicazione via mpsc (comandi) e broadcast (aggiornamenti).

use std::collections::HashSet;
use tokio::sync::{mpsc, oneshot, broadcast};
use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;

use crate::topology::engine::PrometeoTopologyEngine;
use crate::topology::persistence::PrometeoState;
use crate::topology::vital::TensionState;
use crate::topology::dream::SleepPhase;


use super::state::*;
use super::api;
use super::ws;

/// Avvia il server web.
pub async fn run(port: u16) {
    let (cmd_tx, cmd_rx) = mpsc::channel::<EngineCommand>(64);
    let (broadcast_tx, _) = broadcast::channel::<String>(128);

    let state = AppState {
        cmd_tx: cmd_tx.clone(),
        broadcast_tx: broadcast_tx.clone(),
    };

    // Thread OS dedicato per l'engine (non e Send)
    let broadcast_tx_clone = broadcast_tx.clone();
    std::thread::spawn(move || {
        engine_loop(cmd_rx, broadcast_tx_clone);
    });

    // Auto-dream: tick autonomo ogni 3 secondi.
    // Skip (non Burst) per evitare accumulo di tick durante il caricamento iniziale.
    let dream_cmd_tx = cmd_tx.clone();
    let dream_broadcast = broadcast_tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(3));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            let (tx, rx) = oneshot::channel();
            if dream_cmd_tx.send(EngineCommand::Dream { ticks: 1, reply: tx }).await.is_ok() {
                if let Ok(snapshot) = rx.await {
                    let update = serde_json::json!({
                        "type": "state_update",
                        "data": &snapshot,
                    });
                    let _ = dream_broadcast.send(update.to_string());
                }
            }
        }
    });

    let app = Router::new()
        .route("/", get(api::index))
        .route("/api/state", get(api::get_state))
        .route("/api/input", post(api::post_input))
        .route("/api/dream", post(api::post_dream))
        .route("/api/grow", post(api::post_grow))
        .route("/api/topology", get(api::get_topology))
        .route("/api/navigate/{from}/{to}", get(api::get_navigate))
        .route("/api/projection", get(api::get_projection))
        .route("/api/introspect", get(api::get_introspect))
        .route("/api/why", get(api::get_why))
        .route("/api/ask", get(api::get_ask))
        .route("/api/generate", get(api::get_generate))
        .route("/api/save", post(api::post_save))
        .route("/api/will", get(api::get_will))
        .route("/api/compounds", get(api::get_compounds))
        .route("/api/wordfield", get(api::get_wordfield))
        .route("/api/phase/{a}/{b}", get(api::get_phase))
        .route("/api/tension/{a}/{b}", get(api::get_tension))
        .route("/api/locus-simulate", post(api::post_locus_simulate))
        .route("/api/narrative", get(api::get_narrative))
        .route("/api/thoughts", get(api::get_thoughts))
        .route("/api/visuals", get(api::get_visuals))
        .route("/api/simpdb", get(api::get_simpdb))
        .route("/api/universe", get(api::get_universe))
        .route("/api/word_neighbors", get(api::get_word_neighbors))
        .route("/ws", get(ws::ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    println!("╔══════════════════════════════════════════════╗");
    println!("║  PROMETEO — Topologia Cognitiva 8D          ║");
    println!("║  Web UI: http://localhost:{}               ║", port);
    println!("╚══════════════════════════════════════════════╝");

    let listener = tokio::net::TcpListener::bind(&addr).await
        .expect("Impossibile avviare il server");
    axum::serve(listener, app).await
        .expect("Errore nel server");
}

// ═══════════════════════════════════════════════════════════════
// Engine loop: gira nel thread OS dedicato
// ═══════════════════════════════════════════════════════════════

fn engine_loop(
    mut cmd_rx: mpsc::Receiver<EngineCommand>,
    _broadcast_tx: broadcast::Sender<String>,
) {
    use std::path::Path;

    // Carica stato salvato o crea nuovo.
    // Priorita: SimplDB .bin (veloce, mmap) → JSON (legacy) → bootstrap
    // Su Android: controlla anche /sdcard/ per facilitare il trasferimento manuale del .bin
    let binary_paths = [
        "prometeo_topology_state.bin",
        "prometeo_state.bin",
        "/sdcard/prometeo_topology_state.bin",
        "/sdcard/prometeo_state.bin",
    ];
    let json_paths = [
        "prometeo_topology_state.json",
        "prometeo_state.json",
    ];
    let mut engine = {
        let mut loaded = None;
        // Prova prima il formato binario SimplDB
        for path_str in &binary_paths {
            if Path::new(path_str).exists() {
                match PrometeoState::load_from_binary(Path::new(path_str)) {
                    Ok(state) => {
                        println!("[engine] Stato .bin caricato da: {} ({} parole)",
                            path_str, state.lexicon.words.len());
                        let mut eng = PrometeoTopologyEngine::new();
                        state.restore_lexicon(&mut eng);
                        eng.lexicon.apply_curated_signatures();
                        eng.recompute_all_word_affinities();
                        loaded = Some(eng);
                        break;
                    }
                    Err(e) => eprintln!("[engine] Errore .bin {}: {}", path_str, e),
                }
            }
        }
        // Fallback JSON
        if loaded.is_none() {
            for path_str in &json_paths {
                if let Ok(state) = PrometeoState::load_from_file(Path::new(path_str)) {
                    println!("[engine] Stato .json caricato da: {} ({} parole)",
                        path_str, state.lexicon.words.len());
                    let mut eng = PrometeoTopologyEngine::new();
                    state.restore_lexicon(&mut eng);
                    eng.lexicon.apply_curated_signatures();
                    eng.recompute_all_word_affinities();
                    loaded = Some(eng);
                    break;
                }
            }
        }
        loaded.unwrap_or_else(|| {
            println!("[engine] Nessuno stato trovato — bootstrap ({} parole)", 36);
            PrometeoTopologyEngine::new()
        })
    };

    // Carica il Knowledge Graph (se disponibile)
    engine.load_kg_from_file(Path::new("prometeo_kg.json"));

    // Phase 43B — Narrativa fondativa: solo al primo avvio (is_born == false).
    if !engine.narrative_self.is_born {
        engine.initialize_founding_narrative();
        println!("[engine] Narrativa fondativa cristallizzata — Prometeo nasce");
    }

    // Loop sincrono: ricevi comandi dal canale mpsc
    while let Some(cmd) = cmd_rx.blocking_recv() {
        match cmd {
            EngineCommand::Receive { input, reply } => {
                let response = engine.receive(&input);
                let generated = engine.generate_willed();
                let snapshot = build_snapshot(&mut engine);
                let stance = format!("{:?}", engine.narrative_self.stance);
                let intention = engine.narrative_self.pending_intention
                    .as_ref()
                    .map(|i| format!("{:?}", i))
                    .unwrap_or_else(|| "Express".to_string());
                let topic_continuity = engine.narrative_self.topic_continuity;
                let _ = reply.send(InputResponse {
                    generated_text: generated.text,
                    keywords: response.keywords,
                    state: snapshot,
                    stance,
                    intention,
                    topic_continuity,
                });
            }
            EngineCommand::GetState { reply } => {
                let snapshot = build_snapshot(&mut engine);
                let _ = reply.send(snapshot);
            }
            EngineCommand::GetTopology { reply } => {
                let dto = build_topology(&engine);
                let _ = reply.send(dto);
            }
            EngineCommand::Navigate { from, to, reply } => {
                let dto = build_navigation(&engine, &from, &to);
                let _ = reply.send(dto);
            }
            EngineCommand::Dream { ticks, reply } => {
                for _ in 0..ticks {
                    engine.autonomous_tick();
                }
                let snapshot = build_snapshot(&mut engine);
                let _ = reply.send(snapshot);
            }
            EngineCommand::Grow { reply } => {
                let events = engine.grow();
                let new_f = events.iter().filter(|e| matches!(e, crate::topology::growth::GrowthEvent::NewFractal { .. })).count();
                let new_c = events.iter().filter(|e| matches!(e, crate::topology::growth::GrowthEvent::NewConnection { .. })).count();
                let descs: Vec<String> = events.iter().map(|e| format!("{:?}", e)).collect();
                let _ = reply.send(GrowthDto {
                    events: descs,
                    new_fractals: new_f,
                    new_connections: new_c,
                });
            }
            EngineCommand::Introspect { reply } => {
                let intro = engine.introspect();
                let _ = reply.send(IntrospectionDto {
                    fractal_count: intro.fractal_count,
                    simplex_count: intro.simplex_count,
                    conceptual_gaps: intro.conceptual_gaps,
                    disconnected_worlds: intro.disconnected_worlds,
                    densest_region: intro.densest_region.map(|(n, c)| format!("{} ({})", n, c)),
                    sparsest_region: intro.sparsest_region.map(|(n, c)| format!("{} ({})", n, c)),
                    field_energy: intro.field_energy,
                    emergent_dimensions: intro.emergent_dimensions,
                    most_experienced: intro.most_experienced.map(|(n, c)| format!("{} ({})", n, c)),
                    least_experienced: intro.least_experienced.map(|(n, c)| format!("{} ({})", n, c)),
                });
            }
            EngineCommand::Why { reply } => {
                let trace = engine.why();
                let _ = reply.send(WhyDto {
                    explanation: trace.explanation,
                    fractal_sequence: trace.fractal_sequence.iter()
                        .map(|(name, act)| FractalActiveDto { name: name.clone(), activation: *act })
                        .collect(),
                    propagation_bridges: trace.propagation_bridges,
                });
            }
            EngineCommand::Ask { reply } => {
                let questions = engine.ask();
                let _ = reply.send(questions.iter().map(|q| QuestionDto {
                    text: q.text.clone(),
                    question_type: format!("{:?}", q.question_type),
                    priority: q.urgency,
                }).collect());
            }
            EngineCommand::Projection { reply } => {
                let proj = engine.holographic_projection();
                let _ = reply.send(proj.map(|p| ProjectionDto {
                    from_name: p.from_name,
                    projections: p.projections.iter().map(|fp| ProjectionItemDto {
                        name: fp.name.clone(),
                        proximity: fp.proximity,
                        dimensional_resonance: fp.dimensional_resonance,
                        distortion: fp.distortion,
                        apparent_center: fp.apparent_center.values().to_vec(),
                    }).collect(),
                }));
            }
            EngineCommand::Generate { reply } => {
                let gen = engine.generate();
                let _ = reply.send(GenerateDto {
                    text: gen.text,
                    structure: format!("{:?}", gen.structure),
                    cluster_count: gen.cluster_count,
                });
            }
            EngineCommand::Save { reply } => {
                let state = PrometeoState::capture(&engine);
                let ok = state.save_to_file(Path::new("prometeo_topology_state.json")).is_ok();
                if ok { println!("[engine] Stato salvato su disco"); }
                let _ = reply.send(ok);
            }
            EngineCommand::GetWill { reply } => {
                let dto = build_will(&engine);
                let _ = reply.send(dto);
            }
            EngineCommand::GetCompounds { reply } => {
                let dto = build_compounds(&engine);
                let _ = reply.send(dto);
            }
            EngineCommand::GetWordField { reply } => {
                let dto = build_word_field(&engine);
                let _ = reply.send(dto);
            }
            EngineCommand::GetPhase { word_a, word_b, reply } => {
                let dto = build_phase(&engine, &word_a, &word_b);
                let _ = reply.send(dto);
            }
            EngineCommand::GetTension { pole_a, pole_b, reply } => {
                let dto = build_tension(&engine, &pole_a, &pole_b);
                let _ = reply.send(dto);
            }
            EngineCommand::GetNarrative { reply } => {
                use crate::web::state::{NarrativeDto, NarrativeTurnDto, NarrativePositionDto};
                let ns = &engine.narrative_self;
                let act_str = |act: &crate::topology::input_reading::InputAct| -> String {
                    format!("{:?}", act)
                };
                let recent: Vec<NarrativeTurnDto> = ns.turns.iter().rev().take(8).map(|t| NarrativeTurnDto {
                    turn_id:    t.turn_id,
                    act:        act_str(&t.received_act),
                    stance:     t.stance.as_str().to_string(),
                    intention:  format!("{:?}", t.intention),
                    intensity:  t.intensity,
                    awareness:  t.awareness.clone(),
                    crystallized: false,
                }).collect();
                let crys: Vec<NarrativeTurnDto> = ns.crystallized.iter().rev().map(|t| NarrativeTurnDto {
                    turn_id:    t.turn_id,
                    act:        act_str(&t.received_act),
                    stance:     t.stance.as_str().to_string(),
                    intention:  format!("{:?}", t.intention),
                    intensity:  t.intensity,
                    awareness:  t.awareness.clone(),
                    crystallized: true,
                }).collect();
                let pos: Vec<NarrativePositionDto> = ns.positions.iter().map(|(k, (s, i))| NarrativePositionDto {
                    act_key:   k.clone(),
                    stance:    s.as_str().to_string(),
                    intention: format!("{:?}", i),
                }).collect();
                let dto = NarrativeDto {
                    stance:            ns.stance.as_str().to_string(),
                    pending_intention: ns.pending_intention.as_ref().map(|i| format!("{:?}", i)),
                    topic_continuity:  ns.topic_continuity,
                    is_born:           ns.is_born,
                    turn_count:        ns.turns.len(),
                    recent_turns:      recent,
                    crystallized:      crys,
                    positions:         pos,
                };
                let _ = reply.send(dto);
            }
            EngineCommand::GetThoughts { reply } => {
                let thoughts = crate::topology::thought::generate_thoughts(&engine);
                let dto: Vec<api::ThoughtDto> = thoughts.into_iter().map(|t| {
                    use crate::topology::thought::{ThoughtData, ThoughtKind};
                    let kind = match t.kind {
                        ThoughtKind::Tension       => "tension",
                        ThoughtKind::Gap           => "gap",
                        ThoughtKind::MissingBridge => "missing_bridge",
                        ThoughtKind::Disconnection => "disconnection",
                        ThoughtKind::Hypothesis    => "hypothesis",
                    }.to_string();
                    let detail = match &t.data {
                        ThoughtData::TensionData { phase, word_a, word_b } =>
                            serde_json::json!({ "phase_pi": phase / std::f64::consts::PI, "word_a": word_a, "word_b": word_b }),
                        ThoughtData::GapData { simplex_count, word_count, activation_count } =>
                            serde_json::json!({ "simplex_count": simplex_count, "word_count": word_count, "activation_count": activation_count }),
                        ThoughtData::MissingBridgeData { proximity, shared_simplices } =>
                            serde_json::json!({ "proximity": proximity, "shared_simplices": shared_simplices }),
                        ThoughtData::DisconnectionData { components } =>
                            serde_json::json!({ "components": components }),
                        ThoughtData::HypothesisData { simplex_id, dimension, activation_count } =>
                            serde_json::json!({ "simplex_id": simplex_id, "dimension": dimension, "activation_count": activation_count }),
                    };
                    api::ThoughtDto { kind, fractal_names: t.fractal_names, words: t.words, strength: t.strength, detail }
                }).collect();
                let _ = reply.send(dto);
            }
            EngineCommand::GetVisuals { reply } => {
                use crate::topology::fractal_visuals::{fractal_svg_from_registry, compose_simplex_svg, FRACTAL_COUNT};
                use crate::topology::simplex::SharedStructureType;

                // Attivazioni correnti dai frattali nel campo parole
                let acts = engine.word_topology.emerge_fractal_activations(&engine.lexicon);
                let act_map: std::collections::HashMap<u32, f64> = acts.into_iter().collect();

                let fractals = (0..FRACTAL_COUNT as u32).filter_map(|id| {
                    let name = engine.registry.get(id)?.name.clone();
                    let svg = fractal_svg_from_registry(id, &engine.registry)?;
                    let activation = *act_map.get(&id).unwrap_or(&0.0);
                    Some(api::FractalVisualDto { id, name, svg, activation })
                }).collect();

                // Simplessi: prende tutti, ordina per activation desc, max 24
                let mut simplices: Vec<api::SimplexVisualDto> = engine.complex.iter()
                    .map(|(_, s)| {
                        let name = s.shared_faces.iter()
                            .find_map(|f| {
                                if let SharedStructureType::EmergentProperty(n) = &f.structure {
                                    Some(n.clone())
                                } else { None }
                            })
                            .unwrap_or_else(|| {
                                s.vertices.iter()
                                    .map(|&fid| engine.registry.get(fid)
                                        .map(|f| f.name.as_str()).unwrap_or("?"))
                                    .collect::<Vec<_>>().join("+")
                            });
                        let fractal_names: Vec<String> = s.vertices.iter()
                            .map(|&fid| engine.registry.get(fid)
                                .map(|f| f.name.clone())
                                .unwrap_or_default())
                            .collect();
                        let svg = compose_simplex_svg(&s.vertices, &name);
                        let strength = s.shared_faces.iter()
                            .map(|f| f.strength).sum::<f64>().min(1.0);
                        api::SimplexVisualDto {
                            name,
                            fractal_names,
                            svg,
                            strength,
                            activation: s.current_activation,
                        }
                    })
                    .collect();

                simplices.sort_by(|a, b| b.activation.partial_cmp(&a.activation)
                    .unwrap_or(std::cmp::Ordering::Equal));
                simplices.truncate(24);

                let _ = reply.send(api::VisualsDto { fractals, simplices });
            }
            EngineCommand::GetUniverse { reply } => {
                let dto = build_universe(&engine);
                let _ = reply.send(dto);
            }
            EngineCommand::GetWordNeighbors { word, reply } => {
                let dto = build_word_neighbors(&engine, &word);
                let _ = reply.send(dto);
            }
            EngineCommand::SimulateLocus { locus_name, reply } => {
                let dto = engine.simulate_locus_view(&locus_name).map(|v| LociSimDto {
                    locus_name: v.locus_name,
                    visible_fractals: v.visible.iter()
                        .map(|(name, vis)| FractalActiveDto { name: name.clone(), activation: *vis })
                        .collect(),
                    active_fractals: v.active_fractals.iter()
                        .map(|(name, act)| FractalActiveDto { name: name.clone(), activation: *act })
                        .collect(),
                    generated_text: v.generated_text,
                });
                let _ = reply.send(dto);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// Conversioni engine → DTO
// ═══════════════════════════════════════════════════════════════

fn build_snapshot(engine: &mut PrometeoTopologyEngine) -> StateSnapshot {
    let vital = engine.vital_state();
    let active = engine.active_fractals();
    let report = engine.report();

    let locus = if let Some((name, horizon)) = engine.where_am_i() {
        let trail: Vec<String> = engine.locus.trail.iter()
            .filter_map(|&fid| engine.registry.get(fid).map(|f| f.name.clone()))
            .collect();
        let sub_pos: Vec<SubDimDto> = engine.locus.sub_position.iter()
            .map(|(dim, &val)| SubDimDto { dim_index: dim.index() as u8, value: val })
            .collect();
        let visible: Vec<VisibleFractalDto> = engine.what_i_see().iter()
            .map(|(name, vis)| VisibleFractalDto { name: name.clone(), visibility: *vis })
            .collect();
        Some(LocusDto {
            fractal_name: name,
            fractal_id: engine.locus.position.unwrap_or(0),
            horizon,
            trail,
            sub_position: sub_pos,
            visible,
        })
    } else {
        None
    };

    // Firma campo: media pesata delle attivazioni
    let field_sig = engine.locus.full_position(&engine.registry)
        .map(|p| p.values().to_vec())
        .unwrap_or_else(|| vec![0.5; 8]);

    let (dream_phase, dream_depth) = match engine.dream.phase {
        SleepPhase::Awake => ("Awake".to_string(), 0.0),
        SleepPhase::WakefulDream { depth } => ("WakefulDream".to_string(), depth),
        SleepPhase::LightSleep { depth } => ("LightSleep".to_string(), depth),
        SleepPhase::DeepSleep { depth } => ("DeepSleep".to_string(), depth),
        SleepPhase::REM { depth } => ("REM".to_string(), depth),
    };

    StateSnapshot {
        vital: VitalDto {
            activation: vital.activation,
            saturation: vital.saturation,
            curiosity: vital.curiosity,
            fatigue: vital.fatigue,
            tension: match vital.tension {
                TensionState::Calm => "Calm",
                TensionState::Alert => "Alert",
                TensionState::Tense => "Tense",
                TensionState::Overloaded => "Overloaded",
            }.to_string(),
        },
        active_fractals: active.iter()
            .map(|(name, act)| FractalActiveDto { name: name.clone(), activation: *act })
            .collect(),
        locus,
        dream_phase,
        dream_depth,
        report: ReportDto {
            fractal_count: report.fractal_count,
            simplex_count: report.simplex_count,
            max_dimension: report.max_dimension,
            connected_components: report.connected_components,
            memory_stm: report.stm_count,
            memory_mtm: report.mtm_count,
            memory_ltm: report.ltm_count,
            dream_cycles: report.dream_cycles,
            total_perturbations: report.total_perturbations,
            vocabulary_size: report.vocabulary_size,
            emergent_dimensions: report.emergent_dimensions,
        },
        field_signature: field_sig,
    }
}

fn build_topology(engine: &PrometeoTopologyEngine) -> TopologyDto {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen_edges: HashSet<(u32, u32)> = HashSet::new();

    let bootstrap_ids: HashSet<u32> = [0u32, 1, 2, 3, 4, 5].into_iter().collect();

    for (&id, fractal) in engine.registry.iter() {
        let simplex_count = engine.complex.simplices_of(id).len();
        let activation: f64 = engine.complex.simplices_of(id)
            .iter()
            .filter_map(|sid| engine.complex.get(*sid))
            .map(|s| s.current_activation)
            .sum::<f64>();

        nodes.push(TopologyNode {
            id,
            name: fractal.name.clone(),
            activation: activation.min(1.0),
            is_locus: engine.locus.position == Some(id),
            is_bootstrap: bootstrap_ids.contains(&id),
            simplex_count,
        });
    }

    // Archi dai simplessi
    for (_, simplex) in engine.complex.iter() {
        let strength = simplex.shared_faces.iter()
            .map(|f| f.strength)
            .sum::<f64>()
            .min(1.0)
            .max(0.1);

        for i in 0..simplex.vertices.len() {
            for j in (i + 1)..simplex.vertices.len() {
                let a = simplex.vertices[i];
                let b = simplex.vertices[j];
                let edge = if a < b { (a, b) } else { (b, a) };
                if seen_edges.insert(edge) {
                    edges.push(TopologyEdge {
                        source: a,
                        target: b,
                        strength,
                    });
                }
            }
        }
    }

    TopologyDto { nodes, edges }
}

fn build_universe(engine: &PrometeoTopologyEngine) -> UniverseDto {
    // Mappa attivazioni correnti dal word_topology
    let act_map: std::collections::HashMap<String, f64> = engine.word_topology
        .all_activations()
        .into_iter()
        .map(|(w, a)| (w.to_string(), a))
        .collect();

    // Frattali
    let fractals: Vec<UniverseFractal> = engine.registry.iter().map(|(&id, fractal)| {
        let activation: f64 = engine.complex.simplices_of(id)
            .iter()
            .filter_map(|sid| engine.complex.get(*sid))
            .map(|s| s.current_activation)
            .sum::<f64>();
        let lower = (id / 8) as u8;
        let upper = (id % 8) as u8;
        UniverseFractal {
            id,
            name: fractal.name.clone(),
            activation: activation.min(1.0),
            is_bootstrap: lower == upper,
            lower,
            upper,
        }
    }).collect();

    // Parole: filtra per stabilità minima, prendi top 8000
    let mut words: Vec<UniverseWord> = engine.lexicon.patterns_iter()
        .filter_map(|(_, pattern)| {
            if pattern.stability < 0.05 { return None; }
            let (dominant_fractal, max_aff) = pattern.fractal_affinities.iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(&k, &v)| (k, v))
                .unwrap_or((0, 0.0));
            if max_aff < 0.01 { return None; }
            let activation = act_map.get(&pattern.word).copied().unwrap_or(0.0);
            Some(UniverseWord {
                w: pattern.word.clone(),
                f: dominant_fractal,
                s: (pattern.stability.min(1.0) * 100.0) as u8,
                a: (activation.min(1.0) * 100.0) as u8,
            })
        })
        .collect();

    words.sort_by(|a, b| b.s.cmp(&a.s));
    words.truncate(8000);

    UniverseDto { fractals, words }
}

fn build_word_neighbors(engine: &PrometeoTopologyEngine, word: &str) -> WordNeighborsDto {
    let fractal_id = engine.lexicon.get(word)
        .and_then(|p| p.fractal_affinities.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(&k, _)| k))
        .unwrap_or(0);

    let neighbors = if let Some(id) = engine.word_topology.word_id(word) {
        let adj = engine.word_topology.adjacency_list(id);
        let mut nbrs: Vec<WordNeighborDto> = adj.iter()
            .filter_map(|&nid| {
                let name = engine.word_topology.word_name(nid)?;
                let weight = engine.word_topology.edge_weight_between(word, name)?;
                let fid = engine.lexicon.get(name)
                    .and_then(|p| p.fractal_affinities.iter()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                        .map(|(&k, _)| k))
                    .unwrap_or(0);
                Some(WordNeighborDto { word: name.to_string(), weight, fractal_id: fid })
            })
            .collect();
        nbrs.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
        nbrs.truncate(16);
        nbrs
    } else {
        Vec::new()
    };

    WordNeighborsDto { word: word.to_string(), fractal_id, neighbors }
}

fn build_navigation(engine: &PrometeoTopologyEngine, from: &str, to: &str) -> Option<NavigationDto> {
    let from_id = engine.find_fractal(from)?;
    let to_id = engine.find_fractal(to)?;
    let path = engine.navigate(from_id, to_id)?;

    Some(NavigationDto {
        from_name: engine.registry.get(from_id)?.name.clone(),
        to_name: engine.registry.get(to_id)?.name.clone(),
        steps: path.steps.iter().map(|s| NavStepDto {
            fractal_name: s.fractal_name.clone(),
            shared_structures: s.shared_structures.clone(),
            cumulative_cost: s.cumulative_cost,
        }).collect(),
        total_cost: path.total_cost,
        explanation: path.explanation,
    })
}

fn intention_name(i: &crate::topology::will::Intention) -> &'static str {
    use crate::topology::will::Intention;
    match i {
        Intention::Express { .. }  => "Express",
        Intention::Explore { .. }  => "Explore",
        Intention::Question { .. } => "Question",
        Intention::Remember { .. } => "Remember",
        Intention::Withdraw { .. } => "Withdraw",
        Intention::Reflect        => "Reflect",
        Intention::Dream { .. }   => "Dream",
        Intention::Instruct { .. } => "Instruct",
    }
}

fn build_will(engine: &PrometeoTopologyEngine) -> WillDto {
    use crate::topology::dream::SleepPhase;

    let (intention, drive, undercurrents, codon) = if let Some(will) = engine.current_will() {
        let name = intention_name(&will.intention).to_string();
        let under: Vec<UndercurrentDto> = will.undercurrents.iter()
            .map(|(i, p)| UndercurrentDto {
                name: intention_name(i).to_string(),
                pressure: *p,
            })
            .collect();
        (name, will.drive, under, will.codon)
    } else {
        ("Dream".to_string(), 0.0, Vec::new(), [0usize, 1usize])
    };

    let dream_phase = match engine.dream.phase {
        SleepPhase::Awake                => "Awake",
        SleepPhase::WakefulDream { .. }  => "WakefulDream",
        SleepPhase::LightSleep { .. }    => "LightSleep",
        SleepPhase::DeepSleep { .. }     => "DeepSleep",
        SleepPhase::REM { .. }           => "REM",
    }.to_string();

    WillDto { intention, drive, undercurrents, dream_phase, codon }
}

fn build_compounds(engine: &PrometeoTopologyEngine) -> Vec<CompoundDto> {
    engine.compound_states().iter().map(|c| {
        let fractal_names: Vec<String> = c.fractals.iter()
            .filter_map(|&fid| engine.registry.get(fid).map(|f| f.name.clone()))
            .collect();
        CompoundDto {
            name: c.name.to_string(),
            fractals: fractal_names,
            strength: c.strength,
            order: c.order,
        }
    }).collect()
}

fn build_word_field(engine: &PrometeoTopologyEngine) -> WordFieldDto {
    let top = engine.word_topology.most_active(20);
    let top_words = top.iter().map(|v| {
        // Frattale primario: l'affinita piu alta
        let fractal = engine.lexicon.get(&v.word)
            .and_then(|p| {
                p.fractal_affinities.iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                    .and_then(|(&fid, _)| engine.registry.get(fid).map(|f| f.name.clone()))
            })
            .unwrap_or_else(|| "?".to_string());
        WordActivationDto {
            word: v.word.clone(),
            activation: v.activation,
            fractal,
        }
    }).collect();

    WordFieldDto {
        top_words,
        total_energy: engine.word_topology.field_energy(),
        vertex_count: engine.word_topology.vertex_count(),
        edge_count: engine.word_topology.edge_count(),
    }
}

fn build_phase(engine: &PrometeoTopologyEngine, word_a: &str, word_b: &str) -> PhaseDto {
    use std::f64::consts::PI;

    let phase_rad = engine.word_topology.edge_phase(word_a, word_b)
        .unwrap_or(PI / 2.0);
    let phase_deg = phase_rad.to_degrees();
    let cos_value = phase_rad.cos();

    let label = if phase_deg < 60.0 {
        "Risonanza"
    } else if phase_deg < 120.0 {
        "Tensione"
    } else {
        "Opposizione"
    }.to_string();

    let (co_affirmed, co_negated) = engine.lexicon.get(word_a)
        .map(|p| (
            p.co_affirmed.get(word_b).copied().unwrap_or(0),
            p.co_negated.get(word_b).copied().unwrap_or(0),
        ))
        .unwrap_or((0, 0));

    PhaseDto {
        word_a: word_a.to_string(),
        word_b: word_b.to_string(),
        phase_rad,
        phase_deg,
        label,
        cos_value,
        co_affirmed,
        co_negated,
    }
}

fn build_tension(engine: &PrometeoTopologyEngine, pole_a: &str, pole_b: &str) -> Vec<TensionWordDto> {
    engine.lexicon.find_tension_words(pole_a, pole_b)
        .iter()
        .take(10)
        .map(|tw| TensionWordDto {
            word: tw.word.clone(),
            position: tw.position,
            distance_to_a: tw.distance_to_axis,
            distance_to_b: tw.distance_to_axis,
        })
        .collect()
}
