/// API REST — Handler per tutti gli endpoint.

use axum::{
    extract::State,
    extract::Path,
    extract::Query,
    response::{Html, IntoResponse, Response},
    body::Body,
    Json,
};
use axum::http::{StatusCode, header};
use serde::Deserialize;
use tokio::sync::oneshot;

use super::state::*;

// ═══════════════════════════════════════════════════════════════
// GET / — Serve la dashboard HTML
// ═══════════════════════════════════════════════════════════════

static INDEX_HTML: &str = include_str!("index.html");

pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

// ═══════════════════════════════════════════════════════════════
// GET /api/state — Snapshot completo
// ═══════════════════════════════════════════════════════════════

pub async fn get_state(State(state): State<AppState>) -> Json<StateSnapshot> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetState { reply: tx }).await;
    match rx.await {
        Ok(snapshot) => Json(snapshot),
        Err(_) => Json(StateSnapshot::default()),
    }
}

// ═══════════════════════════════════════════════════════════════
// POST /api/input — Invia testo all'engine
// ═══════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct InputRequest {
    pub text: String,
}

pub async fn post_input(
    State(state): State<AppState>,
    Json(req): Json<InputRequest>,
) -> Json<InputResponse> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Receive {
        input: req.text,
        reply: tx,
    }).await;
    match rx.await {
        Ok(response) => {
            // Broadcast stato aggiornato ai WebSocket
            let update = serde_json::json!({
                "type": "state_update",
                "data": &response.state,
            });
            let _ = state.broadcast_tx.send(update.to_string());
            Json(response)
        }
        Err(_) => Json(InputResponse {
            generated_text: String::new(),
            keywords: Vec::new(),
            state: StateSnapshot::default(),
            stance: "Open".to_string(),
            intention: "Express".to_string(),
            topic_continuity: 0.5,
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// POST /api/dream — Forza sogno
// ═══════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct DreamRequest {
    pub ticks: Option<u32>,
}

pub async fn post_dream(
    State(state): State<AppState>,
    Json(req): Json<DreamRequest>,
) -> Json<StateSnapshot> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Dream {
        ticks: req.ticks.unwrap_or(20),
        reply: tx,
    }).await;
    match rx.await {
        Ok(snapshot) => {
            let update = serde_json::json!({
                "type": "state_update",
                "data": &snapshot,
            });
            let _ = state.broadcast_tx.send(update.to_string());
            Json(snapshot)
        }
        Err(_) => Json(StateSnapshot::default()),
    }
}

// ═══════════════════════════════════════════════════════════════
// POST /api/grow — Crescita strutturale
// ═══════════════════════════════════════════════════════════════

pub async fn post_grow(State(state): State<AppState>) -> Json<GrowthDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Grow { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(GrowthDto {
            events: Vec::new(),
            new_fractals: 0,
            new_connections: 0,
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/topology — Grafo completo
// ═══════════════════════════════════════════════════════════════

pub async fn get_topology(State(state): State<AppState>) -> Json<TopologyDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetTopology { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(TopologyDto {
            nodes: Vec::new(),
            edges: Vec::new(),
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/navigate/:from/:to — Geodetica
// ═══════════════════════════════════════════════════════════════

pub async fn get_navigate(
    State(state): State<AppState>,
    Path((from, to)): Path<(String, String)>,
) -> Json<Option<NavigationDto>> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Navigate {
        from,
        to,
        reply: tx,
    }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(None),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/projection — Proiezione olografica
// ═══════════════════════════════════════════════════════════════

pub async fn get_projection(State(state): State<AppState>) -> Json<Option<ProjectionDto>> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Projection { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(None),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/introspect — Introspezione
// ═══════════════════════════════════════════════════════════════

pub async fn get_introspect(State(state): State<AppState>) -> Json<IntrospectionDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Introspect { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(IntrospectionDto {
            fractal_count: 0,
            simplex_count: 0,
            conceptual_gaps: 0,
            disconnected_worlds: 0,
            densest_region: None,
            sparsest_region: None,
            field_energy: 0.0,
            emergent_dimensions: 0,
            most_experienced: None,
            least_experienced: None,
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/why — Spiegazione ultimo output
// ═══════════════════════════════════════════════════════════════

pub async fn get_why(State(state): State<AppState>) -> Json<WhyDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Why { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(WhyDto {
            explanation: String::new(),
            fractal_sequence: Vec::new(),
            propagation_bridges: Vec::new(),
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/ask — Domande curiosita
// ═══════════════════════════════════════════════════════════════

pub async fn get_ask(State(state): State<AppState>) -> Json<Vec<QuestionDto>> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Ask { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(Vec::new()),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/generate — Genera testo dal campo
// ═══════════════════════════════════════════════════════════════

pub async fn get_generate(State(state): State<AppState>) -> Json<GenerateDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Generate { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(GenerateDto {
            text: String::new(),
            structure: String::new(),
            cluster_count: 0,
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// POST /api/save — Salva stato
// ═══════════════════════════════════════════════════════════════

pub async fn post_save(State(state): State<AppState>) -> Json<bool> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::Save { reply: tx }).await;
    match rx.await {
        Ok(ok) => Json(ok),
        Err(_) => Json(false),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/will — Stato volontà corrente
// ═══════════════════════════════════════════════════════════════

pub async fn get_will(State(state): State<AppState>) -> Json<WillDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetWill { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(WillDto::default()),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/compounds — Composti frattali attivi
// ═══════════════════════════════════════════════════════════════

pub async fn get_compounds(State(state): State<AppState>) -> Json<Vec<CompoundDto>> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetCompounds { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(Vec::new()),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/wordfield — Campo parole top attive
// ═══════════════════════════════════════════════════════════════

pub async fn get_wordfield(State(state): State<AppState>) -> Json<WordFieldDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetWordField { reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(WordFieldDto::default()),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/phase/:a/:b — Fase tra due parole
// ═══════════════════════════════════════════════════════════════

pub async fn get_phase(
    State(state): State<AppState>,
    Path((a, b)): Path<(String, String)>,
) -> Json<PhaseDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetPhase { word_a: a, word_b: b, reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(PhaseDto::default()),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/tension/:a/:b — Parole di tensione tra due poli
// ═══════════════════════════════════════════════════════════════

pub async fn get_tension(
    State(state): State<AppState>,
    Path((a, b)): Path<(String, String)>,
) -> Json<Vec<TensionWordDto>> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetTension { pole_a: a, pole_b: b, reply: tx }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(Vec::new()),
    }
}

// ═══════════════════════════════════════════════════════════════
// POST /api/locus-simulate — Simula dal punto di vista di un locus
// ═══════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct LocusSimRequest {
    pub locus: String,
}

pub async fn post_locus_simulate(
    State(state): State<AppState>,
    Json(req): Json<LocusSimRequest>,
) -> Json<Option<LociSimDto>> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::SimulateLocus {
        locus_name: req.locus,
        reply: tx,
    }).await;
    match rx.await {
        Ok(dto) => Json(dto),
        Err(_) => Json(None),
    }
}

// ═══════════════════════════════════════════════════════════════
// GET /api/simpdb — Serve il file SimplDB binario per download mobile
// ═══════════════════════════════════════════════════════════════

pub async fn get_simpdb() -> Response {
    // Cerca prima il formato v3 (.bin), poi il legacy JSON
    let paths = [
        "prometeo_topology_state.bin",
        "prometeo_state.bin",
    ];

    for path in &paths {
        match tokio::fs::read(path).await {
            Ok(bytes) => {
                return (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "application/octet-stream"),
                        (header::CONTENT_DISPOSITION, "attachment; filename=\"prometeo_state.bin\""),
                    ],
                    bytes,
                ).into_response();
            }
            Err(_) => continue,
        }
    }

    (StatusCode::NOT_FOUND, "SimplDB non disponibile — usa :save prima").into_response()
}

// ═══════════════════════════════════════════════════════════════
// /api/thoughts — osservazioni topologiche interne
// ═══════════════════════════════════════════════════════════════

#[derive(serde::Serialize)]
pub struct ThoughtDto {
    pub kind: String,
    pub fractal_names: Vec<String>,
    pub words: Vec<String>,
    pub strength: f64,
    pub detail: serde_json::Value,
}

pub async fn get_thoughts(State(state): State<AppState>) -> Json<Vec<ThoughtDto>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetThoughts { reply: tx }).await;
    Json(rx.await.unwrap_or_default())
}

// ═══════════════════════════════════════════════════════════════
// GET /api/narrative — stato NarrativeSelf
// ═══════════════════════════════════════════════════════════════

pub async fn get_narrative(State(state): State<AppState>) -> Json<super::state::NarrativeDto> {
    use super::state::{NarrativeDto, NarrativeTurnDto, NarrativePositionDto};
    let (tx, rx) = tokio::sync::oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetNarrative { reply: tx }).await;
    Json(rx.await.unwrap_or(NarrativeDto {
        stance: "aperto".into(),
        pending_intention: None,
        topic_continuity: 0.5,
        is_born: false,
        turn_count: 0,
        recent_turns: vec![],
        crystallized: vec![],
        positions: vec![],
    }))
}

// ═══════════════════════════════════════════════════════════════
// /api/visuals — grammatica visiva: SVG dei frattali + simplessi
// ═══════════════════════════════════════════════════════════════

#[derive(serde::Serialize)]
pub struct FractalVisualDto {
    pub id: u32,
    pub name: String,
    pub svg: String,
    pub activation: f64,
}

#[derive(serde::Serialize)]
pub struct SimplexVisualDto {
    pub name: String,
    pub fractal_names: Vec<String>,
    pub svg: String,
    pub strength: f64,
    pub activation: f64,
}

#[derive(serde::Serialize)]
pub struct VisualsDto {
    pub fractals: Vec<FractalVisualDto>,
    pub simplices: Vec<SimplexVisualDto>,
}

pub async fn get_visuals(State(state): State<AppState>) -> Json<VisualsDto> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetVisuals { reply: tx }).await;
    Json(rx.await.unwrap_or(VisualsDto { fractals: vec![], simplices: vec![] }))
}

// ═══════════════════════════════════════════════════════════════
// GET /api/universe — Galassia esplorabile: frattali + parole
// ═══════════════════════════════════════════════════════════════

pub async fn get_universe(State(state): State<AppState>) -> Json<UniverseDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetUniverse { reply: tx }).await;
    Json(rx.await.unwrap_or_default())
}

// ═══════════════════════════════════════════════════════════════
// GET /api/word_neighbors?word=xxx — Vicini di una parola
// ═══════════════════════════════════════════════════════════════

pub async fn get_word_neighbors(
    State(state): State<AppState>,
    Query(params): Query<WordQuery>,
) -> Json<WordNeighborsDto> {
    let (tx, rx) = oneshot::channel();
    let _ = state.cmd_tx.send(EngineCommand::GetWordNeighbors {
        word: params.word,
        reply: tx,
    }).await;
    Json(rx.await.unwrap_or_default())
}

// ═══════════════════════════════════════════════════════════════
// Default per StateSnapshot (usato in caso di errore)
// ═══════════════════════════════════════════════════════════════

impl Default for StateSnapshot {
    fn default() -> Self {
        Self {
            vital: VitalDto {
                activation: 0.0,
                saturation: 0.0,
                curiosity: 0.0,
                fatigue: 0.0,
                tension: "Calm".to_string(),
            },
            active_fractals: Vec::new(),
            locus: None,
            dream_phase: "Awake".to_string(),
            dream_depth: 0.0,
            report: ReportDto {
                fractal_count: 0,
                simplex_count: 0,
                max_dimension: 0,
                connected_components: 0,
                memory_stm: 0,
                memory_mtm: 0,
                memory_ltm: 0,
                dream_cycles: 0,
                total_perturbations: 0,
                vocabulary_size: 0,
                emergent_dimensions: 0,
            },
            field_signature: vec![0.5; 8],
        }
    }
}
