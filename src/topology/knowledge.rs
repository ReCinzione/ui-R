/// Memoria Procedurale — Conoscenza dichiarativa, non template.
///
/// Prometeo impara fatti e regole attraverso il campo topologico:
///   `engine.teach("il fuoco brucia")` → cristallizza la co-occorrenza in word_topology.
///
/// La KnowledgeBase è solo un registro di ciò che è stato insegnato.
/// Quando un fatto è rilevante (parole del contesto matchano), le sue parole
/// vengono richiamate come boost leggero nel campo — non come testo hardcoded.
///
/// Differenza dalla filosofia precedente (template):
///   ❌ "ciao" → template → [echo] + [emozione] (puppet theater)
///   ✅ "ciao" → attiva RELAZIONE nel campo → Phase 3 genera da lì
///   ✅ :know "un saluto si ricambia" → teach() topologico + recall_boost futuro

use crate::topology::fractal::FractalId;
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════════════════
// KnowledgeDomain — categorie di conoscenza
// ═══════════════════════════════════════════════════════════════════════════

/// Dominio della conoscenza procedurale/dichiarativa.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KnowledgeDomain {
    /// Convenzioni sociali (saluti, cortesie, turni di parola)
    Social,
    /// Come si forma il dialogo (domanda→risposta, eco, conferma)
    Dialogue,
    /// Come si fa X (procedure, sequenze di azioni)
    Procedural,
    /// Come gestire l'incertezza (non so, forse, mi sembra)
    Epistemic,
    /// Conoscenza di sé (cosa sono, come funziono)
    Self_,
    /// Sintassi e forma (struttura frasale, punteggiatura)
    Syntax,
    /// Espressione emotiva — stati interni, sentimenti, sensazioni corporee
    Emotional,
}

impl KnowledgeDomain {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "social" | "sociale" => KnowledgeDomain::Social,
            "dialogue" | "dialogo" => KnowledgeDomain::Dialogue,
            "procedural" | "procedurale" => KnowledgeDomain::Procedural,
            "epistemic" | "epistemica" => KnowledgeDomain::Epistemic,
            "self" | "se" | "sé" => KnowledgeDomain::Self_,
            "syntax" | "sintassi" => KnowledgeDomain::Syntax,
            "emotional" | "emotivo" | "emozione" => KnowledgeDomain::Emotional,
            _ => KnowledgeDomain::Dialogue,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            KnowledgeDomain::Social => "sociale",
            KnowledgeDomain::Dialogue => "dialogo",
            KnowledgeDomain::Procedural => "procedurale",
            KnowledgeDomain::Epistemic => "epistemica",
            KnowledgeDomain::Self_ => "sé",
            KnowledgeDomain::Syntax => "sintassi",
            KnowledgeDomain::Emotional => "emotivo",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// KnowledgeEntry — fatto dichiarativo/procedurale
// ═══════════════════════════════════════════════════════════════════════════

/// Una voce di conoscenza: fatto, regola, convenzione.
/// Non produce testo — informa il campo topologico quando è rilevante.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub id: u64,
    pub domain: KnowledgeDomain,
    /// La conoscenza espressa in italiano
    pub content: String,
    /// Parole che rendono questa conoscenza rilevante
    pub trigger_words: Vec<String>,
    /// Frattali associati a questa conoscenza
    pub trigger_fractals: Vec<FractalId>,
    /// Grado di certezza [0,1]
    pub confidence: f64,
    /// True se insegnata dall'utente
    pub user_taught: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// KnowledgeBase — registro di conoscenza di Prometeo
// ═══════════════════════════════════════════════════════════════════════════

/// Registro delle conoscenze dichiarative.
/// Non genera testo — ricorda cosa è stato insegnato e rileva pertinenza contestuale.
pub struct KnowledgeBase {
    pub entries: Vec<KnowledgeEntry>,
    next_id: u64,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }

    /// Registra una voce di conoscenza (dall'utente via :know).
    pub fn teach_entry(
        &mut self,
        domain: KnowledgeDomain,
        content: &str,
        triggers: Vec<String>,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(KnowledgeEntry {
            id,
            domain,
            content: content.to_string(),
            trigger_words: triggers,
            trigger_fractals: Vec::new(),
            confidence: 0.8,
            user_taught: true,
        });
    }

    /// Registra un'ancora concettuale — insegnata al boot, non dall'utente.
    ///
    /// Le ancore concettuali definiscono COSA è un atto comunicativo,
    /// non un elenco di parole che lo identificano. Funzionano tramite:
    /// - `trigger_words`: una parola-campione (non lista esaustiva)
    /// - `trigger_fractals`: la firma frattale del concetto (universale)
    ///
    /// Differenza da teach_entry:
    ///   teach_entry: "ciao" → Social (lista hardcoded)
    ///   teach_concept: "un saluto è..." → frattale ARMONIA → qualsiasi
    ///                  parola che attiva ARMONIA = saluto (emergente)
    pub fn teach_concept(
        &mut self,
        domain: KnowledgeDomain,
        content: &str,
        sample_word: &str,
        fractals: Vec<FractalId>,
    ) {
        // Non re-insegnare se il concetto è già presente (evita duplicati al restart)
        if self.entries.iter().any(|e| e.domain == domain && !e.user_taught) {
            return;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(KnowledgeEntry {
            id,
            domain,
            content: content.to_string(),
            trigger_words: vec![sample_word.to_string()],
            trigger_fractals: fractals,
            confidence: 1.0,
            user_taught: false,  // concetto di sistema, non insegnato dall'utente
        });
    }

    /// Verifica se le ancore concettuali fondamentali sono già presenti.
    pub fn has_conceptual_anchors(&self) -> bool {
        self.entries.iter().any(|e| !e.user_taught)
    }

    /// Recupera voci di conoscenza rilevanti per il contesto.
    pub fn retrieve_for_context<'a>(
        &'a self,
        input_words: &[String],
        active_fractals: &[(FractalId, f64)],
    ) -> Vec<&'a KnowledgeEntry> {
        self.entries.iter()
            .filter(|e| {
                let word_match = e.trigger_words.iter()
                    .any(|t| input_words.iter().any(|w| w == t));
                let frac_match = e.trigger_fractals.iter()
                    .any(|tf| active_fractals.iter().any(|(f, a)| f == tf && *a > 0.1));
                word_match || frac_match
            })
            .collect()
    }

    /// Recupera voci di conoscenza usando il DELTA frattale — variazione causata dall'input.
    ///
    /// `fractal_delta[i] = attivazione_post[i] - attivazione_pre[i]`
    ///
    /// Usa il delta invece del valore assoluto: così anche in campi saturi si rileva
    /// il CAMBIAMENTO specifico causato dall'input, non il rumore di fondo.
    /// Threshold: delta > 0.05 su almeno un trigger_fractal.
    pub fn retrieve_for_delta<'a>(
        &'a self,
        input_words: &[String],
        fractal_delta: &[(FractalId, f64)],
    ) -> Vec<&'a KnowledgeEntry> {
        const DELTA_THRESHOLD: f64 = 0.05;
        self.entries.iter()
            .filter(|e| {
                // Parola-campione dell'input corrisponde a un trigger_word → conferma diretta
                let word_match = e.trigger_words.iter()
                    .any(|t| input_words.iter().any(|w| w == t));
                // Il campo è cambiato nella direzione prevista dal concetto
                let delta_match = e.trigger_fractals.iter()
                    .any(|tf| fractal_delta.iter()
                        .any(|(f, d)| f == tf && *d > DELTA_THRESHOLD));
                word_match || delta_match
            })
            .collect()
    }

    /// Restituisce le parole da boostare nel campo per le voci pertinenti al contesto.
    ///
    /// Quando una voce di conoscenza è rilevante, le sue parole colorano leggermente
    /// il campo — non producono testo, ma aumentano la probabilità che concetti
    /// correlati emergano nella generazione.
    ///
    /// Il boost è intenzionalmente debole (confidence × 0.15): la conoscenza
    /// informa, non sovrascrive. Il campo resta sovrano.
    pub fn recall_words_for_context(
        &self,
        input_words: &[String],
        active_fractals: &[(FractalId, f64)],
    ) -> Vec<(String, f64)> {
        let relevant = self.retrieve_for_context(input_words, active_fractals);
        let mut boosts: Vec<(String, f64)> = Vec::new();
        for entry in relevant {
            let strength = entry.confidence * 0.15;
            for word in entry.content.split_whitespace() {
                let w: String = word.chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
                    .to_lowercase();
                if w.len() > 3 {
                    boosts.push((w, strength));
                }
            }
        }
        boosts
    }

    /// Numero di voci di conoscenza insegnate.
    pub fn entry_count(&self) -> usize { self.entries.len() }

    /// Tutte le voci di conoscenza insegnate dall'utente.
    pub fn user_entries(&self) -> Vec<&KnowledgeEntry> {
        self.entries.iter().filter(|e| e.user_taught).collect()
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════
// Snapshot per persistenza
// ═══════════════════════════════════════════════════════════════════════════

/// Snapshot serializzabile delle voci insegnate dall'utente.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct KnowledgeSnapshot {
    pub entries: Vec<KnowledgeEntry>,
    pub next_id: u64,
}

impl KnowledgeSnapshot {
    pub fn capture(kb: &KnowledgeBase) -> Self {
        Self {
            entries: kb.entries.clone(),
            next_id: kb.next_id,
        }
    }

    pub fn restore(self) -> KnowledgeBase {
        let mut kb = KnowledgeBase::new();
        kb.entries = self.entries;
        kb.next_id = self.next_id.max(1);
        kb
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Test
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teach_entry() {
        let mut kb = KnowledgeBase::new();
        kb.teach_entry(
            KnowledgeDomain::Social,
            "un saluto si ricambia con un saluto",
            vec!["saluto".to_string(), "ciao".to_string()],
        );
        assert_eq!(kb.entry_count(), 1);
        let entries = kb.retrieve_for_context(
            &["ciao".to_string()],
            &[],
        );
        assert_eq!(entries.len(), 1);
        assert!(entries[0].content.contains("saluto"));
    }

    #[test]
    fn test_recall_words_for_context() {
        let mut kb = KnowledgeBase::new();
        kb.teach_entry(
            KnowledgeDomain::Social,
            "un saluto si ricambia con un saluto",
            vec!["ciao".to_string()],
        );
        let boosts = kb.recall_words_for_context(&["ciao".to_string()], &[]);
        // Deve restituire parole con boost debole
        assert!(!boosts.is_empty(), "recall deve restituire parole quando pertinente");
        for (_, strength) in &boosts {
            assert!(*strength < 0.5, "il boost non deve dominare il campo: {}", strength);
        }
    }

    #[test]
    fn test_snapshot_roundtrip() {
        let mut kb = KnowledgeBase::new();
        kb.teach_entry(
            KnowledgeDomain::Epistemic,
            "dire non so è meglio che inventare",
            vec!["incertezza".to_string()],
        );
        let snap = KnowledgeSnapshot::capture(&kb);
        let restored = snap.restore();
        assert_eq!(restored.entry_count(), 1);
        assert_eq!(restored.entries[0].content, "dire non so è meglio che inventare");
    }

    #[test]
    fn test_no_match_returns_empty() {
        let kb = KnowledgeBase::new();
        let input_words = vec!["albero".to_string(), "pietra".to_string()];
        let active_fractals = vec![];
        let result = kb.retrieve_for_context(&input_words, &active_fractals);
        assert!(result.is_empty(), "nessuna voce deve matchare senza conoscenze registrate");
    }
}
