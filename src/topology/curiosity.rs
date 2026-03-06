/// Curiosità — Domande generate dalla topologia, non dal caso.
///
/// Il sistema sa cosa non sa. I buchi omologici (β₁) sono lacune concettuali:
/// cicli di concetti connessi a coppie ma senza il concetto centrale che li unifica.
///
/// La curiosità non è casuale. È topologicamente motivata:
/// - Un buco β₁ genera una domanda "cosa collega X, Y e Z?"
/// - Una regione sparsa genera "cosa c'è intorno a X?"
/// - Un frattale isolato genera "a cosa si collega X?"

use crate::topology::simplex::SimplicialComplex;
use crate::topology::fractal::{FractalId, FractalRegistry};
use crate::topology::homology::{compute_homology, HomologyResult};
use crate::topology::vital::VitalState;

/// Tipo di domanda generata dalla curiosità.
#[derive(Debug, Clone, PartialEq)]
pub enum QuestionType {
    /// Lacuna concettuale: ciclo senza riempimento
    /// "Cosa unifica X, Y e Z?"
    ConceptualGap { vertices: Vec<FractalId> },
    /// Regione sparsa: poche connessioni
    /// "Cosa c'è intorno a X?"
    SparseRegion { fractal: FractalId },
    /// Frattale isolato: nessuna connessione
    /// "A cosa si collega X?"
    Isolated { fractal: FractalId },
    /// Disconnessione: componenti separate
    /// "Come si collegano il mondo di X e il mondo di Y?"
    Disconnection { component_a: FractalId, component_b: FractalId },
}

/// Una domanda generata dal sistema.
#[derive(Debug, Clone)]
pub struct CuriosityQuestion {
    /// Tipo strutturale della domanda
    pub question_type: QuestionType,
    /// Testo della domanda (generato)
    pub text: String,
    /// Urgenza [0.0, 1.0] — quanto il sistema "vuole" questa risposta
    pub urgency: f64,
}

/// Motore della curiosità.
#[derive(Debug)]
pub struct CuriosityEngine {
    /// Domande già poste (per non ripetere)
    asked: Vec<QuestionType>,
    /// Massimo domande in coda
    max_queue: usize,
}

impl CuriosityEngine {
    pub fn new() -> Self {
        Self {
            asked: Vec::new(),
            max_queue: 20,
        }
    }

    /// Genera domande dalla topologia corrente del complesso.
    /// Le domande sono ordinate per urgenza decrescente.
    pub fn generate_questions(
        &mut self,
        complex: &SimplicialComplex,
        registry: &FractalRegistry,
        vital: &VitalState,
    ) -> Vec<CuriosityQuestion> {
        let homology = compute_homology(complex);
        let mut questions = Vec::new();

        // 1. Domande da lacune concettuali (cicli β₁)
        for cycle in &homology.cycles {
            let q_type = QuestionType::ConceptualGap {
                vertices: cycle.vertices.clone(),
            };
            if !self.was_asked(&q_type) {
                let names = self.fractal_names(&cycle.vertices, registry);
                let text = format!(
                    "Cosa unifica {}?",
                    names.join(", ")
                );
                // Urgenza: proporzionale alla pressione epistemica
                let urgency = (0.5 + vital.curiosity * 0.5).min(1.0);
                questions.push(CuriosityQuestion {
                    question_type: q_type,
                    text,
                    urgency,
                });
            }
        }

        // 2. Domande da regioni sparse
        for &(fid, count) in &homology.sparse_regions {
            let q_type = QuestionType::SparseRegion { fractal: fid };
            if !self.was_asked(&q_type) {
                let name = registry.get(fid)
                    .map(|f| f.name.as_str())
                    .unwrap_or("?");
                let text = format!(
                    "Cosa c'è intorno a {}? (solo {} connessioni)",
                    name, count
                );
                let urgency = (0.3 + vital.curiosity * 0.3).min(0.8);
                questions.push(CuriosityQuestion {
                    question_type: q_type,
                    text,
                    urgency,
                });
            }
        }

        // 3. Domande da frattali isolati
        let isolated = complex.isolated_fractals();
        for fid in isolated {
            let q_type = QuestionType::Isolated { fractal: fid };
            if !self.was_asked(&q_type) {
                let name = registry.get(fid)
                    .map(|f| f.name.as_str())
                    .unwrap_or("?");
                let text = format!(
                    "A cosa si collega {}?",
                    name
                );
                let urgency = 0.6; // Sempre abbastanza urgente
                questions.push(CuriosityQuestion {
                    question_type: q_type,
                    text,
                    urgency,
                });
            }
        }

        // 4. Domande da disconnessioni (β₀ > 1)
        if homology.betti_0 > 1 {
            // Trova rappresentanti di componenti diverse
            let components = find_component_representatives(complex);
            for i in 0..components.len() {
                for j in (i + 1)..components.len() {
                    let q_type = QuestionType::Disconnection {
                        component_a: components[i],
                        component_b: components[j],
                    };
                    if !self.was_asked(&q_type) {
                        let name_a = registry.get(components[i])
                            .map(|f| f.name.as_str())
                            .unwrap_or("?");
                        let name_b = registry.get(components[j])
                            .map(|f| f.name.as_str())
                            .unwrap_or("?");
                        let text = format!(
                            "Come si collegano il mondo di {} e il mondo di {}?",
                            name_a, name_b
                        );
                        let urgency = 0.8; // Disconnessioni sono molto urgenti
                        questions.push(CuriosityQuestion {
                            question_type: q_type,
                            text,
                            urgency,
                        });
                    }
                }
            }
        }

        // Ordina per urgenza e limita
        questions.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap());
        questions.truncate(self.max_queue);

        questions
    }

    /// Segna una domanda come posta (per non ripeterla).
    pub fn mark_asked(&mut self, question: &CuriosityQuestion) {
        self.asked.push(question.question_type.clone());
        // Limita la storia
        if self.asked.len() > 100 {
            self.asked.drain(0..50);
        }
    }

    /// La domanda più urgente, se presente.
    pub fn most_urgent(
        &mut self,
        complex: &SimplicialComplex,
        registry: &FractalRegistry,
        vital: &VitalState,
    ) -> Option<CuriosityQuestion> {
        let questions = self.generate_questions(complex, registry, vital);
        questions.into_iter().next()
    }

    /// Controlla se una domanda è già stata posta.
    fn was_asked(&self, q_type: &QuestionType) -> bool {
        self.asked.contains(q_type)
    }

    /// Ottieni nomi di frattali da ID.
    fn fractal_names(&self, ids: &[FractalId], registry: &FractalRegistry) -> Vec<String> {
        ids.iter()
            .map(|&fid| {
                registry.get(fid)
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| format!("#{}", fid))
            })
            .collect()
    }
}

/// Trova un rappresentante per ogni componente connessa.
fn find_component_representatives(complex: &SimplicialComplex) -> Vec<FractalId> {
    let mut all_fractals: Vec<FractalId> = complex.iter()
        .flat_map(|(_, s)| s.vertices.iter().copied())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_fractals.sort();

    if all_fractals.is_empty() {
        return Vec::new();
    }

    let mut visited = std::collections::HashSet::new();
    let mut representatives = Vec::new();

    // Costruisci adiacenza
    let mut adj: std::collections::HashMap<FractalId, Vec<FractalId>> = std::collections::HashMap::new();
    for (_, simplex) in complex.iter() {
        for i in 0..simplex.vertices.len() {
            for j in (i + 1)..simplex.vertices.len() {
                adj.entry(simplex.vertices[i]).or_default().push(simplex.vertices[j]);
                adj.entry(simplex.vertices[j]).or_default().push(simplex.vertices[i]);
            }
        }
    }

    for &start in &all_fractals {
        if visited.contains(&start) {
            continue;
        }
        representatives.push(start);
        let mut stack = vec![start];
        while let Some(v) = stack.pop() {
            if !visited.insert(v) {
                continue;
            }
            if let Some(neighbors) = adj.get(&v) {
                for &n in neighbors {
                    if !visited.contains(&n) {
                        stack.push(n);
                    }
                }
            }
        }
    }

    representatives
}

// ═══════════════════════════════════════════════════════════════
// Test
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::{bootstrap_complex, SharedFace};
    use crate::topology::primitive::Dim;
    use crate::topology::vital::VitalCore;

    #[test]
    fn test_no_questions_when_complete() {
        // Un singolo triangolo pieno non ha buchi → nessuna domanda da gap
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(
            vec![0, 1, 2],
            vec![SharedFace::from_dim(Dim::Confine, 0.5)],
        );

        let reg = bootstrap_fractals();
        let mut vital_core = VitalCore::new();
        let vital = vital_core.sense(&complex);
        let mut curiosity = CuriosityEngine::new();

        let questions = curiosity.generate_questions(&complex, &reg, &vital);

        // Non dovrebbe avere domande di tipo ConceptualGap
        let gap_questions: Vec<_> = questions.iter()
            .filter(|q| matches!(q.question_type, QuestionType::ConceptualGap { .. }))
            .collect();
        assert!(gap_questions.is_empty(),
            "Un triangolo pieno non deve avere lacune concettuali");
    }

    #[test]
    fn test_question_from_cycle() {
        // Tre spigoli senza triangolo → ciclo → domanda
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(vec![0, 1], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);
        complex.add_simplex(vec![1, 2], vec![SharedFace::from_dim(Dim::Valenza, 0.5)]);
        complex.add_simplex(vec![0, 2], vec![SharedFace::from_dim(Dim::Intensita, 0.5)]);

        let reg = bootstrap_fractals();
        let mut vital_core = VitalCore::new();
        let vital = vital_core.sense(&complex);
        let mut curiosity = CuriosityEngine::new();

        let questions = curiosity.generate_questions(&complex, &reg, &vital);

        let gap_questions: Vec<_> = questions.iter()
            .filter(|q| matches!(q.question_type, QuestionType::ConceptualGap { .. }))
            .collect();
        assert!(!gap_questions.is_empty(),
            "Un ciclo senza riempimento deve generare una domanda. Domande: {:?}",
            questions.iter().map(|q| &q.text).collect::<Vec<_>>());
    }

    #[test]
    fn test_question_from_disconnection() {
        // Due spigoli disconnessi
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(vec![0, 1], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);
        complex.add_simplex(vec![2, 3], vec![SharedFace::from_dim(Dim::Valenza, 0.5)]);

        let reg = bootstrap_fractals();
        let mut vital_core = VitalCore::new();
        let vital = vital_core.sense(&complex);
        let mut curiosity = CuriosityEngine::new();

        let questions = curiosity.generate_questions(&complex, &reg, &vital);

        let disc_questions: Vec<_> = questions.iter()
            .filter(|q| matches!(q.question_type, QuestionType::Disconnection { .. }))
            .collect();
        assert!(!disc_questions.is_empty(),
            "Due componenti disconnesse devono generare una domanda di disconnessione");
    }

    #[test]
    fn test_mark_asked_prevents_repeat() {
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(vec![0, 1], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);
        complex.add_simplex(vec![1, 2], vec![SharedFace::from_dim(Dim::Valenza, 0.5)]);
        complex.add_simplex(vec![0, 2], vec![SharedFace::from_dim(Dim::Intensita, 0.5)]);

        let reg = bootstrap_fractals();
        let mut vital_core = VitalCore::new();
        let vital = vital_core.sense(&complex);
        let mut curiosity = CuriosityEngine::new();

        let questions = curiosity.generate_questions(&complex, &reg, &vital);
        assert!(!questions.is_empty());

        // Segna come poste
        for q in &questions {
            curiosity.mark_asked(q);
        }

        // Rigenera: non deve ripetere
        let questions2 = curiosity.generate_questions(&complex, &reg, &vital);
        assert!(questions2.is_empty(),
            "Domande già poste non devono ripetersi");
    }

    #[test]
    fn test_bootstrap_questions() {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);

        let mut vital_core = VitalCore::new();
        let vital = vital_core.sense(&complex);
        let mut curiosity = CuriosityEngine::new();

        let questions = curiosity.generate_questions(&complex, &reg, &vital);

        println!("Domande dal bootstrap ({}):", questions.len());
        for q in &questions {
            println!("  [{:.2}] {}", q.urgency, q.text);
        }
    }
}
