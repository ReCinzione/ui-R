/// State — Tipi condivisi tra API, WebSocket e engine thread.

use serde::{Serialize, Deserialize};
use tokio::sync::{mpsc, oneshot, broadcast};

// ═══════════════════════════════════════════════════════════════
// AppState: condiviso tra tutti gli handler axum
// ═══════════════════════════════════════════════════════════════

#[derive(Clone)]
pub struct AppState {
    /// Canale per inviare comandi all'engine thread
    pub cmd_tx: mpsc::Sender<EngineCommand>,
    /// Canale broadcast per notificare i client WebSocket
    pub broadcast_tx: broadcast::Sender<String>,
}

// ═══════════════════════════════════════════════════════════════
// Comandi: main thread → engine thread
// ═══════════════════════════════════════════════════════════════

pub enum EngineCommand {
    /// Ricevi input testuale
    Receive {
        input: String,
        reply: oneshot::Sender<InputResponse>,
    },
    /// Stato volontà corrente
    GetWill {
        reply: oneshot::Sender<WillDto>,
    },
    /// Composti frattali attivi
    GetCompounds {
        reply: oneshot::Sender<Vec<CompoundDto>>,
    },
    /// Campo parole: top attive + energia
    GetWordField {
        reply: oneshot::Sender<WordFieldDto>,
    },
    /// Fase tra due parole
    GetPhase {
        word_a: String,
        word_b: String,
        reply: oneshot::Sender<PhaseDto>,
    },
    /// Parole di tensione tra due poli
    GetTension {
        pole_a: String,
        pole_b: String,
        reply: oneshot::Sender<Vec<TensionWordDto>>,
    },
    /// Snapshot stato corrente
    GetState {
        reply: oneshot::Sender<StateSnapshot>,
    },
    /// Grafo completo per visualizzazione
    GetTopology {
        reply: oneshot::Sender<TopologyDto>,
    },
    /// Navigazione geodetica tra due frattali
    Navigate {
        from: String,
        to: String,
        reply: oneshot::Sender<Option<NavigationDto>>,
    },
    /// Forza sogno
    Dream {
        ticks: u32,
        reply: oneshot::Sender<StateSnapshot>,
    },
    /// Crescita strutturale
    Grow {
        reply: oneshot::Sender<GrowthDto>,
    },
    /// Introspezione
    Introspect {
        reply: oneshot::Sender<IntrospectionDto>,
    },
    /// Perche ultimo output
    Why {
        reply: oneshot::Sender<WhyDto>,
    },
    /// Domande curiosita
    Ask {
        reply: oneshot::Sender<Vec<QuestionDto>>,
    },
    /// Proiezione olografica
    Projection {
        reply: oneshot::Sender<Option<ProjectionDto>>,
    },
    /// Genera testo
    Generate {
        reply: oneshot::Sender<GenerateDto>,
    },
    /// Salva stato su disco
    Save {
        reply: oneshot::Sender<bool>,
    },
    /// Simula generazione dal punto di vista di un altro locus
    SimulateLocus {
        locus_name: String,
        reply: oneshot::Sender<Option<LociSimDto>>,
    },
    /// Stato NarrativeSelf — ciclo deliberativo
    GetNarrative {
        reply: oneshot::Sender<NarrativeDto>,
    },
    /// Osservazioni topologiche interne (pensieri)
    GetThoughts {
        reply: oneshot::Sender<Vec<super::api::ThoughtDto>>,
    },
    /// Grammatica visiva: SVG dei 16 frattali + simplessi attivi composti
    GetVisuals {
        reply: oneshot::Sender<super::api::VisualsDto>,
    },
    /// Universo esplorabile: frattali + parole con posizione
    GetUniverse {
        reply: oneshot::Sender<UniverseDto>,
    },
    /// Vicini di una parola nella word topology
    GetWordNeighbors {
        word: String,
        reply: oneshot::Sender<WordNeighborsDto>,
    },
}

// ═══════════════════════════════════════════════════════════════
// DTO: engine → JSON → frontend
// ═══════════════════════════════════════════════════════════════

#[derive(Serialize, Clone, Debug)]
pub struct InputResponse {
    /// Testo generato dal campo
    pub generated_text: String,
    /// Parole chiave dall'emergenza
    pub keywords: Vec<String>,
    /// Stato aggiornato
    pub state: StateSnapshot,
    /// Postura interna (Open, Curious, Reflective, Resonant, Withdrawn)
    pub stance: String,
    /// Intenzione di risposta (Acknowledge, Reflect, Resonate, Explore, Express, Remain)
    pub intention: String,
    /// Continuità tematica rispetto ai turni recenti [0,1]
    pub topic_continuity: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct StateSnapshot {
    /// Vitali
    pub vital: VitalDto,
    /// Frattali attivi (nome, attivazione)
    pub active_fractals: Vec<FractalActiveDto>,
    /// Posizione locus
    pub locus: Option<LocusDto>,
    /// Fase sogno
    pub dream_phase: String,
    /// Profondita sogno
    pub dream_depth: f64,
    /// Report sistema
    pub report: ReportDto,
    /// Firma campo corrente (8 valori)
    pub field_signature: Vec<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct VitalDto {
    pub activation: f64,
    pub saturation: f64,
    pub curiosity: f64,
    pub fatigue: f64,
    pub tension: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct FractalActiveDto {
    pub name: String,
    pub activation: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct LocusDto {
    pub fractal_name: String,
    pub fractal_id: u32,
    pub horizon: f64,
    pub trail: Vec<String>,
    pub sub_position: Vec<SubDimDto>,
    pub visible: Vec<VisibleFractalDto>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SubDimDto {
    pub dim_index: u8,
    pub value: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct VisibleFractalDto {
    pub name: String,
    pub visibility: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct ReportDto {
    pub fractal_count: usize,
    pub simplex_count: usize,
    pub max_dimension: usize,
    pub connected_components: usize,
    pub memory_stm: usize,
    pub memory_mtm: usize,
    pub memory_ltm: usize,
    pub dream_cycles: u64,
    pub total_perturbations: u64,
    pub vocabulary_size: usize,
    pub emergent_dimensions: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct TopologyDto {
    pub nodes: Vec<TopologyNode>,
    pub edges: Vec<TopologyEdge>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TopologyNode {
    pub id: u32,
    pub name: String,
    pub activation: f64,
    pub is_locus: bool,
    pub is_bootstrap: bool,
    pub simplex_count: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct TopologyEdge {
    pub source: u32,
    pub target: u32,
    pub strength: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct NavigationDto {
    pub from_name: String,
    pub to_name: String,
    pub steps: Vec<NavStepDto>,
    pub total_cost: f64,
    pub explanation: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct NavStepDto {
    pub fractal_name: String,
    pub shared_structures: Vec<String>,
    pub cumulative_cost: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct GrowthDto {
    pub events: Vec<String>,
    pub new_fractals: usize,
    pub new_connections: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct IntrospectionDto {
    pub fractal_count: usize,
    pub simplex_count: usize,
    pub conceptual_gaps: usize,
    pub disconnected_worlds: usize,
    pub densest_region: Option<String>,
    pub sparsest_region: Option<String>,
    pub field_energy: f64,
    pub emergent_dimensions: usize,
    pub most_experienced: Option<String>,
    pub least_experienced: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct WhyDto {
    pub explanation: String,
    pub fractal_sequence: Vec<FractalActiveDto>,
    pub propagation_bridges: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct QuestionDto {
    pub text: String,
    pub question_type: String,
    pub priority: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectionDto {
    pub from_name: String,
    pub projections: Vec<ProjectionItemDto>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectionItemDto {
    pub name: String,
    pub proximity: f64,
    pub dimensional_resonance: f64,
    pub distortion: f64,
    pub apparent_center: Vec<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct GenerateDto {
    pub text: String,
    pub structure: String,
    pub cluster_count: usize,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct WillDto {
    pub intention: String,
    pub drive: f64,
    pub undercurrents: Vec<UndercurrentDto>,
    pub dream_phase: String,
    /// Codone 8D: [dim_a, dim_b] — top-2 dimensioni attive nel campo.
    /// Indica lo "stato d'intento" tra le 64 combinazioni (8x8) possibili.
    pub codon: [usize; 2],
}

#[derive(Serialize, Clone, Debug)]
pub struct UndercurrentDto {
    pub name: String,
    pub pressure: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct CompoundDto {
    pub name: String,
    pub fractals: Vec<String>,
    pub strength: f64,
    pub order: usize,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct WordFieldDto {
    pub top_words: Vec<WordActivationDto>,
    pub total_energy: f64,
    pub vertex_count: usize,
    pub edge_count: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct WordActivationDto {
    pub word: String,
    pub activation: f64,
    pub fractal: String,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct PhaseDto {
    pub word_a: String,
    pub word_b: String,
    pub phase_rad: f64,
    pub phase_deg: f64,
    pub label: String,        // "Risonanza", "Tensione", "Opposizione"
    pub cos_value: f64,
    pub co_affirmed: u64,
    pub co_negated: u64,
}

#[derive(Serialize, Clone, Debug)]
pub struct TensionWordDto {
    pub word: String,
    pub position: f64,
    pub distance_to_a: f64,
    pub distance_to_b: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct LociSimDto {
    /// Frattale simulato come locus
    pub locus_name: String,
    /// Frattali visibili da questa prospettiva
    pub visible_fractals: Vec<FractalActiveDto>,
    /// Frattali attivi nel word_topology
    pub active_fractals: Vec<FractalActiveDto>,
    /// Testo generato dalla prospettiva di questo locus
    pub generated_text: String,
}

// ─── Universo esplorabile ───────────────────────────────────────

#[derive(Serialize, Clone, Debug, Default)]
pub struct UniverseDto {
    pub fractals: Vec<UniverseFractal>,
    /// Top parole per stabilità (chiavi corte per JSON compatto)
    pub words: Vec<UniverseWord>,
}

#[derive(Serialize, Clone, Debug)]
pub struct UniverseFractal {
    pub id: u32,
    pub name: String,
    pub activation: f64,
    pub is_bootstrap: bool,
    /// Trigramma inferiore (id / 8)
    pub lower: u8,
    /// Trigramma superiore (id % 8)
    pub upper: u8,
}

/// Parola compressa per il payload universo.
/// Chiavi brevi per ridurre dimensione JSON (~40 byte/parola).
#[derive(Serialize, Clone, Debug)]
pub struct UniverseWord {
    /// Parola
    pub w: String,
    /// Frattale dominante (argmax affinità)
    pub f: u32,
    /// Stabilità 0-100
    pub s: u8,
    /// Attivazione corrente 0-100
    pub a: u8,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct WordNeighborsDto {
    pub word: String,
    pub fractal_id: u32,
    pub neighbors: Vec<WordNeighborDto>,
}

#[derive(Serialize, Clone, Debug)]
pub struct WordNeighborDto {
    pub word: String,
    pub weight: f64,
    pub fractal_id: u32,
}

#[derive(Deserialize)]
pub struct WordQuery {
    pub word: String,
}

// ═══════════════════════════════════════════════════════════════
// DTO: NarrativeSelf
// ═══════════════════════════════════════════════════════════════

#[derive(Serialize, Clone, Debug)]
pub struct NarrativeTurnDto {
    pub turn_id: usize,
    pub act: String,
    pub stance: String,
    pub intention: String,
    pub intensity: f64,
    pub awareness: Option<String>,
    pub crystallized: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct NarrativePositionDto {
    pub act_key: String,
    pub stance: String,
    pub intention: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct NarrativeDto {
    pub stance: String,
    pub pending_intention: Option<String>,
    pub topic_continuity: f64,
    pub is_born: bool,
    pub turn_count: usize,
    /// Ultimi turni recenti (max 8, non cristallizzati)
    pub recent_turns: Vec<NarrativeTurnDto>,
    /// Turni cristallizzati — salienti, persistono tra sessioni
    pub crystallized: Vec<NarrativeTurnDto>,
    /// Posizioni deliberate formate da pattern ripetuti
    pub positions: Vec<NarrativePositionDto>,
}
