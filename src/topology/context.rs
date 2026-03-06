/// Attivazione Contestuale — Il principio del prisma.
///
/// Il contesto non "sceglie un settore". Illumina una regione
/// del complesso simpliciale. Cio che e vicino si illumina,
/// cio che e lontano resta in ombra.
///
/// Questo modulo gestisce anche la perturbazione:
/// l'input non viene parsato, perturba il campo.

use std::collections::HashMap;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::{SimplicialComplex, SimplexId};
use crate::topology::lexicon::Lexicon;

/// Il contesto corrente: definisce quali regioni del complesso sono illuminate.
#[derive(Debug, Clone)]
pub struct Context {
    /// Centro di attenzione: frattali in focus con peso di attenzione
    pub attention: HashMap<FractalId, f64>,
    /// Soglia di attivazione (bassa in REM/metafora, alta in veglia)
    pub threshold: f64,
    /// Passi di propagazione (quanti "salti" topologici percorre la luce)
    pub propagation_depth: usize,
}

impl Context {
    /// Contesto neutro: nessun focus specifico.
    pub fn neutral() -> Self {
        Self {
            attention: HashMap::new(),
            threshold: 0.15,
            propagation_depth: 2,
        }
    }

    /// Contesto focalizzato su un singolo frattale.
    pub fn focused(fractal: FractalId, strength: f64) -> Self {
        let mut attention = HashMap::new();
        attention.insert(fractal, strength.clamp(0.0, 1.0));
        Self {
            attention,
            threshold: 0.15,
            propagation_depth: 2,
        }
    }

    /// Contesto multi-focus.
    pub fn multi_focus(fractals: &[(FractalId, f64)]) -> Self {
        let attention: HashMap<FractalId, f64> = fractals.iter()
            .map(|(id, w)| (*id, w.clamp(0.0, 1.0)))
            .collect();
        Self {
            attention,
            threshold: 0.15,
            propagation_depth: 2,
        }
    }

    /// Contesto metaforico: soglia bassa, propagazione ampia.
    /// Permette connessioni normalmente impossibili.
    pub fn metaphorical(fractals: &[(FractalId, f64)]) -> Self {
        let attention: HashMap<FractalId, f64> = fractals.iter()
            .map(|(id, w)| (*id, w.clamp(0.0, 1.0)))
            .collect();
        Self {
            attention,
            threshold: 0.05, // Soglia bassissima
            propagation_depth: 4, // Propagazione ampia
        }
    }

    /// Quanto questo contesto "guarda" un frattale.
    pub fn attention_on(&self, fractal: FractalId) -> f64 {
        *self.attention.get(&fractal).unwrap_or(&0.0)
    }

    /// Aggiungi un frattale al focus.
    pub fn add_focus(&mut self, fractal: FractalId, weight: f64) {
        let current = self.attention.entry(fractal).or_insert(0.0);
        *current = (*current + weight).min(1.0);
    }
}

/// Risultato di un'attivazione contestuale.
#[derive(Debug, Clone)]
pub struct ActivationResult {
    /// Simplessi attivati con il loro livello di attivazione
    pub activated: Vec<(SimplexId, f64)>,
    /// Frattali raggiunti dall'illuminazione
    pub reached_fractals: Vec<FractalId>,
    /// Dimensioni emergenti accessibili (nome → valore)
    pub accessible_dimensions: HashMap<String, f64>,
}

/// Applica un contesto al complesso simpliciale.
/// Restituisce il risultato dell'illuminazione.
pub fn activate_context(
    complex: &mut SimplicialComplex,
    registry: &FractalRegistry,
    context: &Context,
) -> ActivationResult {
    // Prima: decadi tutto leggermente (il passato sfuma)
    complex.decay_all(0.02);

    // Salva soglia originale, applica quella del contesto
    let original_threshold = complex.activation_threshold;
    complex.activation_threshold = context.threshold;

    let mut all_activated = Vec::new();
    let mut all_reached = Vec::new();

    // Attiva regioni centrate sui frattali in focus
    for (&fractal_id, &weight) in &context.attention {
        let activated = complex.activate_region(fractal_id, weight);
        for &sid in &activated {
            if let Some(s) = complex.get(sid) {
                all_activated.push((sid, s.current_activation));
                for &v in &s.vertices {
                    if !all_reached.contains(&v) {
                        all_reached.push(v);
                    }
                }
            }
        }
    }

    // Propaga l'attivazione
    complex.propagate_activation(context.propagation_depth);

    // Raccogli dimensioni emergenti accessibili
    let mut accessible = HashMap::new();
    for &fid in &all_reached {
        if let Some(fractal) = registry.get(fid) {
            for dim in &fractal.emergent_dimensions {
                accessible.insert(
                    format!("{}.{}", fractal.name, dim.name),
                    dim.mean,
                );
            }
        }
    }

    // Ripristina soglia
    complex.activation_threshold = original_threshold;

    ActivationResult {
        activated: all_activated,
        reached_fractals: all_reached,
        accessible_dimensions: accessible,
    }
}

// ═══════════════════════════════════════════════════════════════
// PERTURBAZIONE: l'input attraversa il campo
// ═══════════════════════════════════════════════════════════════

/// Una perturbazione: vettore di forza nel campo 8D
/// generato da un input esterno.
#[derive(Debug, Clone)]
pub struct Perturbation {
    /// Forza direzionale per ogni frattale toccato
    pub forces: HashMap<FractalId, f64>,
    /// Input originale
    pub origin: String,
    /// Forza complessiva della perturbazione
    pub total_strength: f64,
}

/// Un pattern riconosciuto nell'input: una parola che corrisponde
/// a un pattern noto nel complesso.
#[derive(Debug, Clone)]
pub struct RecognizedPattern {
    pub word: String,
    pub fractal_affinities: Vec<(FractalId, f64)>,
}

/// Crea una perturbazione dall'input testuale usando il lessico.
/// Il lessico e la fonte unica delle affinita parola→frattale.
pub fn create_perturbation(
    input: &str,
    lexicon: &Lexicon,
) -> Perturbation {
    let mut forces: HashMap<FractalId, f64> = HashMap::new();
    let mut total = 0.0;

    for word in input.split_whitespace() {
        let lower = word.to_lowercase();
        if lexicon.is_function_word(&lower) || lower.len() <= 1 {
            continue;
        }

        if let Some(pat) = lexicon.get(&lower) {
            // Parola nota: usa le affinita dal lexicon
            for (&fid, &aff) in &pat.fractal_affinities {
                if aff > 0.1 {
                    let force = forces.entry(fid).or_insert(0.0);
                    *force = (*force + aff).min(1.0);
                    total += aff;
                }
            }
        } else {
            // Parola sconosciuta: perturbazione debole e diffusa
            total += 0.05;
        }
    }

    // Normalizza la forza totale
    let word_count = input.split_whitespace().count().max(1) as f64;
    total = (total / word_count).min(1.0);

    Perturbation {
        forces,
        origin: input.to_string(),
        total_strength: total,
    }
}

/// Applica una perturbazione al complesso simpliciale.
/// I simplessi legati ai frattali perturbati si attivano.
/// I simplessi stabili resistono di piu, quelli fragili si spostano di piu.
pub fn apply_perturbation(
    complex: &mut SimplicialComplex,
    perturbation: &Perturbation,
) -> Vec<SimplexId> {
    let mut activated = Vec::new();

    for (&fractal_id, &force) in &perturbation.forces {
        let sids = complex.activate_region(fractal_id, force);
        activated.extend(sids);
    }

    // Nessuna propagazione qui — spetta all'engine coordinarla
    // per evitare propagazione multipla (perturbation + engine = saturazione)

    activated
}

/// Risposta emergente: i pattern piu attivi dopo la perturbazione.
#[derive(Debug, Clone)]
pub struct EmergentResponse {
    /// Frattali piu attivi (il "significato" della risposta)
    pub active_fractals: Vec<(FractalId, f64)>,
    /// Simplessi piu attivi
    pub active_simplices: Vec<SimplexId>,
    /// Parole-chiave associate ai frattali attivi
    pub keywords: Vec<String>,
}

/// Estrai la risposta emergente dallo stato attuale del complesso.
pub fn emerge_response(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> EmergentResponse {
    let most_active = complex.most_active(10);

    // Raccogli frattali piu attivi
    let mut fractal_activation: HashMap<FractalId, f64> = HashMap::new();
    let mut active_sids = Vec::new();

    for simplex in &most_active {
        active_sids.push(simplex.id);
        for &v in &simplex.vertices {
            let entry = fractal_activation.entry(v).or_insert(0.0);
            *entry = (*entry + simplex.current_activation).min(1.0);
        }
    }

    let mut active_fractals: Vec<(FractalId, f64)> = fractal_activation.into_iter().collect();
    active_fractals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Genera keywords dai nomi dei frattali attivi
    let keywords: Vec<String> = active_fractals.iter()
        .take(5)
        .filter_map(|(fid, _)| registry.get(*fid).map(|f| f.name.clone()))
        .collect();

    EmergentResponse {
        active_fractals,
        active_simplices: active_sids,
        keywords,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::bootstrap_complex;

    fn setup() -> (FractalRegistry, SimplicialComplex, Lexicon) {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);
        let lex = Lexicon::bootstrap();
        (reg, complex, lex)
    }

    #[test]
    fn test_context_activation() {
        let (reg, mut complex, _) = setup();
        let context = Context::focused(0, 0.8); // Focus su SPAZIO
        let result = activate_context(&mut complex, &reg, &context);

        assert!(!result.activated.is_empty(), "Qualcosa deve attivarsi");
        assert!(!result.reached_fractals.is_empty());
        // Gli esagrammi bootstrap non hanno dimensioni emergenti predefinite
        let _ = result.accessible_dimensions.len(); // non crashare
    }

    #[test]
    fn test_metaphorical_context_activates_more() {
        let (reg, mut complex1, _) = setup();
        let (_, mut complex2, _) = setup();

        let normal = Context::focused(0, 0.8);
        let metaphor = Context::metaphorical(&[(0, 0.8)]);

        let r1 = activate_context(&mut complex1, &reg, &normal);
        let r2 = activate_context(&mut complex2, &reg, &metaphor);

        // Il contesto metaforico dovrebbe attivare almeno tanto quanto il normale
        assert!(r2.activated.len() >= r1.activated.len(),
            "Metafora attiva {} vs normale {}", r2.activated.len(), r1.activated.len());
    }

    #[test]
    fn test_perturbation_from_input() {
        let (_, _, lex) = setup();
        let pert = create_perturbation("pensare alla felicità nel tempo", &lex);

        assert!(pert.total_strength > 0.0, "La perturbazione deve avere forza");
        assert!(!pert.forces.is_empty(), "Qualche frattale deve essere perturbato");
    }

    #[test]
    fn test_perturbation_activates_complex() {
        let (_, mut complex, lex) = setup();
        let pert = create_perturbation("andare verso casa nel tempo", &lex);
        let activated = apply_perturbation(&mut complex, &pert);

        assert!(!activated.is_empty(), "La perturbazione deve attivare simplessi");
    }

    #[test]
    fn test_emerge_response() {
        let (reg, mut complex, lex) = setup();
        // Frase ricca che attiva molti frattali
        let pert = create_perturbation("andare verso casa nel tempo della grande felicità", &lex);
        apply_perturbation(&mut complex, &pert);

        let response = emerge_response(&complex, &reg);
        assert!(!response.keywords.is_empty(), "La risposta deve avere keywords");
    }
}
