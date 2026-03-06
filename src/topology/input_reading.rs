/// Comprensione dell'atto comunicativo — Phase 41b.
///
/// La precedente implementazione usava liste hardcoded (GREETING_WORDS, QUESTION_WORDS,
/// SELF_INDICATORS). Questo è "puppet theater": si simula la comprensione enumerando
/// tutti i casi possibili invece di capire il concetto.
///
/// La nuova implementazione:
///   - Nessuna lista tematica hardcoded
///   - Il concetto "saluto" è nella KnowledgeBase (teach_concept) con firma frattale
///   - Il delta frattale rivela cosa l'input ha cambiato nel campo — non il rumore di fondo
///   - L'unico marcatore sintattico mantenuto è `?` (è punteggiatura, non vocabolario)
///
/// "Prometeo non memorizza tutti i saluti: capisce cosa è un saluto."

use crate::topology::fractal::FractalId;
use crate::topology::lexicon::Lexicon;
use crate::topology::knowledge::{KnowledgeBase, KnowledgeDomain};

/// Atto comunicativo rilevato dall'input.
/// Ordine di priorità: Greeting > SelfQuery > Question > EmotionalExpr > Declaration.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InputAct {
    /// Saluto — riconosciuto via dominio Social nella KnowledgeBase
    Greeting,
    /// Domanda su Prometeo stesso — `?` + dominio Self_ nella KnowledgeBase
    SelfQuery,
    /// Domanda generica — solo marcatore sintattico `?`
    Question,
    /// Espressione emotiva — dominio Emotional nella KnowledgeBase (delta EMOZIONE/CORPO)
    EmotionalExpr,
    /// Dichiarazione generica — tutto il resto
    Declaration,
}

/// Lettura strutturata dell'input corrente.
#[derive(Debug, Clone)]
pub struct InputReading {
    pub act: InputAct,
    /// Intensità dell'atto comunicativo (0..1) — media top-3 delta frattali assoluti
    pub intensity: f64,
    /// Parola più stabile dell'input (se presente nel lessico)
    pub salient_word: Option<String>,
}

/// Legge l'atto comunicativo dal DELTA frattale + KnowledgeBase concettuale.
///
/// `frattale_delta[i] = attivazione_post - attivazione_pre` (solo cambiamenti > 0.01).
/// Questo isola il segnale dell'input dal rumore di fondo del campo (identity seed,
/// dogfeed, recall episodico).
///
/// I concetti (saluto, emozione, identità) sono ancore nella KnowledgeBase — non liste
/// di parole. Qualunque parola che attivi la stessa firma frattale viene riconosciuta
/// come appartenente al concetto, anche se non è mai stata vista prima.
pub fn read_input(
    raw_words: &[String],
    raw_text: &str,
    frattale_delta: &[(FractalId, f64)],
    knowledge_base: &KnowledgeBase,
    lexicon: &Lexicon,
) -> InputReading {
    // ── Parola più stabile dell'input ────────────────────────────────────────
    let salient_word = raw_words.iter()
        .filter(|w| w.len() >= 3)
        .filter_map(|w| lexicon.get(w).map(|p| (w.clone(), p.stability)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(w, _)| w);

    // ── Intensità: media top-3 delta assoluti ────────────────────────────────
    let intensity = {
        let mut deltas: Vec<f64> = frattale_delta.iter().map(|(_, d)| d.abs()).collect();
        deltas.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
        let top3: Vec<f64> = deltas.iter().take(3).copied().collect();
        if top3.is_empty() { 0.0 } else { top3.iter().sum::<f64>() / top3.len() as f64 }
    };

    // ── Concetti rilevanti — KnowledgeBase via delta frattale ────────────────
    // retrieve_for_delta: word_match (parola campione) OR delta_match (firma frattale)
    // → qualunque parola che attivi la stessa regione frattale viene riconosciuta
    let relevant = knowledge_base.retrieve_for_delta(raw_words, frattale_delta);
    let has_social    = relevant.iter().any(|e| e.domain == KnowledgeDomain::Social);
    let has_emotional = relevant.iter().any(|e| e.domain == KnowledgeDomain::Emotional);
    let has_self      = relevant.iter().any(|e| e.domain == KnowledgeDomain::Self_);

    // ── `?` come unico marcatore sintattico ──────────────────────────────────
    // Non è vocabolario tematico: è punteggiatura strutturale del linguaggio.
    let has_question_mark = raw_text.contains('?');

    // ── Classificazione (ordine di priorità) ─────────────────────────────────
    let act = if has_social {
        InputAct::Greeting
    } else if has_question_mark && has_self {
        InputAct::SelfQuery
    } else if has_question_mark {
        InputAct::Question
    } else if has_emotional {
        InputAct::EmotionalExpr
    } else {
        InputAct::Declaration
    };

    InputReading { act, intensity, salient_word }
}

#[cfg(test)]
mod tests {
    use super::*;

    // KnowledgeBase con ancore concettuali — specchia seed_conceptual_anchors() in engine.rs
    fn kb_with_anchors() -> KnowledgeBase {
        let mut kb = KnowledgeBase::new();
        kb.teach_concept(
            KnowledgeDomain::Social,
            "un saluto è un atto di avvicinamento sociale: chi saluta vuole connessione",
            "ciao",
            vec![63, 47], // ARMONIA + COMUNICAZIONE
        );
        kb.teach_concept(
            KnowledgeDomain::Emotional,
            "un'emozione è uno stato interno vissuto: chi esprime un'emozione cerca risonanza",
            "paura",
            vec![58, 33], // EMOZIONE + CORPO
        );
        kb.teach_concept(
            KnowledgeDomain::Self_,
            "un'indagine sull'identità è la domanda su chi è Prometeo, cosa sente, cosa vuole",
            "sei",
            vec![32], // IDENTITA
        );
        kb
    }

    fn empty_delta() -> Vec<(FractalId, f64)> { vec![] }
    fn lex() -> Lexicon { Lexicon::bootstrap() }

    #[test]
    fn test_greeting_via_word_match() {
        // "ciao" è la parola-campione del concetto Social → word_match in retrieve_for_delta
        let lex = lex();
        let kb = kb_with_anchors();
        let r = read_input(
            &["ciao".to_string()],
            "ciao",
            &empty_delta(),
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::Greeting);
    }

    #[test]
    fn test_greeting_via_fractal_delta() {
        // "salve" non è la parola-campione, ma attiva ARMONIA(63) → delta_match → Social
        let lex = lex();
        let kb = kb_with_anchors();
        let delta = vec![(63u32, 0.15f64)]; // ARMONIA delta > 0.05
        let r = read_input(
            &["salve".to_string()],
            "salve",
            &delta,
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::Greeting,
            "salve dovrebbe essere riconosciuto come saluto via delta ARMONIA");
    }

    #[test]
    fn test_self_query_via_word_match() {
        // "sei" è la parola-campione del concetto Self_ + `?` → SelfQuery
        let lex = lex();
        let kb = kb_with_anchors();
        let r = read_input(
            &["chi".to_string(), "sei".to_string()],
            "chi sei?",
            &empty_delta(),
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::SelfQuery);
    }

    #[test]
    fn test_self_query_via_fractal_delta() {
        // Una domanda che attiva IDENTITA(32) → SelfQuery anche senza "sei"
        let lex = lex();
        let kb = kb_with_anchors();
        let delta = vec![(32u32, 0.12f64)]; // IDENTITA delta
        let r = read_input(
            &["cosa".to_string(), "pensi".to_string()],
            "cosa pensi?",
            &delta,
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::SelfQuery,
            "domanda con delta IDENTITA → SelfQuery anche senza parola-campione");
    }

    #[test]
    fn test_generic_question() {
        // `?` senza Social/Self_/Emotional → Question generica
        let lex = lex();
        let kb = kb_with_anchors();
        let r = read_input(
            &["cosa".to_string(), "succede".to_string()],
            "cosa succede?",
            &empty_delta(),
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::Question);
    }

    #[test]
    fn test_emotional_expr_via_word_match() {
        // "paura" è la parola-campione del concetto Emotional → word_match → EmotionalExpr
        let lex = lex();
        let kb = kb_with_anchors();
        let r = read_input(
            &["ho".to_string(), "paura".to_string()],
            "ho paura",
            &empty_delta(),
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::EmotionalExpr);
    }

    #[test]
    fn test_emotional_expr_via_fractal_delta() {
        // EMOZIONE(58) delta → Emotional concept → EmotionalExpr
        let lex = lex();
        let kb = kb_with_anchors();
        let delta = vec![(58u32, 0.45f64)]; // EMOZIONE delta
        let r = read_input(
            &["tristezza".to_string()],
            "sento tristezza",
            &delta,
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::EmotionalExpr,
            "qualunque parola che attiva EMOZIONE → EmotionalExpr");
    }

    #[test]
    fn test_declaration_default() {
        let lex = lex();
        let kb = kb_with_anchors();
        let r = read_input(
            &["penso".to_string(), "quindi".to_string(), "sono".to_string()],
            "penso quindi sono",
            &empty_delta(),
            &kb,
            &lex,
        );
        assert_eq!(r.act, InputAct::Declaration);
    }

    #[test]
    fn test_intensity_from_delta() {
        let lex = lex();
        let kb = kb_with_anchors();
        let delta = vec![(58u32, 0.6f64), (32u32, 0.4f64), (33u32, 0.2f64)];
        let r = read_input(&[], "", &delta, &kb, &lex);
        // avg top-3 assoluti = (0.6 + 0.4 + 0.2) / 3 ≈ 0.4
        assert!((r.intensity - 0.4).abs() < 0.01,
            "intensity attesa ~0.4, ottenuta {}", r.intensity);
    }

    #[test]
    fn test_no_anchors_fallback() {
        // Senza ancore concettuali, solo `?` e Declaration funzionano
        let lex = lex();
        let kb = KnowledgeBase::new(); // vuota
        let r = read_input(&["ciao".to_string()], "ciao", &empty_delta(), &kb, &lex);
        // Senza ancora Social, "ciao" → Declaration (non riconosciuto)
        assert_eq!(r.act, InputAct::Declaration,
            "senza KnowledgeBase, ciao non è riconoscibile come saluto");
        let r2 = read_input(&["cosa".to_string()], "cosa succede?", &empty_delta(), &kb, &lex);
        assert_eq!(r2.act, InputAct::Question,
            "senza KB, `?` mantiene Question come fallback sintattico");
    }
}
