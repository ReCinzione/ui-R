/// Navigazione Geodetica — Cammini minimi nel complesso simpliciale.
///
/// Il complesso simpliciale e un grafo dove i nodi sono frattali e
/// gli archi sono i simplessi che li connettono. La navigazione
/// geodetica trova il cammino piu breve (o il piu "facile") tra
/// due concetti — attraversando le connessioni topologiche.
///
/// Il costo di un arco e l'inverso della forza di connessione:
/// regioni dense e attive sono "vicine", regioni sparse e spente
/// sono "lontane".
///
/// Applicazioni:
/// - Analogie: "rosso sta a colore come tristezza sta a emozione"
///   = stessa struttura geodetica in regioni diverse
/// - Associazioni: "da dove arrivo a questo concetto?"
/// - Distanza semantica: quanto sono lontani due concetti?

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::SimplicialComplex;

/// Un passo nel cammino geodetico.
#[derive(Debug, Clone)]
pub struct GeodesicStep {
    /// Il frattale raggiunto
    pub fractal_id: FractalId,
    /// Nome del frattale
    pub fractal_name: String,
    /// Il simplesso attraversato per arrivarci
    pub via_simplex: Option<u32>,
    /// Strutture condivise che hanno permesso il passaggio
    pub shared_structures: Vec<String>,
    /// Costo cumulativo fino a qui
    pub cumulative_cost: f64,
}

/// Un cammino geodetico completo tra due frattali.
#[derive(Debug, Clone)]
pub struct GeodesicPath {
    /// Da dove parte
    pub from: FractalId,
    /// Dove arriva
    pub to: FractalId,
    /// I passi del cammino
    pub steps: Vec<GeodesicStep>,
    /// Costo totale del cammino
    pub total_cost: f64,
    /// Profondita massima (dimensione massima dei simplessi attraversati)
    pub max_depth: usize,
    /// Spiegazione testuale
    pub explanation: String,
}

/// Un'analogia topologica: due cammini con struttura simile.
#[derive(Debug, Clone)]
pub struct TopologicalAnalogy {
    /// Il primo cammino (la "base" dell'analogia)
    pub base_path: GeodesicPath,
    /// Il secondo cammino (il "target")
    pub target_path: GeodesicPath,
    /// Similitudine strutturale [0.0, 1.0]
    pub structural_similarity: f64,
    /// Le strutture condivise in comune nei due cammini
    pub shared_bridge_types: Vec<String>,
    /// Spiegazione testuale
    pub explanation: String,
}

/// Nodo per il priority queue di Dijkstra.
#[derive(Debug, Clone)]
struct DijkstraNode {
    fractal_id: FractalId,
    cost: f64,
}

impl PartialEq for DijkstraNode {
    fn eq(&self, other: &Self) -> bool {
        self.fractal_id == other.fractal_id
    }
}

impl Eq for DijkstraNode {}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Invertito: BinaryHeap e un max-heap, noi vogliamo il minimo
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

/// Calcola il costo di attraversare un simplesso tra due frattali.
/// Costo basso = connessione forte, attiva, profonda.
/// Costo alto = connessione debole, spenta, superficiale.
fn edge_cost(
    complex: &SimplicialComplex,
    from: FractalId,
    to: FractalId,
) -> Option<(f64, u32, Vec<String>)> {
    let shared = complex.shared_simplices(from, to);
    if shared.is_empty() {
        return None;
    }

    // Tra tutti i simplessi condivisi, prendi quello col costo minore
    let mut best_cost = f64::MAX;
    let mut best_simplex = 0u32;
    let mut best_structures = Vec::new();

    for sid in &shared {
        if let Some(simplex) = complex.get(*sid) {
            // Il costo e l'inverso della "facilita" di attraversamento:
            // - connection_strength alta → costo basso
            // - persistence alta → costo basso (connessione stabile)
            // - dimensione alta → costo basso (connessione profonda)
            // - attivazione alta → costo basso (connessione "illuminata")
            let strength = simplex.connection_strength().max(0.01);
            let persistence = simplex.persistence.max(0.01);
            let dim_bonus = 1.0 + simplex.dimension as f64 * 0.3;
            let activation_bonus = 1.0 + simplex.current_activation * 2.0;

            let cost = 1.0 / (strength * persistence * dim_bonus * activation_bonus);

            if cost < best_cost {
                best_cost = cost;
                best_simplex = *sid;
                best_structures = simplex.shared_faces.iter().map(|f| {
                    match &f.structure {
                        crate::topology::simplex::SharedStructureType::PrimitiveDim(dim) => {
                            format!("{:?}", dim)
                        }
                        crate::topology::simplex::SharedStructureType::EmergentProperty(name) => {
                            name.clone()
                        }
                        crate::topology::simplex::SharedStructureType::CovariationPattern { description, .. } => {
                            description.clone()
                        }
                    }
                }).collect();
            }
        }
    }

    if best_cost < f64::MAX {
        Some((best_cost, best_simplex, best_structures))
    } else {
        None
    }
}

/// Trova tutti i vicini topologici di un frattale nel complesso.
/// Un vicino e un frattale connesso direttamente via almeno un simplesso.
fn neighbors(complex: &SimplicialComplex, fractal: FractalId) -> Vec<FractalId> {
    let mut result = HashSet::new();
    for sid in complex.simplices_of(fractal) {
        if let Some(simplex) = complex.get(sid) {
            for &v in &simplex.vertices {
                if v != fractal {
                    result.insert(v);
                }
            }
        }
    }
    result.into_iter().collect()
}

/// Trova il cammino geodetico piu breve tra due frattali.
/// Usa Dijkstra con costo inversamente proporzionale alla forza di connessione.
/// Restituisce None se non esiste un cammino.
pub fn find_geodesic(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    from: FractalId,
    to: FractalId,
) -> Option<GeodesicPath> {
    if from == to {
        let name = registry.get(from).map(|f| f.name.clone()).unwrap_or_default();
        return Some(GeodesicPath {
            from,
            to,
            steps: vec![GeodesicStep {
                fractal_id: from,
                fractal_name: name,
                via_simplex: None,
                shared_structures: vec![],
                cumulative_cost: 0.0,
            }],
            total_cost: 0.0,
            max_depth: 0,
            explanation: "Stesso concetto.".to_string(),
        });
    }

    // Dijkstra
    let mut dist: HashMap<FractalId, f64> = HashMap::new();
    let mut prev: HashMap<FractalId, (FractalId, u32, Vec<String>)> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(from, 0.0);
    heap.push(DijkstraNode { fractal_id: from, cost: 0.0 });

    while let Some(DijkstraNode { fractal_id: current, cost }) = heap.pop() {
        if current == to {
            break;
        }

        // Se abbiamo gia trovato un cammino migliore, skip
        if let Some(&best) = dist.get(&current) {
            if cost > best {
                continue;
            }
        }

        for neighbor in neighbors(complex, current) {
            if let Some((edge, simplex_id, structures)) = edge_cost(complex, current, neighbor) {
                let new_cost = cost + edge;
                let is_better = dist.get(&neighbor).map(|&d| new_cost < d).unwrap_or(true);

                if is_better {
                    dist.insert(neighbor, new_cost);
                    prev.insert(neighbor, (current, simplex_id, structures));
                    heap.push(DijkstraNode { fractal_id: neighbor, cost: new_cost });
                }
            }
        }
    }

    // Ricostruisci il cammino
    if !dist.contains_key(&to) {
        return None; // Nessun cammino
    }

    let mut path_nodes = vec![to];
    let mut current = to;
    while current != from {
        if let Some((prev_node, _, _)) = prev.get(&current) {
            path_nodes.push(*prev_node);
            current = *prev_node;
        } else {
            return None; // Cammino rotto (non dovrebbe succedere)
        }
    }
    path_nodes.reverse();

    // Costruisci i passi
    let mut steps = Vec::new();
    let mut max_depth = 0usize;

    for (i, &fid) in path_nodes.iter().enumerate() {
        let name = registry.get(fid).map(|f| f.name.clone()).unwrap_or_else(|| format!("#{}", fid));
        let cumulative = dist.get(&fid).copied().unwrap_or(0.0);

        let (via, structures) = if i == 0 {
            (None, vec![])
        } else {
            prev.get(&fid).map(|(_, sid, structs)| {
                if let Some(s) = complex.get(*sid) {
                    max_depth = max_depth.max(s.dimension);
                }
                (Some(*sid), structs.clone())
            }).unwrap_or((None, vec![]))
        };

        steps.push(GeodesicStep {
            fractal_id: fid,
            fractal_name: name,
            via_simplex: via,
            shared_structures: structures,
            cumulative_cost: cumulative,
        });
    }

    let total_cost = dist.get(&to).copied().unwrap_or(0.0);
    let explanation = generate_path_explanation(&steps);

    Some(GeodesicPath {
        from,
        to,
        steps,
        total_cost,
        max_depth,
        explanation,
    })
}

/// Trova la distanza geodetica tra due frattali (solo il costo, senza il cammino).
pub fn geodesic_distance(
    complex: &SimplicialComplex,
    from: FractalId,
    to: FractalId,
) -> Option<f64> {
    if from == to {
        return Some(0.0);
    }

    // Dijkstra semplificato (senza ricostruire il cammino)
    let mut dist: HashMap<FractalId, f64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(from, 0.0);
    heap.push(DijkstraNode { fractal_id: from, cost: 0.0 });

    while let Some(DijkstraNode { fractal_id: current, cost }) = heap.pop() {
        if current == to {
            return Some(cost);
        }

        if let Some(&best) = dist.get(&current) {
            if cost > best {
                continue;
            }
        }

        for neighbor in neighbors(complex, current) {
            if let Some((edge, _, _)) = edge_cost(complex, current, neighbor) {
                let new_cost = cost + edge;
                let is_better = dist.get(&neighbor).map(|&d| new_cost < d).unwrap_or(true);

                if is_better {
                    dist.insert(neighbor, new_cost);
                    heap.push(DijkstraNode { fractal_id: neighbor, cost: new_cost });
                }
            }
        }
    }

    None
}

/// Calcola la mappa delle distanze da un frattale a tutti gli altri raggiungibili.
pub fn distance_map(
    complex: &SimplicialComplex,
    from: FractalId,
) -> HashMap<FractalId, f64> {
    let mut dist: HashMap<FractalId, f64> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(from, 0.0);
    heap.push(DijkstraNode { fractal_id: from, cost: 0.0 });

    while let Some(DijkstraNode { fractal_id: current, cost }) = heap.pop() {
        if let Some(&best) = dist.get(&current) {
            if cost > best {
                continue;
            }
        }

        for neighbor in neighbors(complex, current) {
            if let Some((edge, _, _)) = edge_cost(complex, current, neighbor) {
                let new_cost = cost + edge;
                let is_better = dist.get(&neighbor).map(|&d| new_cost < d).unwrap_or(true);

                if is_better {
                    dist.insert(neighbor, new_cost);
                    heap.push(DijkstraNode { fractal_id: neighbor, cost: new_cost });
                }
            }
        }
    }

    dist
}

/// Cerca un'analogia topologica: due cammini con struttura simile.
///
/// "A sta a B come C sta a D" = il cammino A→B ha la stessa struttura del cammino C→D.
/// La struttura e definita dalla sequenza di tipi di ponti attraversati.
pub fn find_analogy(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    a: FractalId,
    b: FractalId,
    c: FractalId,
) -> Option<TopologicalAnalogy> {
    let base_path = find_geodesic(complex, registry, a, b)?;

    // Estrai la "struttura" del cammino base: i tipi di ponti
    let base_bridge_types: Vec<&str> = base_path.steps.iter()
        .flat_map(|s| s.shared_structures.iter().map(|st| st.as_str()))
        .collect();

    // Trova il frattale D piu simile: il vicino di C la cui geodetica C→D
    // ha la struttura piu simile a A→B
    let c_neighbors = neighbors(complex, c);
    let mut best_analogy: Option<TopologicalAnalogy> = None;
    let mut best_similarity = 0.0f64;

    // Cerca tra tutti i frattali raggiungibili da C (non solo vicini diretti)
    let c_distances = distance_map(complex, c);

    for (&candidate, _) in &c_distances {
        if candidate == c || candidate == a || candidate == b {
            continue;
        }

        if let Some(target_path) = find_geodesic(complex, registry, c, candidate) {
            let similarity = path_structural_similarity(&base_path, &target_path);

            if similarity > best_similarity && similarity > 0.3 {
                let shared_types = find_shared_bridge_types(&base_path, &target_path);
                let explanation = generate_analogy_explanation(
                    &base_path, &target_path, similarity, &shared_types
                );

                best_similarity = similarity;
                best_analogy = Some(TopologicalAnalogy {
                    base_path: base_path.clone(),
                    target_path,
                    structural_similarity: similarity,
                    shared_bridge_types: shared_types,
                    explanation,
                });
            }
        }
    }

    best_analogy
}

/// Similitudine strutturale tra due cammini geodetici.
/// Confronta: lunghezza, tipi di ponti, profondita.
fn path_structural_similarity(a: &GeodesicPath, b: &GeodesicPath) -> f64 {
    if a.steps.is_empty() || b.steps.is_empty() {
        return 0.0;
    }

    // 1. Similitudine di lunghezza (stessi hop)
    let len_a = a.steps.len() as f64;
    let len_b = b.steps.len() as f64;
    let len_sim = 1.0 - (len_a - len_b).abs() / (len_a + len_b).max(1.0);

    // 2. Similitudine di profondita (stessa dimensione massima)
    let depth_sim = if a.max_depth == b.max_depth { 1.0 }
    else { 1.0 / (1.0 + (a.max_depth as f64 - b.max_depth as f64).abs()) };

    // 3. Similitudine di tipi di ponte
    let a_types: HashSet<&str> = a.steps.iter()
        .flat_map(|s| s.shared_structures.iter().map(|st| st.as_str()))
        .collect();
    let b_types: HashSet<&str> = b.steps.iter()
        .flat_map(|s| s.shared_structures.iter().map(|st| st.as_str()))
        .collect();

    let bridge_sim = if a_types.is_empty() && b_types.is_empty() {
        0.5
    } else {
        let intersection = a_types.intersection(&b_types).count();
        let union = a_types.union(&b_types).count();
        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    };

    // Media pesata
    len_sim * 0.3 + depth_sim * 0.2 + bridge_sim * 0.5
}

/// Trova i tipi di ponte condivisi tra due cammini.
fn find_shared_bridge_types(a: &GeodesicPath, b: &GeodesicPath) -> Vec<String> {
    let a_types: HashSet<&str> = a.steps.iter()
        .flat_map(|s| s.shared_structures.iter().map(|st| st.as_str()))
        .collect();
    let b_types: HashSet<&str> = b.steps.iter()
        .flat_map(|s| s.shared_structures.iter().map(|st| st.as_str()))
        .collect();

    a_types.intersection(&b_types).map(|s| s.to_string()).collect()
}

/// Genera spiegazione testuale del cammino.
fn generate_path_explanation(steps: &[GeodesicStep]) -> String {
    if steps.len() <= 1 {
        return "Punto di partenza.".to_string();
    }

    let names: Vec<&str> = steps.iter().map(|s| s.fractal_name.as_str()).collect();
    let mut parts = vec![format!("Da {} a {}", names[0], names[names.len() - 1])];

    if steps.len() > 2 {
        let intermediates: Vec<&str> = names[1..names.len()-1].iter().copied().collect();
        parts.push(format!("passando per {}", intermediates.join(", ")));
    }

    // Strutture attraversate
    let all_structures: Vec<&str> = steps.iter()
        .flat_map(|s| s.shared_structures.iter().map(|st| st.as_str()))
        .collect();
    let unique: HashSet<&str> = all_structures.into_iter().collect();
    if !unique.is_empty() {
        let structs: Vec<&str> = unique.into_iter().take(4).collect();
        parts.push(format!("attraverso {}", structs.join(", ")));
    }

    parts.join(", ") + "."
}

/// Genera spiegazione dell'analogia.
fn generate_analogy_explanation(
    base: &GeodesicPath,
    target: &GeodesicPath,
    similarity: f64,
    shared: &[String],
) -> String {
    let base_from = base.steps.first().map(|s| s.fractal_name.as_str()).unwrap_or("?");
    let base_to = base.steps.last().map(|s| s.fractal_name.as_str()).unwrap_or("?");
    let target_from = target.steps.first().map(|s| s.fractal_name.as_str()).unwrap_or("?");
    let target_to = target.steps.last().map(|s| s.fractal_name.as_str()).unwrap_or("?");

    let strength = if similarity > 0.7 { "forte" }
    else if similarity > 0.5 { "moderata" }
    else { "debole" };

    let mut text = format!(
        "{} sta a {} come {} sta a {} (analogia {}, sim={:.2})",
        base_from, base_to, target_from, target_to, strength, similarity
    );

    if !shared.is_empty() {
        text.push_str(&format!(". Entrambi attraversano: {}", shared.join(", ")));
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::bootstrap_complex;

    fn setup() -> (SimplicialComplex, FractalRegistry) {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);
        (complex, reg)
    }

    #[test]
    fn test_geodesic_same_node() {
        let (complex, registry) = setup();
        let path = find_geodesic(&complex, &registry, 0, 0).unwrap();
        assert_eq!(path.total_cost, 0.0);
        assert_eq!(path.steps.len(), 1);
    }

    #[test]
    fn test_geodesic_direct_neighbors() {
        let (complex, registry) = setup();
        // POTERE (0) e TERRA (9) sono connessi direttamente nel ring
        let path = find_geodesic(&complex, &registry, 0, 9).unwrap();
        assert_eq!(path.steps.len(), 2, "POTERE→TERRA e diretto: {:?}",
            path.steps.iter().map(|s| &s.fractal_name).collect::<Vec<_>>());
        assert!(path.total_cost > 0.0);
        assert!(!path.explanation.is_empty());
    }

    #[test]
    fn test_geodesic_indirect() {
        let (complex, registry) = setup();
        // POTERE (0) e SPAZIO (36) — non connessi direttamente nel ring
        // devono passare per TERRA(9)→IMPULSO(18)→DIVENIRE(27)→SPAZIO(36)
        let path = find_geodesic(&complex, &registry, 0, 36);
        assert!(path.is_some(), "Deve esistere un cammino POTERE→SPAZIO");
        let path = path.unwrap();
        assert!(path.steps.len() >= 2, "Il cammino ha almeno 2 passi");
        // Stampa il cammino per debug
        let names: Vec<&str> = path.steps.iter().map(|s| s.fractal_name.as_str()).collect();
        assert!(names.first() == Some(&"POTERE"), "Parte da POTERE");
        assert!(names.last() == Some(&"SPAZIO"), "Arriva a SPAZIO");
    }

    #[test]
    fn test_geodesic_distance() {
        let (complex, _) = setup();
        let d_self = geodesic_distance(&complex, 0, 0);
        assert_eq!(d_self, Some(0.0));

        let d_neighbors = geodesic_distance(&complex, 0, 9); // POTERE-TERRA (ring-adjacent)
        assert!(d_neighbors.is_some());
        assert!(d_neighbors.unwrap() > 0.0);
    }

    #[test]
    fn test_distance_map() {
        let (complex, _) = setup();
        let dmap = distance_map(&complex, 0);
        // POTERE(0) deve raggiungere i vicini nel ring: TERRA(9) e ARMONIA(63)
        assert!(dmap.contains_key(&0));
        assert!(dmap.contains_key(&9), "POTERE deve raggiungere TERRA(9)");
        assert!(dmap.contains_key(&63), "POTERE deve raggiungere ARMONIA(63)");
        assert_eq!(*dmap.get(&0).unwrap(), 0.0, "Distanza da se = 0");
    }

    #[test]
    fn test_activation_affects_cost() {
        let (mut complex, registry) = setup();

        // Cammino senza attivazione
        let path_cold = find_geodesic(&complex, &registry, 0, 36).unwrap();

        // Attiva TERRA(9) — nodo intermedio nel cammino POTERE(0)→TERRA(9)→...→SPAZIO(36)
        complex.activate_region(9, 0.9);
        complex.propagate_activation(2);

        let path_hot = find_geodesic(&complex, &registry, 0, 36).unwrap();

        // Il cammino "caldo" deve costare meno o uguale
        assert!(path_hot.total_cost <= path_cold.total_cost,
            "Attivazione deve ridurre il costo: cold={:.3} hot={:.3}",
            path_cold.total_cost, path_hot.total_cost);
    }

    #[test]
    fn test_analogy() {
        let (complex, registry) = setup();
        // Cerca analogia: SPAZIO sta a TEMPO come ? sta a ?
        let analogy = find_analogy(&complex, &registry, 0, 9, 18); // POTERE:TERRA come ?:IMPULSO
        // Potrebbe non trovare un'analogia forte, ma non deve crashare
        if let Some(a) = &analogy {
            assert!(a.structural_similarity > 0.0);
            assert!(!a.explanation.is_empty());
        }
    }

    #[test]
    fn test_neighbors() {
        let (complex, _) = setup();
        let n = neighbors(&complex, 0); // Vicini di POTERE nel ring
        assert!(!n.is_empty(), "POTERE deve avere vicini");
        assert!(n.contains(&9), "POTERE deve essere vicino a MATERIA");
        assert!(n.contains(&63), "POTERE deve essere vicino a ARMONIA");
    }
}
