/// Visual Perception — Esperimento: Prometeo può "vedere" SVG?
///
/// SVG è già topologico: forme, posizioni, relazioni spaziali.
/// Questo modulo traduce elementi SVG in attivazioni del campo parole.
/// Test: se Prometeo ha vocabolario geometrico, dovrebbe descrivere l'immagine.

use crate::topology::engine::PrometeoTopologyEngine;
use std::collections::HashMap;

/// Un concetto visivo estratto da SVG
#[derive(Debug, Clone)]
pub struct VisualConcept {
    pub shape: String,           // "cerchio", "quadrato", "linea"
    pub position: (f64, f64),    // coordinate (x, y)
    pub size: f64,               // dimensione (raggio, lato, lunghezza)
    pub color: Option<String>,   // "rosso", "blu", "verde"...
    pub relations: Vec<String>,  // "sopra", "dentro", "vicino"...
}

/// Risposta percettiva dopo aver "visto" un SVG
#[derive(Debug)]
pub struct PerceptualResponse {
    pub concepts_detected: usize,
    pub words_activated: Vec<String>,
    pub dominant_fractals: Vec<String>,
    pub description: String,
    pub field_energy: f64,
}

/// Parser SVG minimale — estrae forme base
pub fn parse_svg_simple(svg: &str) -> Vec<VisualConcept> {
    let mut concepts = Vec::new();

    // Estrae il valore di un attributo dalla stringa degli attributi di un tag SVG.
    // Usa " nome=" come prefisso per evitare match accidentali dentro altri attributi
    // (es. "x" dentro "cx", "r" dentro "stroke"). Il primo attributo nel tag ha sempre
    // uno spazio prima perché segue il nome dell'elemento: <circle cx=...>.
    fn get_attr<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
        let prefix = format!(" {}=\"", name);
        let start = attrs.find(prefix.as_str())? + prefix.len();
        let end = attrs[start..].find('"')? + start;
        Some(&attrs[start..end])
    }

    fn pf(s: &str) -> f64 { s.trim().parse().unwrap_or(0.0) }

    // Circle: <circle ... cx="X" cy="Y" r="R" fill="COLOR" .../>
    for cap in regex::Regex::new(r"(?s)<circle([^>]*/?)>").unwrap().captures_iter(svg) {
        let a = &cap[1];
        let cx = get_attr(a, "cx").map(pf).unwrap_or(0.0);
        let cy = get_attr(a, "cy").map(pf).unwrap_or(0.0);
        let r  = get_attr(a, "r").map(pf).unwrap_or(0.0);
        let color = get_attr(a, "fill")
            .or_else(|| get_attr(a, "stroke"))
            .map(|c| parse_color(c));
        concepts.push(VisualConcept {
            shape: "cerchio".to_string(),
            position: (cx, cy),
            size: r,
            color,
            relations: vec![],
        });
    }

    // Rect: <rect ... x="X" y="Y" width="W" height="H" fill="COLOR" .../>
    for cap in regex::Regex::new(r"(?s)<rect([^>]*/?)>").unwrap().captures_iter(svg) {
        let a = &cap[1];
        let x = get_attr(a, "x").map(pf).unwrap_or(0.0);
        let y = get_attr(a, "y").map(pf).unwrap_or(0.0);
        let w = get_attr(a, "width").map(pf).unwrap_or(0.0);
        let h = get_attr(a, "height").map(pf).unwrap_or(0.0);
        let color = get_attr(a, "fill")
            .or_else(|| get_attr(a, "stroke"))
            .map(|c| parse_color(c));
        let shape = if (w - h).abs() < 5.0 { "quadrato" } else { "rettangolo" };
        concepts.push(VisualConcept {
            shape: shape.to_string(),
            position: (x + w / 2.0, y + h / 2.0),
            size: w.max(h),
            color,
            relations: vec![],
        });
    }

    // Line: <line ... x1="X1" y1="Y1" x2="X2" y2="Y2" stroke="COLOR" .../>
    for cap in regex::Regex::new(r"(?s)<line([^>]*/?)>").unwrap().captures_iter(svg) {
        let a = &cap[1];
        let x1 = get_attr(a, "x1").map(pf).unwrap_or(0.0);
        let y1 = get_attr(a, "y1").map(pf).unwrap_or(0.0);
        let x2 = get_attr(a, "x2").map(pf).unwrap_or(0.0);
        let y2 = get_attr(a, "y2").map(pf).unwrap_or(0.0);
        let length = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        concepts.push(VisualConcept {
            shape: "linea".to_string(),
            position: ((x1 + x2) / 2.0, (y1 + y2) / 2.0),
            size: length,
            color: None,
            relations: vec![],
        });
    }

    compute_spatial_relations(&mut concepts);
    concepts
}

/// Traduce colori SVG in parole italiane
fn parse_color(color_str: &str) -> String {
    match color_str.to_lowercase().as_str() {
        "red" | "#ff0000" | "#f00" | "rgb(255,0,0)" => "rosso",
        "blue" | "#0000ff" | "#00f" | "rgb(0,0,255)" => "blu",
        "green" | "#00ff00" | "#0f0" | "rgb(0,255,0)" => "verde",
        "yellow" | "#ffff00" | "#ff0" | "rgb(255,255,0)" => "giallo",
        "black" | "#000000" | "#000" | "rgb(0,0,0)" => "nero",
        "white" | "#ffffff" | "#fff" | "rgb(255,255,255)" => "bianco",
        "orange" | "#ffa500" | "rgb(255,165,0)" => "arancione",
        "purple" | "#800080" | "rgb(128,0,128)" => "viola",
        "pink" | "#ffc0cb" | "rgb(255,192,203)" => "rosa",
        "brown" | "#a52a2a" | "rgb(165,42,42)" => "marrone",
        "gray" | "grey" | "#808080" | "rgb(128,128,128)" => "grigio",
        _ => "colore", // fallback generico
    }.to_string()
}

/// Calcola relazioni spaziali tra concetti (sopra, sotto, vicino...)
fn compute_spatial_relations(concepts: &mut [VisualConcept]) {
    let n = concepts.len();
    if n < 2 { return; }
    
    for i in 0..n {
        for j in (i+1)..n {
            let (x1, y1) = concepts[i].position;
            let (x2, y2) = concepts[j].position;
            
            let dx = x2 - x1;
            let dy = y2 - y1;
            let dist = (dx*dx + dy*dy).sqrt();
            
            // Vicino se distanza < somma raggi * 2
            let threshold = (concepts[i].size + concepts[j].size) * 2.0;
            if dist < threshold {
                concepts[i].relations.push("vicino".to_string());
                concepts[j].relations.push("vicino".to_string());
            }
            
            // Sopra/sotto se differenza Y significativa
            if dy.abs() > dx.abs() && dy.abs() > 20.0 {
                if dy > 0.0 {
                    concepts[i].relations.push("sopra".to_string());
                    concepts[j].relations.push("sotto".to_string());
                } else {
                    concepts[i].relations.push("sotto".to_string());
                    concepts[j].relations.push("sopra".to_string());
                }
            }
        }
    }
}

/// Estensione per PrometeoTopologyEngine: percezione visiva
impl PrometeoTopologyEngine {
    /// Percepisce un SVG: traduce elementi in attivazioni campo parole
    pub fn perceive_svg(&mut self, svg: &str) -> PerceptualResponse {
        // Parse SVG
        let concepts = parse_svg_simple(svg);
        
        if concepts.is_empty() {
            return PerceptualResponse {
                concepts_detected: 0,
                words_activated: vec![],
                dominant_fractals: vec![],
                description: "niente".to_string(),
                field_energy: 0.0,
            };
        }
        
        // Costruisci frase descrittiva dall'SVG per usare receive()
        // Questo crea contesto conversazionale completo
        let mut description_words = Vec::new();
        
        for concept in &concepts {
            description_words.push(concept.shape.clone());
            
            if let Some(ref color) = concept.color {
                description_words.push(color.clone());
            }
            
            let size_word = if concept.size < 20.0 {
                "piccolo"
            } else if concept.size < 50.0 {
                "medio"
            } else {
                "grande"
            };
            description_words.push(size_word.to_string());
            
            description_words.push("qui".to_string());
            
            for rel in &concept.relations {
                if !description_words.contains(rel) {
                    description_words.push(rel.clone());
                }
            }
        }
        
        // Usa receive() per processare come input reale
        // Questo attiva tutto: campo, will, memoria, locus
        let visual_input = description_words.join(" ");
        let activated_words = description_words.clone();
        
        let _ = self.receive(&visual_input);
        
        // Raccogli frattali dominanti
        let active_fractals = self.word_topology.emerge_fractal_activations(&self.lexicon);
        let mut dominant_fractals: Vec<_> = active_fractals.iter()
            .filter(|(_, act)| *act > 0.3)
            .map(|(fid, _)| {
                self.registry.get(*fid)
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| format!("F{}", fid))
            })
            .collect();
        dominant_fractals.sort();
        
        // Genera descrizione emergente
        let generated = self.generate_willed();
        
        // Energia campo
        let field_energy = self.word_topology.field_energy();
        
        PerceptualResponse {
            concepts_detected: concepts.len(),
            words_activated: activated_words,
            dominant_fractals,
            description: generated.text,
            field_energy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_circle() {
        let svg = r#"<svg><circle cx="50" cy="50" r="20" fill="red"/></svg>"#;
        let concepts = parse_svg_simple(svg);
        
        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].shape, "cerchio");
        assert_eq!(concepts[0].position, (50.0, 50.0));
        assert_eq!(concepts[0].size, 20.0);
        assert_eq!(concepts[0].color, Some("rosso".to_string()));
    }
    
    #[test]
    fn test_parse_rect() {
        let svg = r#"<svg><rect x="10" y="10" width="30" height="30" fill="blue"/></svg>"#;
        let concepts = parse_svg_simple(svg);
        
        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].shape, "quadrato");
        assert_eq!(concepts[0].color, Some("blu".to_string()));
    }
    
    #[test]
    fn test_spatial_relations() {
        let svg = r#"<svg>
            <circle cx="50" cy="20" r="10" fill="red"/>
            <circle cx="50" cy="80" r="10" fill="blue"/>
        </svg>"#;
        let concepts = parse_svg_simple(svg);
        
        assert_eq!(concepts.len(), 2);
        assert!(concepts[0].relations.contains(&"sopra".to_string()));
        assert!(concepts[1].relations.contains(&"sotto".to_string()));
    }
}
