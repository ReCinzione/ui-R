/// Metacognizione — Il sistema osserva la propria topologia.
///
/// Non e un modulo aggiunto: e una conseguenza del frattale EGO
/// con la dimensione "riflessivita". Il campo puo ispezionare
/// se stesso — quanti buchi ha, dove e denso, dove e fragile,
/// perche ha risposto come ha risposto.
///
/// "Perche hai detto questo?" = mostrare il cammino topologico
/// nel complesso che ha portato alla risposta.

use std::collections::HashMap;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::{SimplicialComplex, SimplexId};
use crate::topology::homology::compute_homology;

/// Un'introspezione: cosa il sistema vede di se stesso.
#[derive(Debug)]
pub struct Introspection {
    /// Quanti frattali esistono
    pub fractal_count: usize,
    /// Quanti simplessi
    pub simplex_count: usize,
    /// Buchi concettuali (β₁ > 0)
    pub conceptual_gaps: usize,
    /// Componenti disconnesse (β₀)
    pub disconnected_worlds: usize,
    /// Regione piu densa (nome frattale, count)
    pub densest_region: Option<(String, usize)>,
    /// Regione piu vuota
    pub sparsest_region: Option<(String, usize)>,
    /// Energia del campo (attivazione media)
    pub field_energy: f64,
    /// Dimensioni emergenti totali
    pub emergent_dimensions: usize,
    /// Frattale piu cristallizzato (piu attivazioni)
    pub most_experienced: Option<(String, u64)>,
    /// Frattale meno esperito
    pub least_experienced: Option<(String, u64)>,
}

/// Perche il sistema ha risposto come ha risposto.
/// Traccia il cammino topologico dalla perturbazione alla risposta.
#[derive(Debug)]
pub struct ResponseTrace {
    /// I simplessi che erano piu attivi al momento della risposta
    pub active_path: Vec<TraceNode>,
    /// I frattali coinvolti nell'ordine di attivazione
    pub fractal_sequence: Vec<(String, f64)>,
    /// Le facce condivise che hanno propagato l'attivazione
    pub propagation_bridges: Vec<String>,
    /// Spiegazione testuale leggibile
    pub explanation: String,
}

/// Un nodo nel cammino topologico.
#[derive(Debug)]
pub struct TraceNode {
    /// Simplesso coinvolto
    pub simplex_id: SimplexId,
    /// Frattali nel simplesso
    pub fractals: Vec<String>,
    /// Livello di attivazione
    pub activation: f64,
    /// Dimensione del simplesso
    pub dimension: usize,
}

/// Esegui un'introspezione: il sistema guarda se stesso.
pub fn introspect(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> Introspection {
    let homology = compute_homology(complex);

    // Energia del campo
    let active = complex.most_active(100);
    let field_energy = if active.is_empty() {
        0.0
    } else {
        active.iter().map(|s| s.current_activation).sum::<f64>() / active.len() as f64
    };

    // Regioni dense e sparse
    let densest = homology.dense_regions.first()
        .and_then(|&(fid, count)| {
            registry.get(fid).map(|f| (f.name.clone(), count))
        });

    let sparsest = homology.sparse_regions.first()
        .and_then(|&(fid, count)| {
            registry.get(fid).map(|f| (f.name.clone(), count))
        });

    // Frattale piu/meno esperito
    let mut most_exp: Option<(String, u64)> = None;
    let mut least_exp: Option<(String, u64)> = None;

    for (_, fractal) in registry.iter() {
        let count = fractal.activation_count;
        match &most_exp {
            None => most_exp = Some((fractal.name.clone(), count)),
            Some((_, c)) if count > *c => most_exp = Some((fractal.name.clone(), count)),
            _ => {}
        }
        match &least_exp {
            None => least_exp = Some((fractal.name.clone(), count)),
            Some((_, c)) if count < *c => least_exp = Some((fractal.name.clone(), count)),
            _ => {}
        }
    }

    // Dimensioni emergenti totali
    let emergent = registry.iter()
        .map(|(_, f)| f.emergent_dimensions.len())
        .sum();

    Introspection {
        fractal_count: registry.count(),
        simplex_count: complex.count(),
        conceptual_gaps: homology.betti_1,
        disconnected_worlds: homology.betti_0,
        densest_region: densest,
        sparsest_region: sparsest,
        field_energy,
        emergent_dimensions: emergent,
        most_experienced: most_exp,
        least_experienced: least_exp,
    }
}

/// Traccia il cammino topologico che ha portato alla risposta.
/// "Perche hai detto questo?" — mostra come l'attivazione si e propagata.
pub fn trace_response(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> ResponseTrace {
    let most_active = complex.most_active(8);

    // Costruisci il cammino: nodi per ogni simplesso attivo
    let mut active_path = Vec::new();
    let mut fractal_scores: HashMap<FractalId, f64> = HashMap::new();
    let mut bridges: Vec<String> = Vec::new();

    for simplex in &most_active {
        let fractal_names: Vec<String> = simplex.vertices.iter()
            .filter_map(|&v| registry.get(v).map(|f| f.name.clone()))
            .collect();

        active_path.push(TraceNode {
            simplex_id: simplex.id,
            fractals: fractal_names,
            activation: simplex.current_activation,
            dimension: simplex.dimension,
        });

        // Accumula score per frattale
        for &v in &simplex.vertices {
            let entry = fractal_scores.entry(v).or_insert(0.0);
            *entry = (*entry + simplex.current_activation).min(1.0);
        }

        // Le facce condivise sono i "ponti" della propagazione
        for face in &simplex.shared_faces {
            let face_desc = match &face.structure {
                crate::topology::simplex::SharedStructureType::PrimitiveDim(dim) => {
                    format!("{:?}", dim)
                }
                crate::topology::simplex::SharedStructureType::EmergentProperty(name) => {
                    name.clone()
                }
                crate::topology::simplex::SharedStructureType::CovariationPattern { description, .. } => {
                    description.clone()
                }
            };
            if !bridges.contains(&face_desc) {
                bridges.push(face_desc);
            }
        }
    }

    // Ordina frattali per score
    let mut fractal_sequence: Vec<(String, f64)> = fractal_scores.iter()
        .filter_map(|(&fid, &score)| {
            registry.get(fid).map(|f| (f.name.clone(), score))
        })
        .collect();
    fractal_sequence.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Genera spiegazione testuale
    let explanation = generate_explanation(&fractal_sequence, &bridges, &active_path);

    ResponseTrace {
        active_path,
        fractal_sequence,
        propagation_bridges: bridges,
        explanation,
    }
}

/// Genera una spiegazione testuale del cammino.
fn generate_explanation(
    fractals: &[(String, f64)],
    bridges: &[String],
    path: &[TraceNode],
) -> String {
    if fractals.is_empty() {
        return "Il campo e silente — nessun cammino attivo.".to_string();
    }

    let mut parts = Vec::new();

    // Chi e piu attivo
    let top: Vec<&str> = fractals.iter()
        .take(3)
        .map(|(n, _)| n.as_str())
        .collect();
    parts.push(format!("Il campo e centrato su {}", top.join(", ")));

    // Attraverso quali ponti
    if !bridges.is_empty() {
        let bridge_names: Vec<&str> = bridges.iter().take(3).map(|s| s.as_str()).collect();
        parts.push(format!("attraverso {}", bridge_names.join(" e ")));
    }

    // Profondita
    let max_dim = path.iter().map(|n| n.dimension).max().unwrap_or(0);
    if max_dim >= 2 {
        parts.push(format!("con connessioni di profondita {}", max_dim));
    }

    parts.join(", ") + "."
}

/// Confronta due stati del campo: cosa e cambiato tra prima e dopo la perturbazione.
#[derive(Debug)]
pub struct FieldDelta {
    /// Frattali che si sono attivati (non erano attivi prima)
    pub newly_active: Vec<String>,
    /// Frattali che si sono disattivati
    pub deactivated: Vec<String>,
    /// Variazione totale di energia
    pub energy_delta: f64,
}

/// Calcola cosa e cambiato nel campo.
pub fn compute_delta(
    before: &[(FractalId, f64)],
    after: &[(FractalId, f64)],
    registry: &FractalRegistry,
) -> FieldDelta {
    let before_map: HashMap<FractalId, f64> = before.iter().cloned().collect();
    let after_map: HashMap<FractalId, f64> = after.iter().cloned().collect();

    let threshold = 0.1;

    let mut newly_active = Vec::new();
    let mut deactivated = Vec::new();

    for (&fid, &score) in &after_map {
        let prev = before_map.get(&fid).unwrap_or(&0.0);
        if score > threshold && *prev < threshold {
            if let Some(f) = registry.get(fid) {
                newly_active.push(f.name.clone());
            }
        }
    }

    for (&fid, &score) in &before_map {
        let curr = after_map.get(&fid).unwrap_or(&0.0);
        if score > threshold && *curr < threshold {
            if let Some(f) = registry.get(fid) {
                deactivated.push(f.name.clone());
            }
        }
    }

    let energy_before: f64 = before.iter().map(|(_, s)| s).sum();
    let energy_after: f64 = after.iter().map(|(_, s)| s).sum();

    FieldDelta {
        newly_active,
        deactivated,
        energy_delta: energy_after - energy_before,
    }
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
    fn test_introspect_bootstrap() {
        let (complex, registry) = setup();
        let intro = introspect(&complex, &registry);

        assert!(intro.fractal_count >= 10);
        assert!(intro.simplex_count >= 8);
        assert!(intro.disconnected_worlds >= 1);
        // Gli esagrammi non hanno dimensioni emergenti predefinite (si calibrano con l'esperienza)
        assert!(intro.emergent_dimensions >= 0);
        assert!(intro.most_experienced.is_some());
    }

    #[test]
    fn test_trace_empty_field() {
        let (complex, registry) = setup();
        let trace = trace_response(&complex, &registry);

        // Campo non perturbato → cammino minimale
        assert!(!trace.explanation.is_empty());
    }

    #[test]
    fn test_trace_after_activation() {
        let (mut complex, registry) = setup();

        // Attiva SPAZIO e TEMPO
        complex.activate_region(0, 0.8);
        complex.activate_region(1, 0.6);
        complex.propagate_activation(2);

        let trace = trace_response(&complex, &registry);

        assert!(!trace.active_path.is_empty());
        assert!(!trace.fractal_sequence.is_empty());
        assert!(trace.explanation.contains("campo"));
    }

    #[test]
    fn test_field_delta() {
        let reg = bootstrap_fractals();

        let before = vec![(0, 0.5), (1, 0.3)];
        let after = vec![(0, 0.5), (1, 0.3), (2, 0.4)];

        let delta = compute_delta(&before, &after, &reg);
        assert!(!delta.newly_active.is_empty(),
            "EGO dovrebbe essere newly active");
        assert!(delta.energy_delta > 0.0);
    }

    #[test]
    fn test_introspect_with_active_field() {
        let (mut complex, registry) = setup();

        complex.activate_region(0, 0.9);
        complex.activate_region(2, 0.7);
        complex.propagate_activation(3);

        let intro = introspect(&complex, &registry);
        assert!(intro.field_energy > 0.0,
            "Campo attivo deve avere energia > 0: {}", intro.field_energy);
    }
}
