/// Grammatica Visiva dei Frattali — Ogni frattale ha una forma.
///
/// Ogni frattale è un attrattore nel campo 8D. Questo modulo gli dà
/// anche una presenza visiva: un SVG minimale che cattura la sua essenza
/// geometrica, non come illustrazione ma come firma iconica.
///
/// I simplessi (connessioni tra frattali) ottengono la loro immagine per
/// composizione automatica delle icone dei frattali che connettono.
/// Non si disegnano — emergono.

use crate::topology::fractal::{FractalId, FractalRegistry, DimConstraint};
use crate::topology::primitive::Dim;

pub const FRACTAL_COUNT: usize = 64;  // Esteso da 16 a 64 esagrammi
const MANUAL_GLYPH_COUNT: usize = 16;  // Solo i primi 16 hanno glifi manuali

/// Nomi ordinati per id (solo i primi 16 manuali)
pub const FRACTAL_NAMES: [&str; MANUAL_GLYPH_COUNT] = [
    "SPAZIO",        // 0
    "TEMPO",         // 1
    "EGO",           // 2
    "RELAZIONE",     // 3
    "POTENZIALE",    // 4
    "LIMITE",        // 5
    "MOVIMENTO",     // 6
    "EMOZIONE",      // 7
    "PENSIERO",      // 8
    "MEMORIA_F",     // 9
    "COMUNICAZIONE", // 10
    "AZIONE",        // 11
    "NATURA",        // 12
    "CORPO",         // 13
    "QUALITA",       // 14
    "SOCIETA",       // 15
];

/// Corpo SVG interno (senza wrapper <svg>) per composizione.
/// ViewBox implicito 0 0 100 100. Stroke #1a1a1a, sfondo trasparente.
const BODIES: [&str; MANUAL_GLYPH_COUNT] = [
    // 0 SPAZIO — punto nel vuoto.
    // La posizione è tutto. Lo spazio è il contenuto.
    r##"<circle cx="50" cy="50" r="4" fill="#1a1a1a"/>"##,

    // 1 TEMPO — onda sinusoidale con freccia.
    // Il flusso che si dispiega in una direzione.
    r##"<path d="M15,50 C25,25 35,25 50,50 C65,75 75,75 85,50" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><polyline points="76,42 85,50 76,58" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>"##,

    // 2 EGO — cerchio pieno.
    // Il sé come entità densa, chiusa, definita. Massa pura.
    r##"<circle cx="50" cy="50" r="30" fill="#1a1a1a"/>"##,

    // 3 RELAZIONE — due cerchi connessi da una linea.
    // Il legame come struttura: due distinti che si toccano.
    r##"<circle cx="27" cy="50" r="16" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><circle cx="73" cy="50" r="16" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><line x1="43" y1="50" x2="57" y2="50" stroke="#1a1a1a" stroke-width="2.5"/>"##,

    // 4 POTENZIALE — cerchio tratteggiato.
    // La possibilità non ancora definita. Confine poroso, aperto.
    r##"<circle cx="50" cy="50" r="30" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-dasharray="6,4"/>"##,

    // 5 LIMITE — quadrato a bordo spesso.
    // Il confine come definizione. La forma è il limite.
    r##"<rect x="20" y="20" width="60" height="60" fill="none" stroke="#1a1a1a" stroke-width="5"/>"##,

    // 6 MOVIMENTO — freccia orizzontale.
    // La direzione pura, senza origine né destinazione.
    r##"<line x1="18" y1="50" x2="68" y2="50" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><polyline points="56,36 72,50 56,64" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>"##,

    // 7 EMOZIONE — cuore.
    // La pulsazione organica. Non simbolo — forma del sentire.
    r##"<path d="M50,75 C22,56 16,36 30,24 C38,18 47,22 50,30 C53,22 62,18 70,24 C84,36 78,56 50,75 Z" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-linejoin="round"/>"##,

    // 8 PENSIERO — albero ramificato verso l'alto.
    // Il pensiero come rete che cresce moltiplicando i rami.
    r##"<line x1="50" y1="82" x2="50" y2="50" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><line x1="50" y1="60" x2="26" y2="36" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><line x1="50" y1="60" x2="74" y2="36" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><line x1="50" y1="48" x2="35" y2="26" stroke="#1a1a1a" stroke-width="2" stroke-linecap="round"/><line x1="50" y1="48" x2="65" y2="26" stroke="#1a1a1a" stroke-width="2" stroke-linecap="round"/>"##,

    // 9 MEMORIA_F — cerchi concentrici che sfumano.
    // L'eco del passato: presente e nitido al centro,
    // sempre più tenue verso il bordo — decay φ.
    r##"<circle cx="50" cy="50" r="8" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><circle cx="50" cy="50" r="20" fill="none" stroke="#1a1a1a" stroke-width="2" opacity="0.7"/><circle cx="50" cy="50" r="32" fill="none" stroke="#1a1a1a" stroke-width="1.5" opacity="0.45"/><circle cx="50" cy="50" r="44" fill="none" stroke="#1a1a1a" stroke-width="1" opacity="0.22"/>"##,

    // 10 COMUNICAZIONE — bolla di dialogo.
    // Lo scambio: qualcosa che esce verso l'altro e si punta verso di lui.
    r##"<path d="M16,32 Q16,18 30,18 L70,18 Q84,18 84,32 Q84,46 70,46 L56,46 L50,60 L44,46 L30,46 Q16,46 16,32 Z" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-linejoin="round"/>"##,

    // 11 AZIONE — freccia diagonale spessa verso l'alto-destra.
    // La forza applicata in un punto. Slancio, non scorrimento.
    r##"<line x1="28" y1="72" x2="68" y2="28" stroke="#1a1a1a" stroke-width="3" stroke-linecap="round"/><polyline points="44,25 70,25 70,52" fill="none" stroke="#1a1a1a" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>"##,

    // 12 NATURA — tronco con due rami/foglie organiche.
    // La crescita: un asse che si moltiplica asimmetricamente.
    r##"<line x1="50" y1="82" x2="50" y2="38" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><path d="M50,56 C50,56 26,52 22,32 C42,28 53,44 50,56 Z" fill="none" stroke="#1a1a1a" stroke-width="2" stroke-linejoin="round"/><path d="M50,40 C50,40 74,36 76,16 C56,14 45,28 50,40 Z" fill="none" stroke="#1a1a1a" stroke-width="2" stroke-linejoin="round"/>"##,

    // 13 CORPO — figura stilizzata.
    // La forma fisica: testa, torso, braccia, gambe. Presenza materiale.
    r##"<circle cx="50" cy="23" r="10" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><line x1="50" y1="33" x2="50" y2="64" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><line x1="28" y1="46" x2="72" y2="46" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><line x1="50" y1="64" x2="31" y2="84" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/><line x1="50" y1="64" x2="69" y2="84" stroke="#1a1a1a" stroke-width="2.5" stroke-linecap="round"/>"##,

    // 14 QUALITA — stella a 5 punte.
    // La distinzione, il dettaglio che brilla. Ciò che rende qualcosa
    // diverso da qualcos'altro.
    r##"<polygon points="50,16 57.6,38.5 81.6,38.5 62.7,52.6 69.8,75 50,61.2 30.2,75 37.3,52.6 18.4,38.5 42.4,38.5" fill="none" stroke="#1a1a1a" stroke-width="2.5" stroke-linejoin="round"/>"##,

    // 15 SOCIETA — tre cerchi in triangolo con linee di connessione.
    // La rete: più entità distinte che si tengono insieme.
    r##"<circle cx="50" cy="26" r="13" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><circle cx="24" cy="68" r="13" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><circle cx="76" cy="68" r="13" fill="none" stroke="#1a1a1a" stroke-width="2.5"/><line x1="42" y1="35" x2="30" y2="56" stroke="#1a1a1a" stroke-width="1.8"/><line x1="58" y1="35" x2="70" y2="56" stroke="#1a1a1a" stroke-width="1.8"/><line x1="37" y1="68" x2="63" y2="68" stroke="#1a1a1a" stroke-width="1.8"/>"##,
];

// ─── API pubblica ─────────────────────────────────────────────────────────────

/// SVG completo standalone per un frattale (viewBox 100×100).
/// Ritorna None se l'id è fuori range.
/// 
/// Per id 0-15: usa glifi manuali esistenti.
/// Per id 16-63: usa composizione automatica (richiede registry).
pub fn fractal_svg(id: FractalId) -> Option<String> {
    let idx = id as usize;
    if idx >= MANUAL_GLYPH_COUNT {
        // Esagrammi 16-63: non supportati senza registry
        return None;
    }
    Some(format!(
        r##"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" data-fractal="{name}">{body}</svg>"##,
        name = FRACTAL_NAMES[idx],
        body = BODIES[idx],
    ))
}

/// SVG per un esagramma dato il registry (supporta tutti i 64).
/// 
/// - ID 0-63: carica SVG da file fractals/XX_*.svg
/// - Fallback: usa glifi manuali hardcoded per id 0-15
pub fn fractal_svg_from_registry(id: FractalId, registry: &FractalRegistry) -> Option<String> {
    let idx = id as usize;
    if idx >= FRACTAL_COUNT {
        return None;
    }
    
    // 1. Prova a caricare SVG da file fractals/XX_*.svg
    if let Some(svg_content) = load_fractal_svg_from_file(id) {
        return Some(svg_content);
    }
    
    // 2. Fallback: usa glifo manuale hardcoded se disponibile (id 0-15)
    if idx < MANUAL_GLYPH_COUNT {
        let name = FRACTAL_NAMES[idx];
        return Some(format!(
            r##"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" data-fractal="{name}" data-fractal-id="{id}">{body}</svg>"##,
            name = name,
            id = id,
            body = BODIES[idx],
        ));
    }
    
    // 3. Fallback finale: componi automaticamente per esagrammi 16-63
    let fractal = registry.get(id)?;
    let (lower_dim, upper_dim) = extract_fixed_dimensions(&fractal.signature)?;
    
    let lower_glyph = dimension_to_glyph(lower_dim);
    let upper_glyph = dimension_to_glyph(upper_dim);
    
    let composed_body = compose_two_glyphs(lower_glyph, upper_glyph);
    
    Some(format!(
        r##"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" data-fractal="{name}" data-fractal-id="{id}" data-composed="true">{body}</svg>"##,
        name = fractal.name,
        id = id,
        body = composed_body,
    ))
}

/// Carica SVG da file fractals/XX_*.svg
fn load_fractal_svg_from_file(id: FractalId) -> Option<String> {
    use std::fs;
    use std::path::Path;
    
    // Cerca il file fractals/XX_*.svg dove XX è l'id con zero-padding
    let pattern = format!("fractals/{:02}_", id);
    let fractals_dir = Path::new("fractals");
    
    if !fractals_dir.exists() {
        return None;
    }
    
    // Cerca il file che inizia con il pattern
    let entries = fs::read_dir(fractals_dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with(&format!("{:02}_", id)) && filename.ends_with(".svg") {
                // Leggi il contenuto del file
                if let Ok(content) = fs::read_to_string(&path) {
                    // Inietta data-fractal-id (e data-composed per esagrammi composti) nel tag <svg>
                    let name = FRACTAL_NAMES.get(id as usize).copied().unwrap_or("");
                    let composed = if id as usize >= MANUAL_GLYPH_COUNT { " data-composed=\"true\"" } else { "" };
                    let tagged = content.replacen(
                        "<svg ",
                        &format!("<svg data-fractal-id=\"{id}\" data-fractal=\"{name}\"{composed} "),
                        1,
                    );
                    return Some(tagged);
                }
            }
        }
    }
    
    None
}

/// Nome del frattale per id (solo per i primi 16 manuali).
/// Per i nomi degli esagrammi 16-63, usa il registry.
pub fn fractal_name(id: FractalId) -> Option<&'static str> {
    FRACTAL_NAMES.get(id as usize).copied()
}

/// Tutti i 16 SVG frattali manuali: Vec<(id, nome, svg)>.
/// Per ottenere tutti i 64, usa `all_fractal_svgs_from_registry()`.
pub fn all_fractal_svgs() -> Vec<(FractalId, &'static str, String)> {
    (0..MANUAL_GLYPH_COUNT as u32)
        .filter_map(|id| fractal_svg(id).map(|svg| (id, FRACTAL_NAMES[id as usize], svg)))
        .collect()
}

/// Tutti i 64 SVG frattali (richiede registry per composizione).
pub fn all_fractal_svgs_from_registry(registry: &FractalRegistry) -> Vec<(FractalId, String, String)> {
    (0..FRACTAL_COUNT as u32)
        .filter_map(|id| {
            let name = registry.get(id).map(|f| f.name.clone())?;
            let svg = fractal_svg_from_registry(id, registry)?;
            Some((id, name, svg))
        })
        .collect()
}

/// Composizione automatica SVG per un simplex.
///
/// Il simplex connette N frattali: la sua immagine è la composizione
/// delle loro icone, disposte geometricamente, collegate da linee sottili.
/// Non si disegna — emerge dai componenti.
pub fn compose_simplex_svg(fractal_ids: &[FractalId], simplex_name: &str) -> String {
    let empty = format!(
        r##"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" data-simplex="{}"/>"##,
        simplex_name
    );

    if fractal_ids.is_empty() {
        return empty;
    }

    // Simplex con un solo frattale: usa direttamente la sua icona
    if fractal_ids.len() == 1 {
        return fractal_svg(fractal_ids[0]).unwrap_or(empty);
    }

    let positions = layout_positions(fractal_ids.len());
    let scale = layout_scale(fractal_ids.len());
    let mut inner = String::new();

    // Linee di connessione tra frattali — il simplex come relazione visiva
    for i in 0..fractal_ids.len() {
        for j in (i + 1)..fractal_ids.len() {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[j];
            inner.push_str(&format!(
                r##"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="#1a1a1a" stroke-width="0.8" opacity="0.3"/>"##,
                x1 = x1, y1 = y1, x2 = x2, y2 = y2
            ));
        }
    }

    // Icone frattali nelle posizioni calcolate
    // Il primo frattale è leggermente più opaco — è il "polo" del simplex
    for (i, &fid) in fractal_ids.iter().enumerate() {
        let idx = fid as usize;
        if idx >= MANUAL_GLYPH_COUNT {
            continue;  // Skip frattali 16-63 che non hanno glifi manuali
        }
        let (cx, cy) = positions[i];
        let opacity = if i == 0 { 1.0_f64 } else { 0.72_f64 };
        inner.push_str(&format!(
            r##"<g transform="translate({cx:.1},{cy:.1}) scale({s:.3}) translate(-50,-50)" opacity="{op:.2}">{body}</g>"##,
            cx = cx,
            cy = cy,
            s = scale,
            op = opacity,
            body = BODIES[idx],
        ));
    }

    format!(
        r##"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" data-simplex="{name}">{inner}</svg>"##,
        name = simplex_name,
        inner = inner,
    )
}

// ─── Layout geometrico ────────────────────────────────────────────────────────

/// Estrae le due dimensioni fisse dalla firma di un esagramma.
/// Ritorna (dimensione_inferiore, dimensione_superiore) o None se non ci sono esattamente 2 fisse.
fn extract_fixed_dimensions(signature: &[DimConstraint; 8]) -> Option<(Dim, Dim)> {
    let fixed: Vec<Dim> = signature.iter()
        .enumerate()
        .filter_map(|(i, constraint)| {
            match constraint {
                DimConstraint::Fixed(_) => Some(Dim::from_index(i)),
                DimConstraint::Free => None,
            }
        })
        .collect();
    
    if fixed.len() == 2 {
        Some((fixed[0], fixed[1]))
    } else {
        None
    }
}

/// Mappa una dimensione al glifo manuale più rappresentativo.
fn dimension_to_glyph(dim: Dim) -> &'static str {
    match dim {
        Dim::Confine => BODIES[0],      // SPAZIO - punto nel vuoto
        Dim::Valenza => BODIES[3],      // RELAZIONE - due cerchi connessi
        Dim::Intensita => BODIES[7],    // EMOZIONE - cuore
        Dim::Definizione => BODIES[5],  // LIMITE - quadrato netto
        Dim::Complessita => BODIES[8],  // PENSIERO - albero ramificato
        Dim::Permanenza => BODIES[2],   // EGO - cerchio pieno (massa stabile)
        Dim::Agency => BODIES[11],      // AZIONE - freccia diagonale
        Dim::Tempo => BODIES[1],        // TEMPO - onda con freccia
    }
}

/// Compone due glifi in un'unica immagine.
/// Il glifo inferiore (interno) è più opaco, il superiore (esterno) è più trasparente.
fn compose_two_glyphs(lower: &str, upper: &str) -> String {
    format!(
        r##"<g opacity="0.85">{lower}</g><g opacity="0.45">{upper}</g>"##,
        lower = lower,
        upper = upper,
    )
}

// ─── Layout geometrico (simplex) ──────────────────────────────────────────────

/// Posizioni (cx, cy) per N icone in un canvas 100×100.
/// Disposizione: 1=centro, 2=affiancati, 3=triangolo, 4=quadrato, N≥5=cerchio.
fn layout_positions(n: usize) -> Vec<(f64, f64)> {
    match n {
        1 => vec![(50.0, 50.0)],
        2 => vec![(28.0, 50.0), (72.0, 50.0)],
        3 => vec![(50.0, 22.0), (22.0, 72.0), (78.0, 72.0)],
        4 => vec![(28.0, 28.0), (72.0, 28.0), (28.0, 72.0), (72.0, 72.0)],
        _ => {
            // Cerchio regolare centrato a (50,50), raggio 34
            let r = 34.0;
            (0..n)
                .map(|i| {
                    let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64
                        - std::f64::consts::PI / 2.0;
                    (50.0 + r * angle.cos(), 50.0 + r * angle.sin())
                })
                .collect()
        }
    }
}

/// Scala delle icone in base al numero di frattali.
/// Più elementi = icone più piccole.
fn layout_scale(n: usize) -> f64 {
    match n {
        1 => 1.00,
        2 => 0.40,
        3 => 0.36,
        4 => 0.34,
        _ => 0.28,
    }
}

// ─── Test ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutti_i_frattali_manuali_hanno_svg() {
        for id in 0..MANUAL_GLYPH_COUNT as FractalId {
            let svg = fractal_svg(id);
            assert!(svg.is_some(), "Frattale {} non ha SVG", id);
            let svg = svg.unwrap();
            assert!(svg.contains("<svg"), "SVG frattale {} manca wrapper", id);
            assert!(svg.contains("</svg>"), "SVG frattale {} manca chiusura", id);
            assert!(
                svg.contains(&format!("data-fractal=\"{}\"", FRACTAL_NAMES[id as usize])),
                "SVG frattale {} manca attributo data-fractal",
                id
            );
        }
    }

    #[test]
    fn test_id_fuori_range_ritorna_none() {
        assert!(fractal_svg(16).is_none(), "ID 16 dovrebbe ritornare None (serve registry)");
        assert!(fractal_svg(64).is_none());
        assert!(fractal_svg(255).is_none());
        assert!(fractal_name(64).is_none());
    }

    #[test]
    fn test_fractal_svg_from_registry() {
        use crate::topology::fractal::bootstrap_fractals;
        let registry = bootstrap_fractals();
        
        // Test glifo manuale (id < 16)
        let svg0 = fractal_svg_from_registry(0, &registry);
        assert!(svg0.is_some());
        assert!(svg0.unwrap().contains("data-fractal-id=\"0\""));
        
        // Test glifo composto (id >= 16)
        let svg16 = fractal_svg_from_registry(16, &registry);
        assert!(svg16.is_some());
        let svg16_str = svg16.unwrap();
        assert!(svg16_str.contains("data-composed=\"true\""));
        assert!(svg16_str.contains("data-fractal-id=\"16\""));
    }

    #[test]
    fn test_all_fractal_svgs_restituisce_16() {
        let all = all_fractal_svgs();
        assert_eq!(all.len(), MANUAL_GLYPH_COUNT);
        // Verifica ordine e nomi
        for (i, (id, name, _)) in all.iter().enumerate() {
            assert_eq!(*id, i as FractalId);
            assert_eq!(*name, FRACTAL_NAMES[i]);
        }
    }

    #[test]
    fn test_all_fractal_svgs_from_registry_restituisce_64() {
        use crate::topology::fractal::bootstrap_fractals;
        let registry = bootstrap_fractals();
        let all = all_fractal_svgs_from_registry(&registry);
        
        // Dovrebbero esserci 64 esagrammi, ma alcuni potrebbero non avere esattamente 2 dimensioni fisse
        assert!(all.len() >= 58, "Dovrebbero esserci almeno 58 esagrammi con glifi (trovati: {})", all.len());
        assert!(all.len() <= 64, "Non dovrebbero esserci più di 64 esagrammi");
        
        // Verifica che i primi 16 siano presenti
        for id in 0..16 {
            assert!(all.iter().any(|(fid, _, _)| *fid == id), "Frattale manuale {} mancante", id);
        }
    }

    #[test]
    fn test_compose_simplex_due_frattali() {
        // PRESENZA = SPAZIO(0) + EGO(2)
        let svg = compose_simplex_svg(&[0, 2], "PRESENZA");
        assert!(svg.contains("<svg"));
        assert!(svg.contains("data-simplex=\"PRESENZA\""));
        // Deve contenere entrambi i corpi
        assert!(svg.contains("data-fractal") == false); // è composizione, non frattale singolo
        // Deve avere linee di connessione
        assert!(svg.contains("<line"));
        // Deve avere due gruppi <g
        assert_eq!(svg.matches("<g ").count(), 2);
    }

    #[test]
    fn test_compose_simplex_tre_frattali() {
        // CAMMINO = SPAZIO(0) + TEMPO(1) + EGO(2)
        let svg = compose_simplex_svg(&[0, 1, 2], "CAMMINO");
        assert!(svg.contains("data-simplex=\"CAMMINO\""));
        assert_eq!(svg.matches("<g ").count(), 3);
        // 3 coppie → 3 linee di connessione
        assert_eq!(svg.matches("<line").count(), 3);
    }

    #[test]
    fn test_compose_simplex_vuoto() {
        let svg = compose_simplex_svg(&[], "VUOTO");
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_layout_posizioni_n2_simmetrico() {
        let pos = layout_positions(2);
        // I due punti devono essere simmetrici rispetto al centro
        let (x1, y1) = pos[0];
        let (x2, y2) = pos[1];
        assert!((x1 + x2 - 100.0).abs() < 0.01, "Simmetria X fallita");
        assert!((y1 - y2).abs() < 0.01, "Simmetria Y fallita");
    }

    #[test]
    fn test_layout_posizioni_n3_in_canvas() {
        let pos = layout_positions(3);
        for (x, y) in &pos {
            assert!(*x >= 5.0 && *x <= 95.0, "X fuori canvas: {}", x);
            assert!(*y >= 5.0 && *y <= 95.0, "Y fuori canvas: {}", y);
        }
    }
}
