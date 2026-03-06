/// Ragionamento come Navigazione Topologica.
///
/// Ragionare non e manipolare simboli — e navigare il campo topologico.
///
/// - "Se A allora B" = esiste un cammino A→B con alta attivazione
/// - Ragionamento analogico = isomorfismo locale tra regioni
/// - Ragionamento abduttivo = quale frattale, se attivato, spiegherebbe
///   l'attivazione osservata?
/// - Implicazione = cammino geodetico forte (basso costo, alta attivazione)
/// - Contraddizione = frattali in regioni disconnesse o con alta distanza

use std::collections::HashMap;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::SimplicialComplex;
use crate::topology::navigation::{find_geodesic, geodesic_distance, distance_map, GeodesicPath};

/// Un'implicazione topologica: "da A segue B".
/// Forte se il cammino e breve, attivo, e profondo.
#[derive(Debug, Clone)]
pub struct Implication {
    /// Premessa
    pub premise: FractalId,
    /// Conclusione
    pub conclusion: FractalId,
    /// Forza dell'implicazione [0.0, 1.0]
    pub strength: f64,
    /// Il cammino geodetico che la supporta
    pub path: GeodesicPath,
    /// Tipo di implicazione
    pub kind: ImplicationType,
}

/// Tipi di implicazione.
#[derive(Debug, Clone, PartialEq)]
pub enum ImplicationType {
    /// Diretta: A e B sono vicini (1 hop)
    Direct,
    /// Mediata: A→C→B (passando per intermediari)
    Mediated,
    /// Debole: il cammino esiste ma e lungo o costoso
    Weak,
    /// Nessuna: non esiste cammino (disconnessione)
    None,
}

/// Un'abduzione: "cosa spiegherebbe lo stato attuale?"
#[derive(Debug, Clone)]
pub struct Abduction {
    /// Il frattale che, se attivato, spiegherebbe l'osservazione
    pub hypothesis: FractalId,
    /// Nome del frattale
    pub hypothesis_name: String,
    /// Quanto bene spiega l'osservazione [0.0, 1.0]
    pub explanatory_power: f64,
    /// Quanti frattali attivi raggiunge
    pub reach: usize,
    /// Costo medio per raggiungere i frattali attivi
    pub mean_cost: f64,
}

/// Il risultato di un ragionamento.
#[derive(Debug)]
pub struct ReasoningResult {
    /// L'implicazione trovata (se applicabile)
    pub implication: Option<Implication>,
    /// Le abduzioni migliori
    pub abductions: Vec<Abduction>,
    /// Spiegazione testuale
    pub explanation: String,
}

/// Valuta l'implicazione "da A segue B" nel campo topologico.
/// Un'implicazione forte ha: cammino breve, costo basso, profondita alta.
pub fn evaluate_implication(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    premise: FractalId,
    conclusion: FractalId,
) -> Implication {
    match find_geodesic(complex, registry, premise, conclusion) {
        None => Implication {
            premise,
            conclusion,
            strength: 0.0,
            path: GeodesicPath {
                from: premise,
                to: conclusion,
                steps: vec![],
                total_cost: f64::MAX,
                max_depth: 0,
                explanation: "Nessun cammino — concetti disconnessi.".to_string(),
            },
            kind: ImplicationType::None,
        },
        Some(path) => {
            let hops = path.steps.len();
            let cost = path.total_cost;

            // Forza: inversamente proporzionale al costo, bonus per profondita
            let depth_bonus = 1.0 + path.max_depth as f64 * 0.2;
            let strength = (depth_bonus / (1.0 + cost)).min(1.0);

            let kind = if hops <= 2 {
                ImplicationType::Direct
            } else if strength > 0.3 {
                ImplicationType::Mediated
            } else {
                ImplicationType::Weak
            };

            Implication {
                premise,
                conclusion,
                strength,
                path,
                kind,
            }
        }
    }
}

/// Ragionamento abduttivo: "cosa spiegherebbe l'attivazione attuale?"
///
/// Cerca il frattale che, se fosse la causa, raggiungerebbe il maggior
/// numero di frattali attualmente attivi con il minor costo.
pub fn abduce(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> Vec<Abduction> {
    // Trova i frattali attualmente attivi
    let active_simplices = complex.active_simplices();
    let mut active_fractals: HashMap<FractalId, f64> = HashMap::new();

    for simplex in &active_simplices {
        for &v in &simplex.vertices {
            let entry = active_fractals.entry(v).or_insert(0.0);
            *entry = (*entry + simplex.current_activation).min(1.0);
        }
    }

    if active_fractals.is_empty() {
        return vec![];
    }

    let active_ids: Vec<FractalId> = active_fractals.keys().copied().collect();

    // Per ogni frattale nel registro, calcola quanto bene "spiegherebbe"
    // l'attivazione corrente come causa
    let mut abductions = Vec::new();

    for (&candidate, _) in registry.iter() {
        let dmap = distance_map(complex, candidate);

        let mut reach = 0usize;
        let mut total_cost = 0.0f64;

        for &active_fid in &active_ids {
            if let Some(&dist) = dmap.get(&active_fid) {
                if dist < 10.0 { // Raggiungibile con costo ragionevole
                    reach += 1;
                    total_cost += dist;
                }
            }
        }

        if reach == 0 {
            continue;
        }

        let mean_cost = total_cost / reach as f64;
        // Potere esplicativo: quanti attivi raggiunge (normalizzato) / costo medio
        let coverage = reach as f64 / active_ids.len() as f64;
        let explanatory_power = (coverage / (1.0 + mean_cost)).min(1.0);

        let name = registry.get(candidate)
            .map(|f| f.name.clone())
            .unwrap_or_else(|| format!("#{}", candidate));

        abductions.push(Abduction {
            hypothesis: candidate,
            hypothesis_name: name,
            explanatory_power,
            reach,
            mean_cost,
        });
    }

    abductions.sort_by(|a, b| b.explanatory_power.partial_cmp(&a.explanatory_power).unwrap());
    abductions.truncate(5); // Top 5
    abductions
}

/// Trova contraddizioni: coppie di frattali attivi che sono topologicamente
/// molto distanti (non dovrebbero essere attivi insieme).
pub fn find_contradictions(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> Vec<Contradiction> {
    let active_simplices = complex.active_simplices();
    let mut active_fractals: Vec<FractalId> = Vec::new();

    for simplex in &active_simplices {
        for &v in &simplex.vertices {
            if !active_fractals.contains(&v) {
                active_fractals.push(v);
            }
        }
    }

    let mut contradictions = Vec::new();

    for i in 0..active_fractals.len() {
        for j in (i + 1)..active_fractals.len() {
            let a = active_fractals[i];
            let b = active_fractals[j];

            let distance = geodesic_distance(complex, a, b);
            let tension = match distance {
                None => 1.0, // Disconnessi → massima tensione
                Some(d) if d > 5.0 => (d / 10.0).min(1.0), // Molto lontani
                _ => 0.0, // Vicini → nessuna contraddizione
            };

            if tension > 0.3 {
                let name_a = registry.get(a).map(|f| f.name.clone()).unwrap_or_default();
                let name_b = registry.get(b).map(|f| f.name.clone()).unwrap_or_default();

                contradictions.push(Contradiction {
                    fractal_a: a,
                    fractal_b: b,
                    name_a,
                    name_b,
                    tension,
                    disconnected: distance.is_none(),
                });
            }
        }
    }

    contradictions.sort_by(|a, b| b.tension.partial_cmp(&a.tension).unwrap());
    contradictions
}

/// Una contraddizione: due concetti attivi che "non dovrebbero" esserlo insieme.
#[derive(Debug, Clone)]
pub struct Contradiction {
    pub fractal_a: FractalId,
    pub fractal_b: FractalId,
    pub name_a: String,
    pub name_b: String,
    /// Tensione [0.0, 1.0] — quanto sono in conflitto
    pub tension: f64,
    /// Sono completamente disconnessi?
    pub disconnected: bool,
}

/// Ragionamento completo: implicazione + abduzione + contraddizioni.
pub fn reason(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    premise: Option<FractalId>,
    conclusion: Option<FractalId>,
) -> ReasoningResult {
    let implication = match (premise, conclusion) {
        (Some(p), Some(c)) => Some(evaluate_implication(complex, registry, p, c)),
        _ => None,
    };

    let abductions = abduce(complex, registry);
    let contradictions = find_contradictions(complex, registry);

    let explanation = generate_reasoning_explanation(&implication, &abductions, &contradictions);

    ReasoningResult {
        implication,
        abductions,
        explanation,
    }
}

/// Genera spiegazione testuale del ragionamento.
fn generate_reasoning_explanation(
    implication: &Option<Implication>,
    abductions: &[Abduction],
    contradictions: &[Contradiction],
) -> String {
    let mut parts = Vec::new();

    if let Some(imp) = implication {
        match imp.kind {
            ImplicationType::Direct => {
                parts.push(format!("Implicazione diretta (forza {:.2})", imp.strength));
            }
            ImplicationType::Mediated => {
                parts.push(format!("Implicazione mediata in {} passi (forza {:.2})",
                    imp.path.steps.len(), imp.strength));
            }
            ImplicationType::Weak => {
                parts.push("Implicazione debole — il cammino esiste ma e costoso".to_string());
            }
            ImplicationType::None => {
                parts.push("Nessuna implicazione — concetti disconnessi".to_string());
            }
        }
    }

    if !abductions.is_empty() {
        let top = &abductions[0];
        parts.push(format!(
            "Ipotesi migliore: {} (potere {:.2}, raggiunge {} frattali attivi)",
            top.hypothesis_name, top.explanatory_power, top.reach
        ));
    }

    if !contradictions.is_empty() {
        let top = &contradictions[0];
        if top.disconnected {
            parts.push(format!(
                "Contraddizione: {} e {} sono attivi ma disconnessi",
                top.name_a, top.name_b
            ));
        } else {
            parts.push(format!(
                "Tensione: {} e {} sono attivi ma distanti (tensione {:.2})",
                top.name_a, top.name_b, top.tension
            ));
        }
    }

    if parts.is_empty() {
        "Campo silente — nessun ragionamento attivo.".to_string()
    } else {
        parts.join(". ") + "."
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
    fn test_direct_implication() {
        let (complex, registry) = setup();
        // POTERE (0) → MATERIA (9): connessi direttamente nel ring
        let imp = evaluate_implication(&complex, &registry, 0, 9);
        assert!(imp.strength > 0.0, "POTERE→MATERIA deve avere forza > 0");
        assert_eq!(imp.kind, ImplicationType::Direct);
    }

    #[test]
    fn test_mediated_implication() {
        let (complex, registry) = setup();
        // POTERE (0) → SPAZIO (36): non direttamente connessi (4 hop nel ring)
        let imp = evaluate_implication(&complex, &registry, 0, 36);
        assert!(imp.strength > 0.0);
        assert!(imp.path.steps.len() >= 2);
    }

    #[test]
    fn test_implication_strength_varies() {
        let (complex, registry) = setup();
        // Connessioni dirette devono essere piu forti di quelle mediate
        let direct = evaluate_implication(&complex, &registry, 0, 9);
        let mediated = evaluate_implication(&complex, &registry, 0, 36);
        assert!(direct.strength >= mediated.strength,
            "Diretta ({:.2}) deve essere >= mediata ({:.2})",
            direct.strength, mediated.strength);
    }

    #[test]
    fn test_abduce_silent_field() {
        let (complex, registry) = setup();
        // Campo silente → nessuna abduzione
        let abductions = abduce(&complex, &registry);
        // Potrebbe avere risultati se ci sono simplessi con attivazione residua
        // L'importante e che non crashi
        assert!(abductions.len() <= 5);
    }

    #[test]
    fn test_abduce_active_field() {
        let (mut complex, registry) = setup();
        // Attiva POTERE e MATERIA
        complex.activate_region(0, 0.8);
        complex.activate_region(9, 0.7);
        complex.propagate_activation(2);

        let abductions = abduce(&complex, &registry);
        assert!(!abductions.is_empty(), "Campo attivo deve produrre abduzioni");
        // La migliore ipotesi deve avere potere esplicativo > 0
        assert!(abductions[0].explanatory_power > 0.0);
    }

    #[test]
    fn test_no_contradictions_in_bootstrap() {
        let (complex, registry) = setup();
        // Bootstrap senza attivazione → nessuna contraddizione
        let contradictions = find_contradictions(&complex, &registry);
        assert!(contradictions.is_empty(),
            "Senza attivazione non ci devono essere contraddizioni");
    }

    #[test]
    fn test_full_reasoning() {
        let (mut complex, registry) = setup();
        complex.activate_region(0, 0.8);
        complex.propagate_activation(2);

        let result = reason(&complex, &registry, Some(0), Some(9));
        assert!(result.implication.is_some());
        assert!(!result.explanation.is_empty());
    }

    #[test]
    fn test_reasoning_without_premise() {
        let (mut complex, registry) = setup();
        complex.activate_region(32, 0.9); // IDENTITA
        complex.propagate_activation(2);

        let result = reason(&complex, &registry, None, None);
        assert!(result.implication.is_none());
        assert!(!result.explanation.is_empty());
    }
}
