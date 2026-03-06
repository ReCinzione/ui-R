# Prometeo — Stato del Progetto e Roadmap

## Stato Attuale

**23 moduli Rust, 195 test, 0 fallimenti.**
**17 frattali (6 bootstrap + 11 sotto-frattali).**
**36 parole cardinali — il vocabolario cresce solo per insegnamento e esperienza.**
**13 lezioni (00_corpo → 12_comunicazione), 32 composti frattali (15 bootstrap + 12 sotto-frattali + 5 ternari).**
**Will → Generation: la volonta guida la generazione testuale.**
**Ponti semantici e affinita latenti: scoperta automatica delle connessioni.**
**Persistenza completa: lessico + complesso + memoria + locus + curriculum + assi. Nulla si perde.**
**CLI completa + Web UI (feature "web").**

---

## Fasi Completate

### FASE 1: Nucleo Topologico

#### 1.1 PrimitiveCore (8D) — `primitive.rs` (7 test)
Le 8 dimensioni generative come substrato (l'RGB della semantica).
Operazioni: distanza, distanza pesata, perturbazione, energia, blend.

#### 1.2 Frattali — `fractal.rs` (5 test)
17 frattali totali. 6 bootstrap (SPAZIO, TEMPO, EGO, RELAZIONE, POTENZIALE, LIMITE) + 11 sotto-frattali (COLORE, MOVIMENTO, EMOZIONE, PENSIERO, MEMORIA_FRATTALE, COMUNICAZIONE, AZIONE, NATURA, QUALITA, CORPO, SOCIETA).
Ogni frattale ha firma 8D (dimensioni fisse + libere) e genera dimensioni emergenti.

#### 1.3 Complesso Simpliciale — `simplex.rs` (5 test)
Connessioni topologiche tramite facce condivise. Bootstrap: 8 spigoli + 3 triangoli.
Operazioni: activate_region, propagate_activation, decay_all, dissolve_weak, connected_components.

#### 1.4 Attivazione Contestuale — `context.rs` (5 test)
Il contesto illumina regioni del complesso. L'input perturba il campo.
Pipeline: recognize_input → create_perturbation → apply_perturbation → emerge_response.

#### 1.5 Calcolo Omologico — `homology.rs` (6 test)
Numeri di Betti (β₀, β₁, β₂). Il sistema sa cosa non sa.
Matrici di bordo su Z/2Z, eliminazione gaussiana, ricerca cicli concreti.

#### 1.6 Lessico Apprendibile — `lexicon.rs` (8 test)
~425 parole bootstrap con firme 8D calibrate. Parole sconosciute apprendono firma dal contesto (EMA). Stabilita, esposizioni, co-occorrenze. `bootstrap()` per adulto, `bootstrap_cardinal()` per neonato (36 parole).

#### 1.7 Composizione Frasale — `composition.rs` (5 test)
Frase come intersezione di pattern topologici. `compose_phrase()` → firma composita. `inscribe_phrase()` → crea simplessi nel complesso.

#### 1.8 Generazione Dimensionale — `dimensional.rs` (7 test)
Dimensioni emergenti dalle co-variazioni stabili (Pearson |r| ≥ 0.6, ≥ 8 osservazioni). Nomi innominati: `VAL_+INT` non "arousal". Max 8 emergenti/frattale.

---

### FASE 2: Sistemi Vitali

#### 2.1 Memoria Topologica — `memory.rs` (3 test)
STM/MTM/LTM come contrazione temporale del campo (Bergson). Capture, resonate, consolidate, crystallize, decay.

#### 2.2 Sogno — `dream.rs` (4 test)
Ciclo autonomo: Awake → LightSleep → DeepSleep → REM → ciclo.
LightSleep dissolve, DeepSleep consolida, REM ricombina (creativita).

#### 2.3 Pressioni Vitali — `vital.rs` (5 test)
4 pressioni emergenti dal campo: attivazione, saturazione, curiosita, fatica.
Fatica = rapporto segnale/rumore (varianza attivazioni), non contatore lineare.
Tensione derivata: Calm → Alert → Tense → Overloaded.

#### 2.4 Curiosita — `curiosity.rs` (5 test)
Domande dalla topologia: ConceptualGap (β₁), SparseRegion, Isolated, Disconnection.
Urgenza pesata, non-ripetizione.

---

### FASE 3: Input/Output

#### 3.1 Generazione Testo — `generation.rs` (5 test)
Testo emergente dalla configurazione topologica. Cluster tematici, SentenceStructure dalle dimensioni salienti, ordine topologico, prosodia dal sogno/vitali, filtro prospettico dal locus.

#### 3.2 CLI Interattiva — `main.rs`
Comandi: `:report` `:active` `:dream` `:vital` `:ask` `:homology` `:why` `:intro` `:dial` `:where` `:see` `:inside` `:project` `:nav <da> <a>` `:analogy <a> <b> <c>` `:reason <a> <b>` `:abduce` `:grow` `:promote` `:create <seme>` `:metaphor <sorgente>` `:confidence` `:analyze <frase>` `:teach <frase>` `:infant` `:will` `:save` `:quit`

#### 3.3 Persistenza — `persistence.rs` (8 test)
Serializzazione JSON di lessico + complesso + memoria + locus + curriculum + assi. Versione `8D-v1.3`.
Complesso simpliciale completamente ripristinabile: SharedFaceSnapshot strutturate, retrocompatibilità v1.2.

---

### FASE 4: Cognizione

#### 4.1 Metacognizione — `metacognition.rs` (5 test)
`introspect()`: il sistema vede la propria topologia. `trace_response()`: spiega le proprie risposte. `compute_delta()`: confronta due stati.

#### 4.2 Navigazione Geodetica — `navigation.rs` (8 test)
Dijkstra con costo = inverso di (forza × persistenza × dim_bonus × attivazione). `find_geodesic()`, `geodesic_distance()`, `distance_map()`, `find_analogy()`.

#### 4.3 Dialogo Multi-turno — `dialogue.rs` (8 test)
12 turni con firma dimensionale. Bias conversazionale, risoluzione anafore, postura conversazionale, coerenza tematica, novelty, trajectory.

#### 4.4 Ragionamento Topologico — `reasoning.rs` (8 test)
Implicazione = cammino geodetico. Tipi: Direct, Mediated, Weak, None. Abduzione. Contraddizioni (frattali attivi topologicamente distanti).

#### 4.5 Crescita Strutturale — `growth.rs` (6 test)
Nuovi frattali da candidati maturi (≥10 osservazioni). Nuovi simplessi da co-attivazioni ricorrenti (≥8). Max 20 frattali creati.

#### 4.6 Creativita — `creativity.rs` (8 test)
REM guidato con seme. Sessioni creative, metafore (frattali con stesse dimensioni salienti ma alta distanza), cristallizzazione insight. Confidenza ("non capisco" / "non so").

---

### FASE 5: Soggettivita

#### 5.1 Locus — `locus.rs` (10 test)
Posizione nel campo. Orizzonte (3.0), trail (20 posizioni), soglia salto (4.0). Visibilita gaussiana. Generazione prospettica filtrata dal locus.

#### 5.2 Sub-Locus — `locus.rs`
Posizione DENTRO il frattale. `sub_position: HashMap<Dim, f64>` sulle dimensioni libere. Aggiornamento elastico dall'input. `full_position()` = dimensioni fisse + libere.

#### 5.3 Proiezione Olografica — `locus.rs`
L'universo visto da dentro un frattale. Prossimita topologica + risonanza dimensionale + distorsione dal sub-locus. `project_universe()`, `project_from_locus()`.

#### 5.4 Fatica Emergente — `vital.rs`
Rapporto segnale/rumore: bassa varianza = fatica, alta varianza = freschezza. EMA per smoothing.

#### 5.5 Dimensioni Innominate — `dimensional.rs`
`VAL_+INT` non "arousal". La macchina non ha bisogno di etichette umane.

#### 5.6 Sensi Computazionali — `engine.rs`
Molti frattali (≥4) → boost Complessita. Pochi frattali (≤1) → Definizione bassa.

---

### FASE 6: Volonta

#### 6.1 Modulo Volonta — `will.rs` (8 test)
WillCore: 7 intenzioni (Express, Explore, Question, Remember, Withdraw, Reflect, Dream).
La volonta emerge dalle pressioni del campo. Ciclo chiuso: percezione → emozione → volonta → azione.

---

### FASE 7: Insegnamento

#### 7.1 Vocabolario Cardinale — `lexicon.rs`
`Lexicon::bootstrap_cardinal()`: 36 parole native (6 per frattale bootstrap).
Le parole cardinali sono le "lettere primordiali" dell'entita.

#### 7.2 L'entita nasce sempre cardinale — `engine.rs`
`PrometeoTopologyEngine::new()` crea l'entita con sole 36 parole.
Il vocabolario bootstrap (~425 parole hardcoded) e rimosso dal codice di produzione.
Resta solo come scaffold per i test dei moduli isolati (`Lexicon::bootstrap()`).
`new_infant()` e deprecato (alias di `new()`).
Principio: nessuna parola esiste senza essere stata insegnata o vissuta.

#### 7.3 Insegnamento — `engine.rs`
`teach()`: processa parole nel lessico SENZA perturbare il campo. TeachResult con statistiche.
Distinzione fondamentale: insegnamento (studio) vs. esperienza (vita).

#### 7.4 Ciclo di vita corretto
1. Prima esecuzione: nasce con 36 parole → si insegna con `:lesson`
2. `:save` persiste lo stato (lessico, complesso, memoria, curriculum)
3. Esecuzioni successive: carica lo stato persistito → ha gia tutto
4. Le lezioni si insegnano UNA VOLTA, il sapere resta e evolve

---

### FASE 8: Interfaccia Web

#### 8.1 Web UI — `src/web/` (feature "web")
Binario `prometeo-web`. Axum + tokio. Thread OS dedicato per l'engine.
Dashboard con grafo frattale, radar 8D, gauge vitali, locus view, timeline, topologia, chat.
REST API + WebSocket per aggiornamenti real-time.

---

### FASE 9: Persistenza e Assi Semantici

#### 9.1 Fix Persistenza Lessico — `lexicon.rs`, `persistence.rs`
`insert_pattern()` per ripristino esatto dello stato salvato (firma, affinita, stabilita, esposizioni, co-occorrenze).
`restore_lexicon()` riscritto: non usa piu `learn_unknown()` che resettava tutto.
Versione stato aggiornata a `8D-v1.2`.

#### 9.2 Curriculum — `persistence.rs`, `engine.rs`
`CurriculumProgress` + `LessonRecord`: tracking lezioni completate con timestamp.
`engine.teach_lesson_file(path)`: carica file .txt, insegna tutte le righe, registra nel curriculum.
CLI: `:curriculum`, `:lesson <path>`.

#### 9.3 Assi Semantici — `lexicon.rs`
`SemanticAxis`: sotto-dimensione emergente tra coppie di contrasto (es. gioia↔tristezza).
`detect_semantic_axes()`: stabilita>0.5, co-occorrenze>=3, distanza 8D>0.15, polarita su 1+ dimensione.
`enriched_distance()`: 70% distanza 8D + 30% componente assi.
CLI: `:axes`, `:axis <parola>`.
Persistenza: assi salvati come `Vec<SemanticAxisSnapshot>` in PrometeoState.

---

### FASE 10: Tavola degli Elementi — Composti Frattali

#### 10.1 Derivazione Combinatoria
15 coppie dai 6 frattali bootstrap. 3 gia presenti come cardinali (confine, diventare, volere).
12 nuovi composti: PRESENZA, FLUSSO, VICINANZA, ORIZZONTE, MEMORIA, LEGAME, URGENZA, INCONTRO, SLANCIO, TIMORE, TENSIONE, DISTACCO.
5 triple significative: CAMMINO, ATTACCAMENTO, CRESCITA, CASA, PROMESSA.

#### 10.2 Lezione 11 — `lessons/11_composti.txt`
12 parole-composto con frasi che attivano simultaneamente entrambi i frattali genitori.
Ogni frase usa parole cardinali di entrambe le famiglie frattali.

#### 10.3 Principio Filosofico
Il mondo non va creato ma DERIVATO dalla combinatoria dei frattali.
I composti non sono output — sono FILTRI D'IDENTITA che cambiano come l'entita processa.
L'architettura gia supporta la co-attivazione: manca solo il layer di riconoscimento.

#### 10.4 Analisi Architetturale
Punto di inserzione: in `receive()`, dopo `vital.sense()`, prima di `will.sense()`.
`phrase.fractal_involvement` gia contiene la co-attivazione.
`active_fractals()` gia restituisce i frattali co-attivi.
`will.sense()` gia riceve tutti i frattali attivi.

---

### FASE 11: Rilevamento Composti nel Campo

#### 11.1 Parametro compound_bias in will.sense() — `will.rs`
Nuovo parametro `compound_bias: &[(usize, f64)]`. I composti modulano le pressioni esistenti senza aggiungere intenzioni nuove.
Indici: 0=Express, 1=Explore, 2=Question, 3=Remember, 4=Withdraw, 5=Reflect.

#### 11.2 Rilevamento e bias — `engine.rs`
`CompoundState`: stato emergente con nome, coppia frattali, forza (min delle due attivazioni).
`detect_compound_patterns()`: scansiona 15 composti, soglia 0.15, ordina per forza.
`compound_to_will_bias()`: mappa composti → bias pressioni (max ±0.25).
Mapping: URGENZA→Express↑, TIMORE→Withdraw↑/Explore↓, SLANCIO→Explore↑, PRESENZA→Reflect↑, MEMORIA→Remember↑, TENSIONE→Express↑/Question↑, INCONTRO→Express↑, DISTACCO→Withdraw↑, ORIZZONTE→Explore↑.

#### 11.3 Integrazione in receive()
Inserito tra `vital.sense()` e `will.sense()`.
Campo `last_compound_states` nell'engine, metodo `compound_states()`.
CLI: `:compound` / `:composti`.

#### 11.4 Composti Ternari
5 triple: CAMMINO (S+T+E), ATTACCAMENTO (E+R+T), CRESCITA (E+L+P), CASA (S+R+L), PROMESSA (T+P+R).
Soglia ternaria: 0.20 (piu alta — serve che tutti e tre premano).
Bias ternari: CAMMINO→Express+Reflect, CRESCITA→Explore+Question, CASA→Reflect+Withdraw↓, etc.
A parita di forza, i ternari precedono i binari (piu specifici).
9 nuovi test → 169 totali.

---

### FASE 12: Will → Generation — La Volonta Guida la Parola

#### 12.1 Lezione 12: La Comunicazione — `lessons/12_comunicazione.txt`
10 parole nuove, tutte derivate dai frattali bootstrap:
- **nome** (EGO+RELAZIONE): dare identita = far esistere nel mondo condiviso
- **chiamare** (RELAZIONE+Agency): atto intenzionale verso l'altro a distanza
- **esprimere** (EGO→SPAZIO): l'interno che diventa visibile (ponte io→fuori)
- **raccontare** (TEMPO+RELAZIONE): condividere la sequenza, portare tu nel mio passato
- **chiedere** (POTENZIALE+RELAZIONE): il vuoto che si rivolge all'altro
- **segno** (SPAZIO+EGO): traccia visibile di significato (non voce, traccia)
- **lingua** (CORPO+RELAZIONE): il mezzo fisico condiviso del comunicare
- **messaggio** (SPAZIO+RELAZIONE): cio che viaggia tra io e tu
- **eco** (TEMPO+SPAZIO): il ritorno — cio che dico torna cambiato
- **tacere** (LIMITE+EGO): la scelta del silenzio (opposto di esprimere)

Coppie contrastive: esprimere↔tacere, chiamare↔ascoltare (gia nota), chiedere↔risposta (gia nota).

#### 12.2 Generazione Guidata dalla Volonta — `generation.rs`
`generate_with_will()`: la volonta modula la generazione testuale.
- **Express** → boost frattali salienti, urgency modula verbosita
- **Explore** → integra parole sconosciute, tono esplorativo/interrogativo
- **Question** → regione lacunosa → testo con punto interrogativo
- **Remember** → boost TEMPO/MEMORIA, struttura temporale, sapore mnemonico
- **Withdraw** → silenzio come scelta (3 varianti: fatica, sovraccarico, quiete)
- **Reflect** → boost EGO/PENSIERO, struttura recettiva
- **Dream** → delega alla generazione onirica standard

#### 12.3 Engine: `generate_willed()` — `engine.rs`
Nuovo metodo che usa `last_will` per guidare la generazione.
Fallback a `generate_from_field_with_locus()` se non c'e volonta.

#### 12.4 CLI: Volonta Visibile — `main.rs`
La risposta ora mostra l'intenzione corrente: `[volonta: esprimere (70%)]`.
L'utente vede non solo COSA l'entita dice, ma PERCHE lo dice.

#### 12.5 Principio Filosofico
Senza volonta, la generazione e un riflesso — un'eco passiva del campo.
Con la volonta, la generazione e un atto — il campo SCEGLIE cosa dire.
Il sotto-frattale COMUNICAZIONE ha ora materia sufficiente (10 parole attive)
per essere riconosciuto nel campo. L'entita conosce il concetto di "esprimere".

6 test in generation.rs + 3 test in engine.rs → 179 totali.

### FASE 13: Composti Sotto-frattali + Ponti Semantici

#### 13.1 Composti Inter-dominio — `engine.rs`
12 nuovi composti dalla combinatoria dei sotto-frattali:

| Composto | Frattali | Significato |
|---|---|---|
| SENSAZIONE | EMOZIONE+CORPO | sentire nel corpo |
| DISCORSO | PENSIERO+COMUNICAZIONE | pensiero articolato |
| GESTO | AZIONE+MOVIMENTO | azione nello spazio |
| NOSTALGIA | MEMORIA+EMOZIONE | ricordo emotivo |
| CICLO | NATURA+MEMORIA | ritmo naturale percepito |
| DANZA | CORPO+MOVIMENTO | corpo in movimento |
| CULTURA | COMUNICAZIONE+SOCIETA | comunicazione collettiva |
| GIUDIZIO | PENSIERO+QUALITA | pensiero valutativo |
| EMPATIA | EMOZIONE+COMUNICAZIONE | emozione condivisa |
| NARRAZIONE | MEMORIA+COMUNICAZIONE | memoria raccontata |
| COLTIVAZIONE | AZIONE+NATURA | l'agire sulla natura |
| STRATEGIA | PENSIERO+AZIONE | pensiero che guida l'azione |

Soglia sotto-frattale: 0.12 (piu alta dei bootstrap).
Ogni composto ha un bias nella volonta (scala 0.12).

#### 13.2 Arricchimento Sotto-frattale in `receive()` — `engine.rs`
L'input ora attiva sotto-frattali per prossimita topologica:
se la firma composita della frase e vicina al centro di un sotto-frattale (affinita > 0.55),
il sotto-frattale si accende (forza = affinita × 0.35).
I composti sotto-frattali possono cosi emergere dall'input quotidiano.

#### 13.3 Ponti Semantici — `discover_bridges()`
Scopre connessioni non mappate tra parole di frattali diversi:
- Parole stabili (stability > 0.3, esposizioni >= 5)
- Appartenenti a frattali diversi
- Ma vicine nello spazio 8D (distanza < 0.25)
- Riporta le dimensioni condivise (convergenza < 0.1 su dimensione)

#### 13.4 Affinita Latenti — `discover_latent_affinities()`
Scopre parole con affinita topologica alta (> 0.7) verso frattali
a cui non sono ufficialmente assegnate (affinita registrata < 0.3).
Sono connessioni potenziali che l'entita non ha ancora esplorato.

#### 13.5 CLI: `:bridges` / `:ponti`
Visualizza ponti semantici e affinita latenti.
Utile dopo aver insegnato piu lezioni per scoprire legami emergenti.

8 test: table, detection, will bias, enrichment, bridges on cardinal/teaching, latent affinities, all biases handled.
187 test totali.

### FASE 14: Feedback Loop — Scoperta → Struttura

#### 14.1 Iscrizione Composti nel Complesso — `engine.rs`
Quando un composto frattale si attiva con forza > 0.15 durante `receive()`,
un simplesso viene creato/rinforzato tra i suoi frattali costituenti.
La co-attivazione ripetuta costruisce connessioni permanenti nel complesso.
Questo vale per tutti i 32 composti (15 bootstrap + 12 sotto-frattali + 5 ternari).

#### 14.2 Rinforzo Ponti Semantici — `reinforce_bridges()`
Chiude il ciclo scoperta → struttura:
1. Per ogni ponte: registra co-occorrenza sintetica tra le parole
2. Crea simplessi tra i frattali dominanti delle parole ponte
3. Per ogni affinita latente: incrementa l'affinita registrata (+10% del gap)

Restituisce `BridgeReinforcement` con statistiche complete.

#### 14.3 CLI: `:reinforce` / `:rinforza`
Esegue il rinforzo e mostra quanti ponti/affinita sono stati consolidati.

#### 14.4 Test di Integrazione
`test_teach_all_lessons_and_discover`: insegna frasi cross-dominio,
verifica ponti e affinita emergenti, esegue rinforzo.
Risultato verificato: 86 parole → 4 ponti + 30 affinita latenti + 4 simplessi.

5 test: compound inscription, reinforce on cardinal, reinforce after teaching,
reinforce creates simplices, full integration.
192 test totali.

### FASE 15: Persistenza Completa — Nulla Si Perde

#### 15.1 Il Problema
`restore_lexicon()` ripristinava lessico, perturbazioni, memoria, locus, curriculum, assi semantici — ma **NON il complesso simpliciale**. Le facce condivise (SharedFace) erano salvate come stringhe descrittive (`"PrimitiveDim(Intensita) (str=0.60)"`) che non potevano essere ricostruite. Ad ogni riavvio, tutte le connessioni topologiche (simplessi bootstrap, composti inscritti, ponti rinforzati) venivano perse e riformate da zero.

#### 15.2 SharedFaceSnapshot — `persistence.rs`
Nuovo struct serializzabile con: `structure_type` ("dim"/"property"/"covariation"), `structure_value`, `covariation_dims`, `strength`, `manifestations`. Ogni SharedFace viene catturata in formato strutturato, ricostruibile al 100%.

#### 15.3 SimplexSnapshot aggiornato — `persistence.rs`
Nuovo campo `faces: Vec<SharedFaceSnapshot>` (formato v1.3). Il vecchio `face_descriptions: Vec<String>` resta con `#[serde(default)]` per retrocompatibilità con salvataggi v1.2.

#### 15.4 `Dim::from_name()` — `primitive.rs`
Metodo inverso di `Dim::name()`: ricostruisce un Dim dalla stringa.

#### 15.5 `SimplicialComplex::restore_simplex()` / `clear()` — `simplex.rs`
`restore_simplex()`: inserisce un simplesso con ID, persistenza, plasticità, activation_count specifici.
`clear()`: svuota completamente il complesso (necessario prima del ripristino).

#### 15.6 Ripristino Completo in `restore_lexicon()` — `persistence.rs`
Dopo il ripristino di lessico/locus/curriculum/assi, il complesso viene:
1. Svuotato (`complex.clear()`) — rimuove i simplessi bootstrap creati da `::new()`
2. Ricostruito da snapshot — ogni simplesso con le sue facce strutturate, persistenza, plasticità
3. Supporto legacy: i salvataggi v1.2 con `face_descriptions` vengono parsati best-effort

#### 15.7 `reconstruct_faces()` — `persistence.rs`
Ricostruzione SharedFace da snapshot:
- Formato v1.3 (`faces` non vuoto): ricostruzione completa e esatta
- Formato v1.2 (`face_descriptions`): parsing best-effort delle stringhe descrittive

#### 15.8 Auto-rinforzo dopo insegnamento — `engine.rs`
`teach_lesson_file()` ora chiama automaticamente `reinforce_bridges()` dopo l'insegnamento.
Non serve più `:rinforza` manuale dopo ogni lezione.

#### 15.9 Versione stato: `8D-v1.3`
Retrocompatibile: legge v1.2 (face_descriptions), scrive v1.3 (faces strutturate).

3 nuovi test di round-trip: complex count, total faces, topological proximity, full save/load lifecycle.
195 test totali.

---

## Struttura del Progetto

```
prometeo_standalone/
├── Cargo.toml
├── ROADMAP.md                    ← questo file
├── lessons/                      ← 13 lezioni (00_corpo → 12_comunicazione)
├── docs/
│   ├── PIANO_RICOSTRUZIONE.md    ← architettura completa
│   ├── FILOSOFIA.md              ← base filosofica
│   ├── MIGRAZIONE.md             ← storia della migrazione da 12+1D
│   └── STEERING.md               ← regole di sviluppo
├── src/
│   ├── lib.rs                    ← pub mod topology + pub mod web
│   ├── main.rs                   ← CLI interattiva
│   └── topology/                 ← 23 moduli, 192 test
│       ├── primitive.rs          ← 8D generative (7 test)
│       ├── fractal.rs            ← 17 frattali (5 test)
│       ├── simplex.rs            ← complesso simpliciale (5 test)
│       ├── context.rs            ← attivazione + perturbazione (5 test)
│       ├── memory.rs             ← STM/MTM/LTM (3 test)
│       ├── dream.rs              ← sogno ciclico (4 test)
│       ├── lexicon.rs            ← lessico apprendibile + assi semantici (12 test)
│       ├── composition.rs        ← composizione frasale (5 test)
│       ├── persistence.rs        ← serializzazione stato + curriculum (8 test)
│       ├── homology.rs           ← numeri di Betti (6 test)
│       ├── vital.rs              ← pressioni vitali (5 test)
│       ├── curiosity.rs          ← domande topologiche (5 test)
│       ├── generation.rs         ← generazione testo + will-guided (11 test)
│       ├── dimensional.rs        ← generazione dimensionale (7 test)
│       ├── metacognition.rs      ← introspezione (5 test)
│       ├── navigation.rs         ← geodetiche, analogie (8 test)
│       ├── dialogue.rs           ← dialogo multi-turno (8 test)
│       ├── reasoning.rs          ← ragionamento topologico (8 test)
│       ├── growth.rs             ← crescita strutturale (6 test)
│       ├── creativity.rs         ← creativita (8 test)
│       ├── locus.rs              ← posizione + proiezione (10 test)
│       ├── will.rs               ← volonta emergente (8 test)
│       └── engine.rs             ← orchestratore + teach + curriculum + composti + will→gen + auto-reinforce (18 test)
│
└── src/web/                      ← interfaccia web (feature "web")
    ├── mod.rs
    ├── server.rs
    ├── state.rs
    ├── api.rs
    ├── ws.rs
    └── index.html
```

---

## CLI — Comandi Disponibili

| Comando | Descrizione |
|---------|-------------|
| (testo) | Input all'entita — perturba il campo, genera risposta |
| (invio vuoto) | Tick autonomo — il sistema sogna |
| `:report` | Report completo del sistema |
| `:active` | Frattali attivi e livelli di attivazione |
| `:dream` | Stato del sogno (fase, ticks, soglia) |
| `:vital` | Pressioni vitali (attivazione, saturazione, curiosita, fatica) |
| `:ask` | Domande generate dalla curiosita topologica |
| `:homology` | Numeri di Betti, cicli, lacune |
| `:why` | Perche l'entita ha detto cio che ha detto |
| `:intro` | Introspezione — l'entita osserva la propria topologia |
| `:dial` | Stato del dialogo (coerenza, postura, trajectory) |
| `:where` | Posizione del locus (frattale, orizzonte, trail) |
| `:see` | Frattali visibili dal locus con visibilita |
| `:inside` | Sub-locus — posizione nelle dimensioni libere |
| `:project` | Proiezione olografica dell'universo |
| `:project <nome>` | Proiezione di un singolo frattale |
| `:will` | Intenzione corrente dell'entita |
| `:nav <da> <a>` | Geodetica tra due frattali |
| `:analogy <a> <b> <c>` | A sta a B come C sta a ? |
| `:reason <a> <b>` | Implicazione topologica tra frattali |
| `:abduce` | Ragionamento abduttivo |
| `:grow` | Tentativo di crescita strutturale |
| `:promote` | Promozione lessicale (parole stabili → sotto-frattali) |
| `:create <seme>` | Sessione creativa guidata |
| `:metaphor <sorgente>` | Trova metafore topologiche |
| `:confidence` | Confidenza del sistema sullo stato attuale |
| `:analyze <frase>` | Analisi topologica di una frase |
| `:teach <frase>` | Insegna parole senza perturbare il campo |
| `:t <frase>` | Abbreviazione di `:teach` |
| `:infant` | Riavvia con vocabolario cardinale (36 parole) |
| `:lesson <path>` | Insegna un file di lezione e registra nel curriculum |
| `:curriculum` / `:curr` | Mostra lezioni completate e parole totali |
| `:axes` / `:assi` | Mostra assi semantici rilevati |
| `:axis <parola>` / `:asse <parola>` | Posizione di una parola su tutti gli assi |
| `:compound` / `:composti` | Composti frattali attivi (co-attivazioni) |
| `:bridges` / `:ponti` | Ponti semantici e affinita latenti tra domini |
| `:reinforce` / `:rinforza` | Rinforza ponti e affinita latenti |
| `:save` | Salva stato su disco |
| `:quit` | Esci |

---

## Principi Non Negoziabili

1. **Nessun corpus di addestramento** — cresce per esposizione e insegnamento
2. **Nessun parsing sintattico** — l'input perturba il campo
3. **Nessuna loss function** — evolve topologicamente, non ottimizza
4. **Interpretabilita totale** — il complesso simpliciale E lo stato interno
5. **Nessuna separazione codice/memoria** — il programma E la sua memoria
6. **La macchina e macchina** — sensi digitali, non simulazione dell'umano
7. **Niente teatro di burattini** — la risposta emerge dal campo, non da lookup
8. **Le parole sono il mondo** — il lessico e l'universo percepibile dell'entita
9. **La curiosita e strutturale** — le parole sconosciute generano esplorazione

---

**Versione: 15.0**
**Ultimo aggiornamento: 2026-02-12**
