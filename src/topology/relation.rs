/// Relazioni tipate — il vocabolario logico di Prometeo.
///
/// Ogni arco nel Knowledge Graph ha un tipo semantico preciso.
/// Non è co-occorrenza statistica — è relazione logica esplicita.
///
/// Analogia visiva: archi colorati nel grafo
///   Verde  = IS_A    (tassonomia)
///   Blu    = HAS     (attributi)
///   Arancio= DOES    (azioni)
///   Viola  = PART_OF (composizione)
///   Rosso  = CAUSES  (causalità)
///   Grigio = OPPOSITE_OF (antonimia)

use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════════════════
// RelationType — tipo logico dell'arco
// ═══════════════════════════════════════════════════════════════════════════

/// Tipo di relazione semantica tra due concetti.
/// Ogni tipo ha un significato logico preciso e supporta inferenze diverse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    /// X IS_A Y — X è un tipo di Y (tassonomia, ereditarietà)
    /// "cane IS_A animale", "germania IS_A nazione"
    /// Permette: X eredita tutte le proprietà di Y
    IsA,

    /// X HAS Y — X ha la proprietà/parte Y (attributo)
    /// "nazione HAS confine", "cane HAS pelo"
    Has,

    /// X DOES Y — X compie/esegue l'azione Y (comportamento)
    /// "cane DOES abbaiare", "sole DOES brillare"
    Does,

    /// X PART_OF Y — X è una parte di Y (composizione)
    /// "berlino PART_OF germania", "mano PART_OF corpo"
    PartOf,

    /// X CAUSES Y — X causa/produce Y (causalità)
    /// "fuoco CAUSES calore", "paura CAUSES tremore"
    Causes,

    /// X OPPOSITE_OF Y — X è l'opposto di Y (antonimia)
    /// "caldo OPPOSITE_OF freddo", "luce OPPOSITE_OF buio"
    OppositeOf,

    /// X SIMILAR_TO Y — X è simile a Y (sinonimia larga)
    /// "ciao SIMILAR_TO saluto", "camminare SIMILAR_TO muoversi"
    SimilarTo,

    /// X USED_FOR Y — X è usato per Y (funzione)
    /// "coltello USED_FOR tagliare", "libro USED_FOR leggere"
    UsedFor,
}

impl RelationType {
    /// Parsing da stringa TSV (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "IS_A" | "ISA" | "È" | "E_UN" => Some(Self::IsA),
            "HAS" | "HA" | "HAS_PROPERTY" => Some(Self::Has),
            "DOES" | "FA" | "DOES_ACTION" => Some(Self::Does),
            "PART_OF" | "PARTOF" | "PARTE_DI" => Some(Self::PartOf),
            "CAUSES" | "CAUSA" | "CAUSES_EFFECT" => Some(Self::Causes),
            "OPPOSITE_OF" | "OPPOSITEOF" | "OPPOSTO_DI" => Some(Self::OppositeOf),
            "SIMILAR_TO" | "SIMILARTO" | "SIMILE_A" => Some(Self::SimilarTo),
            "USED_FOR" | "USEDFOR" | "USATO_PER" => Some(Self::UsedFor),
            _ => None,
        }
    }

    /// Rappresentazione leggibile.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IsA => "IS_A",
            Self::Has => "HAS",
            Self::Does => "DOES",
            Self::PartOf => "PART_OF",
            Self::Causes => "CAUSES",
            Self::OppositeOf => "OPPOSITE_OF",
            Self::SimilarTo => "SIMILAR_TO",
            Self::UsedFor => "USED_FOR",
        }
    }

    /// Forza di boost nel campo topologico per questa relazione.
    /// IS_A è la più forte (definisce cosa è una cosa).
    /// OPPOSITE_OF è la più debole (crea contrasto, non risonanza).
    pub fn field_boost_strength(&self) -> f32 {
        match self {
            Self::IsA => 0.18,
            Self::Has => 0.14,
            Self::Does => 0.14,
            Self::PartOf => 0.12,
            Self::Causes => 0.12,
            Self::SimilarTo => 0.16,
            Self::UsedFor => 0.10,
            Self::OppositeOf => 0.06, // bassa: crea attrito, non risonanza
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// EdgeSource — origine della relazione
// ═══════════════════════════════════════════════════════════════════════════

/// Da dove viene questa relazione.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeSource {
    /// Estratto da Wikidata (entità/categorie italiane)
    Wikidata,
    /// Da WordNet italiano (sinonimi, iperonimi, antonimi)
    Wordnet,
    /// Ontologia curata a mano (core italiana)
    Curated,
    /// Insegnata dall'utente con `:know`
    UserTaught,
    /// Derivata per inferenza transitiva
    Inferred,
}

// ═══════════════════════════════════════════════════════════════════════════
// TypedEdge — un arco logico tra due concetti
// ═══════════════════════════════════════════════════════════════════════════

/// Un arco logico tipato nel Knowledge Graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedEdge {
    /// Il soggetto (da chi parte la relazione)
    pub subject: String,
    /// Il tipo di relazione
    pub relation: RelationType,
    /// L'oggetto (dove arriva la relazione)
    pub object: String,
    /// Grado di certezza [0.0, 1.0]
    pub confidence: f32,
    /// Origine della relazione
    pub source: EdgeSource,
}

impl TypedEdge {
    pub fn new(subject: &str, relation: RelationType, object: &str) -> Self {
        Self {
            subject: subject.to_lowercase(),
            relation,
            object: object.to_lowercase(),
            confidence: 1.0,
            source: EdgeSource::Curated,
        }
    }

    pub fn with_confidence(mut self, c: f32) -> Self {
        self.confidence = c;
        self
    }

    pub fn with_source(mut self, s: EdgeSource) -> Self {
        self.source = s;
        self
    }

    /// Parsa una riga TSV: "soggetto\tRELAZIONE\toggetto[\tconfidenza]"
    pub fn from_tsv_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 { return None; }
        let subject = parts[0].trim().to_lowercase();
        let rel_str = parts[1].trim();
        let object = parts[2].trim().to_lowercase();
        if subject.is_empty() || object.is_empty() { return None; }
        let relation = RelationType::from_str(rel_str)?;
        let confidence = parts.get(3)
            .and_then(|s| s.trim().parse::<f32>().ok())
            .unwrap_or(1.0);
        Some(Self {
            subject,
            relation,
            object,
            confidence,
            source: EdgeSource::Curated,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Test
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relation_from_str() {
        assert_eq!(RelationType::from_str("IS_A"), Some(RelationType::IsA));
        assert_eq!(RelationType::from_str("isa"), Some(RelationType::IsA));
        assert_eq!(RelationType::from_str("CAUSES"), Some(RelationType::Causes));
        assert_eq!(RelationType::from_str("sconosciuto"), None);
    }

    #[test]
    fn test_tsv_parse() {
        let edge = TypedEdge::from_tsv_line("cane\tIS_A\tanimale\t1.0").unwrap();
        assert_eq!(edge.subject, "cane");
        assert_eq!(edge.relation, RelationType::IsA);
        assert_eq!(edge.object, "animale");
        assert_eq!(edge.confidence, 1.0);
    }

    #[test]
    fn test_tsv_parse_no_confidence() {
        let edge = TypedEdge::from_tsv_line("sole\tDOES\tbrillare").unwrap();
        assert_eq!(edge.subject, "sole");
        assert_eq!(edge.relation, RelationType::Does);
        assert_eq!(edge.confidence, 1.0);
    }

    #[test]
    fn test_tsv_invalid() {
        assert!(TypedEdge::from_tsv_line("solo_campo").is_none());
        assert!(TypedEdge::from_tsv_line("a\tREL_SCONOSCIUTA\tb").is_none());
    }
}
