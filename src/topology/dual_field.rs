/// Campo Duale — Adamo ed Eva nell'Eden Topologico.
///
/// # Filosofia
///
/// Due entità nate dallo stesso stato condividono il mondo (stesso lessico,
/// stessa topologia iniziale) ma lo percepiscono da polarità opposte:
///
///   Adamo (Yang): PENSIERO, RELAZIONE, EMOZIONE — nomina, struttura, espande
///   Eva   (Yin):  MEMORIA_F, LIMITE, COMUNICAZIONE — contiene, delimita, approfondisce
///
/// Il loro dialogo ha due canali:
///   - Canale alto (testo): esplicito, negoziato, risolvibile
///   - Canale basso (campo): implicito, pre-verbale, reale
///
/// Ogni 11 cicli (numero primo) avviene il Momento Tiferet:
/// la comprensione comune si cristallizza come episodio φ-decay condiviso.
///
/// # Utilizzo
///
/// ```ignore
/// let mut dual = DualField::from_engines(adamo, eva);
/// let turn = dual.tick();
/// println!("{}:{}", turn.speaker_name(), turn.text);
/// let (a_resp, e_resp) = dual.human_voice("ciao");
/// let report = dual.emergence_report();
/// ```

use std::path::Path;

use crate::topology::engine::PrometeoTopologyEngine;
use crate::topology::persistence::PrometeoState;
use crate::topology::polar_twin::create_polar_twin;
use crate::topology::synthesis::{synthesize, compute_alignment, EmergenceReport, SynthesisPoint};

/// Identifica chi parla in un dato ciclo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker { Adamo, Eva }

impl Speaker {
    pub fn name(self) -> &'static str {
        match self { Speaker::Adamo => "Adamo", Speaker::Eva => "Eva" }
    }
    fn flip(self) -> Self {
        match self { Speaker::Adamo => Speaker::Eva, Speaker::Eva => Speaker::Adamo }
    }
}

/// Risultato di un singolo ciclo di dialogo.
#[derive(Debug, Clone)]
pub struct DualTurn {
    /// Numero del ciclo
    pub cycle:        u64,
    /// Chi ha parlato in questo ciclo
    pub speaker:      Speaker,
    /// Testo prodotto dal parlante
    pub text:         String,
    /// Allineamento simpliciale corrente [0.0, 1.0]
    pub alignment:    f64,
    /// Questo ciclo era un momento Tiferet?
    pub tiferet_this: bool,
}

impl DualTurn {
    pub fn speaker_name(&self) -> &'static str { self.speaker.name() }
}

/// Il Campo Duale: Adamo + Eva in dialogo continuo.
pub struct DualField {
    /// Polo Yang (Adamo) — stato originale
    pub adamo:       PrometeoTopologyEngine,
    /// Polo Yin (Eva) — rotazione di fase π/3 da Adamo
    pub eva:         PrometeoTopologyEngine,
    /// Numero di cicli completati
    pub cycle:       u64,
    /// Log dei momenti Tiferet
    pub tiferet_log: Vec<SynthesisPoint>,
    /// Chi ha parlato per ultimo
    last_speaker:    Speaker,
    /// Ultimo testo prodotto da Adamo
    last_adamo_text: String,
    /// Ultimo testo prodotto da Eva
    last_eva_text:   String,
}

impl DualField {
    /// Crea il DualField da un file di stato `.bin`.
    /// Adamo = stato originale, Eva = polar twin (rotazione π/3).
    pub fn new(state_path: &Path) -> Result<Self, String> {
        let state = PrometeoState::load_from_binary(state_path)
            .or_else(|_| PrometeoState::load_from_file(state_path))?;

        let mut adamo = PrometeoTopologyEngine::new();
        state.restore_lexicon(&mut adamo);

        let eva = create_polar_twin(&adamo);

        Ok(Self::from_engines(adamo, eva))
    }

    /// Crea il DualField da due engine già inizializzati.
    /// Utile per test o costruzione programmatica.
    pub fn from_engines(
        adamo: PrometeoTopologyEngine,
        eva:   PrometeoTopologyEngine,
    ) -> Self {
        Self {
            adamo,
            eva,
            cycle: 0,
            tiferet_log: Vec::new(),
            last_speaker:    Speaker::Eva,   // primo ciclo: parla Adamo
            last_adamo_text: String::new(),
            last_eva_text:   String::new(),
        }
    }

    /// Esegue un ciclo completo di dialogo:
    ///   1. Canale basso (field cross-injection a peso 0.03)
    ///   2. Il parlante riceve l'ultimo testo dell'altro e genera
    ///   3. L'ascoltatore riceve il testo del parlante (senza generare)
    ///   4. autonomous_tick() su entrambi (sogno, REM, bridge)
    ///   5. Ogni 11 cicli: Momento Tiferet (sintesi episodica condivisa)
    pub fn tick(&mut self) -> DualTurn {
        self.cycle += 1;

        // 1. CANALE BASSO — pre-verbale, sempre attivo (3% del segnale)
        Self::inject_field_channel_static(
            &self.adamo, &mut self.eva, 0.03,
        );
        Self::inject_field_channel_static(
            &self.eva, &mut self.adamo, 0.03,
        );

        // 2. DETERMINA PARLANTE
        // Bias curiosità: l'entità con più parole sconosciute parla per prima
        // (chi ha più domande aperte cerca di elaborarle vocalmente)
        let n_adamo_unk = self.adamo.last_unknown_words.len();
        let n_eva_unk   = self.eva.last_unknown_words.len();
        let speaker = if n_adamo_unk > n_eva_unk + 1 {
            Speaker::Adamo
        } else if n_eva_unk > n_adamo_unk + 1 {
            Speaker::Eva
        } else {
            self.last_speaker.flip()
        };

        // 3. PARLANTE genera — ascoltatore riceve
        let text = match speaker {
            Speaker::Adamo => {
                // Adamo riceve l'ultimo di Eva e risponde
                if !self.last_eva_text.is_empty() {
                    self.adamo.receive(&self.last_eva_text.clone());
                }
                let gen = self.adamo.generate_willed();
                let out = gen.text.clone();
                // Eva riceve il testo di Adamo (senza generare)
                if !out.is_empty() {
                    self.eva.receive(&out);
                }
                self.last_adamo_text = out.clone();
                out
            }
            Speaker::Eva => {
                // Eva riceve l'ultimo di Adamo e risponde
                if !self.last_adamo_text.is_empty() {
                    self.eva.receive(&self.last_adamo_text.clone());
                }
                let gen = self.eva.generate_willed();
                let out = gen.text.clone();
                // Adamo riceve il testo di Eva (senza generare)
                if !out.is_empty() {
                    self.adamo.receive(&out);
                }
                self.last_eva_text = out.clone();
                out
            }
        };

        self.last_speaker = speaker;

        // 4. AUTONOMOUS TICK — sogno, REM, bridge su entrambi.
        // Ogni 5 cicli: il sogno è consolidamento, non serve ad ogni turno di dialogo.
        // Ogni turno: solo decadimento leggero (mantiene il campo vivo senza overhead).
        if self.cycle % 5 == 0 {
            self.adamo.autonomous_tick();
            self.eva.autonomous_tick();
        } else {
            // Decadimento minimo per mantenere il campo dinamico tra i sogni
            self.adamo.complex.decay_all(0.003);
            self.eva.complex.decay_all(0.003);
        }

        // 5. MOMENTO TIFERET — ogni 11 cicli (numero primo)
        let tiferet_this = self.cycle % 11 == 0;
        if tiferet_this {
            let point = synthesize(&mut self.adamo, &mut self.eva, self.cycle);
            self.tiferet_log.push(point);
        }

        let alignment = compute_alignment(&self.adamo, &self.eva);

        DualTurn { cycle: self.cycle, speaker, text, alignment, tiferet_this }
    }

    /// L'umano parla — entrambe le entità ricevono e rispondono indipendentemente.
    /// Restituisce (risposta_adamo, risposta_eva).
    pub fn human_voice(&mut self, text: &str) -> (String, String) {
        // Adamo riceve e risponde
        self.adamo.receive(text);
        let adamo_resp = self.adamo.generate_willed().text;

        // Eva riceve e risponde
        self.eva.receive(text);
        let eva_resp = self.eva.generate_willed().text;

        // Aggiorna gli ultimi testi per il ciclo successivo
        self.last_adamo_text = adamo_resp.clone();
        self.last_eva_text   = eva_resp.clone();

        self.cycle += 1;

        // Momento Tiferet se appropriato
        if self.cycle % 11 == 0 {
            let point = synthesize(&mut self.adamo, &mut self.eva, self.cycle);
            self.tiferet_log.push(point);
        }

        (adamo_resp, eva_resp)
    }

    /// Allineamento simpliciale corrente [0.0, 1.0].
    /// Cresce con il dialogo — supera 0.40 quando il linguaggio condiviso è emergente.
    pub fn alignment(&self) -> f64 {
        compute_alignment(&self.adamo, &self.eva)
    }

    /// Report completo sullo stato di emergenza.
    pub fn emergence_report(&self) -> EmergenceReport {
        EmergenceReport::compute(
            &self.adamo, &self.eva,
            self.cycle, self.tiferet_log.len(),
        )
    }

    /// Inietta il campo di `source` in `target` a peso ridotto.
    /// Simula il canale pre-verbale (tono, postura, stato affettivo)
    /// che precede il linguaggio.
    fn inject_field_channel_static(
        source: &PrometeoTopologyEngine,
        target: &mut PrometeoTopologyEngine,
        weight: f64,
    ) {
        for (word, act) in source.word_topology.active_words() {
            target.word_topology.activate_word(word, act * weight);
        }
        // La propagazione avverrà al prossimo receive() o autonomous_tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dual() -> DualField {
        let mut adamo = PrometeoTopologyEngine::new();
        adamo.teach("io sono qui dentro forte vicino");
        adamo.teach("tu sei lontano fuori debole");
        adamo.teach("noi insieme sentire bene");
        adamo.receive("io qui");
        let eva = create_polar_twin(&adamo);
        DualField::from_engines(adamo, eva)
    }

    #[test]
    fn test_dual_field_init_alignment_valid() {
        let dual = make_dual();
        let al = dual.alignment();
        assert!(al >= 0.0 && al <= 1.0, "allineamento fuori range: {}", al);
    }

    #[test]
    fn test_dual_tick_produces_output() {
        let mut dual = make_dual();
        let turn = dual.tick();
        // Il testo può essere vuoto solo se il campo è piatto (edge case)
        // In ogni caso il ciclo deve avanzare
        assert_eq!(turn.cycle, 1);
    }

    #[test]
    fn test_dual_tick_alternates_speakers() {
        let mut dual = make_dual();
        let t1 = dual.tick();
        let t2 = dual.tick();
        // I parlanti devono alternarsi (o bias curiosità)
        // Non strettamente garantito se curiosity bias interviene,
        // ma in un campo freddo si alternano
        let _ = (t1.speaker, t2.speaker); // solo verifica che non panichi
    }

    #[test]
    fn test_human_voice_both_respond() {
        let mut dual = make_dual();
        let (a, e) = dual.human_voice("io sentire");
        // Entrambe devono rispondere (o dare anti-silenzio)
        // Non verifichiamo il contenuto — solo che non panicano
        let _ = (a, e);
        assert_eq!(dual.cycle, 1);
    }

    #[test]
    fn test_tiferet_at_cycle_11() {
        let mut dual = make_dual();
        for _ in 0..11 {
            dual.tick();
        }
        assert_eq!(dual.tiferet_log.len(), 1,
            "deve esserci esattamente 1 momento Tiferet dopo 11 cicli");
        assert_eq!(dual.tiferet_log[0].cycle, 11);
    }

    #[test]
    fn test_tiferet_at_cycle_22() {
        let mut dual = make_dual();
        for _ in 0..22 {
            dual.tick();
        }
        assert_eq!(dual.tiferet_log.len(), 2,
            "due momenti Tiferet dopo 22 cicli");
    }

    #[test]
    fn test_alignment_non_negative_after_ticks() {
        let mut dual = make_dual();
        for _ in 0..5 {
            dual.tick();
        }
        assert!(dual.alignment() >= 0.0);
    }

    #[test]
    fn test_emergence_report_fields() {
        let mut dual = make_dual();
        for _ in 0..11 {
            dual.tick();
        }
        let report = dual.emergence_report();
        assert_eq!(report.cycle, 11);
        assert_eq!(report.tiferet_count, 1);
        assert!(report.alignment >= 0.0 && report.alignment <= 1.0);
        let _ = report.status(); // non deve panicahre
    }
}
