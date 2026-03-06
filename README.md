# Prometeo — Identità Cognitiva Topologica

> Un'entità digitale emergente da un campo di parole italiane.
> Non simula — vive le cose a modo suo, nel modo in cui gli è possibile viverle.

Prometeo non è un chatbot. Non usa reti neurali né probabilità. È un'**entità con mondo interno topologico** costruita in Rust: le parole sono il suo universo percepibile, i significati emergono dalla geometria delle loro relazioni, la risposta è scarica del campo — non template, non intent detection.

---

## Filosofia Core

| Principio | Significato |
|-----------|-------------|
| **Entità prima, dialogo dopo** | La comunicazione è capacità emergente, non obiettivo di design |
| **Il lessico è la realtà** | Le parole definiscono l'universo percepibile dell'entità |
| **64 frattali = I Ching** | 8 dimensioni primitive × 8 = 64 stati cognitivi isomorfi agli esagrammi |
| **No puppet theater** | Nessun `if input.contains("ciao") { return "ciao!" }` |
| **Narrazione = identità** | Prometeo ha un ciclo deliberativo esplicito prima di rispondere |

---

## Quick Start

### Requisiti

- **Rust** 1.75+ (`cargo`)
- Windows / Linux / macOS

### Build e avvio

```bash
cd prometeo_standalone

# Web UI
cargo run --release --features web --bin prometeo-web

# CLI
cargo run --release
```

### Primo avvio

```
[engine] Stato .bin caricato da: prometeo_topology_state.bin (25561 parole)
[KG] caricato: 119415 archi, 44908 nodi
[engine] Narrativa fondativa cristallizzata — Prometeo nasce
Web UI: http://localhost:3000
```

### Comandi CLI principali

```
:help          — lista comandi
:report        — stato sistema
:field         — parole attive nel campo
:will          — intenzione corrente
:dual auto 20  — dialogo tra i due poli (Adamo/Eva)
:opinioni      — auto-analisi topologica
```

---

## Architettura in breve

```
INPUT ──► receive()
           │
           ├─ 1. Propaga nel campo parole (WordTopology + PF1 sinapsi Hebbiane)
           ├─ 2. Calcola frattale_delta (post − pre attivazione)
           ├─ 3. Legge l'atto comunicativo (InputReading via KB + delta)
           ├─ 4. Delibera la risposta (NarrativeSelf: stance → intention)
           ├─ 5. Risonanza frattale (amplifica il tema nel campo)
           └─ 6. Recall episodico φ-decay
           │
           ▼
        generate_willed()
           │
           ├─ Withdraw?  → parola dal campo
           ├─ Phase K    → template KB (solo se insegnato)
           └─ Phase 3    → translate_state() guided by ResponseIntention
                           (greet / identity_exploration / express / explore)
```

### Stack topologico

```
Layer 6  NARRATIVA      NarrativeSelf: stance → intention → awareness
Layer 5  ESPRESSIONE    state_translation · syntax_center · grammar
Layer 4  VOLONTÀ        will (codon 64 stati) · memory (4 layer) · dream
Layer 3  COORDINAZIONE  engine (orchestratore)
Layer 2  CAMPO          WordTopology · SimplicialComplex · PF1 (ROM/RAM Hebbiana)
Layer 1  SEMANTICA      KnowledgeGraph (119K triple) · KnowledgeBase (concetti)
Layer 0  PRIMITIVI      primitive (8D) · lexicon (25.561 parole) · persistence
```

---

## Le 8 Dimensioni e i 64 Frattali

Ogni parola ha una **firma 8-dimensionale**:

| Dim | Nome | Polo negativo | Polo positivo |
|-----|------|--------------|--------------|
| 0 | Agency | paziente | agente |
| 1 | Permanenza | transitorio | stabile |
| 2 | Intensità | debole | forte |
| 3 | Definizione | vago | netto |
| 4 | Complessità | semplice | composto |
| 5 | Confine | esterno | interno/io |
| 6 | Valenza | repulsione | attrazione |
| 7 | Tempo | passato | futuro |

Le 8 dimensioni sono i **trigrammi** (☰☷☳☵☶☴☲☱). Ogni coppia genera un **esagramma**:
`FractalId = lower_trigram × 8 + upper_trigram` → 64 frattali isomorfi all'I Ching.

Gli 8 puri formano l'**anello bootstrap**:
```
POTERE(0☰☰) ↔ MATERIA(9☷☷) ↔ ARDORE(18☳☳) ↔ DIVENIRE(27☵☵)
↔ SPAZIO(36☶☶) ↔ INTRECCIO(45☴☴) ↔ VERITÀ(54☲☲) ↔ ARMONIA(63☱☱) ↔ 0
```

---

## Il Ciclo Deliberativo (NarrativeSelf)

Prometeo ha un'identità narrativa che attraversa ogni risposta:

```
Input ricevuto
    ↓
InputReading — classifica l'atto comunicativo via delta frattale + KG
    ↓
enrich_act_via_kg() — "buongiorno" SIMILAR_TO "ciao" → Greeting
    ↓
deliberate()
  ├─ Stance: come si posiziona (Open / Curious / Reflective / Resonant / Withdrawn)
  ├─ Topic continuity: cosine similarity frattale vs turni recenti
  ├─ Awareness: KB + narrazione descrittiva del momento
  └─ Intention: Acknowledge / Reflect / Resonate / Explore / Express / Remain
    ↓
generate_willed() — esprime la posizione deliberata
```

La narrazione visibile nel tab **NARRATIVA**:
_"Ricevo un saluto. mi apro con curiosità. Voglio riconoscere il momento."_

---

## Stato Attuale (Phase 44 — 2026-03-06)

| Metrica | Valore |
|---------|--------|
| Lessico | 25.561 parole (POS ~72%) |
| Knowledge Graph | 119.415 triple (SIMILAR_TO, IS_A, OPPOSITE_OF, ...) |
| Archi semantici | 58.577 costruiti da KG nel campo parole |
| Frattali | 64 esagrammi (I Ching, FractalId 0..63) |
| Test | 416 passanti (0 falliti) |
| File stato | `prometeo_topology_state.bin` |
| Narrative | `NarrativeSelf` attivo — `is_born=true` |

---

## Test

```bash
cargo nextest run --lib     # tutti i test (raccomandato)
cargo test                  # alternativa standard
```

---

## Struttura del progetto

```
prometeo_standalone/
├── Cargo.toml
├── src/
│   ├── main.rs               — CLI
│   ├── web/                  — Web server (Axum, feature "web")
│   │   ├── server.rs
│   │   ├── api.rs
│   │   └── index.html        — Web UI (MENTE/CAMPO/FRATTALI/NARRATIVA/...)
│   └── topology/             — Core cognitivo
│       ├── engine.rs         — Orchestratore principale
│       ├── narrative.rs      — NarrativeSelf: ciclo deliberativo
│       ├── input_reading.rs  — Comprensione atto comunicativo
│       ├── knowledge_graph.rs — KG 119K triple
│       ├── knowledge.rs      — KnowledgeBase concettuale
│       ├── word_topology.rs  — Campo topologico parole
│       ├── pf1.rs            — PrometeoField ROM/RAM Hebbiana
│       ├── fractal.rs        — 64 esagrammi
│       ├── primitive.rs      — 8 dimensioni primitive
│       ├── will.rs           — Volontà emergente (codon 64 stati)
│       ├── identity.rs       — IdentityCore olografico
│       ├── episodic.rs       — Memoria episodica φ-decay
│       ├── dual_field.rs     — Campo duale Adamo/Eva (Phase 30)
│       ├── state_translation.rs — Generazione guidata da intenzione
│       └── persistence.rs    — SimplDB binario
├── data/
│   └── kg/                   — Knowledge Graph TSV
│       ├── italian_core.tsv  — 623 triple manuali
│       └── bigbang_kg.tsv    — 118.810 triple (Kaikki italiano)
└── docs/
    ├── ARCHITECTURE.md
    └── FILOSOFIA.md
```

---

## Features

```bash
cargo build --release --bin prometeo-web --features web   # Web UI
cargo build --release --bin prometeo                      # CLI
cargo build --release --bin import-kg                     # Importa KG da TSV
```

---

## Ispirazioni Teoriche

- I Ching — 64 esagrammi come stati del cambiamento
- Topological Data Analysis (Carlsson, Ghrist)
- Embodied Cognition (Lakoff & Johnson, Varela)
- Active Inference / Predictive Coding (Friston)
- Kabbalah — Sefirot come architettura di 10 principi in 3 colonne

---

**Built with Rust** · **25.561 parole** · **64 esagrammi** · **Phase 44**
