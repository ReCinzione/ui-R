/// Composizione Frasale — La frase come operazione topologica.
///
/// "io cammino nel parco" non e 4 parole sommate.
/// E l'intersezione dei pattern di io, cammino, parco nello spazio 8D.
/// Il risultato e una struttura nuova — un simplesso frasale.

use std::collections::HashMap;
use crate::topology::primitive::PrimitiveCore;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::{SimplicialComplex, SharedFace, SimplexId};
use crate::topology::lexicon::{Lexicon, WordActivation};

/// Un pattern frasale: la struttura topologica di una frase intera.
#[derive(Debug, Clone)]
pub struct PhrasePattern {
    /// Le attivazioni delle singole parole (in ordine)
    pub word_activations: Vec<WordActivation>,
    /// La firma composita 8D della frase
    pub composite_signature: PrimitiveCore,
    /// I frattali coinvolti con peso aggregato
    pub fractal_involvement: HashMap<FractalId, f64>,
    /// Forza complessiva della frase
    pub total_strength: f64,
}

/// Compone una frase in un pattern topologico.
///
/// Non somma le parole — le interseca:
/// - La firma composita e la media pesata delle firme delle parole
/// - I frattali coinvolti sono quelli condivisi da piu parole (intersezione)
/// - L'ordine influenza il peso (prime parole = tema, ultime = commento)
pub fn compose_phrase(lexicon: &mut Lexicon, input: &str, registry: &FractalRegistry) -> PhrasePattern {
    let activations = lexicon.process_input(input, registry);

    if activations.is_empty() {
        return PhrasePattern {
            word_activations: Vec::new(),
            composite_signature: PrimitiveCore::neutral(),
            fractal_involvement: HashMap::new(),
            total_strength: 0.0,
        };
    }

    let n = activations.len() as f64;

    // Firma composita: media pesata con peso posizionale
    // Le prime parole pesano di piu (sono il "tema")
    let mut composite = PrimitiveCore::new([0.0; 8]);
    let mut total_weight = 0.0;

    for (i, act) in activations.iter().enumerate() {
        // Peso posizionale: la prima parola pesa di piu
        let positional_weight = 1.0 - (i as f64 / (n + 1.0)) * 0.5;
        let weight = act.strength * positional_weight;
        composite.perturb_towards(&act.signature, weight / (total_weight + weight).max(0.001));
        total_weight += weight;
    }

    // Coinvolgimento frattali: aggregazione con boosting per intersezione
    let mut fractal_counts: HashMap<FractalId, usize> = HashMap::new();
    let mut fractal_scores: HashMap<FractalId, f64> = HashMap::new();

    for act in &activations {
        for &(fid, aff) in &act.affinities {
            *fractal_counts.entry(fid).or_insert(0) += 1;
            let score = fractal_scores.entry(fid).or_insert(0.0);
            *score += aff * act.strength;
        }
    }

    // Coinvolgimento: score base normalizzato + bonus per intersezione
    let mut involvement: HashMap<FractalId, f64> = HashMap::new();
    for (&fid, &score) in &fractal_scores {
        let count = *fractal_counts.get(&fid).unwrap_or(&1);
        // Score base normalizzato per numero di parole
        let base = score / n;
        // Bonus per frattali menzionati da piu parole (intersezione)
        let intersection_bonus = if count > 1 { count as f64 * 0.5 } else { 1.0 };
        let final_score = (base * intersection_bonus).min(1.0);
        if final_score > 0.03 {
            involvement.insert(fid, final_score);
        }
    }

    let total_strength = if n > 0.0 {
        (total_weight / n).min(1.0)
    } else {
        0.0
    };

    PhrasePattern {
        word_activations: activations,
        composite_signature: composite,
        fractal_involvement: involvement,
        total_strength,
    }
}

/// Crea un simplesso frasale nel complesso se la frase coinvolge
/// abbastanza frattali con forza sufficiente.
/// Questo e il meccanismo di apprendimento strutturale:
/// le frasi creano connessioni tra frattali.
pub fn inscribe_phrase(
    complex: &mut SimplicialComplex,
    phrase: &PhrasePattern,
    min_involvement: f64,
) -> Option<SimplexId> {
    // Seleziona i frattali abbastanza coinvolti, ordinati per score decrescente
    // per garantire che i PIU ATTIVI siano inclusi (non i più bassi per ID)
    let mut scored: Vec<(f64, FractalId)> = phrase.fractal_involvement.iter()
        .filter(|(_, &score)| score > min_involvement)
        .map(|(&fid, &score)| (score, fid))
        .collect();
    scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    scored.dedup_by_key(|x| x.1);

    // Serve almeno un 1-simplesso (2 vertici)
    if scored.len() < 2 {
        return None;
    }

    // Limita a 4 vertici (3-simplesso max) per non creare mostri — prendi i TOP 4 per score
    scored.truncate(4);

    // Forma canonica: ordina per ID (necessario per deduplicazione simplessi)
    let mut vertices: Vec<FractalId> = scored.iter().map(|(_, fid)| *fid).collect();
    vertices.sort();

    // Crea facce condivise dalla firma composita
    let mut faces = Vec::new();

    // Faccia basata sulla dimensione 8D piu prominente
    let sig = &phrase.composite_signature;
    let mut max_dim = crate::topology::primitive::Dim::Confine;
    let mut max_val = 0.0_f64;
    for dim in crate::topology::primitive::Dim::ALL {
        let deviation = (sig.get(dim) - 0.5).abs();
        if deviation > max_val {
            max_val = deviation;
            max_dim = dim;
        }
    }
    faces.push(SharedFace::from_dim(max_dim, phrase.total_strength));

    // Faccia basata sulla co-attivazione
    if phrase.word_activations.len() >= 2 {
        let keywords: Vec<String> = phrase.word_activations.iter()
            .filter(|a| a.is_known)
            .map(|a| a.word.clone())
            .collect();
        if !keywords.is_empty() {
            let label = keywords.join("+");
            faces.push(SharedFace::from_property(&label, phrase.total_strength * 0.8));
        }
    }

    // Deduplicazione: se esiste già un simplesso con gli stessi vertici, rinforza invece di creare
    if let Some(existing_id) = complex.find_simplex_with_vertices(&vertices) {
        if let Some(s) = complex.get_mut(existing_id) {
            s.activate(phrase.total_strength * 0.5);
        }
        return Some(existing_id);
    }

    let sid = complex.add_simplex(vertices, faces);
    if let Some(s) = complex.get_mut(sid) {
        s.activate(phrase.total_strength);
    }
    Some(sid)
}

/// Analisi della composizione: cosa "dice" una frase dal punto di vista topologico.
#[derive(Debug)]
pub struct CompositionAnalysis {
    /// Frattali dominanti
    pub dominant_fractals: Vec<(FractalId, f64)>,
    /// Dimensioni 8D piu deviate dal neutro
    pub salient_dimensions: Vec<(crate::topology::primitive::Dim, f64)>,
    /// Parole riconosciute vs ignote
    pub known_words: usize,
    pub unknown_words: usize,
    /// Forza complessiva
    pub strength: f64,
}

/// Analizza la composizione topologica di una frase.
pub fn analyze_composition(phrase: &PhrasePattern) -> CompositionAnalysis {
    let mut dominants: Vec<(FractalId, f64)> = phrase.fractal_involvement.iter()
        .map(|(&k, &v)| (k, v))
        .collect();
    dominants.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut dims: Vec<(crate::topology::primitive::Dim, f64)> = crate::topology::primitive::Dim::ALL.iter()
        .map(|&d| (d, (phrase.composite_signature.get(d) - 0.5).abs()))
        .collect();
    dims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let known = phrase.word_activations.iter().filter(|a| a.is_known).count();
    let unknown = phrase.word_activations.len() - known;

    CompositionAnalysis {
        dominant_fractals: dominants,
        salient_dimensions: dims,
        known_words: known,
        unknown_words: unknown,
        strength: phrase.total_strength,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::simplex::bootstrap_complex;
    use crate::topology::fractal::bootstrap_fractals;

    #[test]
    fn test_compose_simple_phrase() {
        let mut lex = Lexicon::bootstrap();
        let reg = bootstrap_fractals();
        let phrase = compose_phrase(&mut lex, "pensare alla felicità", &reg);

        assert!(!phrase.word_activations.is_empty());
        assert!(phrase.total_strength > 0.0);
        assert!(!phrase.fractal_involvement.is_empty());
    }

    #[test]
    fn test_composition_intersection() {
        let mut lex = Lexicon::bootstrap();
        let reg = bootstrap_fractals();

        // Frase con parole di frattali diversi
        let phrase = compose_phrase(&mut lex, "pensare al tempo passato", &reg);

        // Deve coinvolgere sia PENSIERO (pensare) che TEMPO (tempo, passato)
        let has_pensiero = phrase.fractal_involvement.contains_key(&9);
        let has_tempo = phrase.fractal_involvement.contains_key(&1);
        assert!(has_pensiero || has_tempo,
            "Deve coinvolgere PENSIERO o TEMPO: {:?}", phrase.fractal_involvement);
    }

    #[test]
    fn test_inscribe_phrase_creates_simplex() {
        let mut lex = Lexicon::bootstrap();
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let mut complex = bootstrap_complex(&ids);

        let initial_count = complex.count();

        let phrase = compose_phrase(&mut lex, "pensare al tempo della felicità", &reg);
        let result = inscribe_phrase(&mut complex, &phrase, 0.01);

        assert!(result.is_some(),
            "Deve creare un simplesso. Coinvolgimento: {:?}", phrase.fractal_involvement);
        assert!(complex.count() > initial_count, "Il complesso deve crescere");
    }

    #[test]
    fn test_analyze_composition() {
        let mut lex = Lexicon::bootstrap();
        let reg = bootstrap_fractals();
        let phrase = compose_phrase(&mut lex, "la gioia del tramonto rosso", &reg);
        let analysis = analyze_composition(&phrase);

        assert!(analysis.known_words > 0);
        assert!(analysis.strength > 0.0);
    }

    #[test]
    fn test_unknown_words_in_phrase() {
        let mut lex = Lexicon::bootstrap();
        let reg = bootstrap_fractals();
        let phrase = compose_phrase(&mut lex, "il quixplat brilla nella serendipita", &reg);
        let analysis = analyze_composition(&phrase);

        assert!(analysis.unknown_words > 0, "Deve avere parole ignote");
        // Le parole ignote perturbano debolmente
        let unknown_acts: Vec<&WordActivation> = phrase.word_activations.iter()
            .filter(|a| !a.is_known)
            .collect();
        for act in &unknown_acts {
            assert!(act.strength < 0.3,
                "Parola ignota '{}' deve perturbare debolmente: {}", act.word, act.strength);
        }
    }
}
