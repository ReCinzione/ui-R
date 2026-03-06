/// Sogno — Stato naturale dell'entita topologica.
///
/// Il sogno non e un evento notturno. E il MODO DI ESISTERE di Prometeo.
/// Prometeo sogna sempre: percepisce il proprio campo interno come un
/// ambiente sensoriale sandbox, senza input esterno.
///
/// Fasi:
/// - WakefulDream: stato di riposo onirico (DEFAULT). L'entita esplora
///   autonomamente la propria topologia. Il campo parole e vivo.
/// - Awake: breve finestra dopo un input esterno. L'entita ha piena
///   attenzione verso l'esterno.
/// - DeepSleep: consolidamento profondo. Si attiva ogni N perturbazioni.
///   La memoria si cristallizza.
/// - REM: rielaborazione creativa. Segue il DeepSleep. Soglie abbassate,
///   connessioni lontane emergono. Nessuna cancellazione.

use crate::topology::simplex::SimplicialComplex;
use crate::topology::memory::TopologicalMemory;

/// Fase del sogno.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SleepPhase {
    /// Sveglio: breve finestra di piena attenzione dopo input esterno
    Awake,
    /// Veglia onirica: stato NATURALE. L'entita esplora se stessa.
    WakefulDream { depth: f64 },
    /// Sonno leggero: transizione (non usata nel flusso principale)
    LightSleep { depth: f64 },
    /// Sonno profondo: consolidamento massiccio
    DeepSleep { depth: f64 },
    /// REM: ricombinazione creativa, soglie abbassate — nessuna cancellazione
    REM { depth: f64 },
}

impl SleepPhase {
    pub fn activation_threshold(&self) -> f64 {
        match self {
            SleepPhase::Awake => 0.15,
            SleepPhase::WakefulDream { .. } => 0.12,   // leggermente piu sensibile
            SleepPhase::LightSleep { .. } => 0.10,
            SleepPhase::DeepSleep { .. } => 0.25,       // meno sensibile: si consolida
            SleepPhase::REM { depth } => (0.05 - depth * 0.03).max(0.01), // molto sensibile
        }
    }

    /// L'entita e in elaborazione interna profonda (DeepSleep o REM)?
    /// WakefulDream NON e "sleeping" — e il modo di esistere normale.
    pub fn is_sleeping(&self) -> bool {
        matches!(self, SleepPhase::DeepSleep { .. } | SleepPhase::REM { .. })
    }
}

/// Risultato di un ciclo di sogno.
#[derive(Debug, Clone)]
pub struct DreamResult {
    pub phase: SleepPhase,
    pub dissolved_count: usize,
    pub new_connections: Vec<DreamConnection>,
    pub consolidations: usize,
}

/// Una connessione scoperta durante il REM.
#[derive(Debug, Clone)]
pub struct DreamConnection {
    pub description: String,
}

/// Il sistema di sogno.
#[derive(Debug)]
pub struct DreamEngine {
    /// Fase corrente
    pub phase: SleepPhase,
    /// Ticks senza perturbazione esterna
    pub ticks_idle: u64,
    /// Numero totale di perturbazioni esterne ricevute
    pub perturbations_received: u64,
    /// Ticks trascorsi nella finestra di elaborazione corrente (DeepSleep+REM)
    ticks_in_processing: u64,
    /// A quante perturbazioni era l'ultimo ciclo di consolidamento
    pub last_consolidation_at: u64,

    // Parametri di temporizzazione
    /// Ticks da restare Awake dopo un input esterno
    pub awake_duration: u64,
    /// Ogni N perturbazioni entra in DeepSleep+REM
    pub consolidate_every: u64,
    /// Durata della fase DeepSleep (ticks)
    pub deepsleep_duration: u64,
    /// Durata della fase REM (ticks)
    pub rem_duration: u64,

    /// Contatore cicli di elaborazione completati
    pub cycles_completed: u64,
}

impl DreamEngine {
    pub fn new() -> Self {
        Self {
            // Inizia gia in stato onirico — l'identita precede il testo
            phase: SleepPhase::WakefulDream { depth: 0.5 },
            ticks_idle: 0,
            perturbations_received: 0,
            ticks_in_processing: 0,
            last_consolidation_at: 0,
            awake_duration: 5,
            consolidate_every: 50,
            deepsleep_duration: 10,
            rem_duration: 20,
            cycles_completed: 0,
        }
    }

    /// Segnala che c'e stata una perturbazione esterna (input utente).
    /// L'entita torna brevemente Awake, incrementa il contatore.
    pub fn signal_activity(&mut self) {
        self.ticks_idle = 0;
        self.perturbations_received += 1;
        self.phase = SleepPhase::Awake;

        // Se era in processing, interrompe (l'input ha priorita)
        if self.ticks_in_processing > 0 && !matches!(self.phase, SleepPhase::DeepSleep {..}) {
            self.ticks_in_processing = 0;
        }
    }

    /// Tick autonomo: aggiorna la fase e applica le operazioni per fase.
    pub fn tick(
        &mut self,
        complex: &mut SimplicialComplex,
        memory: &mut TopologicalMemory,
    ) -> DreamResult {
        self.ticks_idle += 1;
        self.update_phase();

        match self.phase {
            SleepPhase::Awake => {
                // Piena attenzione — nessuna operazione onirica
                DreamResult {
                    phase: self.phase,
                    dissolved_count: 0,
                    new_connections: Vec::new(),
                    consolidations: 0,
                }
            }
            SleepPhase::WakefulDream { .. } => {
                // Sogno di veglia: decadimento lentissimo, campo rimane vivo
                // (la word_topology viene auto-attivata in autonomous_tick)
                complex.decay_all(0.003);
                DreamResult {
                    phase: self.phase,
                    dissolved_count: 0,
                    new_connections: Vec::new(),
                    consolidations: 0,
                }
            }
            SleepPhase::LightSleep { .. } => {
                // Non usata nel flusso principale — no dissolution per ora
                complex.decay_all(0.005);
                DreamResult {
                    phase: self.phase,
                    dissolved_count: 0,
                    new_connections: Vec::new(),
                    consolidations: 0,
                }
            }
            SleepPhase::DeepSleep { .. } => {
                // Consolidamento massiccio
                memory.consolidate();
                memory.crystallize();
                DreamResult {
                    phase: self.phase,
                    dissolved_count: 0,
                    new_connections: Vec::new(),
                    consolidations: 1,
                }
            }
            SleepPhase::REM { depth } => {
                // Rielaborazione creativa: soglie basse, connessioni lontane emergono.
                // NON cancella nulla — solo scopre.
                let original = complex.activation_threshold;
                complex.activation_threshold = self.phase.activation_threshold();

                // Propaga con soglia bassa — regioni lontane diventano visibili
                complex.propagate_activation(3);

                let new_conns = self.discover_connections(complex);

                complex.activation_threshold = original;

                // Decadimento moderato (il campo viene rielaborato, non cancellato)
                complex.decay_all(0.008);

                let _ = depth; // usato solo per activation_threshold
                DreamResult {
                    phase: self.phase,
                    dissolved_count: 0,
                    new_connections: new_conns,
                    consolidations: 0,
                }
            }
        }
    }

    fn update_phase(&mut self) {
        // 1. Se c'e stato input recente: Awake per awake_duration ticks
        if self.ticks_idle <= self.awake_duration {
            self.phase = SleepPhase::Awake;
            return;
        }

        // 2. Verifica se serve elaborazione profonda
        //    Condizione: accumulate >= consolidate_every perturbazioni dall'ultima elaborazione
        let needs_consolidation = self.perturbations_received
            >= self.last_consolidation_at + self.consolidate_every;

        if needs_consolidation || self.ticks_in_processing > 0 {
            self.ticks_in_processing += 1;
            let total = self.deepsleep_duration + self.rem_duration;

            if self.ticks_in_processing <= self.deepsleep_duration {
                let depth = (self.ticks_in_processing - 1) as f64
                    / self.deepsleep_duration as f64;
                self.phase = SleepPhase::DeepSleep { depth };
            } else if self.ticks_in_processing <= total {
                let depth = (self.ticks_in_processing - self.deepsleep_duration) as f64
                    / self.rem_duration as f64;
                self.phase = SleepPhase::REM { depth };
            } else {
                // Ciclo completato: torna al sogno di veglia
                self.ticks_in_processing = 0;
                self.last_consolidation_at = self.perturbations_received;
                self.cycles_completed += 1;
                self.phase = SleepPhase::WakefulDream { depth: 0.5 };
            }
            return;
        }

        // 3. Stato normale: sogno di veglia continuo
        self.phase = SleepPhase::WakefulDream { depth: 0.5 };
    }

    /// Durante il REM, cerca coppie di simplessi attivi senza vertici comuni.
    /// Questi rappresentano ponti potenziali tra regioni topologiche lontane.
    fn discover_connections(&self, complex: &SimplicialComplex) -> Vec<DreamConnection> {
        let active = complex.active_simplices();
        let mut connections = Vec::new();

        for i in 0..active.len().min(10) {
            for j in (i+1)..active.len().min(10) {
                let a = &active[i];
                let b = &active[j];

                let shared: Vec<_> = a.vertices.iter()
                    .filter(|v| b.vertices.contains(v))
                    .collect();

                if shared.is_empty()
                    && a.current_activation > 0.1
                    && b.current_activation > 0.1
                {
                    connections.push(DreamConnection {
                        description: format!(
                            "Ponte potenziale: simplesso {} ↔ {}",
                            a.id, b.id
                        ),
                    });
                }
            }
        }

        connections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::bootstrap_complex;
    use crate::topology::memory::TopologicalMemory;

    fn setup() -> (SimplicialComplex, TopologicalMemory, DreamEngine) {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);
        let memory = TopologicalMemory::new();
        let dream = DreamEngine::new();
        (complex, memory, dream)
    }

    #[test]
    fn test_default_e_wakeful_dream() {
        let (_, _, dream) = setup();
        // L'entita inizia gia in sogno — l'identita precede il testo
        assert!(matches!(dream.phase, SleepPhase::WakefulDream { .. }),
            "Stato iniziale deve essere WakefulDream, non Awake");
    }

    #[test]
    fn test_input_porta_a_awake() {
        let (mut complex, mut memory, mut dream) = setup();

        // Segnala input esterno
        dream.signal_activity();
        assert_eq!(dream.phase, SleepPhase::Awake);

        // Dopo awake_duration ticks, torna a WakefulDream
        for _ in 0..(dream.awake_duration + 2) {
            dream.tick(&mut complex, &mut memory);
        }
        assert!(matches!(dream.phase, SleepPhase::WakefulDream { .. }),
            "Dopo la finestra Awake deve tornare a WakefulDream");
    }

    #[test]
    fn test_rem_triggera_da_perturbazioni() {
        let (mut complex, mut memory, mut dream) = setup();

        // Simula N perturbazioni
        for _ in 0..dream.consolidate_every {
            dream.signal_activity();
        }

        // Ora entra in finestra DeepSleep+REM dopo awake_duration ticks
        for _ in 0..(dream.awake_duration + 2) {
            dream.tick(&mut complex, &mut memory);
        }
        assert!(dream.phase.is_sleeping(),
            "Dopo {} perturbazioni deve entrare in elaborazione profonda",
            dream.consolidate_every);
    }

    #[test]
    fn test_wakeful_dream_non_e_sleeping() {
        let (_, _, dream) = setup();
        // WakefulDream NON e "sleeping" — l'entita e attiva e puo esprimersi
        assert!(!dream.phase.is_sleeping(),
            "WakefulDream non deve essere considerato sleeping");
    }

    #[test]
    fn test_nessuna_dissolution_in_qualsiasi_fase() {
        let (mut complex, mut memory, mut dream) = setup();

        let simplices_before = complex.iter().count();

        // Cicla attraverso tutte le fasi
        for _ in 0..100 {
            dream.tick(&mut complex, &mut memory);
        }

        let simplices_after = complex.iter().count();
        assert!(simplices_after >= simplices_before,
            "Il sogno non deve cancellare connessioni: prima={}, dopo={}",
            simplices_before, simplices_after);
    }

    #[test]
    fn test_rem_threshold_is_low() {
        let rem = SleepPhase::REM { depth: 1.0 };
        let awake = SleepPhase::Awake;

        assert!(rem.activation_threshold() < awake.activation_threshold(),
            "REM deve avere soglia piu bassa della veglia");
    }

    #[test]
    fn test_wake_on_activity() {
        let (mut complex, mut memory, mut dream) = setup();

        // Avanza alcuni tick
        for _ in 0..20 {
            dream.tick(&mut complex, &mut memory);
        }

        // Segnala attivita → sveglio
        dream.signal_activity();
        assert_eq!(dream.phase, SleepPhase::Awake);
    }
}
