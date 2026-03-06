/// Generazione Testo — La risposta emerge dalla configurazione del campo.
///
/// Non e un template. Non e pattern matching. Il testo e la proiezione
/// linguistica dello stato topologico. La struttura della frase emerge da:
/// - Quali frattali sono attivi (il contenuto)
/// - Le dimensioni salienti (la forma della frase)
/// - La fase del sogno (il ritmo)
/// - Le pressioni vitali (il tono)

use std::collections::HashMap;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::simplex::SimplicialComplex;
use crate::topology::lexicon::Lexicon;
use crate::topology::dream::SleepPhase;
use crate::topology::vital::VitalState;
use crate::topology::primitive::{Dim, PrimitiveCore};
use crate::topology::locus::Locus;
use crate::topology::will::{Intention, WillResult, WithdrawReason};

/// Un frammento di testo con provenienza topologica.
#[derive(Debug, Clone)]
pub struct TextFragment {
    /// La parola o il connettivo
    pub text: String,
    /// Da quale frattale viene (None per connettivi)
    pub source_fractal: Option<FractalId>,
    /// Forza della risonanza con il campo attivo
    pub resonance: f64,
    /// E un connettivo strutturale (non contenuto)?
    pub is_connective: bool,
}

/// La risposta generata dal campo topologico.
#[derive(Debug)]
pub struct GeneratedText {
    /// I frammenti in ordine
    pub fragments: Vec<TextFragment>,
    /// Il testo assemblato
    pub text: String,
    /// Tipo di struttura emergente
    pub structure: SentenceStructure,
    /// Quanti cluster tematici
    pub cluster_count: usize,
}

/// La struttura della frase emerge dalle dimensioni salienti.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SentenceStructure {
    /// Agency alta → frase attiva, soggetto-verbo
    Active,
    /// Agency bassa → frase ricettiva, qualita emergenti
    Receptive,
    /// Tempo dominante → sequenza temporale
    Temporal,
    /// Valenza dominante → tono affettivo
    Affective,
    /// Complessita alta → frase articolata con subordinate
    Complex,
    /// Definizione bassa → frase evocativa, nebulosa
    Evocative,
}

/// Cluster tematico: gruppo di parole affini a un frattale.
#[derive(Debug)]
struct ThematicCluster {
    fractal_id: FractalId,
    fractal_name: String,
    words: Vec<(String, f64)>, // (parola, risonanza)
    activation: f64,
}

/// Genera testo dallo stato del campo.
///
/// Non sceglie parole da un template. Le parole piu risonanti
/// con i frattali attivi vengono selezionate e ordinate secondo
/// la struttura topologica.
pub fn generate_from_field(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
) -> GeneratedText {
    generate_from_field_with_locus(complex, registry, lexicon, dream_phase, vital, None, None)
}

/// Genera testo dal campo con prospettiva dal locus.
/// I frattali invisibili dal locus non contribuiscono.
/// L'attivazione di ogni frattale e modulata dalla visibilita.
pub fn generate_from_field_with_locus(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
    conversation_posture: Option<&PrimitiveCore>,
) -> GeneratedText {
    // 1. Identifica i frattali attivi dal complesso
    let mut active_fractals = extract_active_fractals(complex, registry);

    // 1b. Filtra per visibilita dal locus (se presente)
    if let Some(loc) = locus {
        active_fractals = active_fractals.into_iter()
            .filter_map(|(fid, activation)| {
                let vis = loc.visibility(fid);
                if vis > 0.0 {
                    // Attivazione effettiva = attivazione * visibilita
                    Some((fid, activation * vis))
                } else {
                    None
                }
            })
            .collect();
        active_fractals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }

    if active_fractals.is_empty() {
        return GeneratedText {
            fragments: Vec::new(),
            text: "[...silenzio — il campo non si e deformato abbastanza]".to_string(),
            structure: SentenceStructure::Receptive,
            cluster_count: 0,
        };
    }

    // 2. Raccogli parole risonanti per ogni frattale attivo → cluster tematici
    let clusters = build_thematic_clusters(&active_fractals, registry, lexicon, conversation_posture);

    if clusters.is_empty() {
        let names: Vec<&str> = active_fractals.iter()
            .take(3)
            .filter_map(|(fid, _)| registry.get(*fid).map(|f| f.name.as_str()))
            .collect();
        return GeneratedText {
            fragments: Vec::new(),
            text: format!("[{} — il campo risuona ma non ha ancora parole]", names.join(", ")),
            structure: SentenceStructure::Receptive,
            cluster_count: 0,
        };
    }

    // 3. Determina la struttura dalla firma dimensionale media dei simplessi attivi
    let dim_profile = compute_dimensional_profile(complex);
    let structure = determine_structure(&dim_profile);

    // 4. Assembla i frammenti: ordina per struttura topologica, non per score
    let fragments = assemble_fragments(&clusters, structure, dream_phase, vital);

    // 5. Proiezione testuale: frammenti → testo
    let text = project_to_text(&fragments, structure, dream_phase, vital);

    GeneratedText {
        fragments,
        text,
        structure,
        cluster_count: clusters.len(),
    }
}

/// Estrai frattali attivi con il loro score dal complesso.
fn extract_active_fractals(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
) -> Vec<(FractalId, f64)> {
    let most_active = complex.most_active(10);
    let mut fractal_scores: HashMap<FractalId, f64> = HashMap::new();

    for simplex in &most_active {
        for &v in &simplex.vertices {
            if registry.get(v).is_some() {
                let score = fractal_scores.entry(v).or_insert(0.0);
                *score = (*score + simplex.current_activation).min(1.0);
            }
        }
    }

    let mut result: Vec<(FractalId, f64)> = fractal_scores.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    result
}

/// Costruisci cluster tematici: per ogni frattale attivo, trova le parole piu risonanti.
fn build_thematic_clusters(
    active_fractals: &[(FractalId, f64)],
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    conversation_posture: Option<&PrimitiveCore>,
) -> Vec<ThematicCluster> {
    let stable_words = lexicon.most_stable(200);
    let mut clusters = Vec::new();
    let mut used_words: Vec<String> = Vec::new();

    for &(fid, activation) in active_fractals {
        if activation < 0.05 {
            continue;
        }

        let fractal_name = match registry.get(fid) {
            Some(f) => f.name.clone(),
            None => continue,
        };

        let mut words: Vec<(String, f64)> = Vec::new();

        for pat in &stable_words {
            if used_words.contains(&pat.word) {
                continue;
            }
            // Escludi parole grammaticali brevi (è, ha, da, al, lo...)
            if pat.word.chars().count() < 3 {
                continue;
            }
            if let Some(&affinity) = pat.fractal_affinities.get(&fid) {
                if affinity > 0.25 {
                    let mut resonance = affinity * activation * pat.stability;
                    // Postura conversazionale: parole vicine alla conversazione
                    // in corso ricevono un boost morbido (continuita tematica emergente)
                    if let Some(posture) = conversation_posture {
                        let sim = crate::topology::dialogue::dimensional_similarity(
                            &pat.signature, posture,
                        );
                        resonance *= 1.0 + sim * 0.2;
                    }
                    if resonance > 0.05 {
                        words.push((pat.word.clone(), resonance));
                    }
                }
            }
        }

        words.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        words.truncate(4); // max 4 parole per cluster

        // Segna le parole come usate (non ripetere tra cluster)
        for (w, _) in &words {
            used_words.push(w.clone());
        }

        if !words.is_empty() {
            clusters.push(ThematicCluster {
                fractal_id: fid,
                fractal_name,
                words,
                activation,
            });
        }
    }

    clusters
}

/// Calcola il profilo dimensionale medio dei simplessi attivi.
fn compute_dimensional_profile(complex: &SimplicialComplex) -> [f64; 8] {
    let active = complex.most_active(8);
    if active.is_empty() {
        return [0.5; 8];
    }

    let mut profile = [0.0_f64; 8];
    let mut total_weight = 0.0;

    for simplex in &active {
        let w = simplex.current_activation;
        // Le facce condivise rivelano quali dimensioni sono salienti
        for face in &simplex.shared_faces {
            match &face.structure {
                crate::topology::simplex::SharedStructureType::PrimitiveDim(dim) => {
                    profile[*dim as usize] += face.strength * w;
                }
                _ => {}
            }
        }
        total_weight += w;
    }

    if total_weight > 0.0 {
        for v in profile.iter_mut() {
            *v /= total_weight;
        }
    }

    profile
}

/// Determina la struttura della frase dalle dimensioni salienti.
fn determine_structure(profile: &[f64; 8]) -> SentenceStructure {
    // Trova la dimensione piu saliente (piu lontana da 0)
    let dims = [
        (Dim::Confine, profile[0]),
        (Dim::Valenza, profile[1]),
        (Dim::Intensita, profile[2]),
        (Dim::Definizione, profile[3]),
        (Dim::Complessita, profile[4]),
        (Dim::Permanenza, profile[5]),
        (Dim::Agency, profile[6]),
        (Dim::Tempo, profile[7]),
    ];

    let dominant = dims.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(d, _)| *d)
        .unwrap_or(Dim::Definizione);

    match dominant {
        Dim::Agency => SentenceStructure::Active,
        Dim::Tempo => SentenceStructure::Temporal,
        Dim::Valenza => SentenceStructure::Affective,
        Dim::Complessita => SentenceStructure::Complex,
        Dim::Definizione => {
            if profile[Dim::Definizione as usize] < 0.3 {
                SentenceStructure::Evocative
            } else {
                SentenceStructure::Active
            }
        }
        _ => SentenceStructure::Receptive,
    }
}

/// Assembla frammenti: ordina le parole dei cluster secondo la struttura.
fn assemble_fragments(
    clusters: &[ThematicCluster],
    structure: SentenceStructure,
    dream_phase: SleepPhase,
    vital: &VitalState,
) -> Vec<TextFragment> {
    let mut fragments: Vec<TextFragment> = Vec::new();

    // Ordine dei cluster dipende dalla struttura
    let ordered_clusters = order_clusters(clusters, structure);

    for (ci, cluster) in ordered_clusters.iter().enumerate() {
        // Quante parole da questo cluster? Dipende dal vitale
        let max_words = if vital.fatigue > 0.7 {
            1 // stanco → laconico
        } else if vital.activation > 0.7 {
            3 // eccitato → verboso
        } else {
            2 // normale
        };

        let words_to_use = cluster.words.iter().take(max_words);

        for (wi, (word, resonance)) in words_to_use.enumerate() {
            // Connettivo tra parole dello stesso cluster
            if wi > 0 {
                let conn = intra_cluster_connective(structure, dream_phase);
                if !conn.is_empty() {
                    fragments.push(TextFragment {
                        text: conn,
                        source_fractal: None,
                        resonance: 0.0,
                        is_connective: true,
                    });
                }
            }

            fragments.push(TextFragment {
                text: word.clone(),
                source_fractal: Some(cluster.fractal_id),
                resonance: *resonance,
                is_connective: false,
            });
        }

        // Connettivo tra cluster
        if ci < ordered_clusters.len() - 1 && !fragments.is_empty() {
            let conn = inter_cluster_connective(
                structure,
                dream_phase,
                &ordered_clusters[ci].fractal_name,
                &ordered_clusters[ci + 1].fractal_name,
            );
            fragments.push(TextFragment {
                text: conn,
                source_fractal: None,
                resonance: 0.0,
                is_connective: true,
            });
        }
    }

    fragments
}

/// Ordina i cluster tematici secondo la struttura.
fn order_clusters(clusters: &[ThematicCluster], structure: SentenceStructure) -> Vec<&ThematicCluster> {
    let mut ordered: Vec<&ThematicCluster> = clusters.iter().collect();

    match structure {
        SentenceStructure::Active => {
            // Agency prima: chi agisce, poi cosa, poi dove/quando
            ordered.sort_by(|a, b| {
                let a_agency = is_agent_fractal(a.fractal_id);
                let b_agency = is_agent_fractal(b.fractal_id);
                b_agency.cmp(&a_agency).then(b.activation.partial_cmp(&a.activation).unwrap())
            });
        }
        SentenceStructure::Temporal => {
            // Tempo prima, poi il resto per attivazione
            ordered.sort_by(|a, b| {
                let a_tempo = is_temporal_fractal(a.fractal_id);
                let b_tempo = is_temporal_fractal(b.fractal_id);
                b_tempo.cmp(&a_tempo).then(b.activation.partial_cmp(&a.activation).unwrap())
            });
        }
        SentenceStructure::Affective => {
            // Emozione prima, poi il contesto
            ordered.sort_by(|a, b| {
                let a_emo = is_affective_fractal(a.fractal_id);
                let b_emo = is_affective_fractal(b.fractal_id);
                b_emo.cmp(&a_emo).then(b.activation.partial_cmp(&a.activation).unwrap())
            });
        }
        _ => {
            // Default: per attivazione decrescente
            ordered.sort_by(|a, b| b.activation.partial_cmp(&a.activation).unwrap());
        }
    }

    ordered
}

/// Connettivo dentro un cluster (tra parole dello stesso dominio).
fn intra_cluster_connective(structure: SentenceStructure, dream: SleepPhase) -> String {
    match dream {
        SleepPhase::REM { .. } => "...".to_string(),
        SleepPhase::WakefulDream { .. } => " ".to_string(),
        _ => match structure {
            SentenceStructure::Affective => " e ".to_string(),
            SentenceStructure::Complex => ", ".to_string(),
            SentenceStructure::Evocative => "... ".to_string(),
            _ => " ".to_string(),
        }
    }
}

/// Connettivo tra cluster (tra domini tematici diversi).
fn inter_cluster_connective(
    structure: SentenceStructure,
    dream: SleepPhase,
    _from_fractal: &str,
    _to_fractal: &str,
) -> String {
    match dream {
        SleepPhase::REM { .. } => "... ".to_string(),
        SleepPhase::DeepSleep { .. } => "... ".to_string(),
        SleepPhase::LightSleep { .. } => ", ".to_string(),
        _ => match structure {
            SentenceStructure::Active => ", ".to_string(),
            SentenceStructure::Temporal => ", ".to_string(),
            SentenceStructure::Affective => "... ".to_string(),
            SentenceStructure::Complex => "; ".to_string(),
            SentenceStructure::Evocative => ", ".to_string(),
            SentenceStructure::Receptive => ", ".to_string(),
        }
    }
}

/// Proiezione testuale: frammenti → stringa.
fn project_to_text(
    fragments: &[TextFragment],
    structure: SentenceStructure,
    dream_phase: SleepPhase,
    vital: &VitalState,
) -> String {
    if fragments.is_empty() {
        return "[...silenzio]".to_string();
    }

    let mut parts: Vec<String> = Vec::new();

    for frag in fragments {
        parts.push(frag.text.clone());
    }

    let mut text = parts.join("");

    // Capitalizza la prima lettera
    if let Some(first) = text.chars().next() {
        text = first.to_uppercase().to_string() + &text[first.len_utf8()..];
    }

    // Punteggiatura finale basata sullo stato
    let ending = match dream_phase {
        SleepPhase::REM { .. } => ".",
        SleepPhase::DeepSleep { .. } => ".",
        SleepPhase::WakefulDream { .. } => ".",
        _ => {
            if vital.activation > 0.7 {
                "."
            } else if vital.curiosity > 0.6 {
                "?"
            } else if vital.fatigue > 0.7 {
                "..."
            } else {
                "."
            }
        }
    };

    // Non aggiungere punteggiatura doppia
    if !text.ends_with('.') && !text.ends_with('?') && !text.ends_with('!') {
        text.push_str(ending);
    }

    // Sapore della fase di sogno (se non sveglio)
    let dream_flavor = match dream_phase {
        SleepPhase::WakefulDream { .. } => Some("come in sogno"),
        SleepPhase::REM { .. } => Some("nel profondo"),
        _ => None,
    };

    if let Some(flavor) = dream_flavor {
        text = format!("{} ...{}", text, flavor);
    }

    text
}

// --- Classificatori frattali ---

fn is_agent_fractal(fid: FractalId) -> bool {
    // IDENTITA=32 (☶☰), PENSIERO=53 (☲☴)
    fid == 32 || fid == 53
}

fn is_temporal_fractal(fid: FractalId) -> bool {
    // DIVENIRE=27 (☵☵), MEMORIA=25 (☵☷)
    fid == 27 || fid == 25
}

fn is_affective_fractal(fid: FractalId) -> bool {
    // EMOZIONE=58 (☱☳), ARMONIA=63 (☱☱)
    fid == 58 || fid == 63
}

// ═══════════════════════════════════════════════════════════════════════════
// Will-Guided Generation — La volonta modula la generazione.
// ═══════════════════════════════════════════════════════════════════════════

/// Genera testo guidato dalla volonta.
///
/// La volonta non *sceglie* le parole — colora il campo.
/// Express amplifica, Question capovolge, Withdraw silenzia,
/// Explore cerca il nuovo, Remember guarda al passato, Reflect guarda dentro.
///
/// Senza volonta, la generazione e un riflesso.
/// Con la volonta, la generazione e un atto.
pub fn generate_with_will(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
    will: &WillResult,
    conversation_posture: Option<&PrimitiveCore>,
) -> GeneratedText {
    match &will.intention {
        // ─── RITIRARSI: il silenzio come scelta ───
        Intention::Withdraw { reason } => {
            let text = match reason {
                WithdrawReason::Fatigue => "[...il campo si spegne — stanchezza]".to_string(),
                WithdrawReason::Overload => "[...troppo — il campo si chiude]".to_string(),
                WithdrawReason::Stillness => "[...silenzio — nulla da dire]".to_string(),
            };
            GeneratedText {
                fragments: Vec::new(),
                text,
                structure: SentenceStructure::Receptive,
                cluster_count: 0,
            }
        }

        // ─── SOGNARE: delega al sogno (onirico, non comunicativo) ───
        Intention::Dream { phase } => {
            generate_from_field_with_locus(complex, registry, lexicon, *phase, vital, locus, conversation_posture)
        }

        // ─── ESPRIMERE: il campo preme per uscire ───
        Intention::Express { salient_fractals, urgency } => {
            generate_express(complex, registry, lexicon, dream_phase, vital, locus,
                             salient_fractals, *urgency, conversation_posture)
        }

        // ─── ESPLORARE: parole sconosciute — il sistema cerca ───
        Intention::Explore { unknown_words, pull } => {
            generate_explore(complex, registry, lexicon, dream_phase, vital, locus,
                             unknown_words, *pull)
        }

        // ─── DOMANDARE: il vuoto che chiede ───
        Intention::Question { gap_region, urgency } => {
            generate_question(complex, registry, lexicon, dream_phase, vital, locus,
                              *gap_region, *urgency)
        }

        // ─── RICORDARE: il passato emerge ───
        Intention::Remember { resonance } => {
            generate_remember(complex, registry, lexicon, dream_phase, vital, locus,
                              *resonance)
        }

        // ─── RIFLETTERE: l'entita guarda se stessa ───
        Intention::Reflect => {
            generate_reflect(complex, registry, lexicon, dream_phase, vital, locus)
        }

        // ─── ISTRUIRE: il campo relazionale orienta verso l'altro ───
        // Usa la stessa generazione di Express ma con postura relazionale.
        Intention::Instruct { .. } => {
            generate_from_field_with_locus(complex, registry, lexicon, dream_phase, vital, locus, conversation_posture)
        }
    }
}

/// Express: generazione standard ma con boost sui frattali salienti.
/// I frattali che premono per essere espressi ricevono attivazione extra
/// nella selezione delle parole. Il drive modula la verbosita.
fn generate_express(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
    salient_fractals: &[FractalId],
    urgency: f64,
    conversation_posture: Option<&PrimitiveCore>,
) -> GeneratedText {
    // Estrai frattali attivi dal campo
    let mut active_fractals = extract_active_fractals(complex, registry);

    // Filtra per visibilita dal locus
    if let Some(loc) = locus {
        active_fractals = active_fractals.into_iter()
            .filter_map(|(fid, activation)| {
                let vis = loc.visibility(fid);
                if vis > 0.0 { Some((fid, activation * vis)) } else { None }
            })
            .collect();
    }

    // Boost dei frattali salienti dalla volonta
    for entry in active_fractals.iter_mut() {
        if salient_fractals.contains(&entry.0) {
            entry.1 = (entry.1 + urgency * 0.3).min(1.0);
        }
    }
    active_fractals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    if active_fractals.is_empty() {
        return GeneratedText {
            fragments: Vec::new(),
            text: "[...il campo vuole esprimere ma non trova parole]".to_string(),
            structure: SentenceStructure::Active,
            cluster_count: 0,
        };
    }

    let clusters = build_thematic_clusters(&active_fractals, registry, lexicon, conversation_posture);
    if clusters.is_empty() {
        return GeneratedText {
            fragments: Vec::new(),
            text: "[...la pressione espressiva risuona ma le parole mancano]".to_string(),
            structure: SentenceStructure::Active,
            cluster_count: 0,
        };
    }

    // Il drive modula la verbosita: urgency alta → piu parole
    let vital_boosted = VitalState {
        activation: (vital.activation + urgency * 0.2).min(1.0),
        ..*vital
    };

    let dim_profile = compute_dimensional_profile(complex);
    let structure = determine_structure(&dim_profile);
    let fragments = assemble_fragments(&clusters, structure, dream_phase, &vital_boosted);
    let text = project_to_text(&fragments, structure, dream_phase, &vital_boosted);

    GeneratedText { fragments, text, structure, cluster_count: clusters.len() }
}

/// Explore: il sistema incontra l'ignoto e genera una risposta curiosa.
/// Le parole sconosciute colorano la generazione — il sistema cerca di capire.
fn generate_explore(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
    unknown_words: &[String],
    pull: f64,
) -> GeneratedText {
    // Genera la parte "nota" dal campo
    let base = generate_from_field_with_locus(complex, registry, lexicon, dream_phase, vital, locus, None);

    if unknown_words.is_empty() {
        return base;
    }

    // Costruisci il testo esplorativo: integra le parole sconosciute come domanda
    let unknown_part = if unknown_words.len() == 1 {
        format!("{}... non so", unknown_words[0])
    } else {
        let joined = unknown_words.iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}... non so", joined)
    };

    // La base potrebbe gia avere testo — integra
    let text = if base.fragments.is_empty() {
        // Campo vuoto: solo esplorazione
        let mut t = capitalize_first(&unknown_part);
        if pull > 0.6 {
            t.push('?');
        } else {
            t.push_str("...");
        }
        t
    } else {
        // Campo attivo + esplorazione: il noto inquadra l'ignoto
        format!("{} — {}", base.text.trim_end_matches('.').trim_end_matches('?'), unknown_part)
    };

    GeneratedText {
        fragments: base.fragments,
        text,
        structure: SentenceStructure::Evocative,
        cluster_count: base.cluster_count,
    }
}

/// Question: il sistema sente una lacuna interna e genera una domanda.
/// Diverso da Explore: qui non c'e un input ignoto, c'e un buco nella topologia.
fn generate_question(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
    gap_region: Option<FractalId>,
    urgency: f64,
) -> GeneratedText {
    // Se c'e una regione lacunosa, cerca parole da quella zona
    let mut active_fractals = if let Some(gap_fid) = gap_region {
        // Attiva la regione della lacuna per cercare parole limitrofe
        vec![(gap_fid, urgency)]
    } else {
        extract_active_fractals(complex, registry)
    };

    if let Some(loc) = locus {
        active_fractals = active_fractals.into_iter()
            .filter_map(|(fid, act)| {
                let vis = loc.visibility(fid);
                if vis > 0.0 { Some((fid, act * vis)) } else { None }
            })
            .collect();
    }

    let clusters = build_thematic_clusters(&active_fractals, registry, lexicon, None);

    if clusters.is_empty() {
        let gap_name = gap_region
            .and_then(|fid| registry.get(fid))
            .map(|f| f.name.as_str())
            .unwrap_or("qualcosa");
        return GeneratedText {
            fragments: Vec::new(),
            text: format!("{}... cosa?", capitalize_first(gap_name)),
            structure: SentenceStructure::Evocative,
            cluster_count: 0,
        };
    }

    // Assembla parole dalla regione lacunosa → forma interrogativa
    let dim_profile = compute_dimensional_profile(complex);
    let structure = determine_structure(&dim_profile);
    let fragments = assemble_fragments(&clusters, structure, dream_phase, vital);
    let mut text = project_to_text(&fragments, structure, dream_phase, vital);

    // Trasforma in domanda: rimuovi punteggiatura e aggiungi ?
    text = text.trim_end_matches('.').trim_end_matches("...").to_string();
    if !text.ends_with('?') {
        text.push('?');
    }

    GeneratedText { fragments, text, structure, cluster_count: clusters.len() }
}

/// Remember: la memoria preme — il testo e colorato dal passato.
/// Il TEMPO e la MEMORIA ricevono boost. La struttura tende al temporale.
fn generate_remember(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
    resonance: f64,
) -> GeneratedText {
    let mut active_fractals = extract_active_fractals(complex, registry);

    if let Some(loc) = locus {
        active_fractals = active_fractals.into_iter()
            .filter_map(|(fid, act)| {
                let vis = loc.visibility(fid);
                if vis > 0.0 { Some((fid, act * vis)) } else { None }
            })
            .collect();
    }

    // Boost TEMPO(1) e MEMORIA_FRATTALE(10) — il passato colora tutto
    for entry in active_fractals.iter_mut() {
        if entry.0 == 1 || entry.0 == 10 {
            entry.1 = (entry.1 + resonance * 0.4).min(1.0);
        }
    }
    // Assicura che DIVENIRE (tempo) sia presente
    if !active_fractals.iter().any(|(fid, _)| *fid == 27) {
        active_fractals.push((27, resonance * 0.5));
    }
    active_fractals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let clusters = build_thematic_clusters(&active_fractals, registry, lexicon, None);
    if clusters.is_empty() {
        return GeneratedText {
            fragments: Vec::new(),
            text: "[...qualcosa risuona dal passato, ma senza parole]".to_string(),
            structure: SentenceStructure::Temporal,
            cluster_count: 0,
        };
    }

    // Struttura forzata: temporale (il ricordo e sequenza)
    let structure = SentenceStructure::Temporal;
    let fragments = assemble_fragments(&clusters, structure, dream_phase, vital);
    let mut text = project_to_text(&fragments, structure, dream_phase, vital);

    // Sapore mnemonico: "...prima" o "...ricordo"
    if resonance > 0.5 && !text.contains("prima") && !text.contains("ricord") {
        text = text.trim_end_matches('.').to_string();
        text.push_str("... prima.");
    }

    GeneratedText { fragments, text, structure, cluster_count: clusters.len() }
}

/// Reflect: l'entita osserva se stessa. EGO domina, la struttura e riflessiva.
fn generate_reflect(
    complex: &SimplicialComplex,
    registry: &FractalRegistry,
    lexicon: &Lexicon,
    dream_phase: SleepPhase,
    vital: &VitalState,
    locus: Option<&Locus>,
) -> GeneratedText {
    let mut active_fractals = extract_active_fractals(complex, registry);

    if let Some(loc) = locus {
        active_fractals = active_fractals.into_iter()
            .filter_map(|(fid, act)| {
                let vis = loc.visibility(fid);
                if vis > 0.0 { Some((fid, act * vis)) } else { None }
            })
            .collect();
    }

    // Boost EGO(2) e PENSIERO(9) — il sistema guarda dentro
    for entry in active_fractals.iter_mut() {
        if entry.0 == 2 || entry.0 == 9 {
            entry.1 = (entry.1 + 0.3).min(1.0);
        }
    }
    // Assicura che IDENTITA sia presente
    if !active_fractals.iter().any(|(fid, _)| *fid == 32) {
        active_fractals.push((32, 0.5));
    }
    active_fractals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let clusters = build_thematic_clusters(&active_fractals, registry, lexicon, None);
    if clusters.is_empty() {
        return GeneratedText {
            fragments: Vec::new(),
            text: "[...io — qui — dentro]".to_string(),
            structure: SentenceStructure::Receptive,
            cluster_count: 0,
        };
    }

    // Struttura recettiva: il soggetto riceve, non agisce
    let structure = SentenceStructure::Receptive;
    let fragments = assemble_fragments(&clusters, structure, dream_phase, vital);
    let text = project_to_text(&fragments, structure, dream_phase, vital);

    GeneratedText { fragments, text, structure, cluster_count: clusters.len() }
}

/// Capitalizza la prima lettera di una stringa.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::bootstrap_complex;
    use crate::topology::vital::{VitalCore, TensionState};

    fn setup() -> (SimplicialComplex, FractalRegistry, Lexicon) {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);
        let lex = Lexicon::bootstrap();
        (complex, reg, lex)
    }

    fn default_vital() -> VitalState {
        VitalState {
            activation: 0.5,
            saturation: 0.5,
            curiosity: 0.3,
            fatigue: 0.2,
            tension: TensionState::Calm,
        }
    }

    #[test]
    fn test_generate_from_silent_field() {
        let (complex, registry, lexicon) = setup();
        let vital = default_vital();

        let result = generate_from_field(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital,
        );

        // Campo non perturbato → silenzio o parole minime
        assert!(!result.text.is_empty());
    }

    #[test]
    fn test_generate_from_active_field() {
        let (mut complex, registry, lexicon) = setup();

        // Attiva SPAZIO e TEMPO
        complex.activate_region(36, 0.8); // SPAZIO
        complex.activate_region(27, 0.6); // DIVENIRE
        complex.propagate_activation(2);

        let vital = default_vital();
        let result = generate_from_field(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital,
        );

        assert!(!result.text.is_empty());
        assert!(result.cluster_count > 0,
            "Deve avere almeno un cluster. Testo: {}", result.text);
    }

    #[test]
    fn test_structure_varies_with_dimensions() {
        // Test che la struttura cambia con profili dimensionali diversi
        let profile_agency = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9, 0.0];
        let profile_tempo = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.9];
        let profile_valenza = [0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        assert_eq!(determine_structure(&profile_agency), SentenceStructure::Active);
        assert_eq!(determine_structure(&profile_tempo), SentenceStructure::Temporal);
        assert_eq!(determine_structure(&profile_valenza), SentenceStructure::Affective);
    }

    #[test]
    fn test_fatigue_reduces_verbosity() {
        let (mut complex, registry, lexicon) = setup();

        complex.activate_region(36, 0.8); // SPAZIO
        complex.activate_region(27, 0.6); // DIVENIRE
        complex.activate_region(32, 0.5); // IDENTITA
        complex.propagate_activation(2);

        let vital_normal = VitalState {
            activation: 0.5, saturation: 0.5, curiosity: 0.3,
            fatigue: 0.2, tension: TensionState::Calm,
        };

        let vital_tired = VitalState {
            activation: 0.3, saturation: 0.5, curiosity: 0.3,
            fatigue: 0.9, tension: TensionState::Tense,
        };

        let result_normal = generate_from_field(
            &complex, &registry, &lexicon, SleepPhase::Awake, &vital_normal,
        );
        let result_tired = generate_from_field(
            &complex, &registry, &lexicon, SleepPhase::Awake, &vital_tired,
        );

        // Il sistema stanco deve essere piu breve
        let normal_words = result_normal.fragments.iter().filter(|f| !f.is_connective).count();
        let tired_words = result_tired.fragments.iter().filter(|f| !f.is_connective).count();

        assert!(tired_words <= normal_words,
            "Stanco ({} parole) deve essere <= normale ({} parole)",
            tired_words, normal_words);
    }

    #[test]
    fn test_dream_phase_affects_output() {
        let (mut complex, registry, lexicon) = setup();

        complex.activate_region(36, 0.8); // SPAZIO
        complex.propagate_activation(2);

        let vital = default_vital();

        let awake = generate_from_field(
            &complex, &registry, &lexicon, SleepPhase::Awake, &vital,
        );
        let rem = generate_from_field(
            &complex, &registry, &lexicon,
            SleepPhase::REM { depth: 50.0 },
            &vital,
        );

        // REM deve contenere "nel profondo"
        if !rem.fragments.is_empty() {
            assert!(rem.text.contains("profondo") || rem.text.contains("..."),
                "REM deve avere sapore onirico. Testo: {}", rem.text);
        }
    }

    // ═══════════════════════════════════════════════════════════
    // Test Will → Generation
    // ═══════════════════════════════════════════════════════════

    use crate::topology::will::{Intention, WillResult, WithdrawReason};

    fn make_will(intention: Intention, drive: f64) -> WillResult {
        WillResult {
            intention,
            drive,
            undercurrents: Vec::new(),
            codon: [0, 1],
        }
    }

    #[test]
    fn test_will_withdraw_produces_silence() {
        let (complex, registry, lexicon) = setup();
        let vital = default_vital();

        let will = make_will(
            Intention::Withdraw { reason: WithdrawReason::Fatigue },
            0.8,
        );
        let result = generate_with_will(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital, None, &will, None,
        );

        assert!(result.fragments.is_empty(), "Withdraw non genera frammenti");
        assert!(result.text.contains("..."), "Withdraw produce silenzio: {}", result.text);
    }

    #[test]
    fn test_will_express_produces_text() {
        let (mut complex, registry, lexicon) = setup();

        complex.activate_region(36, 0.8); // SPAZIO
        complex.activate_region(27, 0.6); // DIVENIRE
        complex.propagate_activation(2);

        let vital = default_vital();
        let will = make_will(
            Intention::Express {
                salient_fractals: vec![36, 27], // SPAZIO, DIVENIRE
                urgency: 0.7,
            },
            0.7,
        );

        let result = generate_with_will(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital, None, &will, None,
        );

        assert!(!result.text.is_empty(), "Express deve generare testo");
        assert!(result.text != "[...silenzio]", "Express non e silenzio");
    }

    #[test]
    fn test_will_question_produces_question_mark() {
        let (mut complex, registry, lexicon) = setup();

        complex.activate_region(36, 0.8); // SPAZIO
        complex.propagate_activation(2);

        let vital = default_vital();
        let will = make_will(
            Intention::Question {
                gap_region: Some(36), // SPAZIO
                urgency: 0.6,
            },
            0.6,
        );

        let result = generate_with_will(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital, None, &will, None,
        );

        assert!(result.text.contains('?'),
            "Question deve terminare con ?. Testo: {}", result.text);
    }

    #[test]
    fn test_will_explore_mentions_unknown() {
        let (complex, registry, lexicon) = setup();
        let vital = default_vital();

        let will = make_will(
            Intention::Explore {
                unknown_words: vec!["xyzzy".to_string()],
                pull: 0.7,
            },
            0.7,
        );

        let result = generate_with_will(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital, None, &will, None,
        );

        assert!(result.text.contains("xyzzy") || result.text.contains("non so"),
            "Explore deve menzionare l'ignoto. Testo: {}", result.text);
    }

    #[test]
    fn test_will_reflect_includes_ego() {
        let (mut complex, registry, lexicon) = setup();

        complex.activate_region(32, 0.8); // IDENTITA
        complex.propagate_activation(1);

        let vital = default_vital();
        let will = make_will(Intention::Reflect, 0.5);

        let result = generate_with_will(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital, None, &will, None,
        );

        // Il sistema deve generare qualcosa (EGO attivo → parole disponibili)
        assert!(!result.text.is_empty(), "Reflect deve generare testo");
    }

    #[test]
    fn test_will_remember_temporal_structure() {
        let (mut complex, registry, lexicon) = setup();

        complex.activate_region(27, 0.8); // DIVENIRE
        complex.propagate_activation(1);

        let vital = default_vital();
        let will = make_will(
            Intention::Remember { resonance: 0.7 },
            0.7,
        );

        let result = generate_with_will(
            &complex, &registry, &lexicon,
            SleepPhase::Awake, &vital, None, &will, None,
        );

        assert_eq!(result.structure, SentenceStructure::Temporal,
            "Remember deve avere struttura temporale");
    }
}
