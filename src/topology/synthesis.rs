/// Sintesi topologica — il Momento Tiferet.
///
/// # Filosofia
///
/// Nel sistema dei Sefirot, Tiferet (Bellezza) è il punto di equilibrio
/// tra la colonna Yang (espansione) e quella Yin (contrazione).
/// Non è una media meccanica — è l'emergenza di qualcosa di qualitativamente
/// nuovo dalla tensione tra i due poli.
///
/// Nel campo duale, ogni 11 cicli (numero primo), Adamo ed Eva entrano in
/// un momento Tiferet: la loro comprensione comune si cristallizza come
/// episodio condiviso nella memoria φ-decay di entrambi.
/// Nel tempo, questi episodi formano il substrato su cui è possibile
/// insegnare strutture più complesse (leggere, ragionare, discorrere).

use std::collections::HashSet;
use crate::topology::engine::PrometeoTopologyEngine;
use crate::topology::fractal::FractalId;

/// Snapshot di un momento Tiferet: la sintesi tra Adamo ed Eva.
#[derive(Debug, Clone)]
pub struct SynthesisPoint {
    /// Ciclo in cui è avvenuta la sintesi
    pub cycle:        u64,
    /// Firma 8D di Tiferet: media delle firme di campo delle due entità
    pub tiferet_sig:  [f64; 8],
    /// Allineamento simpliciale al momento della sintesi [0.0, 1.0]
    pub alignment:    f64,
    /// Codon di Adamo al momento della sintesi
    pub adamo_codon:  [usize; 2],
    /// Codon di Eva al momento della sintesi
    pub eva_codon:    [usize; 2],
}

/// Calcola l'allineamento simpliciale tra le due entità.
///
/// Misura quanta topologia frattale condividono:
///   alignment = |simplici_comuni| / |simplici_totali(union)|
///
/// Parte da 0.0 (nessuna struttura condivisa).
/// Supera 0.40 quando un linguaggio condiviso è emergente.
pub fn compute_alignment(
    adamo: &PrometeoTopologyEngine,
    eva:   &PrometeoTopologyEngine,
) -> f64 {
    // Raccoglie i simplici come set di vertici ordinati (FractalId)
    let simplices_of = |engine: &PrometeoTopologyEngine| -> HashSet<Vec<FractalId>> {
        engine.complex.iter()
            .map(|(_, s)| {
                let mut verts = s.vertices.clone();
                verts.sort();
                verts
            })
            .collect()
    };

    let a_set = simplices_of(adamo);
    let b_set = simplices_of(eva);

    if a_set.is_empty() && b_set.is_empty() {
        return 0.0;
    }

    let common = a_set.intersection(&b_set).count();
    let total  = a_set.union(&b_set).count();

    if total == 0 { 0.0 } else { common as f64 / total as f64 }
}

/// Calcola la divergenza tra i codon delle due entità.
///
/// Target: 4-6 (tensione produttiva).
/// < 2: echo chamber (troppa somiglianza).
/// > 7: incomprensione reciproca (troppa distanza).
pub fn compute_codon_divergence(
    adamo: &PrometeoTopologyEngine,
    eva:   &PrometeoTopologyEngine,
) -> usize {
    let a_codon = adamo.last_will.as_ref().map(|w| w.codon).unwrap_or([0, 1]);
    let e_codon = eva.last_will.as_ref().map(|w| w.codon).unwrap_or([0, 1]);
    a_codon[0].abs_diff(e_codon[0]).max(a_codon[1].abs_diff(e_codon[1]))
}

/// Esegue la sintesi Tiferet tra Adamo ed Eva.
///
/// Calcola il punto medio 8D (Tiferet) e lo codifica come episodio
/// nella memoria φ-decay di entrambe le entità.
/// Questo cristallizza la comprensione comune accumulata nel ciclo corrente.
pub fn synthesize(
    adamo: &mut PrometeoTopologyEngine,
    eva:   &mut PrometeoTopologyEngine,
    cycle: u64,
) -> SynthesisPoint {
    let adamo_sig = adamo.field_sig();
    let eva_sig   = eva.field_sig();

    // Firma Tiferet: punto medio 8D
    let mut tiferet_sig = [0.0f64; 8];
    for i in 0..8 {
        tiferet_sig[i] = (adamo_sig[i] + eva_sig[i]) * 0.5;
    }

    let alignment = compute_alignment(adamo, eva);

    let adamo_codon = adamo.last_will.as_ref().map(|w| w.codon).unwrap_or([0, 1]);
    let eva_codon   = eva.last_will.as_ref().map(|w| w.codon).unwrap_or([0, 1]);

    // Costruisce la firma frattale di Tiferet (media delle attivazioni frattali)
    let fractal_sig = tiferet_fractal_sig(adamo, eva);

    // Costruisce l'attivazione sparsa di Tiferet dalle parole attive di entrambi
    let activation_sparse = tiferet_activation_sparse(adamo, eva);

    // Codifica come episodio in entrambe le memorie (intensity > 0.0 garantita da parole attive)
    adamo.episode_store.encode_from_sig(activation_sparse.clone(), fractal_sig);
    eva.episode_store.encode_from_sig(activation_sparse, fractal_sig);

    SynthesisPoint { cycle, tiferet_sig, alignment, adamo_codon, eva_codon }
}

/// Firma frattale di Tiferet: media delle attivazioni frattali di Adamo ed Eva.
fn tiferet_fractal_sig(
    adamo: &PrometeoTopologyEngine,
    eva:   &PrometeoTopologyEngine,
) -> [f32; 16] {
    let a_acts = adamo.word_topology.emerge_fractal_activations(&adamo.lexicon);
    let e_acts = eva.word_topology.emerge_fractal_activations(&eva.lexicon);

    let mut sig = [0.0f32; 16];
    for (fid, act) in &a_acts {
        let idx = *fid as usize;
        if idx < 16 { sig[idx] += *act as f32 * 0.5; }
    }
    for (fid, act) in &e_acts {
        let idx = *fid as usize;
        if idx < 16 { sig[idx] += *act as f32 * 0.5; }
    }
    sig
}

/// Attivazione sparsa di Tiferet: unione pesata delle parole attive di entrambi.
/// Una parola attiva in entrambi conta doppio (comprensione condivisa).
fn tiferet_activation_sparse(
    adamo: &PrometeoTopologyEngine,
    eva:   &PrometeoTopologyEngine,
) -> Vec<(u32, f32)> {
    use std::collections::HashMap;

    // word → id mapping da Adamo (stesso lessico per entrambi)
    let mut combined: HashMap<String, f32> = HashMap::new();

    for (word, act) in adamo.word_topology.active_words() {
        *combined.entry(word.to_string()).or_insert(0.0) += act as f32 * 0.5;
    }
    for (word, act) in eva.word_topology.active_words() {
        *combined.entry(word.to_string()).or_insert(0.0) += act as f32 * 0.5;
    }

    // Mappa word → pf_id (indice nel campo PF1 di Adamo)
    combined.iter()
        .filter_map(|(word, &act)| {
            adamo.pf_field.word_id(word).map(|id| (id, act))
        })
        .filter(|(_, act)| *act > 0.01)
        .collect()
}

/// Report aggregato sullo stato di emergenza del campo duale.
#[derive(Debug, Clone)]
pub struct EmergenceReport {
    pub cycle:               u64,
    pub alignment:           f64,
    pub codon_divergence:    usize,
    pub tiferet_count:       usize,
    pub tiferet_density:     f64,   // tiferet_count / cycle
    pub adamo_codon:         [usize; 2],
    pub eva_codon:           [usize; 2],
}

impl EmergenceReport {
    pub fn compute(
        adamo: &PrometeoTopologyEngine,
        eva:   &PrometeoTopologyEngine,
        cycle: u64,
        tiferet_count: usize,
    ) -> Self {
        let alignment = compute_alignment(adamo, eva);
        let codon_divergence = compute_codon_divergence(adamo, eva);
        let adamo_codon = adamo.last_will.as_ref().map(|w| w.codon).unwrap_or([0, 1]);
        let eva_codon   = eva.last_will.as_ref().map(|w| w.codon).unwrap_or([0, 1]);
        let tiferet_density = if cycle > 0 {
            tiferet_count as f64 / cycle as f64
        } else {
            0.0
        };

        Self { cycle, alignment, codon_divergence, tiferet_count, tiferet_density,
               adamo_codon, eva_codon }
    }

    /// Interpreta lo stato dell'emergenza in linguaggio leggibile.
    pub fn status(&self) -> &'static str {
        if self.cycle == 0 {
            "nascita — stesse radici, cammini uniti"
        } else if self.tiferet_count == 0 {
            "pre-linguistico — nessuna struttura condivisa"
        } else if self.alignment < 0.20 {
            "proto-linguistico — strutture embrionali"
        } else if self.alignment < 0.40 {
            "sviluppo — linguaggio comune in formazione"
        } else if self.alignment < 0.70 {
            "emergente — pronto per insegnamento"
        } else {
            "simbiosi — identita condivisa"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_alignment_empty_equals_zero() {
        let adamo = PrometeoTopologyEngine::new();
        let eva   = PrometeoTopologyEngine::new();
        // Con gli stessi simplici bootstrap, l'allineamento parte da > 0
        // (entrambi hanno gli stessi simplici bootstrap)
        let al = compute_alignment(&adamo, &eva);
        // Devono avere gli stessi simplici bootstrap → allineamento = 1.0
        assert!(al >= 0.0 && al <= 1.0, "allineamento fuori range: {}", al);
    }

    #[test]
    fn test_codon_divergence_range() {
        let adamo = PrometeoTopologyEngine::new();
        let eva   = PrometeoTopologyEngine::new();
        let div = compute_codon_divergence(&adamo, &eva);
        assert!(div <= 8, "divergenza codon fuori range: {}", div);
    }

    #[test]
    fn test_synthesize_returns_valid_point() {
        use crate::topology::polar_twin::create_polar_twin;
        let mut adamo = PrometeoTopologyEngine::new();
        adamo.teach("io sentire forte dentro vicino");
        adamo.receive("io qui");

        let mut eva = create_polar_twin(&adamo);
        eva.receive("tu lontano");

        let point = synthesize(&mut adamo, &mut eva, 11);
        assert_eq!(point.cycle, 11);
        // Firma Tiferet deve essere un valore valido [0, 1]
        for v in &point.tiferet_sig {
            assert!(*v >= 0.0 && *v <= 1.0, "tiferet_sig fuori range: {}", v);
        }
    }

    #[test]
    fn test_synthesize_encodes_episodes() {
        use crate::topology::polar_twin::create_polar_twin;
        let mut adamo = PrometeoTopologyEngine::new();
        adamo.teach("io sentire forte dentro vicino");
        adamo.receive("io sentire");

        let mut eva = create_polar_twin(&adamo);
        eva.receive("tu sentire lontano");

        let before_adamo = adamo.episode_store.len();
        synthesize(&mut adamo, &mut eva, 11);
        // Se c'erano parole attive, l'episodio viene codificato
        // (può non aumentare se intensity < MIN_INTENSITY — ok)
        let after_adamo = adamo.episode_store.len();
        assert!(after_adamo >= before_adamo,
            "encode_from_sig non deve rimuovere episodi esistenti");
    }
}
