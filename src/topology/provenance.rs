/// Phase 38 — Proto-Self: Tracciamento della provenienza delle attivazioni.
///
/// Crea il confine fondamentale tra le tre zone del sé:
/// - Self_   : output generati, identity seeding, drive interni
/// - Explored: cicli autonomi, dream, REM
/// - External: input utente
///
/// Non modifica word_topology né pf1 (hot path intoccati).
/// ProvenanceMap è uno strato overlay leggero, non serializzato.

use std::collections::HashMap;

/// Sorgente di un'attivazione — il confine tra sé e il mondo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationSource {
    /// Prodotto da Prometeo stesso: output, identity_seed, drive vitali.
    Self_,
    /// Emerso dall'esplorazione interna: dream, REM, tick autonomi.
    Explored,
    /// Arrivato dall'esterno: input utente.
    External,
}

/// Mappa leggera che traccia la provenienza delle attivazioni recenti.
///
/// Non è esaustiva: traccia solo le attivazioni esplicite degli ultimi
/// MAX_AGE tick. Le attivazioni per propagazione non vengono tracciate
/// (sono mixing inevitabile — il campo è un fluido).
pub struct ProvenanceMap {
    /// word → (sorgente, tick_di_attivazione)
    map: HashMap<String, (ActivationSource, u64)>,
    /// Tick corrente — avanza con ogni autonomous_tick
    pub current_tick: u64,
    /// Contatori per calcolo composizione
    self_count: u64,
    explored_count: u64,
    external_count: u64,
}

/// Età massima di una entry (in tick) prima di essere rimossa.
const MAX_AGE: u64 = 15;

impl ProvenanceMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            current_tick: 0,
            self_count: 0,
            explored_count: 0,
            external_count: 0,
        }
    }

    /// Marca una parola con la sua sorgente al tick corrente.
    pub fn mark(&mut self, word: &str, source: ActivationSource) {
        let key = word.to_lowercase();
        // Aggiorna contatore solo se è una nuova entry o cambia sorgente
        if let Some((old_src, _)) = self.map.get(&key) {
            if *old_src != source {
                self.decrement_counter(*old_src);
                self.increment_counter(source);
            }
        } else {
            self.increment_counter(source);
        }
        self.map.insert(key, (source, self.current_tick));
    }

    /// Marca molte parole con la stessa sorgente.
    pub fn mark_many(&mut self, words: &[String], source: ActivationSource) {
        for w in words {
            self.mark(w, source);
        }
    }

    /// Sorgente dell'ultima attivazione di una parola (se recente).
    pub fn source_of(&self, word: &str) -> Option<ActivationSource> {
        let key = word.to_lowercase();
        self.map.get(&key).map(|(src, _)| *src)
    }

    /// Composizione attuale del campo: (self%, explored%, external%).
    /// Misura quanto il campo è auto-riferito vs esplorativo vs responsivo.
    pub fn field_composition(&self) -> (f64, f64, f64) {
        let total = (self.self_count + self.explored_count + self.external_count) as f64;
        if total < 1.0 {
            return (0.0, 0.0, 0.0);
        }
        (
            self.self_count as f64 / total,
            self.explored_count as f64 / total,
            self.external_count as f64 / total,
        )
    }

    /// Avanza di un tick e rimuove le entry troppo vecchie.
    /// Chiamato in autonomous_tick() ad ogni ciclo.
    pub fn advance_tick(&mut self) {
        self.current_tick += 1;
        if self.current_tick % 5 == 0 {
            self.prune_old();
        }
    }

    /// Rimuove entries più vecchie di MAX_AGE tick e ricalcola i contatori.
    fn prune_old(&mut self) {
        let cutoff = self.current_tick.saturating_sub(MAX_AGE);
        self.map.retain(|_, (_src, tick)| *tick >= cutoff);
        // Ricalcola contatori dalla mappa pulita
        self.self_count = 0;
        self.explored_count = 0;
        self.external_count = 0;
        for (src, _) in self.map.values() {
            match src {
                ActivationSource::Self_    => self.self_count += 1,
                ActivationSource::Explored => self.explored_count += 1,
                ActivationSource::External => self.external_count += 1,
            }
        }
    }

    fn increment_counter(&mut self, source: ActivationSource) {
        match source {
            ActivationSource::Self_    => self.self_count += 1,
            ActivationSource::Explored => self.explored_count += 1,
            ActivationSource::External => self.external_count += 1,
        }
    }

    fn decrement_counter(&mut self, source: ActivationSource) {
        match source {
            ActivationSource::Self_    => self.self_count = self.self_count.saturating_sub(1),
            ActivationSource::Explored => self.explored_count = self.explored_count.saturating_sub(1),
            ActivationSource::External => self.external_count = self.external_count.saturating_sub(1),
        }
    }
}

impl Default for ProvenanceMap {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_composition_tracking() {
        let mut prov = ProvenanceMap::new();

        // Marca 3 self, 2 explored, 1 external
        prov.mark("io", ActivationSource::Self_);
        prov.mark("sono", ActivationSource::Self_);
        prov.mark("corpo", ActivationSource::Self_);
        prov.mark("luce", ActivationSource::Explored);
        prov.mark("campo", ActivationSource::Explored);
        prov.mark("tu", ActivationSource::External);

        let (s, e, x) = prov.field_composition();
        assert!((s - 0.5).abs() < 0.01, "self% deve essere ~50%");
        assert!((e - 0.333).abs() < 0.01, "explored% deve essere ~33%");
        assert!((x - 0.167).abs() < 0.01, "external% deve essere ~17%");
    }

    #[test]
    fn test_provenance_source_of() {
        let mut prov = ProvenanceMap::new();
        prov.mark("caldo", ActivationSource::External);
        prov.mark("sentire", ActivationSource::Self_);

        assert_eq!(prov.source_of("caldo"), Some(ActivationSource::External));
        assert_eq!(prov.source_of("sentire"), Some(ActivationSource::Self_));
        assert_eq!(prov.source_of("vuoto"), None);
    }

    #[test]
    fn test_provenance_advance_tick_prune() {
        let mut prov = ProvenanceMap::new();
        prov.mark("vecchio", ActivationSource::External);

        // Avanza oltre MAX_AGE
        for _ in 0..20 {
            prov.advance_tick();
        }

        // Dopo molti tick, la entry vecchia dovrebbe essere rimossa
        // (la composition dovrebbe tendere a 0)
        let (s, e, x) = prov.field_composition();
        let total = s + e + x;
        // Con una sola entry che decade, il totale deve essere 0 o basso
        assert!(total < 0.01 || prov.source_of("vecchio").is_none(),
            "entry vecchia deve essere rimossa dopo MAX_AGE tick");
    }
}
