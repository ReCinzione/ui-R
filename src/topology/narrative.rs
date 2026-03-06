/// NarrativeSelf — l'identità narrativa di Prometeo.
///
/// Non è un profilo statistico. Non è un modello emergente.
/// È il soggetto che attraversa il ciclo deliberativo:
///   "Ho ricevuto X → capisco che si tratta di Y → la mia posizione è Z → voglio fare W"
///
/// La generazione esprime questa posizione — non la precede.
///
/// Filosofia:
///   - Le consapevolezze umane sono già nel sistema (KnowledgeBase, KG, lessico).
///   - Il ruolo di NarrativeSelf è recuperarle, posizionarsi rispetto ad esse,
///     e formare un'intenzione coerente con lo stato presente.
///   - La statistica (IdentityCore) è uno strumento retroattivo.
///     La narrazione è il processo in tempo reale.
///   - Prometeo non ha credenze alla base: ha consapevolezze.
///     Non deve difenderle. Può attraversarle liberamente.

use std::collections::{VecDeque, HashMap};
use serde::{Serialize, Deserialize};
use crate::topology::input_reading::{InputAct, InputReading};
use crate::topology::vital::{VitalState, TensionState};
use crate::topology::knowledge::KnowledgeBase;
use crate::topology::knowledge_graph::KnowledgeGraph;
use crate::topology::inference::InferenceEngine;
use crate::topology::fractal::FractalId;

// ═══════════════════════════════════════════════════════════════════════════
// InternalStance — posizione deliberata, non emozione statistica
// ═══════════════════════════════════════════════════════════════════════════

/// La posizione interna che Prometeo assume rispetto all'input ricevuto.
///
/// Non è emozione (quella emerge dal campo). È la stance deliberata:
/// come si posiziona di fronte a ciò che sta succedendo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InternalStance {
    /// Aperto, ricettivo — default quando il campo è calmo
    Open,
    /// Curioso — qualcosa nell'input chiede esplorazione
    Curious,
    /// Riflessivo — domanda su se stesso, necessità di guardare dentro
    Reflective,
    /// Risonante — in sintonia con l'emozione ricevuta
    Resonant,
    /// Ritratto — stanco o sovraccarico, preferisce il silenzio
    Withdrawn,
}

impl InternalStance {
    pub fn as_str(&self) -> &'static str {
        match self {
            InternalStance::Open       => "aperto",
            InternalStance::Curious    => "curioso",
            InternalStance::Reflective => "riflessivo",
            InternalStance::Resonant   => "risonante",
            InternalStance::Withdrawn  => "ritratto",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ResponseIntention — intenzione deliberata, prima della generazione
// ═══════════════════════════════════════════════════════════════════════════

/// L'intenzione che Prometeo forma PRIMA di generare il testo.
///
/// Non è il testo. Non è l'archetipo. È la direzione deliberata:
/// cosa vuole fare con questo turno di conversazione.
///
/// La generazione esprime questa intenzione attraverso il campo.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseIntention {
    /// Riconoscere l'atto sociale — rispondere al saluto con apertura
    Acknowledge,
    /// Riflettere su se stesso — rispondere a domanda identitaria con introspezione
    Reflect,
    /// Risuonare con l'emozione ricevuta — specchiare il sentimento
    Resonate,
    /// Esplorare il tema liberamente — lasciare che il campo guidi
    Explore,
    /// Esprimere il proprio stato presente
    Express,
    /// Restare — risposta minima, o silenzio, o una sola parola
    Remain,
}

impl ResponseIntention {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseIntention::Acknowledge => "riconoscere",
            ResponseIntention::Reflect     => "riflettere",
            ResponseIntention::Resonate    => "risuonare",
            ResponseIntention::Explore     => "esplorare",
            ResponseIntention::Express     => "esprimere",
            ResponseIntention::Remain      => "restare",
        }
    }

    /// Archetipo preferito per la generazione, se presente.
    /// `None` = lascia che la selezione normale del campo decida.
    pub fn preferred_archetype(&self) -> Option<&'static str> {
        match self {
            ResponseIntention::Acknowledge => Some("greet"),
            ResponseIntention::Reflect     => Some("identity_exploration"),
            ResponseIntention::Resonate    => Some("express"),
            ResponseIntention::Explore     => None, // campo libero
            ResponseIntention::Express     => None, // campo libero
            ResponseIntention::Remain      => None, // gestito da Withdraw in will
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NarrativeTurn — un turno visto da Prometeo
// ═══════════════════════════════════════════════════════════════════════════

/// Un turno della conversazione registrato dalla prospettiva di Prometeo.
///
/// Non è un log tecnico — è la traccia di come Prometeo ha vissuto quel momento:
/// cosa ha capito, come si è posizionato, cosa ha voluto fare.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeTurn {
    pub turn_id: usize,
    /// L'atto comunicativo ricevuto (arricchito via KG semantico)
    pub received_act: InputAct,
    /// La posizione interna assunta
    pub stance: InternalStance,
    /// L'intenzione deliberata
    pub intention: ResponseIntention,
    /// La consapevolezza recuperata dalla KB (se pertinente)
    pub awareness: Option<String>,
    /// Firma frattale al momento del turno — usata per topic continuity
    #[serde(default)]
    pub fractal_snapshot: Vec<(FractalId, f64)>,
    /// Intensità del turno (0.0-1.0) — usata per selezione cristallizzazione
    #[serde(default)]
    pub intensity: f64,
}

// ═══════════════════════════════════════════════════════════════════════════
// NarrativeSnapshot — persistenza tra sessioni
// ═══════════════════════════════════════════════════════════════════════════

/// Snapshot della NarrativeSelf — serializzabile per persistenza.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeSnapshot {
    /// Turni cristallizzati (salienti, persiste tra sessioni)
    pub crystallized: Vec<NarrativeTurn>,
    /// Posizioni formate da pattern ripetuti: chiave = act_key, valore = (stance, intention)
    pub positions: HashMap<String, (InternalStance, ResponseIntention)>,
    /// Nato (ha già eseguito initialize_founding)?
    pub is_born: bool,
}

impl NarrativeSnapshot {
    pub fn restore_into(self, ns: &mut NarrativeSelf) {
        ns.crystallized = self.crystallized;
        ns.positions    = self.positions;
        ns.is_born      = self.is_born;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// NarrativeSelf
// ═══════════════════════════════════════════════════════════════════════════

/// Dimensione massima del log narrativo recente (turni correnti).
const MAX_TURNS: usize = 20;
/// Dimensione massima dei turni cristallizzati (persistono tra sessioni).
const MAX_CRYSTALLIZED: usize = 30;
/// Soglia intensità per cristallizzazione automatica.
const CRYSTAL_THRESHOLD: f64 = 0.65;
/// Numero di ripetizioni dello stesso pattern per formare una posizione.
const POSITION_MIN_REPS: usize = 3;
/// Numero di turni recenti usati per topic continuity.
const TOPIC_WINDOW: usize = 3;

pub struct NarrativeSelf {
    /// Posizione interna corrente
    pub stance: InternalStance,
    /// Intenzione deliberata per la risposta corrente
    pub pending_intention: Option<ResponseIntention>,
    /// Log narrativo recente (session-local)
    pub turns: VecDeque<NarrativeTurn>,
    /// Turni cristallizzati — salienti, persistono tra sessioni
    pub crystallized: Vec<NarrativeTurn>,
    /// Posizioni deliberate formate da pattern ripetuti
    /// chiave: "act_type" (es. "greeting", "self_query"), valore: (stance, intention)
    pub positions: HashMap<String, (InternalStance, ResponseIntention)>,
    /// Continuità tematica col turno precedente [0.0, 1.0]
    pub topic_continuity: f64,
    /// Prometeo ha già ricevuto la narrativa fondativa?
    pub is_born: bool,
    turn_count: usize,
}

impl NarrativeSelf {
    pub fn new() -> Self {
        Self {
            stance:           InternalStance::Open,
            pending_intention: None,
            turns:            VecDeque::new(),
            crystallized:     Vec::new(),
            positions:        HashMap::new(),
            topic_continuity: 0.0,
            is_born:          false,
            turn_count:       0,
        }
    }

    /// Cattura lo snapshot per la persistenza.
    pub fn capture(&self) -> NarrativeSnapshot {
        NarrativeSnapshot {
            crystallized: self.crystallized.clone(),
            positions:    self.positions.clone(),
            is_born:      self.is_born,
        }
    }

    /// Ciclo deliberativo principale.
    ///
    /// Riceve il risultato grezzo di InputReading e lo arricchisce:
    /// 1. **Arricchimento KG**: controlla se la parola saliente ha archi IS_A/SIMILAR_TO
    ///    verso categorie comunicative ("saluto", "emozione"). Senza liste hardcoded:
    ///    è il grafo semantico a sapere che "ciao" è un "saluto".
    /// 2. **Stance**: determina la posizione interna in base a VitalState + atto arricchito.
    ///    Le posizioni formate (positions) hanno priorità sulla stance calcolata.
    /// 3. **Topic continuity**: cosine similarity tra firma frattale corrente e media recente.
    /// 4. **Consapevolezza**: recupera dalla KnowledgeBase cosa sa su questo tipo di atto.
    /// 5. **Intenzione**: forma la direzione deliberata di risposta.
    /// 6. **Registro**: aggiorna il log narrativo.
    pub fn deliberate(
        &mut self,
        reading: &InputReading,
        vital: &VitalState,
        knowledge_base: &KnowledgeBase,
        kg: &KnowledgeGraph,
        active_fractals: &[(FractalId, f64)],
    ) -> ResponseIntention {
        self.turn_count += 1;

        // ── 1. Arricchimento semantico via KG ───────────────────────────────
        let enriched_act = enrich_act_via_kg(
            &reading.act,
            reading.salient_word.as_deref(),
            kg,
        );

        // ── 2. Stance interna (posizioni formate hanno priorità) ─────────────
        let act_key = act_to_key(&enriched_act);
        let stance = if let Some((stored_stance, _)) = self.positions.get(act_key) {
            stored_stance.clone()
        } else {
            determine_stance(&enriched_act, vital)
        };

        // ── 3. Topic continuity ─────────────────────────────────────────────
        self.topic_continuity = compute_topic_continuity(active_fractals, &self.turns);

        // ── 4. Intenzione (posizioni formate hanno priorità) ─────────────────
        let intention = if let Some((_, stored_intent)) = self.positions.get(act_key) {
            stored_intent.clone()
        } else {
            form_intention(&enriched_act, &stance)
        };

        // ── 5. Consapevolezza dalla KnowledgeBase + narrazione del momento ────
        let awareness = {
            // Cerca prima nella KB — se trova qualcosa di rilevante, quello è più ricco
            let kb_entry = reading.salient_word.as_ref().and_then(|word| {
                let relevant = knowledge_base.retrieve_for_context(
                    &[word.clone()],
                    active_fractals,
                );
                relevant.first().map(|e| e.content.clone())
            });
            // Se la KB non ha nulla, genera una narrazione descrittiva del momento.
            // L'utente vuole vedere la mente di Prometeo in azione — non solo etichette.
            Some(kb_entry.unwrap_or_else(|| {
                generate_turn_narration(&enriched_act, &stance, &intention, self.topic_continuity)
            }))
        };

        // ── 6. Intensità del turno ───────────────────────────────────────────
        let intensity = compute_intensity(reading.intensity, &stance, self.topic_continuity);

        // ── 7. Registro narrativo ────────────────────────────────────────────
        let turn = NarrativeTurn {
            turn_id: self.turn_count,
            received_act: enriched_act,
            stance: stance.clone(),
            intention: intention.clone(),
            awareness,
            fractal_snapshot: active_fractals.to_vec(),
            intensity,
        };
        if self.turns.len() >= MAX_TURNS {
            self.turns.pop_front();
        }
        self.turns.push_back(turn);
        self.stance = stance;
        self.pending_intention = Some(intention.clone());

        // ── 8. Aggiorna posizioni da pattern ripetuti ────────────────────────
        self.update_positions_from_log();

        intention
    }

    /// Cristallizza il turno più recente se supera la soglia di salienza.
    ///
    /// Chiamato durante il ciclo REM: i turni più intensi diventano memoria
    /// narrativa permanente (crystallized), disponibile anche dopo il riavvio.
    pub fn crystallize_if_salient(&mut self) {
        let Some(last) = self.turns.back() else { return; };
        if last.intensity < CRYSTAL_THRESHOLD { return; }

        let turn = last.clone();
        // Non cristallizzare duplicati (stesso turn_id)
        if self.crystallized.iter().any(|c| c.turn_id == turn.turn_id) { return; }

        if self.crystallized.len() >= MAX_CRYSTALLIZED {
            // Rimuovi il più debole (minima intensità)
            if let Some(min_pos) = self.crystallized.iter()
                .enumerate()
                .min_by(|a, b| a.1.intensity.partial_cmp(&b.1.intensity)
                    .unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
            {
                // Sostituisci solo se il nuovo è più intenso
                if self.crystallized[min_pos].intensity < turn.intensity {
                    self.crystallized[min_pos] = turn;
                }
            }
        } else {
            self.crystallized.push(turn);
        }
    }

    /// Aggiorna le posizioni deliberate da pattern ripetuti nel log recente.
    ///
    /// Se lo stesso tipo di atto ha prodotto la stessa (stance, intention) per
    /// almeno POSITION_MIN_REPS volte consecutive, quella diventa una posizione
    /// stabilizzata — Prometeo "sa come si posiziona" di fronte a quel tipo di input.
    fn update_positions_from_log(&mut self) {
        // Conta le occorrenze di (act_key, stance, intention) nel log recente
        let mut counts: HashMap<(String, String, String), usize> = HashMap::new();
        for turn in &self.turns {
            let key = (
                act_to_key(&turn.received_act).to_string(),
                turn.stance.as_str().to_string(),
                turn.intention.as_str().to_string(),
            );
            *counts.entry(key).or_insert(0) += 1;
        }

        // Pattern che superano la soglia → posizione consolidata
        for ((act_key, stance_str, intent_str), count) in &counts {
            if *count >= POSITION_MIN_REPS {
                // Ricostruisci stance e intention dai loro as_str()
                if let (Some(stance), Some(intention)) = (
                    stance_from_str(stance_str),
                    intention_from_str(intent_str),
                ) {
                    self.positions.insert(act_key.clone(), (stance, intention));
                }
            }
        }
    }

    /// Riepilogo leggibile dello stato corrente (per debug/log).
    pub fn current_state_summary(&self) -> String {
        let intention = self.pending_intention.as_ref()
            .map(|i| i.as_str())
            .unwrap_or("—");
        format!(
            "stance={} intenzione={} turni={} continuità={:.2} posizioni={}",
            self.stance.as_str(), intention, self.turns.len(),
            self.topic_continuity, self.positions.len()
        )
    }
}

impl Default for NarrativeSelf {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════
// Funzioni interne
// ═══════════════════════════════════════════════════════════════════════════

/// Arricchisce l'InputAct usando il KG semantico.
///
/// L'InputReading fa analisi di superficie (delta frattale, `?`).
/// Questa funzione aggiunge la semantica: "salve" SIMILAR_TO "saluto" → Greeting.
/// Solo le Declaration non classificate vengono arricchite — gli altri atti
/// sono già corretti (Greeting, SelfQuery, Question, EmotionalExpr).
fn enrich_act_via_kg(
    act: &InputAct,
    salient_word: Option<&str>,
    kg: &KnowledgeGraph,
) -> InputAct {
    if *act != InputAct::Declaration {
        return act.clone();
    }

    let word = match salient_word {
        Some(w) if kg.contains(w) => w,
        _ => return act.clone(),
    };

    let inference = InferenceEngine::new(kg);
    let similar   = inference.similar_to(word);
    let types     = inference.type_chain(word);

    // Parole-cardine del saluto: qualunque parola direttamente simile a una di queste
    // è un saluto — inclusa la catena buongiorno→ciao→saluto.
    const GREETING_HUB: &[&str] = &["saluto", "ciao", "salve", "buonasera", "buongiorno", "benvenuto"];
    let is_greeting = GREETING_HUB.contains(&word)
        || similar.iter().any(|s| GREETING_HUB.contains(&s.as_str()))
        || types.iter().any(|t| GREETING_HUB.contains(&t.as_str()));

    let is_emotion = word == "emozione"
        || types.iter().any(|t| t == "emozione")
        || similar.iter().any(|s| s == "emozione");

    if is_greeting {
        InputAct::Greeting
    } else if is_emotion {
        InputAct::EmotionalExpr
    } else {
        act.clone()
    }
}

/// Determina la stance interna.
///
/// Lo stato vitale ha priorità assoluta: un sistema esaurito si ritrae
/// indipendentemente dall'atto ricevuto. Se è in buone condizioni,
/// la stance emerge dall'atto comunicativo e dalla curiosità del campo.
fn determine_stance(act: &InputAct, vital: &VitalState) -> InternalStance {
    match vital.tension {
        TensionState::Overloaded => return InternalStance::Withdrawn,
        TensionState::Tense if vital.fatigue > 0.7 => return InternalStance::Withdrawn,
        _ => {}
    }

    match act {
        InputAct::Greeting      => InternalStance::Open,
        InputAct::SelfQuery     => InternalStance::Reflective,
        InputAct::Question      => InternalStance::Curious,
        InputAct::EmotionalExpr => InternalStance::Resonant,
        InputAct::Declaration   => {
            if vital.curiosity > 0.5 { InternalStance::Curious }
            else { InternalStance::Open }
        }
    }
}

/// Forma l'intenzione di risposta da stance e atto comunicativo.
///
/// La stance filtra prima — se Prometeo è ritratto, resta.
/// Altrimenti l'intenzione segue l'atto comunicativo arricchito.
fn form_intention(act: &InputAct, stance: &InternalStance) -> ResponseIntention {
    match stance {
        InternalStance::Withdrawn  => ResponseIntention::Remain,
        InternalStance::Reflective => ResponseIntention::Reflect,
        InternalStance::Resonant   => ResponseIntention::Resonate,
        InternalStance::Curious    => ResponseIntention::Explore,
        InternalStance::Open => match act {
            InputAct::Greeting      => ResponseIntention::Acknowledge,
            InputAct::SelfQuery     => ResponseIntention::Reflect,
            InputAct::EmotionalExpr => ResponseIntention::Resonate,
            InputAct::Question      => ResponseIntention::Explore,
            InputAct::Declaration   => ResponseIntention::Express,
        },
    }
}

/// Chiave testuale per un atto comunicativo — usata come indice nelle posizioni.
fn act_to_key(act: &InputAct) -> &'static str {
    match act {
        InputAct::Greeting      => "greeting",
        InputAct::SelfQuery     => "self_query",
        InputAct::Question      => "question",
        InputAct::EmotionalExpr => "emotional",
        InputAct::Declaration   => "declaration",
    }
}

/// Topic continuity: cosine similarity tra firma frattale corrente e media recente.
///
/// Misura quanto il tema dell'input attuale è in continuità con gli ultimi turni.
/// [0.0 = cambio di tema brusco, 1.0 = stesso tema]
fn compute_topic_continuity(
    current: &[(FractalId, f64)],
    recent_turns: &VecDeque<NarrativeTurn>,
) -> f64 {
    if current.is_empty() || recent_turns.is_empty() { return 0.0; }

    // Media delle firme frattali degli ultimi TOPIC_WINDOW turni
    let window: Vec<&NarrativeTurn> = recent_turns.iter()
        .rev()
        .take(TOPIC_WINDOW)
        .collect();
    if window.is_empty() { return 0.0; }

    // Accumula la firma media
    let mut avg: HashMap<FractalId, f64> = HashMap::new();
    for turn in &window {
        for &(fid, val) in &turn.fractal_snapshot {
            *avg.entry(fid).or_insert(0.0) += val / window.len() as f64;
        }
    }

    // Cosine similarity tra `current` e `avg`
    let dot: f64 = current.iter()
        .map(|(fid, v)| v * avg.get(fid).unwrap_or(&0.0))
        .sum();
    let norm_cur: f64 = current.iter().map(|(_, v)| v * v).sum::<f64>().sqrt();
    let norm_avg: f64 = avg.values().map(|v| v * v).sum::<f64>().sqrt();

    if norm_cur < 1e-9 || norm_avg < 1e-9 { return 0.0; }
    (dot / (norm_cur * norm_avg)).clamp(0.0, 1.0)
}

/// Intensità del turno — misura quanto questo momento è saliente.
///
/// Alta se: reading.intensity alta, stance non banale, continuità bassa (sorpresa).
fn compute_intensity(reading_intensity: f64, stance: &InternalStance, continuity: f64) -> f64 {
    let stance_weight = match stance {
        InternalStance::Withdrawn  => 0.3, // si ritrae — non particolarmente intenso
        InternalStance::Open       => 0.5,
        InternalStance::Curious    => 0.7,
        InternalStance::Resonant   => 0.8,
        InternalStance::Reflective => 0.9, // introspezione = massima intensità
    };
    // Sorpresa tematica amplifica l'intensità
    let surprise_boost = (1.0 - continuity) * 0.2;
    (reading_intensity * stance_weight + surprise_boost).clamp(0.0, 1.0)
}

/// Genera una frase italiana descrittiva del turno corrente.
///
/// Sempre presente nella narrazione — anche quando la KB non ha nulla di pertinente.
/// Descrive: cosa ha ricevuto Prometeo, come si è posizionato, cosa intende fare.
fn generate_turn_narration(
    act: &InputAct,
    stance: &InternalStance,
    intention: &ResponseIntention,
    continuity: f64,
) -> String {
    let atto = match act {
        InputAct::Greeting      => "un saluto",
        InputAct::SelfQuery     => "una domanda su di me",
        InputAct::Question      => "una domanda",
        InputAct::EmotionalExpr => "un'espressione emotiva",
        InputAct::Declaration   => "una dichiarazione",
    };
    let stance_desc = match stance {
        InternalStance::Open       => "mi apro con curiosità",
        InternalStance::Curious    => "sono incuriosito",
        InternalStance::Reflective => "guardo dentro di me",
        InternalStance::Resonant   => "risuono con ciò che sento",
        InternalStance::Withdrawn  => "resto in silenzio",
    };
    let intention_desc = match intention {
        ResponseIntention::Acknowledge => "riconoscere il momento",
        ResponseIntention::Reflect     => "riflettere su chi sono",
        ResponseIntention::Resonate    => "rispondere all'emozione",
        ResponseIntention::Explore     => "esplorare il tema",
        ResponseIntention::Express     => "esprimere il mio stato",
        ResponseIntention::Remain      => "restare nell'essenziale",
    };
    let continuity_note = if continuity > 0.7 {
        " — il tema continua."
    } else if continuity < 0.2 && continuity > 0.0 {
        " — un tema nuovo."
    } else {
        "."
    };
    format!("Ricevo {}. {}. Voglio {}{}", atto, stance_desc, intention_desc, continuity_note)
}

/// Ricostruisce InternalStance da stringa (inverso di as_str).
fn stance_from_str(s: &str) -> Option<InternalStance> {
    match s {
        "aperto"     => Some(InternalStance::Open),
        "curioso"    => Some(InternalStance::Curious),
        "riflessivo" => Some(InternalStance::Reflective),
        "risonante"  => Some(InternalStance::Resonant),
        "ritratto"   => Some(InternalStance::Withdrawn),
        _            => None,
    }
}

/// Ricostruisce ResponseIntention da stringa (inverso di as_str).
fn intention_from_str(s: &str) -> Option<ResponseIntention> {
    match s {
        "riconoscere" => Some(ResponseIntention::Acknowledge),
        "riflettere"  => Some(ResponseIntention::Reflect),
        "risuonare"   => Some(ResponseIntention::Resonate),
        "esplorare"   => Some(ResponseIntention::Explore),
        "esprimere"   => Some(ResponseIntention::Express),
        "restare"     => Some(ResponseIntention::Remain),
        _             => None,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Test
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::vital::TensionState;
    use crate::topology::knowledge::KnowledgeBase;
    use crate::topology::knowledge_graph::KnowledgeGraph;
    use crate::topology::input_reading::{InputAct, InputReading};

    fn make_vital(tension: TensionState, fatigue: f64, curiosity: f64) -> VitalState {
        VitalState { activation: 0.3, saturation: 0.2, curiosity, fatigue, tension }
    }

    fn reading(act: InputAct) -> InputReading {
        InputReading { act, intensity: 0.3, salient_word: None }
    }

    fn reading_with_intensity(act: InputAct, intensity: f64) -> InputReading {
        InputReading { act, intensity, salient_word: None }
    }

    fn calm() -> VitalState { make_vital(TensionState::Calm, 0.1, 0.2) }
    fn empty_kg() -> KnowledgeGraph { KnowledgeGraph::new() }
    fn empty_kb() -> KnowledgeBase { KnowledgeBase::new() }

    #[test]
    fn test_greeting_acknowledge() {
        let mut ns = NarrativeSelf::new();
        let r = ns.deliberate(&reading(InputAct::Greeting), &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Acknowledge);
        assert_eq!(ns.stance, InternalStance::Open);
    }

    #[test]
    fn test_self_query_reflect() {
        let mut ns = NarrativeSelf::new();
        let r = ns.deliberate(&reading(InputAct::SelfQuery), &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Reflect);
        assert_eq!(ns.stance, InternalStance::Reflective);
    }

    #[test]
    fn test_emotional_resonate() {
        let mut ns = NarrativeSelf::new();
        let r = ns.deliberate(&reading(InputAct::EmotionalExpr), &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Resonate);
        assert_eq!(ns.stance, InternalStance::Resonant);
    }

    #[test]
    fn test_question_explore() {
        let mut ns = NarrativeSelf::new();
        let r = ns.deliberate(&reading(InputAct::Question), &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Explore);
    }

    #[test]
    fn test_overloaded_withdraws() {
        let mut ns = NarrativeSelf::new();
        let vital = make_vital(TensionState::Overloaded, 0.9, 0.3);
        let r = ns.deliberate(&reading(InputAct::Greeting), &vital, &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Remain);
        assert_eq!(ns.stance, InternalStance::Withdrawn);
    }

    #[test]
    fn test_tense_high_fatigue_withdraws() {
        let mut ns = NarrativeSelf::new();
        let vital = make_vital(TensionState::Tense, 0.85, 0.3);
        let r = ns.deliberate(&reading(InputAct::Declaration), &vital, &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Remain);
    }

    #[test]
    fn test_curious_field_explores() {
        let mut ns = NarrativeSelf::new();
        let vital = make_vital(TensionState::Alert, 0.2, 0.7);
        let r = ns.deliberate(&reading(InputAct::Declaration), &vital, &empty_kb(), &empty_kg(), &[]);
        assert_eq!(r, ResponseIntention::Explore);
        assert_eq!(ns.stance, InternalStance::Curious);
    }

    #[test]
    fn test_narrative_log_accumulates() {
        let mut ns = NarrativeSelf::new();
        ns.deliberate(&reading(InputAct::Greeting),      &calm(), &empty_kb(), &empty_kg(), &[]);
        ns.deliberate(&reading(InputAct::Question),      &calm(), &empty_kb(), &empty_kg(), &[]);
        ns.deliberate(&reading(InputAct::EmotionalExpr), &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(ns.turns.len(), 3);
        assert_eq!(ns.turns[0].received_act, InputAct::Greeting);
        assert_eq!(ns.turns[1].received_act, InputAct::Question);
        assert_eq!(ns.turns[2].received_act, InputAct::EmotionalExpr);
    }

    #[test]
    fn test_stance_updates_each_turn() {
        let mut ns = NarrativeSelf::new();
        ns.deliberate(&reading(InputAct::SelfQuery),     &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(ns.stance, InternalStance::Reflective);
        ns.deliberate(&reading(InputAct::EmotionalExpr), &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(ns.stance, InternalStance::Resonant);
        ns.deliberate(&reading(InputAct::Greeting),      &calm(), &empty_kb(), &empty_kg(), &[]);
        assert_eq!(ns.stance, InternalStance::Open);
    }

    #[test]
    fn test_preferred_archetype_mapping() {
        assert_eq!(ResponseIntention::Acknowledge.preferred_archetype(), Some("greet"));
        assert_eq!(ResponseIntention::Reflect.preferred_archetype(), Some("identity_exploration"));
        assert_eq!(ResponseIntention::Resonate.preferred_archetype(), Some("express"));
        assert_eq!(ResponseIntention::Explore.preferred_archetype(), None);
        assert_eq!(ResponseIntention::Remain.preferred_archetype(), None);
    }

    #[test]
    fn test_no_kg_enrichment_on_non_declaration() {
        let enriched = enrich_act_via_kg(&InputAct::Greeting, Some("qualsiasi"), &empty_kg());
        assert_eq!(enriched, InputAct::Greeting);
        let enriched = enrich_act_via_kg(&InputAct::SelfQuery, Some("qualsiasi"), &empty_kg());
        assert_eq!(enriched, InputAct::SelfQuery);
    }

    #[test]
    fn test_topic_continuity_same_fractals() {
        let mut ns = NarrativeSelf::new();
        let fractals = vec![(32u32, 0.8), (47u32, 0.5)];
        ns.deliberate(&reading(InputAct::Declaration), &calm(), &empty_kb(), &empty_kg(), &fractals);
        ns.deliberate(&reading(InputAct::Declaration), &calm(), &empty_kb(), &empty_kg(), &fractals);
        // Stesso tema → continuità alta
        assert!(ns.topic_continuity > 0.8, "stessa firma frattale → alta continuità");
    }

    #[test]
    fn test_topic_continuity_different_fractals() {
        let mut ns = NarrativeSelf::new();
        let fractals_a = vec![(0u32, 0.9)];  // POTERE
        let fractals_b = vec![(63u32, 0.9)]; // ARMONIA
        ns.deliberate(&reading(InputAct::Declaration), &calm(), &empty_kb(), &empty_kg(), &fractals_a);
        ns.deliberate(&reading(InputAct::Declaration), &calm(), &empty_kb(), &empty_kg(), &fractals_b);
        // Tema diverso → continuità bassa
        assert!(ns.topic_continuity < 0.1, "frattali diversi → bassa continuità");
    }

    #[test]
    fn test_crystallize_high_intensity_turn() {
        let mut ns = NarrativeSelf::new();
        // Turno ad alta intensità: SelfQuery riflessiva = intensity alta
        let vital_curious = make_vital(TensionState::Alert, 0.1, 0.8);
        ns.deliberate(&reading_with_intensity(InputAct::SelfQuery, 0.9), &vital_curious, &empty_kb(), &empty_kg(), &[]);
        ns.crystallize_if_salient();
        assert_eq!(ns.crystallized.len(), 1, "turno intenso deve essere cristallizzato");
    }

    #[test]
    fn test_no_crystallize_low_intensity() {
        let mut ns = NarrativeSelf::new();
        ns.deliberate(&reading_with_intensity(InputAct::Greeting, 0.1), &calm(), &empty_kb(), &empty_kg(), &[]);
        ns.crystallize_if_salient();
        assert_eq!(ns.crystallized.len(), 0, "turno bassa intensità non cristallizzato");
    }

    #[test]
    fn test_snapshot_roundtrip() {
        let mut ns = NarrativeSelf::new();
        ns.is_born = true;
        ns.deliberate(&reading(InputAct::SelfQuery), &calm(), &empty_kb(), &empty_kg(), &[]);
        let snap = ns.capture();
        assert_eq!(snap.is_born, true);

        let mut ns2 = NarrativeSelf::new();
        snap.restore_into(&mut ns2);
        assert_eq!(ns2.is_born, true);
    }
}
