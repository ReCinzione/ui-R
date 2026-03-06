/// Dialogo Multi-turno — La conversazione come traiettoria topologica.
///
/// Il dialogo non e una sequenza di domanda-risposta.
/// E una traiettoria nel campo topologico: ogni turno deforma il campo,
/// e il campo deformato e il contesto del turno successivo.
///
/// La STM contiene le impronte recenti — i turni della conversazione.
/// Il "tono" della conversazione emerge dalla postura media del campo
/// negli ultimi turni. I riferimenti anaforici ("quello che hai detto")
/// sono risonanze con le impronte STM recenti.

use std::collections::HashMap;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::SimplicialComplex;
use crate::topology::memory::{TopologicalMemory, FieldImprint};
use crate::topology::composition::PhrasePattern;
use crate::topology::primitive::{PrimitiveCore, Dim};

/// Un turno di conversazione: input + stato del campo al momento della risposta.
#[derive(Debug, Clone)]
pub struct ConversationTurn {
    /// L'input originale
    pub input: String,
    /// I frattali coinvolti con il loro peso
    pub fractal_involvement: Vec<(FractalId, f64)>,
    /// La firma dimensionale composita dell'input
    pub signature: PrimitiveCore,
    /// Numero del turno (0-based)
    pub turn_number: usize,
}

/// Il contesto conversazionale: cosa il sistema "ricorda" della conversazione.
#[derive(Debug)]
pub struct ConversationContext {
    /// Storia dei turni recenti
    turns: Vec<ConversationTurn>,
    /// Capacita massima di turni ricordati
    capacity: usize,
    /// Postura conversazionale: firma media degli ultimi turni
    pub posture: PrimitiveCore,
    /// Tono dominante: il frattale piu presente nella conversazione
    pub dominant_theme: Option<(FractalId, f64)>,
    /// Coerenza tematica: quanto i turni sono simili tra loro [0.0, 1.0]
    pub thematic_coherence: f64,
}

impl ConversationContext {
    pub fn new() -> Self {
        Self {
            turns: Vec::new(),
            capacity: 12,
            posture: PrimitiveCore::neutral(),
            dominant_theme: None,
            thematic_coherence: 0.0,
        }
    }

    /// Registra un nuovo turno di conversazione.
    pub fn record_turn(&mut self, input: &str, phrase: &PhrasePattern) {
        let involvement: Vec<(FractalId, f64)> = phrase.fractal_involvement
            .iter()
            .map(|(&fid, &score)| (fid, score))
            .collect();

        let turn = ConversationTurn {
            input: input.to_string(),
            fractal_involvement: involvement,
            signature: phrase.composite_signature.clone(),
            turn_number: self.turns.len(),
        };

        self.turns.push(turn);

        // Mantieni capacita
        if self.turns.len() > self.capacity {
            self.turns.remove(0);
        }

        // Aggiorna postura e tono
        self.update_posture();
        self.update_dominant_theme();
        self.update_coherence();
    }

    /// Quanti turni di conversazione ci sono.
    pub fn turn_count(&self) -> usize {
        self.turns.len()
    }

    /// Accesso ai turni.
    pub fn turns(&self) -> &[ConversationTurn] {
        &self.turns
    }

    /// Ultimo turno (se esiste).
    pub fn last_turn(&self) -> Option<&ConversationTurn> {
        self.turns.last()
    }

    /// Cerca un riferimento anaforico nella conversazione recente.
    /// "quello che hai detto" → cerca nelle impronte STM il turno che risuona
    /// di piu con il contesto corrente.
    ///
    /// Restituisce il turno piu rilevante e la sua forza di risonanza.
    pub fn resolve_anaphora(
        &self,
        current_phrase: &PhrasePattern,
    ) -> Option<(&ConversationTurn, f64)> {
        if self.turns.len() < 2 {
            return None;
        }

        let current_sig = &current_phrase.composite_signature;
        let current_fractals: HashMap<FractalId, f64> = current_phrase.fractal_involvement.clone();

        let mut best: Option<(&ConversationTurn, f64)> = None;

        // Cerca nei turni precedenti (escluso l'ultimo, che e il turno corrente)
        for turn in self.turns.iter().rev().skip(1) {
            // Risonanza = similitudine dimensionale + sovrapposizione frattale
            let dim_similarity = dimensional_similarity(current_sig, &turn.signature);
            let fractal_overlap = fractal_overlap(&current_fractals, &turn.fractal_involvement);

            let resonance = dim_similarity * 0.4 + fractal_overlap * 0.6;

            match &best {
                None if resonance > 0.2 => best = Some((turn, resonance)),
                Some((_, prev_score)) if resonance > *prev_score => best = Some((turn, resonance)),
                _ => {}
            }
        }

        best
    }

    /// Il contesto conversazionale influenza quali frattali pre-attivare.
    /// Restituisce i frattali da pre-attivare con il loro peso.
    pub fn contextual_bias(&self) -> Vec<(FractalId, f64)> {
        if self.turns.is_empty() {
            return Vec::new();
        }

        let mut fractal_scores: HashMap<FractalId, f64> = HashMap::new();

        // I turni recenti pesano di piu (decadimento esponenziale)
        let n = self.turns.len();
        for (i, turn) in self.turns.iter().enumerate() {
            let recency = ((i + 1) as f64 / n as f64).powi(2); // Piu recente = piu forte
            for &(fid, score) in &turn.fractal_involvement {
                let entry = fractal_scores.entry(fid).or_insert(0.0);
                *entry += score * recency;
            }
        }

        // Normalizza e filtra
        let max_score = fractal_scores.values().cloned().fold(0.0f64, f64::max);
        if max_score <= 0.0 {
            return Vec::new();
        }

        let mut result: Vec<(FractalId, f64)> = fractal_scores.into_iter()
            .map(|(fid, score)| (fid, (score / max_score * 0.3).min(0.3))) // Bias morbido (max 0.3)
            .filter(|(_, score)| *score > 0.05)
            .collect();

        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        result
    }

    /// Aggiorna la postura conversazionale (firma media dei turni recenti).
    fn update_posture(&mut self) {
        if self.turns.is_empty() {
            self.posture = PrimitiveCore::neutral();
            return;
        }

        let n = self.turns.len();
        let mut sum = [0.0f64; 8];
        let mut total_weight = 0.0;

        for (i, turn) in self.turns.iter().enumerate() {
            let weight = (i + 1) as f64 / n as f64; // Piu recente = piu peso
            for d in 0..8 {
                sum[d] += turn.signature.values()[d] * weight;
            }
            total_weight += weight;
        }

        if total_weight > 0.0 {
            for d in 0..8 {
                sum[d] /= total_weight;
            }
        }

        self.posture = PrimitiveCore::new(sum);
    }

    /// Aggiorna il tema dominante.
    fn update_dominant_theme(&mut self) {
        let mut fractal_scores: HashMap<FractalId, f64> = HashMap::new();

        for turn in &self.turns {
            for &(fid, score) in &turn.fractal_involvement {
                *fractal_scores.entry(fid).or_insert(0.0) += score;
            }
        }

        self.dominant_theme = fractal_scores.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }

    /// Aggiorna la coerenza tematica.
    fn update_coherence(&mut self) {
        if self.turns.len() < 2 {
            self.thematic_coherence = 1.0;
            return;
        }

        let mut total_sim = 0.0;
        let mut count = 0;

        // Confronta turni consecutivi
        for i in 1..self.turns.len() {
            let sim = dimensional_similarity(
                &self.turns[i].signature,
                &self.turns[i - 1].signature,
            );
            total_sim += sim;
            count += 1;
        }

        self.thematic_coherence = if count > 0 { total_sim / count as f64 } else { 1.0 };
    }

    /// Il sistema sta parlando di cose nuove o sta ripetendo temi?
    pub fn novelty(&self) -> f64 {
        1.0 - self.thematic_coherence
    }

    /// La conversazione sta divergendo o convergendo?
    /// Positivo = sta convergendo (i turni sono sempre piu simili).
    /// Negativo = sta divergendo.
    pub fn trajectory(&self) -> f64 {
        if self.turns.len() < 3 {
            return 0.0;
        }

        let n = self.turns.len();
        let recent_sim = dimensional_similarity(
            &self.turns[n - 1].signature,
            &self.turns[n - 2].signature,
        );
        let older_sim = dimensional_similarity(
            &self.turns[n - 2].signature,
            &self.turns[n - 3].signature,
        );

        recent_sim - older_sim // Positivo = convergenza
    }
}

/// Stato del dialogo: un report leggibile sulla conversazione.
#[derive(Debug)]
pub struct DialogueState {
    /// Numero di turni
    pub turn_count: usize,
    /// Nome del tema dominante
    pub dominant_theme: Option<String>,
    /// Coerenza tematica [0.0, 1.0]
    pub thematic_coherence: f64,
    /// Novita [0.0, 1.0]
    pub novelty: f64,
    /// Traiettoria (convergenza/divergenza)
    pub trajectory: f64,
    /// Dimensioni salienti della postura conversazionale
    pub salient_dimensions: Vec<(Dim, f64)>,
}

/// Calcola lo stato del dialogo leggibile.
pub fn dialogue_state(
    context: &ConversationContext,
    registry: &FractalRegistry,
) -> DialogueState {
    let dominant_name = context.dominant_theme
        .and_then(|(fid, _)| registry.get(fid).map(|f| f.name.clone()));

    // Dimensioni salienti della postura
    let neutral = PrimitiveCore::neutral();
    let dims = [
        Dim::Confine, Dim::Valenza, Dim::Intensita, Dim::Definizione,
        Dim::Complessita, Dim::Permanenza, Dim::Agency, Dim::Tempo,
    ];
    let mut salient: Vec<(Dim, f64)> = dims.iter()
        .map(|&d| {
            let idx = d as usize;
            let deviation = (context.posture.values()[idx] - neutral.values()[idx]).abs();
            (d, deviation)
        })
        .filter(|(_, dev)| *dev > 0.05)
        .collect();
    salient.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    DialogueState {
        turn_count: context.turn_count(),
        dominant_theme: dominant_name,
        thematic_coherence: context.thematic_coherence,
        novelty: context.novelty(),
        trajectory: context.trajectory(),
        salient_dimensions: salient,
    }
}

/// Similitudine tra due firme dimensionali (coseno normalizzato).
pub fn dimensional_similarity(a: &PrimitiveCore, b: &PrimitiveCore) -> f64 {
    let mut dot = 0.0f64;
    let mut mag_a = 0.0f64;
    let mut mag_b = 0.0f64;

    for i in 0..8 {
        dot += a.values()[i] * b.values()[i];
        mag_a += a.values()[i] * a.values()[i];
        mag_b += b.values()[i] * b.values()[i];
    }

    let denom = (mag_a.sqrt() * mag_b.sqrt()).max(0.001);
    (dot / denom).clamp(0.0, 1.0)
}

/// Sovrapposizione tra insiemi di frattali coinvolti.
fn fractal_overlap(
    current: &HashMap<FractalId, f64>,
    past: &[(FractalId, f64)],
) -> f64 {
    if current.is_empty() || past.is_empty() {
        return 0.0;
    }

    let mut overlap = 0.0;
    let mut total = 0.0;

    for &(fid, score) in past {
        total += score;
        if let Some(&current_score) = current.get(&fid) {
            overlap += score.min(current_score);
        }
    }

    if total <= 0.0 { 0.0 } else { (overlap / total).min(1.0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::bootstrap_complex;
    use crate::topology::lexicon::Lexicon;
    use crate::topology::composition::compose_phrase;

    fn setup() -> (FractalRegistry, Lexicon) {
        let reg = bootstrap_fractals();
        let lexicon = Lexicon::bootstrap();
        (reg, lexicon)
    }

    #[test]
    fn test_empty_context() {
        let ctx = ConversationContext::new();
        assert_eq!(ctx.turn_count(), 0);
        assert!(ctx.contextual_bias().is_empty());
        assert_eq!(ctx.thematic_coherence, 0.0);
    }

    #[test]
    fn test_record_turn() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        let phrase = compose_phrase(&mut lexicon, "pensare al tempo", &reg);
        ctx.record_turn("pensare al tempo", &phrase);

        assert_eq!(ctx.turn_count(), 1);
        assert!(ctx.dominant_theme.is_some());
    }

    #[test]
    fn test_multiple_turns_coherence() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        // Turni sullo stesso tema → alta coerenza
        ctx.record_turn("il tempo passa", &compose_phrase(&mut lexicon, "il tempo passa", &reg));
        ctx.record_turn("il tempo scorre", &compose_phrase(&mut lexicon, "il tempo scorre", &reg));
        ctx.record_turn("il tempo fugge", &compose_phrase(&mut lexicon, "il tempo fugge", &reg));

        assert!(ctx.thematic_coherence > 0.3,
            "Turni simili devono avere coerenza alta: {}", ctx.thematic_coherence);
    }

    #[test]
    fn test_contextual_bias() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        ctx.record_turn("pensare al tempo", &compose_phrase(&mut lexicon, "pensare al tempo", &reg));
        ctx.record_turn("il tempo scorre", &compose_phrase(&mut lexicon, "il tempo scorre", &reg));

        let bias = ctx.contextual_bias();
        assert!(!bias.is_empty(), "Dopo 2 turni deve esserci un bias");
        // Il bias deve essere morbido (max 0.3)
        for &(_, score) in &bias {
            assert!(score <= 0.3, "Bias troppo forte: {}", score);
        }
    }

    #[test]
    fn test_anaphora_resolution() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        let phrase1 = compose_phrase(&mut lexicon, "pensare alla casa", &reg);
        ctx.record_turn("pensare alla casa", &phrase1);

        let phrase2 = compose_phrase(&mut lexicon, "il tempo passa veloce", &reg);
        ctx.record_turn("il tempo passa veloce", &phrase2);

        // Cerca anafora con una frase simile alla prima
        let query = compose_phrase(&mut lexicon, "quella casa grande", &reg);
        let _result = ctx.resolve_anaphora(&query);

        assert!(ctx.turn_count() == 2);
    }

    #[test]
    fn test_capacity_limit() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        // Inserisci piu turni della capacita
        for i in 0..20 {
            let text = format!("turno numero {}", i);
            ctx.record_turn(&text, &compose_phrase(&mut lexicon, &text, &reg));
        }

        assert!(ctx.turn_count() <= 12, "Non deve superare la capacita: {}", ctx.turn_count());
    }

    #[test]
    fn test_novelty_and_trajectory() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        ctx.record_turn("pensare felicità", &compose_phrase(&mut lexicon, "pensare felicità", &reg));
        ctx.record_turn("sentire gioia", &compose_phrase(&mut lexicon, "sentire gioia", &reg));
        ctx.record_turn("andare insieme", &compose_phrase(&mut lexicon, "andare insieme", &reg));

        // Novita + traiettoria devono essere calcolabili senza crash
        let _novelty = ctx.novelty();
        let _trajectory = ctx.trajectory();
        assert!(ctx.turn_count() == 3);
    }

    #[test]
    fn test_dialogue_state() {
        let (reg, mut lexicon) = setup();
        let mut ctx = ConversationContext::new();

        ctx.record_turn("pensare al tempo", &compose_phrase(&mut lexicon, "pensare al tempo", &reg));
        ctx.record_turn("il tempo scorre", &compose_phrase(&mut lexicon, "il tempo scorre", &reg));

        let state = dialogue_state(&ctx, &reg);
        assert_eq!(state.turn_count, 2);
        // Il tema dominante dovrebbe essere qualcosa legato a TEMPO
        assert!(state.dominant_theme.is_some());
    }
}
