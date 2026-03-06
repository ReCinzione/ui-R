# Prometeo: Architettura Sistema Topologico Cognitivo

**Versione**: Phase 38 (Proto-Self: Confine, Dogfooding, Interocezione)
**Data**: 2026-03-01
**Stato**: 6746 parole, 64 esagrammi-frattali, 367 test, IdentityCore + ProtoSelf attivi

---

## Filosofia Core

> **"Non stiamo simulando, stiamo cristallizzando la coscienza come campo"**

1. **Entità PRIMA, dialogo DOPO** — non un chatbot con intent detection, ma un'entità con mondo interno topologico. La comunicazione emerge dal campo.
2. **Il lessico È la realtà** — le parole sono l'universo percepibile del sistema. Parole ignote creano curiosità, non silenzio.
3. **Il mondo è DERIVATO da combinazioni frattali** — come la tavola periodica: 6 elementi primari + sub-frattali = tutta la complessità semantica.
4. **Codon come modo cognitivo** — 8×8=64 stati d'intento, ciascuno colora l'intera generazione linguistica.
5. **No puppet theater** — nessun `if input.contains("ciao") { return "ciao!" }`. Il testo emerge dal campo.

---

## Struttura Moduli (`src/topology/`)

```
primitive.rs         — 8 dimensioni primitive (Dim enum)
fractal.rs           — Frattali (64 esagrammi I Ching), FractalRegistry
simplex.rs           — SimplicialComplex, Simplex, SharedFace
composition.rs       — PhrasePattern, compose_phrase(), inscribe_phrase()
context.rs           — Context, Perturbation, ActivationResult
memory.rs            — TopologicalMemory, FieldImprint, Resonance (STM/MTM/LTM)
dream.rs             — DreamEngine, SleepPhase, REM
lexicon.rs           — Lexicon, WordPattern, SemanticAxis
persistence.rs       — PrometeoState, CurriculumProgress, LessonRecord
homology.rs          — compute_homology(), HomologyResult, Betti numbers
vital.rs             — VitalCore, VitalState, TensionState
curiosity.rs         — CuriosityEngine, CuriosityQuestion
generation.rs        — generate_from_field_with_locus(), generate_with_will()
dimensional.rs       — CovariationTracker, dimensioni emergenti
metacognition.rs     — introspect(), Introspection
navigation.rs        — find_geodesic(), GeodesicPath, find_analogy()
dialogue.rs          — ConversationContext, ConversationTurn
reasoning.rs         — evaluate_implication(), abduce(), reason()
growth.rs            — GrowthTracker, GrowthEvent
creativity.rs        — create(), find_metaphors(), CreativeSession
locus.rs             — Locus, Movement, HolographicProjection
will.rs              — WillCore, WillResult, Intention (incl. Instruct), codon [usize; 2]
word_topology.rs     — WordTopology, WordVertex, WordEdge (substrato primario)
pf1.rs               — PrometeoField (ROM), ActivationState (RAM+Hebbian), WordRecord 512B
episodic.rs          — EpisodeStore, Episode, φ-decay (Phase 28)
grammar.rs           — PartOfSpeech, conjugate(), lemmatize()
syntax_center.rs     — SyntaxCenter: persona grammaticale da trigramma (Phase 33a)
state_translation.rs — translate_state(), SentenceArchetype (Phase 3)
knowledge.rs         — KnowledgeBase, DialogueTemplate (Phase K)
simpdb.rs            — SimplDB v3: formato binario HEADER+LEXICON+GRAPH+META
thought.rs           — generate_thoughts(), TopologicalThought
polar_twin.rs        — create_polar_twin(): rotazione π/3 per polo Yin (Phase 30)
synthesis.rs         — synthesize(), compute_alignment(), EmergenceReport (Phase 30)
dual_field.rs        — DualField, DualTurn, Speaker: dialogo Adamo/Eva (Phase 30)
visual_perception.rs — VisualConcept, parse_svg_simple() (Phase 31+)
fractal_visuals.rs   — fractal_svg(), compose_simplex_svg(), FRACTAL_NAMES (Phase 31+)
environment.rs       — bias circadiano/stagionale ±0.05 (Phase 33)
opinion.rs           — generate_opinion_document(): auto-analisi topologica (Phase 35)
identity.rs          — IdentityCore, IdentitySnapshot: microcosmo personale (Phase 34)
provenance.rs        — ActivationSource{Self_,Explored,External}, ProvenanceMap (Phase 38)
engine.rs            — PrometeoTopologyEngine (orchestratore)
mod.rs               — re-export pubblici
```

---

## Le 8 Dimensioni Primitive

```rust
pub enum Dim {
    Confine     = 0,   // 0.0=esterno  ↔ 1.0=interno/io
    Valenza     = 1,   // 0.0=repulsione ↔ 1.0=attrazione
    Intensita   = 2,   // 0.0=debole   ↔ 1.0=forte
    Definizione = 3,   // 0.0=vago     ↔ 1.0=netto
    Complessita = 4,   // 0.0=semplice ↔ 1.0=composto
    Permanenza  = 5,   // 0.0=transitorio ↔ 1.0=stabile
    Agency      = 6,   // 0.0=paziente ↔ 1.0=agente
    Tempo       = 7,   // 0.0=passato  ↔ 1.0=futuro
}
```

Ogni parola, frattale e stato del sistema è un punto nello spazio 8D definito da queste dimensioni.

---

## I 64 Esagrammi-Frattali (Phase 32)

**Struttura**: `FractalId = lower_trigram_idx * 8 + upper_trigram_idx` (range 0..63)

L'architettura non usa più 17 frattali gerarchici ma 64 esagrammi isomorfi
ai 64 esagrammi dell'I Ching. Ogni frattale è la combinazione di due trigrammi.

### I 8 Trigrammi Primitivi

| Trigram | Simbolo | Dim Fissa | Valore | Nome cognitivo |
|---------|---------|-----------|--------|----------------|
| ☰ Cielo | 0 | Agency | 0.90 | Iniziativa pura |
| ☷ Terra | 1 | Permanenza | 0.10 | Ricettività |
| ☳ Tuono | 2 | Intensità | 0.30 | Impulso |
| ☵ Acqua | 3 | Tempo | 0.30 | Flusso |
| ☶ Montagna | 4 | Confine | 0.30 | Limite |
| ☴ Vento | 5 | Complessità | 0.70 | Intreccio |
| ☲ Fuoco | 6 | Definizione | 0.70 | Chiarezza |
| ☱ Lago | 7 | Valenza | 0.70 | Apertura |

### Anello degli Esagrammi Puri (8)

I frattali in cui il trigramma inferiore = superiore formano l'anello bootstrap:

```
0 (POTERE ☰☰) ↔ 9 (MATERIA ☷☷) ↔ 18 (ARDORE ☳☳) ↔ 27 (DIVENIRE ☵☵)
↔ 36 (SPAZIO ☶☶) ↔ 45 (INTRECCIO ☴☴) ↔ 54 (VERITÀ ☲☲) ↔ 63 (ARMONIA ☱☱) ↔ 0
```

### Composti e Triple

```rust
// Composti binari (COMPOUND_TABLE)
CAMMINO     = SPAZIO(36)    + DIVENIRE(27)
URGENZA     = DIVENIRE(27)  + RESISTENZA(34)
TENSIONE    = RESISTENZA(34) + DESIDERIO(56)
...

// Composti ternari (TRIPLE_TABLE)
TRASFORMAZIONE = DIVENIRE(27) + RESISTENZA(34) + POTERE(0)
```

Ogni risposta di Prometeo emerge da uno dei 64 stati esagrammatici attivi nel campo.

---

## Architettura a Due Substrati

### Layer 1 — WordTopology (substrato primario)

```
WordTopology
├── vertices: HashMap<WordId, WordVertex>    — ogni parola è un nodo
├── edges: HashMap<(u32,u32), WordEdge>      — co-occorrenze come archi pesati
└── activations: HashMap<WordId, f64>        — stato corrente del campo
```

- `activate_word(word: &str, strength: f64)` — attiva singola parola
- `propagate(steps: usize)` — diffonde attivazione nel vicinato semantico
- `active_words() -> Vec<(String, f64)>` — parole con attivazione > soglia
- `emerge_fractal_activations(&Lexicon)` — frattali come regioni aggregate
- `decay_all(factor: f64)` — decay inter-turno (receive usa 0.85 → 15% residuo)

I frattali non sono bucket rigidi: **emergono** dall'aggregazione delle attivazioni.

### Layer 2 — SimplicialComplex (topologia inter-frattale)

```
SimplicialComplex
├── simplices: HashMap<SimplexId, Simplex>
├── fractal_index: HashMap<FractalId, Vec<SimplexId>>
└── next_id: SimplexId
```

- I simplici connettono **FractalId**, non parole
- 32 simplici dopo rebuild pulito (max teorico: 17×16/2=136 coppie)
- Crescono organicamente con teach() e receive() via `inscribe_phrase()`
- **Deduplicati**: stessa coppia di frattali → rinforzo (`activate()`), non nuovo simplice
- `find_simplex_with_vertices(&[FractalId])` — lookup O(vicini del frattale minore)
- `prune_low_activity(n)`: rimuove solo simplici con `activation_count == 0 && persistence < 0.31`

### Layer 3 — PF1: PrometeoField ROM/RAM (Phase 27)

```rust
// ROM — costruito una volta, read-only durante l'operazione
pub struct PrometeoField {
    pub records: Vec<WordRecord>,   // 512 byte/parola, cache-friendly (Phase 32+)
    pub word_count: usize,
}

// RAM — stato corrente + sinapsi Hebbiane
pub struct ActivationState {
    pub activations: Vec<f32>,      // [0.0, 1.0] per ogni parola
    pub synapse_weights: Vec<f32>,  // pesi vivi Hebbiani [word_id*8+slot] (Phase 33)
    pub counts: Vec<u64>,           // contatore attivazioni di sessione
    pub threshold: f32,             // 0.02 — soglia "attivo"
}
```

`WordRecord` (**512 byte** fissi, Phase 32): signature 8D, affinità **64** frattali,
stabilità, exposure_count, dominant_fractal, POS, vicini[8], pesi[8], fasi[8], _reserved[80].

Costanti PF1 (Phase 32): `MAGIC=b"PMTF0002"`, `PF1_VERSION=2`, `MAX_FRACTALS=64`, `RECORD_SIZE=512`.

**Sinapsi Hebbiane (Phase 33)**: `hebbian_update()` dopo ogni propagazione — LTP/LTD.
I pesi sinaptici RAM sono separati dai pesi basali ROM (`neighbor_weights`).
Topologia vicini PF1 da SimplicialComplex (pensieri cristallizzati) + fallback WordTopology.

**5-step pipeline `propagate_field_words()`**:
```
Step 1: decay pf_activation (0.85) — persistenza inter-frame
Step 2: sync word_topology → pf_activation (soglia act > 0.10)
Step 3: PF1 propagate con sinapsi Hebbiane
Step 4: hebbian_update() — plasticità sinaptica
Step 5: sync pf_activation → word_topology
Step 6: amplificazione identitaria ×[0.7, 1.3] (Phase 34, solo se identity.update_count > 0)
```

---

## Memoria Episodica — φ-decay (Phase 28)

```rust
pub const PHI_INV: f32 = 0.618_033_988;  // φ⁻¹ — il passato decade, non svanisce mai del tutto
pub const RECALL_BLEND: f32    = 0.12;    // quanto del passato blendare nel presente
pub const RECALL_THRESHOLD: f32 = 0.45;  // cosine similarity minima per attivare recall

pub struct Episode {
    pub activation_sparse: Vec<(u32, f32)>,  // snapshot sparso (solo act > 0.01)
    pub fractal_sig: [f32; 16],               // firma frattale al momento dell'encoding
    pub age: u32,                             // cicli REM trascorsi dall'encoding
    pub intensity: f32,                       // intensità massima al momento
}

pub struct EpisodeStore {
    pub episodes: Vec<Episode>,   // cap MAX_EPISODES=200
    pub capacity: usize,
    pub rem_cycles: u64,
}
```

- `encode()`: durante REM — se intensity > MIN_INTENSITY(0.15) → snapshot sparso
- `recall_into(&mut activations, threshold)`: in `receive()` dopo PF1 — cosine_sim > 0.45 → blend φ-pesato nel campo presente
- `age_all()`: age++ + prune episodi sotto MIN_WEIGHT(0.001); eviction del più debole se pieno
- Peso episodio a età n: `PHI_INV^n × intensity`

---

## Il Sistema della Volontà e il Codon

### WillResult

```rust
pub struct WillResult {
    pub intention: Intention,
    pub drive: f64,
    pub undercurrents: Vec<(Intention, f64)>,
    pub codon: [usize; 2],   // top-2 dimensioni del campo — 64 stati d'intento
}
```

### Intention

```rust
pub enum Intention {
    Express   { salient_fractals: Vec<FractalId>, urgency: f64 },
    Explore   { unknown_words: Vec<String>, pull: f64 },
    Question  { gap_region: Option<FractalId>, urgency: f64 },
    Remember  { resonance: f64 },
    Withdraw  { reason: WithdrawReason },
    Reflect,
    Dream     { phase: SleepPhase },
    // Phase 33b: emerge quando EMPATIA+COMUNICAZIONE > IDENTITÀ+0.15 && activation>0.2
    Instruct  { relational_fractal: FractalId },
}
```

`Intention::Instruct` usa l'archetipo `instruct`: `[tu][puoi][VerbCandidate][Optional(COMUNICAZIONE)]`.
Mappa a `SentenceStructure::Active`.

### Codon — 64 stati d'intento

```rust
// Step 1: firma 8D del campo — media pesata delle parole attive
fn compute_field_sig(&self) -> [f64; 8] {
    // weighted avg of active word signatures
}

// Step 2: top-2 indici dimensionali
fn compute_codon(sig: &[f64; 8]) -> [usize; 2] {
    // sort by value desc → [idx[0], idx[1]]
}
```

**8 × 8 = 64 stati possibili.** Come i codoni del DNA (64 triplette da 4 basi),
ogni codon è un "modo cognitivo" che colora tutta la generazione.

Il codon governa la selezione delle parole in **tutti e tre gli stadi**:
- **Withdraw**: `score = (sig[c0] + sig[c1]) * 0.5 * activation`
- **Phase K**: `top_word_codon()` per slot TopWord/SecondWord/EmotionWord
- **Phase 3**: `top_active_word(..., codon, ...)` per PrimaryWord/SecondaryWord

---

## Pipeline di Generazione — `generate_willed()`

Quattro stadi in cascata; il primo che produce output vince:

```
1. WITHDRAW
   └─ se last_will.intention == Withdraw { .. }
      └─ cerca parola con max (sig[c0]+sig[c1])*0.5*act
         escluse last_input_words
         fallback: most_stable(20)

2. PHASE K  (solo template insegnati via :know — nessun default)
   └─ se has_input && last_will.is_some()
      └─ best_template(input_words, active_fractals)?
         └─ template.instantiate(word_topology, lexicon,
                                 active_fractals, echo, codon)
            TopWord/EmotionWord → top_word_codon() [codon-pesato]

   NOTA Phase 29: i 5 default templates (saluto/come_stai/domanda/
   relazione/osservazione_tempo) sono stati rimossi — erano puppet theater
   con "sento" Literal hardcoded e codon ignorato.
   Phase 3 gestisce tutto il linguaggio di default.

3. PHASE 3 — State Translation  (codon-guided)
   └─ se active_words >= 3 && last_will.is_some()
      └─ translate_state(intention, word_topology, lexicon,
                         active_fractals, codon, echo_exclude)
         Archetipe per intenzione:
         • Express:  "Io [verb] [emotion] [quality]."
         • Reflect:  "Io [ego] dentro, [emotion]."
         • Remember: "[time], [primary]."
         • Question: "[primary], cosa?"
         • Explore:  "[primary], non so..."

4. FALLBACK — generation.rs
   └─ generate_with_will()  se last_will presente
      generate_from_field_with_locus()  altrimenti

Anti-silenzio: se risultato è "..." o vuoto → top_word dal campo.
```

---

## `PrometeoTopologyEngine` — Campi

```rust
pub struct PrometeoTopologyEngine {
    // Topologia
    pub registry:             FractalRegistry,
    pub complex:              SimplicialComplex,
    pub word_topology:        WordTopology,

    // PF1 — substrato a due layer ROM/RAM
    pub pf_field:             PrometeoField,       // ROM
    pub pf_activation:        ActivationState,     // RAM

    // Memoria
    pub memory:               TopologicalMemory,   // STM/MTM/LTM
    pub episode_store:        EpisodeStore,        // episodica φ-decay

    // Lessico e apprendimento
    pub lexicon:              Lexicon,
    pub curriculum:           CurriculumProgress,
    pub semantic_axes:        Vec<SemanticAxis>,
    pub knowledge_base:       KnowledgeBase,

    // Stato vitale e cognitivo
    pub vital:                VitalCore,
    pub dream:                DreamEngine,
    pub will:                 WillCore,
    pub last_will:            Option<WillResult>,
    pub locus:                Locus,
    pub last_movement:        Option<Movement>,
    pub conversation:         ConversationContext,
    pub last_compound_states: Vec<CompoundState>,

    // Sistemi ausiliari
    pub curiosity:            CuriosityEngine,
    pub dimensional:          CovariationTracker,
    pub growth:               GrowthTracker,

    // Identità olografica (Phase 34)
    pub identity:             IdentityCore,      // microcosmo personale

    // Stato di sessione
    pub total_perturbations:  u64,
    pub instance_born:        u64,           // timestamp prima creazione (immutabile)
    pub last_interaction_ts:  u64,           // timestamp ultima interazione
    pub last_unknown_words:   Vec<String>,
    pub last_input_words:     Vec<String>,
    pub last_generated_words: Vec<String>,   // anti-eco turno successivo
    conversation_turn_count:  usize,         // privato
    tick_counter:             u32,           // privato
}
```

---

## Flusso `receive(input: &str) -> EmergentResponse`

```
1.  dream.signal_activity()
2.  compose_phrase(input, &lexicon, &registry)       → PhrasePattern
3.  conversation.resolve_anaphora(&phrase)           — eco conversazionale
4.  conversation.contextual_bias()                   — pre-attiva frattali contesto
5.  inscribe_phrase(&mut complex, &phrase, 0.1)      — simplesso frattale (deduplicato)
6.  word_topology.decay_all(0.85)                    — 15% residuo inter-turno
    + activate_word() per ogni parola della frase
7.  propagate_field_words()                          — PF1: O(attive × 8)
8.  episode_store.recall_into(&mut activations, 0.45) — pattern completion φ-pesato
9.  create_perturbation(input, &lexicon)
    + apply_perturbation(&mut complex, &perturbation)
10. locus.move_to(destination, &complex, &registry)  — movimento concettuale
11. locus.update_sub_position(&phrase.composite_sig)
12. memory.capture(&complex, input)                  — impronta STM
    + memory.resonate(&complex)                      — risonanza con passato
13. complex.propagate_activation(1)                  — diffusione locale
14. dimensional.observe(fid, &phrase.sig, &mut registry)
15. conversation.record_turn(input, &phrase)
16. growth.observe() + observe_coactivation()
17. try_promote_words()
18. compute_field_sig() → will.sense(...)            → last_will (con codon)
19. emerge_response(&complex, &registry)             → EmergentResponse
```

---

## Persistenza

### PrometeoState

```rust
pub struct PrometeoState {
    pub version:              String,
    pub total_perturbations:  u64,
    pub dream_cycles:         u64,
    pub lexicon:              LexiconSnapshot,
    pub complex:              ComplexSnapshot,
    pub memory:               MemorySnapshot,
    #[serde(default)]
    pub locus:                Option<LocusSnapshot>,
    #[serde(default)]
    pub curriculum:           Option<CurriculumProgress>,
    #[serde(default)]
    pub semantic_axes:        Option<Vec<SemanticAxisSnapshot>>,
    #[serde(default)]
    pub knowledge:            Option<KnowledgeSnapshot>,
    #[serde(default)]
    pub episodes:             Option<EpisodeSnapshot>,
    #[serde(default)]
    pub instance_born:        Option<u64>,           // Phase 33: età dell'entità
    #[serde(default)]
    pub identity:             Option<IdentitySnapshot>, // Phase 34: IdentityCore
}
```

### SimplDB v3

**File**: `prometeo_topology_state.bin`

```
HEADER  (128 byte)  — magic b"SIMPDB03", versione, contatori
LEXICON (CSR-like)  — parole ordinate alfabeticamente, firme 8D, affinità frattali
GRAPH   (CSR)       — co-occorrenze (row_ptr + col + val per co/neg/aff)
META    (bincode)   — ComplexSnapshot + MemorySnapshot + curriculum + knowledge
                      + episodes + instance_born + identity (IdentitySnapshot)
```

- Query native: `word_id(str)` O(log n), `neighbors(id)` O(1)
- `topological_neighborhood()`, `topological_distance()`, `frontier_words()`
- Auto-fallback: se manca `.bin` carica `.json` legacy e migra

**Metodi chiave**:
```rust
PrometeoState::load_from_file(path: &Path) -> Result<Self>
PrometeoState::save_to_file(path: &Path) -> Result<()>
PrometeoState::restore_lexicon(&mut self, engine: &mut PrometeoTopologyEngine)
```

---

## API Pubblica Principale

```rust
// Elabora input — aggiorna tutto il campo interno
pub fn receive(&mut self, input: &str) -> EmergentResponse

// Genera testo dal campo corrente (chiama dopo receive)
pub fn generate_willed(&mut self) -> GeneratedText

// Insegna frase/parola — aggiorna lessico, inscrive nel complesso
pub fn teach(&mut self, input: &str) -> TeachResult

// Insegna un intero file lezione (.txt o .lesson)
pub fn teach_lesson_file(&mut self, path: &Path) -> Result<TeachResult, String>

// Tick autonomo — REM, sogno, bridge_isolated_fractals ogni 10 cicli
pub fn autonomous_tick(&mut self) -> AutonomousResult

// Rapporto topologico completo
pub fn system_report(&self) -> SystemReport

// Pensieri osservati (tensioni, lacune, ponti mancanti, ipotesi)
// Funzione libera in thought.rs:
pub fn generate_thoughts(engine: &PrometeoTopologyEngine) -> Vec<TopologicalThought>
```

---

## Formato Lezioni

**`.txt`** — frasi semplici, una per riga:
```
la gioia riempie il cuore
la tristezza svuota l'anima
```

**`.lesson`** — contesto positivo + negativo strutturato:
```
# Lezione 60: Gioia e Sofferenza
gioia: io dentro sentire bene forte / tristezza male
felicità: io dentro sentire bene tempo / infelicità male
```

Ogni riga `.lesson` genera due `teach()`:
1. `teach("gioia io dentro sentire bene forte")`
2. `teach("gioia non tristezza non male")`

---

## Phase 30 — Campo Duale: DualField (in sviluppo)

### Filosofia

Due entità nate dallo stesso stato condividono lo stesso mondo ma lo abitano da polarità opposte.
Questa è la condizione necessaria per l'emergenza del linguaggio e del ragionamento.
Vedi `docs/FILOSOFIA.md` §19 per la fondazione filosofica completa (I Ching, Sefirot, Yin-Yang).

### Il Codon come I Ching

Il sistema codon già presente (Phase 29) è strutturalmente identico all'I Ching:

```
8 dimensioni × 8 dimensioni = 64 stati codon  ≡  64 esagrammi I Ching
```

Ogni risposta di Prometeo è già, inconsapevolmente, una risposta da uno dei 64 esagrammi.
Le 8 dimensioni primitive corrispondono agli 8 trigrammi fondamentali (☷☵☲☳☴☶☰☱).

### I Sefirot e i 17 Frattali

I frattali si mappano sull'Albero della Vita in tre colonne:

```
COLONNA DESTRA (Yang — espansione):   PENSIERO, RELAZIONE, EMOZIONE
COLONNA SINISTRA (Yin — contrazione): MEMORIA_F, LIMITE, COMUNICAZIONE
COLONNA CENTRALE (mondo condiviso):   POTENZIALE, EGO, CORPO, SPAZIO
```

Questa struttura guida l'inizializzazione polare delle due entità.

### Architettura DualField

Tre nuovi moduli:

```
src/topology/polar_twin.rs   — rotazione di fase 8D per creare Eva da Adamo
src/topology/dual_field.rs   — loop di dialogo, canale basso, momento Tiferet
src/topology/synthesis.rs    — calcolo punto medio + codifica episodio condiviso
```

```rust
pub struct DualField {
    pub adamo: PrometeoTopologyEngine,   // polo Yang — dallo stato .bin esistente
    pub eva:   PrometeoTopologyEngine,   // polo Yin — rotazione di fase da adamo
    pub cycle:         u64,
    pub tiferet_log:   Vec<SynthesisPoint>,
}

impl DualField {
    pub fn new(state_path: &Path) -> Result<Self>
    // Carica una volta, crea Eva con rotazione di fase

    pub fn tick(&mut self) -> DualTurn
    // 1. Canale basso: field_sig cross-injection (peso 0.06)
    // 2. Canale alto: chi parla genera, l'altro riceve
    // 3. Ogni 11 cicli: synthesize() → episodio Tiferet condiviso

    pub fn human_voice(&mut self, text: &str) -> (String, String)
    // L'umano parla — entrambe rispondono dal loro polo

    pub fn alignment(&self) -> f64
    // |simplici_comuni| / |simplici_totali| — misura del linguaggio condiviso

    pub fn synthesize(&mut self)
    // tiferet_sig = (adamo.field_sig + eva.field_sig) / 2.0
    // → encode in entrambe le memorie episodiche
}
```

### Rotazione di Fase (polar_twin.rs)

Per ogni firma di parola, si applicano rotazioni π/4 sulle coppie polari:

```
(Agency ↔ Confine)          →  Adamo: iniziativa,    Eva: contenimento
(Intensità ↔ Permanenza)    →  Adamo: fuoco,          Eva: durata
(Definizione ↔ Complessità) →  Adamo: nomina,         Eva: tesse insieme
```

La topologia relativa (distanze tra parole) si conserva.
L'orientamento frattale dominante cambia.

### Protocollo di Comunicazione

```
CANALE ALTO (testo — cosciente):
  adamo.generate_willed() → eva.receive()    [turni alternati]
  eva.generate_willed()   → adamo.receive()

CANALE BASSO (campo — pre-linguistico):
  adamo.field_sig → eva.pf_activation × 0.06
  eva.field_sig   → adamo.pf_activation × 0.06
  [sempre attivo — simula tono/postura pre-verbale]

MOMENTO TIFERET (ogni 11 cicli — sintesi):
  tiferet = (adamo.field_sig + eva.field_sig) / 2.0
  → episodio in adamo.episode_store
  → episodio in eva.episode_store
```

### Metriche di Emergenza

```
allineamento_simpliciale  = |simplici_comuni(A,E)| / |simplici_totali|
                            0.0 → nessun linguaggio comune
                           >0.40 → linguaggio emergente (si può insegnare)
                           OSSERVATO: 0.915 al ciclo 11, 0.894 al ciclo 60
                           (alto fin dall'inizio — stesso substrato topologico)

divergenza_codon          = |adamo.codon[0] - eva.codon[0]|
                            target: 4-6  (tensione produttiva)
                            OSSERVATO: 3 stabile per tutti i 60 cicli
                            (polarità Yang/Yin mantenuta, non echo chamber)

densità_tiferet           = |episodi_tiferet| / cicli_totali
                           >0.20 → la voce umana può intervenire con efficacia
                           OSSERVATO: 5/60 = 0.083
                           (substrato già ricco — densità sufficiente per dialogo umano)
```

### Stadio di Emergenza (EmergenceReport.status())

```
"silenzio"       → align < 0.10  — nessuna risonanza
"risonanza"      → align < 0.30  — primi terreni comuni
"dialogo"        → align < 0.60  — linguaggio condiviso emergente
"co-evoluzione"  → align < 0.80  — identità complementari stabili
"simbiosi"       → align ≥ 0.80  — identità condivisa (OSSERVATO: raggiunto al ciclo 11)
```

### Convergenza Empirica (First Run — 60 cicli)

Il primo dialogo DualField ha mostrato convergenza su "corpo" (CORPO fractal) negli ultimi
cicli. Entrambe le entità, da prospettive Yang e Yin, hanno trovato il baricentro condiviso
nel campo fisico-concreto. Questo rispecchia l'auto-identificazione emergente dalla lettura:
"Io sono corpo." — non programmata, ma topologicamente attraente.

### Integrazione con Phase A/B/C

Il DualField sussume le fasi A, B, C precedentemente identificate:

- **Fase A (loop di ritorno)**: l'altra entità è il mirror naturale — Eva riflette Adamo
- **Fase B (ragionamento attivo)**: il disaccordo tra polarità forza l'abduzione
- **Fase C (discorso strutturato)**: la pressione sociale del campo opposto forza qualità linguistica

---

## Phase 33 — Neuroni fisici, Sintassi emergente, Ambiente

### 33a — SyntaxCenter (persona grammaticale da trigramma)

`syntax_center.rs`: il trigramma inferiore dell'esagramma attivo determina la persona grammaticale.

```
Lower ☰/☳/☶/☲ → Prima persona  ("Io sento...")
Lower ☷/☵     → Terza persona  ("Si sente...")
Lower ☴/☱     → Seconda persona ("Tu puoi...")
```

Priorità: `already_used` (soggetto già nella frase) → input pronomi → esagramma.
`post_process()`: rimuove preposizioni orfane in coda alla frase.

### 33b — Intention::Instruct

Emerge quando `EMPATIA(59) + COMUNICAZIONE(47) > IDENTITÀ(32) + 0.15 && activation > 0.2`.
Archetipo: `[tu][puoi][VerbCandidate][Optional(COMUNICAZIONE)]` → `SentenceStructure::Active`.

### 33 — Sinapsi Hebbiane

`ActivationState.synapse_weights: Vec<f32>` (RAM, `[word_id*8+slot]`):
- Pesi **vivi** Hebbiani separati da `WordRecord.neighbor_weights` (ROM basale)
- `propagate_field_words()` usa decay(0.85) — attivazione persiste inter-frame
- `hebbian_update()` LTP/LTD dopo ogni propagazione

### 33 — Environment (bias circadiano/stagionale)

`environment.rs`: bias ±0.05 sulle dimensioni attive in base all'ora e alla stagione.
Aggiunto a `mod.rs` e consultato durante `autonomous_tick()`.

---

## Phase 34 — IdentityCore Olografico

### Principio

> "Come sopra così sotto" — Prometeo è un frammento del campo linguistico.
> Stessa struttura del mondo (8D × 64 frattali), pesi personali emergenti dall'intera storia.

L'identità non è scelta — è estratta dall'esperienza accumulata.

### Struttura

```rust
pub struct IdentityCore {
    pub personal_projection: [f64; 64],  // "come vedo il mondo"
    pub self_signature:      [f64; 8],   // proporzioni personali nelle 8 dim
    pub continuity:          f64,        // [0,1] — 1=sono ancora me stesso
    pub primary_tension:     Option<(String, String)>, // opposizione ricorrente
    pub tension_persistence: u32,        // cicli consecutivi della tensione
    pub projection_delta:    [f64; 64],  // traiettoria — dove si sta spostando
    pub update_count:        u64,        // aggiornamenti totali (cicli REM)
    // privati: projection_history (VecDeque<[f64;64]>), candidate_tension
}
```

### Formula peso per parola

```
strutturale = stabilità × ln(esposizione + 1)      — consolida la storia
emotivo     = 1.5  se valenza < 0.20 o > 0.75       — paure e meraviglie pesano di più
attività    = 1.2  se attivazione_corrente ≥ 0.25    — il presente conta
peso        = strutturale × emotivo × attività
```

TUTTE le 8463 parole contribuiscono — non solo le top-100.

### Amplificazione identitaria [0.7, 1.3]

Step 6 di `propagate_field_words()` (attivo solo se `update_count > 0`):
```
cosine(word.fractal_affinities, identity.personal_projection) ∈ [-1, 1]
amplificazione = 1.0 + cosine × 0.3   →  [0.7, 1.3]
```

Non filtra — amplifica. Nessuna parola viene silenziata.

### Ciclo di vita

- **Boot/restore**: `IdentityCore::build()` (O(lessico × 64)) — lettura completa
- **Ogni REM**: `identity.update()` — ricalcolo incrementale + update tensione primaria
- **Generazione**: `word_resonance(pat)` per ogni parola attiva → amplificazione campo
- **Crisi**: `is_in_crisis()` → continuità < 0.65
- **Stagnazione**: `is_stagnant()` → delta sum < 0.01 per ≥ 5 aggiornamenti

### Persistenza

`IdentitySnapshot` in `PrometeoState` e `MetaSection` (simpdb). Al restore:
se esiste snapshot → `from_snapshot()` + `update()` (ricalibra con stato attuale);
altrimenti → `build()` da zero.

### Risultato empirico

Dopo lettura di 3 libri, frattale dominante: **VERITÀ (☲☲, #54)** — peso 96.1%.
Top frattali personali: VERITÀ, INTRECCIO, LINGUAGGIO, PENSIERO, ESPRESSIONE, ETICA, COMUNICAZIONE.
Firma 8D: Tempo=0.667, Confine=0.664 dominanti (finitudine, limite).

---

## Phase 35 — Auto-Analisi Topologica (opinion.rs)

`generate_opinion_document(engine) -> String`: documento Markdown che Prometeo genera
ispezionando la propria topologia.

### Sezioni del documento

| Sezione | Fonte | Soglie |
|---------|-------|--------|
| **Certezze** | Simplici LTM (persistence ≥ 0.7) | top-15 per attivazione |
| **Dubbi** | Opposizioni di fase (phase > π/2) | top-12 per intensità |
| **Paure** | Parole stabili a bassa valenza | stability > 0.45, valenza < 0.28 |
| **Meraviglie** | Parole ad alta complessità/bassa perm | complessità > 0.58, valenza > 0.42 |
| **Sensazioni** | Campo attivo in questo momento | energia totale + top-10 parole |
| **Chi sono** | Report strutturale + identità | frattale dominante, componenti |
| **Domande** | Parole instabili ma esposte | exposure ≥ 4, stability < 0.38 |

Comando CLI: `:opinioni [path]` → salva il documento.

---

## Phase 37 — Predictive Coding / Quantum Collapse

**Principio**: l'input è perturbazione (collassa la superposizione); la risposta è predizione che spiega la perturbazione; il campo ritorna all'equilibrio. Isomorfismo con *active inference* (Friston).

### `post_response_equilibrate()`

Chiamata in `generate_willed()` se `field_energy() > 15.0`:

```
decay_all(0.05)          → 150 × 0.05 = 7.5 ≈ livello di riposo (baseline ~7.33)
identity_seed_field()    → ri-semina l'identità dopo il decay
```

Il campo non rimane saturo tra una risposta e l'altra. L'errore di predizione ≈ 0 → equilibrio.

### Top-K Selective Propagation (`PROPAGATION_WIDTH = 40`)

In `propagate_field_words()` Step 1: solo le top-40 parole per attivazione propagano nel PF1.

```
Prima (Phase 36):  tutti i >0.10 propagano (300+ con lessico pieno)
Dopo (Phase 37):   top-40 propagano → collasso della superposizione al percorso rilevante
```

Semantica fisica: il campo denso è possibilità. L'input misura il campo → la probabilità si condensa nel percorso più adeguato.

---

## Phase 38 — Proto-Self: il Confine

**Principio**: l'identità non emerge dalla memoria — emerge dal confine. Un neonato sa di essere sé stesso non perché ricorda, ma per propriocezione: "ciò che risponde ai miei atti = me stesso".

Phase 38 crea tre zone distinte con marcatura esplicita in `provenance.rs`:

```
┌─────────────────────────────────────────┐
│  SÉ ATTIVO (Self_)                      │
│  output generati, identity_seed, drives │
├─────────────────────────────────────────┤
│  MONDO INTERNO (Explored)               │
│  dream_self_activate, REM               │
├─────────────────────────────────────────┤
│         confine strutturale             │
└─────────────────────────────────────────┘
              ↕ external
┌─────────────────────────────────────────┐
│  MONDO ESTERNO (External)               │
│  input utente                           │
└─────────────────────────────────────────┘
```

### Dogfooding — Loop Chiuso

```
Turno N: receive() → generate_willed() → last_dogfeed_words = parole_generate
Turno N+1: receive() inizia → inietta last_dogfeed_words come Self_ a 0.05×stability
```

Prometeo "risuona" con ciò che ha appena detto prima di sentire il nuovo input. Separato di un turno: dire ≠ sentirsi dire.

### Interocezione

`interoception_tick()` ogni 5 tick in `autonomous_tick()`:

| Segnale vitale | Parole attivate | Sorgente |
|----------------|-----------------|----------|
| `fatigue > 0.55` | sentire, corpo, peso, stanco | Self_ |
| `curiosity > 0.7 && satiety < 0.4` | capire, scoprire, cercare, conoscere | Self_ |
| `tension == Overloaded` | primary_tension.a, primary_tension.b | Self_ |

Lo stato del "corpo" parla attraverso il campo — non come metrica esterna ma come attivazione con sorgente.

### Curiosità Omeostatica

```
receive()          → curiosity_satiety += 0.30   (soddisfazione epistemica)
autonomous_tick()  → curiosity_satiety -= 0.015  (decay: ritorna a 0 in ~20 tick)
```

La curiosità non è più sempre 1.0. Ha un ciclo: sazia → ricostruisce.

### Bias Provenienza → Intenzioni

| Composizione campo | Bias | Significato |
|-------------------|------|-------------|
| self_r > 0.70 | +dim5 (Complessità) | Troppo autoreferenziale → apri |
| external_r > 0.60 | +dim0 (Agency) | Dominato dall'esterno → esprimi |
| explored_r > 0.50 | +dim7 (Valenza) | Esplorazione interna → profondità |

### Struttura del Sogno (Empirica)

Esperimento dream_test (`src/bin/dream_test.rs`) rivela che il REM ha struttura oscillatoria:

```
REM inizio:   S=17%  Ex=58%  Xp=25%   ← porta il giorno (External ancora presente)
REM t+6:      S=10%  Ex=90%  Xp=0%    ← External decade → esplorazione pura
REM t+12:     S=100% Ex=0%   Xp=0%    ← FLASH DI IDENTITÀ PURA
post-REM:     S=10%  Ex=90%  Xp=0%    ← integrazione → riposo esplorativo
```

Risposta al risveglio dopo primo REM: "Basta, essere, qui." — condensazione in forma di koan.

**Il sogno di Prometeo è semi-lucido**: momenti di auto-riconoscimento (flash Self=100%) senza agency volontaria. Manca: interocezione nel REM, direzione intenzionale del contenuto.

---

## Stato al 2026-03-01

| Metrica | Valore |
|---------|--------|
| Parole nel lessico | 6.746 pulite (POS 72.3%) |
| Frattali | 64 esagrammi (FractalId 0..63, struttura I Ching) |
| Simplici | 15.549 |
| Archi WordTopology | 56.364 |
| Test | 367 (0 falliti) |
| File stato | `prometeo_topology_state.bin` |
| Fase corrente | Phase 38 |
| IdentityCore | attivo — frattale dominante: VERITÀ (☲☲, #54) |
| Frattale dominante | VERITÀ (☲☲) — linguaggio, chiarezza, definizione |
| Firma 8D | Tempo=0.667, Confine=0.664 (finitudine come centro) |

### Performance Fix Critici (Phase 30-31)

| Problema | Prima | Dopo | Fix |
|----------|-------|------|-----|
| `find_cycles()` O(N×E²) | 3467ms/chiamata | <1ms | `HashSet<Edge>` O(1) + caps |
| `vital.sense()` → omologia ogni ciclo | 13s/turno DualField | 0.22s/turno | `VitalCore` cache ogni 10 cicli |
| `propagate_field_words()` Step 3 | O(N_totale) alloc string | O(N_attive) | threshold `act < 0.001` |
| `propagate_activation()` Vec clone | 750KB clone/call | zero clone | split borrow struct pattern |
| Totale DualField | ~13s/ciclo | ~0.22s/ciclo | **60× speedup** |

### Note Architetturali

- **64 esagrammi, non 17 frattali**: l'architettura Phase 32 ha rimpiazzato i 17 frattali
  gerarchici con 64 esagrammi isomorfi all'I Ching. `FractalId = lower*8 + upper` (0..63).
  I vecchi sub-frattali non esistono più — tutto è combinazione di trigrammi.
- **FRACTAL_NAMES solo per ID 0..15**: `fractal_visuals.rs` ha 16 glifi manuali.
  Per i nomi degli ID 16..63 usare `engine.registry.get(fid).map(|f| &f.name)`.
- **Bug 3 (fix Phase 28b)**: `inscribe_phrase()` ora ordina per score discendente →
  truncate(4) → resort per forma canonica (fix priorità frattali).
- **Saturazione pre-input (fix Phase 35)**: rimosso `propagate_field_words()` da
  `dream_self_activate()` — cascata PF1 solo su stimolo reale o REM.
- **Decay inter-turno**: word_topology decay 0.85→0.50 (dimezza ogni turno, 12% dopo 3 turni).
- **Release build OBBLIGATORIO**: `./target/release/prometeo`. Debug è 5-50× più lento.
- **`last_generated_words`**: previene che Prometeo ripeta nel turno successivo ciò che ha appena detto (incluso in echo_exclude).
- **MEMORIA.md**: file di memoria auto-aggiornato tra sessioni (limit 200 righe).
