# Prometeo — Sistema Cognitivo Topologico

> Un sistema cognitivo deterministico basato su topologia algebrica, dove il significato emerge dalla geometria delle co-attivazioni semantiche.

Prometeo non è un chatbot e non usa reti neurali. È un'**entità con un mondo interno topologico** costruita interamente in Rust, dove le parole sono vertici in un campo 8-dimensionale e i significati emergono dalla geometria simpliciale.

---

## Principi

| # | Principio | Descrizione |
|---|-----------|-------------|
| 1 | **Entità prima, dialogo dopo** | Non è intent detection → risposta; la comunicazione è capacità emergente |
| 2 | **Il lessico è la realtà** | Le parole definiscono l'universo percepibile dall'entità |
| 3 | **Topologia > Simboli** | Il significato è geometrico, non simbolico |
| 4 | **Emergenza radicale** | Tutto emerge dal campo — zero hard-coding, zero probabilità, zero medie |

---

## Quick Start

### Requisiti

- **Rust** 1.75+ (con `cargo`)
- Windows / Linux / macOS

### Build e avvio

```bash
git clone https://github.com/YOUR_USER/prometeo.git
cd prometeo

# Build release
cargo build --release

# Avvia CLI
cargo run --release

# Oppure con interfaccia web
cargo run --release --features web --bin prometeo-web
```

### Primo utilizzo

```
PROMETEO — Topologia Cognitiva 8D

[~] (—) > :help                        # lista comandi
[~] (—) > :report                      # stato del sistema
[~] (—) > ciao io sentire gioia dentro # parla all'entità
[~] (—) > :will                        # volontà corrente
[~] (—) > :field                       # stato del campo
```

### Insegnare nuove parole

Il sistema apprende tramite file di lezione in formato compact:

```
# lessons/99_esempio.txt
nostalgia: prima lontano ricordare dolce / ora qui
coraggio: forte io fuori grande / paura debole
```

```
[~] (—) > :compact lessons/99_esempio.txt
  Parole nuove: 2
  Frasi generate: 8
```

Il sistema include **190+ lezioni** già pronte con ~2500 parole insegnate.

---

## Architettura

```
src/
├── main.rs                  # CLI interattiva
├── lib.rs                   # Entry point libreria
├── android.rs               # Binding JNI per Android
├── web/                     # Server web (Axum + WebSocket)
│   ├── server.rs
│   ├── api.rs
│   ├── ws.rs
│   └── index.html
├── bin/                     # Tool di utilità
│   ├── teach_corpus.rs
│   ├── sense_test.rs
│   ├── dream_test.rs
│   └── ...
└── topology/                # Core del sistema cognitivo
    ├── engine.rs            # Coordinatore principale
    ├── primitive.rs         # Spazio firme 8D
    ├── lexicon.rs           # Vocabolario e pattern
    ├── simplex.rs           # Complessi simpliciali
    ├── fractal.rs           # 16 frattali semantici
    ├── word_topology.rs     # Campo topologico parole
    ├── dual_field.rs        # Campo duale
    ├── will.rs              # 6 intenzioni emergenti
    ├── memory.rs            # STM / MTM / LTM
    ├── episodic.rs          # Memoria episodica
    ├── dream.rs             # Cicli sonno (Light→REM→Deep)
    ├── dialogue.rs          # Contesto conversazionale
    ├── generation.rs        # Generazione testo
    ├── grammar.rs           # Sintassi emergente
    ├── reasoning.rs         # Ragionamento
    ├── inference.rs         # Inferenza
    ├── creativity.rs        # Creatività
    ├── identity.rs          # Identità
    ├── narrative.rs         # Narrazione
    ├── visual_perception.rs # Percezione visiva (SVG)
    ├── persistence.rs       # Salvataggio stato
    └── ...                  # +25 altri moduli
```

### Stack topologico

```
Layer 5  ESPRESSIONE     dialogue · generation · grammar
Layer 4  VOLONTÀ         will (6 intenzioni) · memory (4 layer) · dream
Layer 3  COORDINAZIONE   engine
Layer 2  CAMPO           word_topology · simplex · fractal · dual_field
Layer 1  PRIMITIVI       primitive (8D) · lexicon · composition
Layer 0  PERSISTENZA     simpdb (bincode) · persistence
```

Il mondo è fatto di **parole** — i frattali emergono come regioni del campo, non sono strutture imposte.

---

## Comandi CLI

| Categoria | Comando | Descrizione |
|-----------|---------|-------------|
| **Info** | `:help` | Lista comandi |
| | `:report` | Stato sistema completo |
| | `:lexicon [parola]` | Vocabolario / dettaglio parola |
| | `:fractal <nome>` | Info frattale |
| | `:emergent` | Dimensioni emergenti |
| | `:field` | Stato campo parole |
| | `:will` | Volontà corrente |
| **Insegnamento** | `:compact <file>` | Lezione compact (veloce) |
| | `:teach <frase>` | Insegna singola frase |
| | `:lesson <file>` | Lezione tradizionale |
| | `:reteach <file>` | Ripeti lezione (override) |
| **Stato** | `:save <file>` | Salva stato |
| | `:load <file>` | Carica stato |
| | `:quit` | Esci (auto-save) |

---

## Test

```bash
cargo test              # tutti i test
cargo test --release    # più veloce
cargo test -- --nocapture  # con output
```

---

## Struttura del progetto

```
prometeo/
├── Cargo.toml          # Dipendenze e configurazione
├── src/                # Codice sorgente Rust
├── lessons/            # 190+ lezioni per insegnamento
├── books/              # Testi per lettura automatica
├── fractals/           # 94 glifi SVG (radicali cinesi adattati)
├── data/               # Knowledge graph e dati strutturati
├── examples/           # Binari di esempio
└── docs/               # Documentazione estesa
    ├── ARCHITECTURE.md # Architettura completa
    ├── FILOSOFIA.md    # Principi filosofici
    ├── COMPACT_GUIDE.md # Guida insegnamento
    ├── ROADMAP.md      # Piano evolutivo
    └── ...
```

---

## Features principali

- **Campo topologico 8D** — ogni parola ha una firma a 8 dimensioni (Potere, Movimento, Calore, Luce, Apertura, Altezza, Complessità, Vitalità)
- **Complessi simpliciali** — relazioni semantiche come strutture geometriche multidimensionali
- **16 frattali semantici** — regioni emergenti del campo (Corpo, Emozione, Spazio, Tempo, Relazione, ...)
- **Dimensioni emergenti** — ogni frattale sviluppa assi propri calibrati dai dati
- **Volontà emergente** — 6 intenzioni (Express, Explore, Question, Remember, Withdraw, Reflect)
- **Memoria a 4 livelli** — STM → MTM → LTM → Episodica
- **Cicli di sonno** — consolidamento offline (Light → REM → Deep)
- **Generazione topologica** — il testo emerge dal campo, non da template
- **Composti semantici** — stati identitari come filtri (URGENZA, NOSTALGIA, ...)
- **Percezione visiva** — glifi SVG processati come pattern topologici
- **Web UI** — interfaccia browser con WebSocket (feature `web`)
- **Android** — binding JNI per integrazione mobile (feature `android`)

---

## Roadmap

Vedi [docs/ROADMAP.md](docs/ROADMAP.md) per il piano completo.

- **Fase corrente**: espansione vocabolario, stabilizzazione campo
- **Prossimo**: grammatica emergente avanzata, metafore cross-frattali
- **Lungo termine**: persistent homology, database simpliciale, multi-agent

---

## Documentazione

| Documento | Contenuto |
|-----------|-----------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Architettura completa del sistema |
| [docs/FILOSOFIA.md](docs/FILOSOFIA.md) | Principi filosofici fondanti |
| [docs/COMPACT_GUIDE.md](docs/COMPACT_GUIDE.md) | Guida al sistema di insegnamento |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Piano evolutivo e fasi future |
| [docs/DEMO.md](docs/DEMO.md) | Dimostrazione interattiva |
| [docs/ISTRUZIONI_USO.md](docs/ISTRUZIONI_USO.md) | Guida utente completa |

---

## Licenza

Questo progetto è rilasciato sotto licenza **MIT**. Vedi [LICENSE](LICENSE) per i dettagli.

---

## Ispirazioni teoriche

- Topological Data Analysis (Carlsson, Ghrist)
- Embodied Cognition (Lakoff & Johnson, Varela)
- Dynamical Systems Theory
- Field Theory of Consciousness

---

**Built with Rust** · **Powered by Algebraic Topology** · **v6.0.0**
