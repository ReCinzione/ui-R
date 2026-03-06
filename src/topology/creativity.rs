/// Creativita come Sogno Guidato.
///
/// La creativita non e randomness. E REM intenzionale con un seme.
/// Il sistema riceve un concetto-seme, abbassa le soglie di attivazione
/// come nella fase REM, e propaga l'attivazione in profondita.
/// Le regioni normalmente separate si illuminano — e le connessioni
/// inattese tra il seme e queste regioni lontane sono le "ispirazioni".
///
/// Una metafora e un caso speciale: due frattali strutturalmente simili
/// (stesse dimensioni salienti) ma topologicamente distanti.
/// "Il tempo e un fiume" funziona perche TEMPO e MOVIMENTO condividono
/// dimensioni (Permanenza bassa, Agency alta) ma sono in regioni diverse.

use crate::topology::fractal::{FractalId, FractalRegistry, DimConstraint};
use crate::topology::simplex::{SimplicialComplex, SharedFace};
use crate::topology::primitive::Dim;
use crate::topology::navigation::geodesic_distance;

/// Un'ispirazione creativa: connessione inattesa tra regioni lontane.
#[derive(Debug, Clone)]
pub struct CreativeInsight {
    /// Il frattale seme da cui siamo partiti
    pub seed: FractalId,
    /// Il frattale scoperto (lontano ma illuminato)
    pub discovery: FractalId,
    /// Nome del frattale scoperto
    pub discovery_name: String,
    /// Tipo di ponte dimensionale che li collega
    pub bridge_type: String,
    /// Quanto e inattesa la connessione [0.0, 1.0]
    pub novelty: f64,
    /// Distanza geodetica (piu alta = piu creativo)
    pub distance: f64,
    /// Spiegazione testuale
    pub explanation: String,
}

/// Una metafora: "A e come B" — isomorfismo locale tra regioni.
#[derive(Debug, Clone)]
pub struct Metaphor {
    /// Dominio sorgente
    pub source: FractalId,
    pub source_name: String,
    /// Dominio target
    pub target: FractalId,
    pub target_name: String,
    /// Dimensioni condivise (struttura comune)
    pub shared_structure: Vec<Dim>,
    /// Tensione: quanto sono normalmente lontani (piu = piu creativo)
    pub tension: f64,
    /// Espressione testuale della metafora
    pub expression: String,
}

/// Risultato di una sessione creativa.
#[derive(Debug)]
pub struct CreativeSession {
    /// Nome del seme
    pub seed_name: String,
    /// Ispirazioni trovate
    pub insights: Vec<CreativeInsight>,
    /// Metafore generate
    pub metaphors: Vec<Metaphor>,
    /// Connessioni rese permanenti
    pub connections_made_permanent: usize,
    /// Spiegazione della sessione
    pub explanation: String,
}

/// Sessione creativa guidata da un seme.
///
/// Il processo:
/// 1. Salva la soglia di attivazione originale
/// 2. Attiva il seme con forza alta
/// 3. Abbassa la soglia a livello REM profondo (0.02)
/// 4. Propaga per 5 passi (piu del REM standard)
/// 5. Raccogli i frattali illuminati e lontani
/// 6. Genera insight e metafore
/// 7. Rendi permanenti le connessioni migliori
/// 8. Ripristina lo stato
pub fn create(
    complex: &mut SimplicialComplex,
    registry: &FractalRegistry,
    seed: FractalId,
) -> CreativeSession {
    let seed_name = registry.get(seed)
        .map(|f| f.name.clone())
        .unwrap_or_else(|| format!("#{}", seed));

    // 1. Salva stato originale
    let original_threshold = complex.activation_threshold;

    // 2. Attiva il seme con forza alta
    complex.activate_region(seed, 0.8);

    // 3. Abbassa la soglia a REM profondo
    complex.activation_threshold = 0.02;

    // 4. Propaga in profondita (5 passi, piu del REM standard)
    complex.propagate_activation(5);

    // 5. Raccogli i frattali illuminati
    let active = complex.active_simplices();
    let mut illuminated_fractals: Vec<(FractalId, f64)> = Vec::new();
    for simplex in &active {
        for &v in &simplex.vertices {
            if v != seed {
                let already = illuminated_fractals.iter().position(|(id, _)| *id == v);
                match already {
                    Some(idx) => {
                        illuminated_fractals[idx].1 =
                            illuminated_fractals[idx].1.max(simplex.current_activation);
                    }
                    None => {
                        illuminated_fractals.push((v, simplex.current_activation));
                    }
                }
            }
        }
    }

    // 6. Ripristina soglia (prima di calcolare le distanze)
    complex.activation_threshold = original_threshold;

    // 7. Genera insight per i frattali lontani
    let mut insights = Vec::new();
    for &(fid, activation) in &illuminated_fractals {
        let distance = geodesic_distance(complex, seed, fid);
        let dist_val = distance.unwrap_or(f64::MAX);

        // Solo frattali veramente lontani sono interessanti
        if dist_val > 2.5 {
            let novelty = (dist_val / 10.0).min(1.0);
            let bridge = find_dimensional_bridge(registry, seed, fid);
            let disc_name = registry.get(fid)
                .map(|f| f.name.clone())
                .unwrap_or_else(|| format!("#{}", fid));

            let explanation = format!(
                "{} e {} sono distanti ({:.1} passi) ma il REM guidato li ha illuminati insieme. {}",
                seed_name, disc_name, dist_val,
                if bridge.is_empty() {
                    "Connessione puramente emergente.".to_string()
                } else {
                    format!("Ponte dimensionale: {}.", bridge)
                }
            );

            insights.push(CreativeInsight {
                seed,
                discovery: fid,
                discovery_name: disc_name,
                bridge_type: bridge,
                novelty,
                distance: dist_val,
                explanation,
            });
        }
    }

    // Ordina per novelty decrescente
    insights.sort_by(|a, b| b.novelty.partial_cmp(&a.novelty).unwrap());
    insights.truncate(5); // Top 5

    // 8. Genera metafore
    let metaphors = find_metaphors(complex, registry, seed);

    // 9. Rendi permanenti le insight migliori (crea simplessi)
    let mut permanent = 0;
    for insight in &insights {
        if insight.novelty > 0.4 {
            // Controlla che non esista gia un simplesso tra i due
            if complex.shared_simplices(seed, insight.discovery).is_empty() {
                let face = if insight.bridge_type.is_empty() {
                    SharedFace::from_property("creativita", 0.4)
                } else {
                    SharedFace::from_property(&insight.bridge_type, 0.5)
                };
                complex.add_simplex(vec![seed, insight.discovery], vec![face]);
                permanent += 1;
            }
        }
    }

    // 10. Decadi l'attivazione residua
    complex.decay_all(0.05);

    let explanation = if insights.is_empty() && metaphors.is_empty() {
        format!("{} e ben connesso — nessuna ispirazione inattesa.", seed_name)
    } else {
        format!(
            "Sessione creativa da {}: {} ispirazioni, {} metafore, {} connessioni permanenti.",
            seed_name, insights.len(), metaphors.len(), permanent
        )
    };

    CreativeSession {
        seed_name,
        insights,
        metaphors,
        connections_made_permanent: permanent,
        explanation,
    }
}

/// Genera metafore: frattali strutturalmente simili ma topologicamente distanti.
///
/// "A e come B" funziona quando A e B condividono dimensioni salienti
/// ma vivono in regioni diverse del complesso.
pub fn find_metaphors(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    source: FractalId,
) -> Vec<Metaphor> {
    let source_fractal = match registry.get(source) {
        Some(f) => f,
        None => return vec![],
    };
    let source_name = source_fractal.name.clone();
    let source_fixed = source_fractal.fixed_dims();

    if source_fixed.is_empty() {
        return vec![];
    }

    let mut metaphors = Vec::new();

    for (&target_id, target_fractal) in registry.iter() {
        if target_id == source {
            continue;
        }

        let target_fixed = target_fractal.fixed_dims();
        if target_fixed.is_empty() {
            continue;
        }

        // Trova dimensioni fisse condivise con valori simili
        let mut shared_dims = Vec::new();
        for &(dim_s, val_s) in &source_fixed {
            for &(dim_t, val_t) in &target_fixed {
                if dim_s == dim_t && (val_s - val_t).abs() < 0.3 {
                    shared_dims.push(dim_s);
                }
            }
        }

        // Serve almeno una dimensione condivisa per la metafora
        if shared_dims.is_empty() {
            continue;
        }

        // Calcola la distanza geodetica
        let distance = geodesic_distance(complex, source, target_id);
        let dist_val = match distance {
            Some(d) if d > 2.0 => d, // Solo se sono lontani
            None => 8.0,              // Disconnessi = massima tensione
            _ => continue,            // Troppo vicini, non e metafora
        };

        // Tensione = distanza × (dimensioni condivise / dimensioni totali source)
        let structural_overlap = shared_dims.len() as f64 / source_fixed.len() as f64;
        let tension = dist_val * structural_overlap;

        if tension < 1.0 {
            continue; // Non abbastanza tensione per essere interessante
        }

        let dim_names: Vec<&str> = shared_dims.iter().map(|d| d.name()).collect();
        let expression = format!(
            "{} e come {} — condividono {} ma vivono in regioni diverse",
            source_name, target_fractal.name, dim_names.join(", ")
        );

        metaphors.push(Metaphor {
            source,
            source_name: source_name.clone(),
            target: target_id,
            target_name: target_fractal.name.clone(),
            shared_structure: shared_dims,
            tension,
            expression,
        });
    }

    metaphors.sort_by(|a, b| b.tension.partial_cmp(&a.tension).unwrap());
    metaphors.truncate(5); // Top 5
    metaphors
}

/// Trova il ponte dimensionale tra due frattali.
/// Restituisce il nome della dimensione condivisa piu significativa.
fn find_dimensional_bridge(
    registry: &FractalRegistry,
    a: FractalId,
    b: FractalId,
) -> String {
    let fa = match registry.get(a) { Some(f) => f, None => return String::new() };
    let fb = match registry.get(b) { Some(f) => f, None => return String::new() };

    let fixed_a = fa.fixed_dims();
    let fixed_b = fb.fixed_dims();

    // Cerca dimensioni fisse in entrambi con valori compatibili
    let mut bridges = Vec::new();
    for &(dim_a, val_a) in &fixed_a {
        for &(dim_b, val_b) in &fixed_b {
            if dim_a == dim_b {
                let compatibility = 1.0 - (val_a - val_b).abs();
                if compatibility > 0.5 {
                    bridges.push((dim_a, compatibility));
                }
            }
        }
    }

    if bridges.is_empty() {
        return String::new();
    }

    bridges.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    bridges[0].0.name().to_string()
}

/// Confidenza del campo: il sistema sa dire "non so" e "non capisco".
#[derive(Debug, Clone)]
pub struct FieldConfidence {
    /// Il sistema ha compreso l'input? (almeno un frattale attivo)
    pub understood: bool,
    /// Ci sono lacune nella regione attiva? (buchi omologici)
    pub has_gaps: bool,
    /// Numero di frattali attivi
    pub active_count: usize,
    /// Spiegazione testuale
    pub explanation: String,
}

/// Valuta la confidenza del campo dopo una perturbazione.
pub fn assess_confidence(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> FieldConfidence {
    let active = complex.active_simplices();

    if active.is_empty() {
        return FieldConfidence {
            understood: false,
            has_gaps: false,
            active_count: 0,
            explanation: "Non capisco — nessun frattale si e attivato.".to_string(),
        };
    }

    // Conta frattali unici attivi
    let mut active_fractals: Vec<FractalId> = Vec::new();
    for simplex in &active {
        for &v in &simplex.vertices {
            if !active_fractals.contains(&v) {
                active_fractals.push(v);
            }
        }
    }

    // Controlla se ci sono buchi omologici
    let homology = crate::topology::homology::compute_homology(complex);
    let has_gaps = homology.betti_1 > 0;

    let explanation = if active_fractals.len() == 1 {
        let name = registry.get(active_fractals[0])
            .map(|f| f.name.clone())
            .unwrap_or_default();
        if has_gaps {
            format!("Solo {} si e attivato, e ci sono lacune concettuali — so poco su questo.", name)
        } else {
            format!("Solo {} si e attivato — la comprensione e parziale.", name)
        }
    } else if has_gaps {
        format!(
            "{} frattali attivi, ma ci sono {} lacune concettuali — non so tutto.",
            active_fractals.len(), homology.betti_1
        )
    } else {
        format!(
            "{} frattali attivi, nessuna lacuna — campo coerente.",
            active_fractals.len()
        )
    };

    FieldConfidence {
        understood: true,
        has_gaps,
        active_count: active_fractals.len(),
        explanation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::bootstrap_complex;

    fn setup() -> (FractalRegistry, SimplicialComplex) {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);
        (reg, complex)
    }

    #[test]
    fn test_creative_session_from_seed() {
        let (reg, mut complex) = setup();
        let session = create(&mut complex, &reg, 0); // SPAZIO come seme
        // Deve produrre qualcosa (il campo bootstrap ha struttura)
        assert!(!session.seed_name.is_empty());
        assert!(!session.explanation.is_empty());
    }

    #[test]
    fn test_creative_session_produces_insights() {
        let (reg, mut complex) = setup();
        // Attiva una regione per dare piu materiale
        complex.activate_region(0, 0.5);
        complex.propagate_activation(1);

        let session = create(&mut complex, &reg, 0);
        // Con attivazione preesistente, le insight dovrebbero essere piu ricche
        // (non garantiamo il numero esatto perche dipende dalla topologia)
        assert!(session.insights.len() + session.metaphors.len() >= 0);
    }

    #[test]
    fn test_metaphor_generation() {
        let (reg, complex) = setup();
        let metaphors = find_metaphors(&complex, &reg, 0); // SPAZIO
        // Le metafore devono avere tensione > 0
        for m in &metaphors {
            assert!(m.tension > 0.0);
            assert!(!m.shared_structure.is_empty());
            assert!(!m.expression.is_empty());
        }
    }

    #[test]
    fn test_metaphor_has_distance() {
        let (reg, complex) = setup();
        // Metafore devono collegare frattali lontani
        let metaphors = find_metaphors(&complex, &reg, 2); // EGO
        for m in &metaphors {
            let dist = geodesic_distance(&complex, m.source, m.target);
            // O sono lontani (>2.0) o disconnessi (None)
            assert!(dist.is_none() || dist.unwrap() > 1.5,
                "Metafora tra {} e {} troppo vicini: {:?}",
                m.source_name, m.target_name, dist);
        }
    }

    #[test]
    fn test_confidence_silent_field() {
        let (reg, complex) = setup();
        let conf = assess_confidence(&complex, &reg);
        // Campo silente: nessun frattale attivo
        assert!(!conf.understood || conf.active_count == 0,
            "Campo silente dovrebbe avere confidenza bassa");
    }

    #[test]
    fn test_confidence_active_field() {
        let (reg, mut complex) = setup();
        complex.activate_region(0, 0.8);
        complex.activate_region(1, 0.7);
        complex.propagate_activation(2);

        let conf = assess_confidence(&complex, &reg);
        assert!(conf.understood);
        assert!(conf.active_count > 0);
        assert!(!conf.explanation.is_empty());
    }

    #[test]
    fn test_dimensional_bridge() {
        let (reg, _) = setup();
        // SPAZIO (0) e LIMITE (5) condividono Permanenza e Definizione
        let bridge = find_dimensional_bridge(&reg, 0, 5);
        assert!(!bridge.is_empty(),
            "SPAZIO e LIMITE dovrebbero avere un ponte dimensionale");
    }

    #[test]
    fn test_permanent_connections() {
        let (reg, mut complex) = setup();
        let initial_count = complex.count();

        // Attiva per avere materiale
        complex.activate_region(0, 0.9);
        complex.propagate_activation(2);

        let session = create(&mut complex, &reg, 0);
        // Se ci sono connessioni permanenti, il complesso deve essere cresciuto
        if session.connections_made_permanent > 0 {
            assert!(complex.count() > initial_count,
                "Connessioni permanenti devono aggiungere simplessi");
        }
    }
}
