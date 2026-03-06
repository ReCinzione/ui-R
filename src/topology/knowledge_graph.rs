/// Knowledge Graph — Grafo di conoscenza con archi tipati.
///
/// Questo è il livello semantico che mancava: le parole non sono solo
/// punti in uno spazio 8D con co-occorrenze statistiche. Hanno relazioni
/// logiche esplicite che definiscono il loro significato.
///
/// Struttura:
///   - Nodi: concetti (parole lowercase)
///   - Archi: relazioni tipate (IS_A, HAS, DOES, PART_OF, CAUSES, ...)
///   - Doppio indice: outgoing[soggetto] + incoming[oggetto]
///
/// Separazione dal campo topologico:
///   - KG = conoscenza del mondo (fatti stabili)
///   - WordTopology = stato attuale del campo (attivazioni)
///   - In receive(): KG informa il campo con boost grounded

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::topology::relation::{RelationType, TypedEdge, EdgeSource};
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════════════════
// KgNode — informazioni aggregate su un concetto
// ═══════════════════════════════════════════════════════════════════════════

/// Target di un arco: oggetto + confidenza.
#[derive(Debug, Clone)]
pub struct KgTarget {
    pub object: String,
    pub confidence: f32,
    pub source: EdgeSource,
}

// ═══════════════════════════════════════════════════════════════════════════
// KnowledgeGraph — il grafo
// ═══════════════════════════════════════════════════════════════════════════

/// Knowledge Graph con archi logici tipati.
/// Accesso O(1) per query dirette. Query inverse (chi IS-A X?) O(k) con k=archi.
pub struct KnowledgeGraph {
    /// outgoing[soggetto][relazione] = Vec<KgTarget>
    outgoing: HashMap<String, HashMap<RelationType, Vec<KgTarget>>>,
    /// incoming[oggetto][relazione] = Vec<soggetto>
    incoming: HashMap<String, HashMap<RelationType, Vec<String>>>,
    /// Numero totale di archi
    pub edge_count: usize,
    /// Numero di nodi unici
    pub node_count: usize,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
            edge_count: 0,
            node_count: 0,
        }
    }

    // ─── Inserimento ────────────────────────────────────────────────────────

    /// Aggiunge un arco al grafo. Deduplicato per (soggetto, relazione, oggetto).
    pub fn add_edge(&mut self, edge: TypedEdge) {
        let subj = edge.subject.clone();
        let obj = edge.object.clone();
        let rel = edge.relation;

        // Conta nodi nuovi
        let is_new_subj = !self.outgoing.contains_key(&subj);
        let is_new_obj = !self.incoming.contains_key(&obj);

        // Outgoing
        let out_map = self.outgoing.entry(subj.clone()).or_default();
        let out_vec = out_map.entry(rel).or_default();

        // Deduplicazione
        if !out_vec.iter().any(|t| t.object == obj) {
            out_vec.push(KgTarget {
                object: obj.clone(),
                confidence: edge.confidence,
                source: edge.source,
            });
            self.edge_count += 1;

            // Incoming
            let in_map = self.incoming.entry(obj.clone()).or_default();
            in_map.entry(rel).or_default().push(subj.clone());

            if is_new_subj { self.node_count += 1; }
            if is_new_obj && obj != subj { self.node_count += 1; }
        }
    }

    /// Aggiunge un arco semplice (subject, rel, object) con confidenza 1.0.
    pub fn add(&mut self, subject: &str, rel: RelationType, object: &str) {
        self.add_edge(TypedEdge::new(subject, rel, object));
    }

    // ─── Query dirette ───────────────────────────────────────────────────────

    /// Tutti gli oggetti connessi a `subject` con la relazione `rel`.
    /// Es: query_objects("cane", IsA) → ["animale", "mammifero"]
    pub fn query_objects<'a>(&'a self, subject: &str, rel: RelationType) -> Vec<&'a str> {
        self.outgoing.get(subject)
            .and_then(|m| m.get(&rel))
            .map(|v| v.iter().map(|t| t.object.as_str()).collect())
            .unwrap_or_default()
    }

    /// Tutti i soggetti che hanno `rel` verso `object`.
    /// Es: query_subjects("animale", IsA) → ["cane", "gatto", "uccello", ...]
    pub fn query_subjects<'a>(&'a self, object: &str, rel: RelationType) -> Vec<&'a str> {
        self.incoming.get(object)
            .and_then(|m| m.get(&rel))
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Tutti gli archi uscenti da `subject` (qualunque relazione).
    pub fn all_outgoing(&self, subject: &str) -> Vec<(RelationType, &str, f32)> {
        match self.outgoing.get(subject) {
            None => vec![],
            Some(m) => {
                let mut result = Vec::new();
                for (rel, targets) in m {
                    for t in targets {
                        result.push((*rel, t.object.as_str(), t.confidence));
                    }
                }
                result
            }
        }
    }

    /// Il nodo esiste nel grafo?
    pub fn contains(&self, word: &str) -> bool {
        self.outgoing.contains_key(word) || self.incoming.contains_key(word)
    }

    // ─── Caricamento da TSV ──────────────────────────────────────────────────

    /// Carica triple da un file TSV.
    /// Formato: soggetto\tRELAZIONE\toggetto[\tconfidenza]
    /// Linee che iniziano con # sono commenti e vengono ignorate.
    pub fn load_from_tsv(&mut self, path: &Path) -> anyhow::Result<usize> {
        let content = fs::read_to_string(path)?;
        let mut count = 0usize;
        for (line_num, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
            match TypedEdge::from_tsv_line(trimmed) {
                Some(edge) => {
                    self.add_edge(edge);
                    count += 1;
                }
                None => {
                    // Segnala solo righe malformate non-commento
                    eprintln!("[KG] riga {} ignorata: {:?}", line_num + 1, trimmed);
                }
            }
        }
        Ok(count)
    }

    /// Carica tutti i file .tsv da una directory.
    pub fn load_from_dir(&mut self, dir: &Path) -> anyhow::Result<usize> {
        let mut total = 0usize;
        if !dir.exists() { return Ok(0); }
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("tsv") {
                match self.load_from_tsv(&path) {
                    Ok(n) => { total += n; }
                    Err(e) => eprintln!("[KG] errore caricando {:?}: {}", path, e),
                }
            }
        }
        Ok(total)
    }

    // ─── Query strutturali ───────────────────────────────────────────────────

    /// Nodi che sono target di almeno `min_children` archi di tipo `rel`.
    /// Utile per trovare categorie: `categories_for(IsA, 2)` → ["animale", "nazione", ...]
    pub fn categories_for(&self, rel: RelationType, min_children: usize) -> Vec<String> {
        self.incoming.iter()
            .filter_map(|(node, rel_map)| {
                rel_map.get(&rel)
                    .filter(|children| children.len() >= min_children)
                    .map(|_| node.clone())
            })
            .collect()
    }

    /// Nodi che hanno almeno `min_targets` archi uscenti di tipo `rel`.
    /// Utile per trovare cluster di similitudine: `nodes_with_min_outgoing(SimilarTo, 2)`.
    pub fn nodes_with_min_outgoing(&self, rel: RelationType, min_targets: usize) -> Vec<String> {
        self.outgoing.iter()
            .filter_map(|(node, rel_map)| {
                rel_map.get(&rel)
                    .filter(|targets| targets.len() >= min_targets)
                    .map(|_| node.clone())
            })
            .collect()
    }

    // ─── Serializzazione ─────────────────────────────────────────────────────

    /// Snapshot serializzabile.
    pub fn to_snapshot(&self) -> KgSnapshot {
        let mut edges = Vec::with_capacity(self.edge_count);
        for (subj, rel_map) in &self.outgoing {
            for (rel, targets) in rel_map {
                for t in targets {
                    edges.push(TypedEdge {
                        subject: subj.clone(),
                        relation: *rel,
                        object: t.object.clone(),
                        confidence: t.confidence,
                        source: t.source,
                    });
                }
            }
        }
        KgSnapshot { edges }
    }

    pub fn from_snapshot(snap: KgSnapshot) -> Self {
        let mut kg = Self::new();
        for edge in snap.edges {
            kg.add_edge(edge);
        }
        kg
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════
// KgSnapshot — persistenza
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KgSnapshot {
    pub edges: Vec<TypedEdge>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Test
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::relation::RelationType;

    fn build_test_kg() -> KnowledgeGraph {
        let mut kg = KnowledgeGraph::new();
        kg.add("cane", RelationType::IsA, "animale");
        kg.add("gatto", RelationType::IsA, "animale");
        kg.add("animale", RelationType::IsA, "essere_vivente");
        kg.add("cane", RelationType::Does, "abbaiare");
        kg.add("cane", RelationType::Has, "pelo");
        kg.add("animale", RelationType::Does, "mangiare");
        kg.add("animale", RelationType::Does, "dormire");
        kg.add("germania", RelationType::IsA, "nazione");
        kg.add("nazione", RelationType::Has, "confine");
        kg.add("nazione", RelationType::Has, "capitale");
        kg.add("caldo", RelationType::OppositeOf, "freddo");
        kg
    }

    #[test]
    fn test_add_and_query() {
        let kg = build_test_kg();
        let is_a = kg.query_objects("cane", RelationType::IsA);
        assert!(is_a.contains(&"animale"), "cane IS-A animale");
        let does = kg.query_objects("cane", RelationType::Does);
        assert!(does.contains(&"abbaiare"), "cane DOES abbaiare");
    }

    #[test]
    fn test_inverse_query() {
        let kg = build_test_kg();
        let animals = kg.query_subjects("animale", RelationType::IsA);
        assert!(animals.contains(&"cane"));
        assert!(animals.contains(&"gatto"));
    }

    #[test]
    fn test_edge_count() {
        let kg = build_test_kg();
        assert!(kg.edge_count >= 10, "deve avere almeno 10 archi: {}", kg.edge_count);
    }

    #[test]
    fn test_no_duplicate_edges() {
        let mut kg = KnowledgeGraph::new();
        kg.add("cane", RelationType::IsA, "animale");
        kg.add("cane", RelationType::IsA, "animale"); // duplicato
        assert_eq!(kg.edge_count, 1, "non deve duplicare archi");
    }

    #[test]
    fn test_all_outgoing() {
        let kg = build_test_kg();
        let out = kg.all_outgoing("cane");
        assert!(out.len() >= 3, "cane ha almeno 3 archi: IS_A, DOES, HAS");
    }

    #[test]
    fn test_contains() {
        let kg = build_test_kg();
        assert!(kg.contains("cane"));
        assert!(kg.contains("animale"));
        assert!(!kg.contains("unicorno"));
    }

    #[test]
    fn test_snapshot_roundtrip() {
        let kg = build_test_kg();
        let snap = kg.to_snapshot();
        let count = kg.edge_count;
        let restored = KnowledgeGraph::from_snapshot(snap);
        assert_eq!(restored.edge_count, count);
        let is_a = restored.query_objects("cane", RelationType::IsA);
        assert!(is_a.contains(&"animale"));
    }

    #[test]
    fn test_tsv_parse_inline() {
        let mut kg = KnowledgeGraph::new();
        // Simula lettura TSV linea per linea
        let lines = [
            "sole\tDOES\tbrillare\t1.0",
            "sole\tCAUSES\tluce",
            "# questo è un commento",
            "",
            "luna\tIS_A\tsatellite",
        ];
        for line in &lines {
            if let Some(edge) = TypedEdge::from_tsv_line(line) {
                kg.add_edge(edge);
            }
        }
        assert_eq!(kg.edge_count, 3);
        assert!(kg.query_objects("sole", RelationType::Does).contains(&"brillare"));
        assert!(kg.query_objects("sole", RelationType::Causes).contains(&"luce"));
    }
}
