/// Memoria Episodica — Phase 28.
///
/// La memoria episodica è il terzo layer temporale di Prometeo:
///
///   ROM  (sempre fu)  — la struttura: firme 8D, archi, PF1
///   RAM  (adesso)     — lo stato presente: ActivationState volatile
///   EPISODIC (fu)     — vissuto: snapshot di attivazioni passate con decadimento φ
///
/// PRINCIPIO FONDAMENTALE:
///   Il passato non scompare — decade secondo il numero aureo.
///   `φ⁻¹ = 0.618...` — ogni ciclo un episodio pesa φ⁻ⁿ del suo peso originale.
///   La spirale aurea: più indietro nel tempo, più sfumato — ma mai zero.
///
/// PATTERN COMPLETION:
///   Un attivazione parziale (soglia coseno > 0.45) risuona con episodi passati.
///   Il passato "completa" il presente: recall_into() blende φ-pesato nel campo corrente.
///   Come l'ippocampo: un frammento riattiva il ricordo intero.
///
/// INTEGRAZIONE:
///   • receive(): dopo propagate_field_words() → recall_into(current, 0.45)
///   • autonomous_tick() REM: encode(current) + age_all()
///   • persistence: EpisodeSnapshot serializzabile per memoria cross-sessione

use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════════════════════════
// COSTANTI
// ═══════════════════════════════════════════════════════════════

/// φ⁻¹ = 1/φ = 0.618033988... — il decadimento naturale della memoria.
/// Peso episodio di età n = PHI_INV^n × intensità originale.
pub const PHI_INV: f32 = 0.618_033_988;

/// Quanto pesa il recall sul campo presente (blending factor).
/// 0.12 = il passato colora il presente senza dominarlo.
pub const RECALL_BLEND: f32 = 0.12;

/// Soglia coseno per il recall: episodi troppo distanti vengono ignorati.
pub const RECALL_THRESHOLD: f32 = 0.45;

/// Numero massimo di episodi mantenuti in memoria.
/// ~200 episodi × ~27KB ≈ 5.4MB — accettabile per un sistema desktop/mobile.
pub const MAX_EPISODES: usize = 200;

/// Soglia minima di intensità per codificare un episodio.
/// Evita di memorizzare stati di quiete pura (nulla da ricordare).
pub const MIN_INTENSITY: f32 = 0.15;

/// Peso minimo sotto cui un episodio viene rimosso (troppo sfumato per risuonare).
pub const MIN_WEIGHT: f32 = 0.001;

// ═══════════════════════════════════════════════════════════════
// EPISODE — snapshot di un momento vissuto
// ═══════════════════════════════════════════════════════════════

/// Un momento cristallizzato del campo di attivazione.
///
/// Codificato durante il REM quando il campo ha intensità sufficiente.
/// Decade con età secondo φ⁻ⁿ — mai cancellato bruscamente, sempre sfumato.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Snapshot sparso delle attivazioni: (word_id, activation).
    /// Sparso perché la maggior parte delle parole è a 0.
    pub activation_sparse: Vec<(u32, f32)>,
    /// Firma frattale al momento della codifica: attivazioni dei 16 frattali.
    pub fractal_sig: [f32; 16],
    /// Età dell'episodio in cicli REM dall'encoding.
    pub age: u32,
    /// Intensità massima al momento della codifica (max(activations)).
    pub intensity: f32,
    /// Unix timestamp (UTC, secondi) del momento di codifica.
    /// Usato per ancorare l'episodio al tempo reale.
    #[serde(default)]
    pub timestamp: u64,
}

impl Episode {
    /// Crea un episodio da un array di attivazioni (formato denso → sparso).
    pub fn encode(activations: &[f32], fractal_sig: [f32; 16]) -> Option<Self> {
        let intensity = activations.iter().cloned().fold(0.0_f32, f32::max);
        if intensity < MIN_INTENSITY {
            return None; // stato troppo quieto — nulla da memorizzare
        }

        // Codifica sparsa: solo le parole con attivazione > 0.01
        let activation_sparse: Vec<(u32, f32)> = activations.iter()
            .enumerate()
            .filter(|(_, &a)| a > 0.01)
            .map(|(i, &a)| (i as u32, a))
            .collect();

        if activation_sparse.is_empty() {
            return None;
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Some(Self {
            activation_sparse,
            fractal_sig,
            age: 0,
            intensity,
            timestamp,
        })
    }

    /// Peso corrente dell'episodio: φ⁻ᵃᵍᵉ × intensità.
    #[inline]
    pub fn weight(&self) -> f32 {
        PHI_INV.powi(self.age as i32) * self.intensity
    }

    /// Norma L2 dell'episodio (per cosine similarity).
    pub fn norm(&self, dense_len: usize) -> f32 {
        let sum_sq: f32 = self.activation_sparse.iter()
            .map(|&(_, a)| a * a)
            .sum();
        sum_sq.sqrt().max(1e-8)
    }

    /// Cosine similarity tra questo episodio e un array denso corrente.
    pub fn cosine_sim(&self, current: &[f32]) -> f32 {
        if current.is_empty() { return 0.0; }

        let dot: f32 = self.activation_sparse.iter()
            .filter_map(|&(id, a)| {
                current.get(id as usize).map(|&c| a * c)
            })
            .sum();

        if dot <= 0.0 { return 0.0; }

        let norm_ep = self.norm(current.len());
        let norm_cur: f32 = current.iter().map(|&a| a * a).sum::<f32>().sqrt().max(1e-8);

        (dot / (norm_ep * norm_cur)).min(1.0)
    }
}

// ═══════════════════════════════════════════════════════════════
// EPISODE STORE — il serbatoio episodico
// ═══════════════════════════════════════════════════════════════

/// Il serbatoio della memoria episodica.
///
/// Mantiene fino a `capacity` episodi ordinati per peso decrescente.
/// Il passato è sempre presente — decade, non svanisce.
pub struct EpisodeStore {
    pub episodes: Vec<Episode>,
    pub capacity: usize,
    /// Contatore cicli REM dall'ultimo encode
    pub rem_cycles: u64,
}

impl EpisodeStore {
    pub fn new(capacity: usize) -> Self {
        Self {
            episodes: Vec::with_capacity(capacity),
            capacity,
            rem_cycles: 0,
        }
    }

    /// Codifica un nuovo episodio dal campo di attivazione corrente.
    ///
    /// Chiamato durante il REM quando l'intensità è sufficiente.
    /// Se la capacità è piena, rimuove l'episodio con peso minore.
    pub fn encode(&mut self, activations: &[f32], fractal_sig: [f32; 16]) {
        let Some(episode) = Episode::encode(activations, fractal_sig) else {
            return;
        };

        if self.episodes.len() >= self.capacity {
            // Evicta l'episodio con peso minore
            if let Some(min_pos) = self.episodes.iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.weight().partial_cmp(&b.weight()).unwrap())
                .map(|(i, _)| i)
            {
                self.episodes[min_pos] = episode;
            }
        } else {
            self.episodes.push(episode);
        }
    }

    /// Richiama la memoria episodica nel campo di attivazione corrente.
    ///
    /// Per ogni episodio con cosine_similarity > threshold:
    ///   peso = similarity × φ⁻ᵃᵍᵉ × intensità
    ///   contributo = RECALL_BLEND × peso_normalizzato × activation_episodio
    ///
    /// Il blending è additivo e cappato a 1.0 — il passato arricchisce, non sovrascrive.
    pub fn recall_into(&self, current: &mut Vec<f32>, threshold: f32) {
        if self.episodes.is_empty() || current.is_empty() {
            return;
        }

        // Prima passata: calcola pesi rilevanti
        let mut relevant: Vec<(usize, f32)> = self.episodes.iter()
            .enumerate()
            .filter_map(|(i, ep)| {
                let sim = ep.cosine_sim(current);
                if sim >= threshold {
                    let w = sim * ep.weight();
                    if w > 0.0 { Some((i, w)) } else { None }
                } else {
                    None
                }
            })
            .collect();

        if relevant.is_empty() {
            return;
        }

        // Normalizza i pesi (somma = 1.0)
        let total_weight: f32 = relevant.iter().map(|(_, w)| w).sum();
        if total_weight <= 0.0 { return; }

        // Seconda passata: blending additivo nel campo corrente
        for (ep_idx, w) in &relevant {
            let blend = RECALL_BLEND * w / total_weight;
            let episode = &self.episodes[*ep_idx];

            for &(word_id, ep_act) in &episode.activation_sparse {
                let idx = word_id as usize;
                if idx < current.len() {
                    current[idx] = (current[idx] + ep_act * blend).min(1.0);
                }
            }
        }
    }

    /// Codifica un episodio da dati gia' calcolati (usato da synthesis::synthesize per il Tiferet).
    /// A differenza di encode(), non richiede il buffer pf_activation — accetta dati sparsi diretti.
    pub fn encode_from_sig(
        &mut self,
        activation_sparse: Vec<(u32, f32)>,
        fractal_sig: [f32; 16],
    ) {
        if activation_sparse.is_empty() { return; }
        let intensity = activation_sparse.iter().map(|&(_, a)| a).fold(0.0_f32, f32::max);
        if intensity < MIN_INTENSITY { return; }

        let episode = Episode { 
            activation_sparse, 
            fractal_sig, 
            age: 0, 
            intensity,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };

        if self.episodes.len() >= self.capacity {
            if let Some(min_pos) = self.episodes.iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.weight().partial_cmp(&b.weight()).unwrap())
                .map(|(i, _)| i)
            {
                self.episodes[min_pos] = episode;
            }
        } else {
            self.episodes.push(episode);
        }
    }

    /// Avanza di un ciclo REM: invecchia tutti gli episodi e rimuove quelli sfumati.
    pub fn age_all(&mut self) {
        self.rem_cycles += 1;
        for ep in self.episodes.iter_mut() {
            ep.age += 1;
        }
        // Rimuovi episodi il cui peso è sceso sotto la soglia minima
        self.episodes.retain(|ep| ep.weight() > MIN_WEIGHT);
    }

    /// Numero di episodi correntemente in memoria.
    pub fn len(&self) -> usize {
        self.episodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty()
    }

    /// Peso totale della memoria episodica (misura della "ricchezza" del vissuto).
    pub fn total_weight(&self) -> f32 {
        self.episodes.iter().map(|ep| ep.weight()).sum()
    }

    /// Snapshot serializzabile per la persistenza cross-sessione.
    pub fn snapshot(&self) -> EpisodeSnapshot {
        EpisodeSnapshot {
            episodes: self.episodes.clone(),
            rem_cycles: self.rem_cycles,
        }
    }

    /// Ripristina da snapshot.
    pub fn restore(&mut self, snapshot: EpisodeSnapshot) {
        self.episodes = snapshot.episodes;
        self.rem_cycles = snapshot.rem_cycles;
        // Rimuovi subito gli episodi troppo sfumati (potrebbero essersi degradati tra sessioni)
        self.episodes.retain(|ep| ep.weight() > MIN_WEIGHT);
    }
}

// ═══════════════════════════════════════════════════════════════
// PERSISTENZA — snapshot serializzabile
// ═══════════════════════════════════════════════════════════════

/// Snapshot serializzabile della memoria episodica.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeSnapshot {
    pub episodes: Vec<Episode>,
    pub rem_cycles: u64,
}

// ═══════════════════════════════════════════════════════════════
// TEST
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn make_activation(len: usize, hot: &[(usize, f32)]) -> Vec<f32> {
        let mut v = vec![0.0f32; len];
        for &(i, a) in hot {
            if i < len { v[i] = a; }
        }
        v
    }

    #[test]
    fn test_phi_inv_costante() {
        // φ⁻¹ deve essere vicino a 0.618033988...
        assert!((PHI_INV - 0.618033988_f32).abs() < 1e-6);
        // φ × φ⁻¹ ≈ 1
        let phi = 1.0 / PHI_INV;
        assert!((phi * PHI_INV - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_encode_episodio_quieto_rifiutato() {
        // Stato di quiete (intensity < 0.15) non deve generare episodio
        let act = vec![0.01f32; 100];
        let sig = [0.0f32; 16];
        assert!(Episode::encode(&act, sig).is_none());
    }

    #[test]
    fn test_encode_episodio_attivo() {
        let mut act = vec![0.0f32; 100];
        act[5] = 0.8;
        act[10] = 0.6;
        act[20] = 0.3;
        let sig = [0.5f32; 16];
        let ep = Episode::encode(&act, sig).expect("deve codificare");
        assert_eq!(ep.age, 0);
        assert!((ep.intensity - 0.8).abs() < 1e-5);
        assert!(ep.activation_sparse.len() >= 3);
        // Il peso iniziale e intensita (PHI_INV^0 = 1.0)
        assert!((ep.weight() - 0.8).abs() < 1e-5);
    }

    #[test]
    fn test_weight_decadimento_phi() {
        let mut ep = Episode {
            activation_sparse: vec![(0, 1.0)],
            fractal_sig: [0.0; 16],
            age: 0,
            intensity: 1.0,
            timestamp: 0,
        };
        // età 0: peso = 1.0
        assert!((ep.weight() - 1.0).abs() < 1e-5);
        ep.age = 1;
        // età 1: peso = PHI_INV ≈ 0.618
        assert!((ep.weight() - PHI_INV).abs() < 1e-5);
        ep.age = 2;
        // età 2: peso = PHI_INV² ≈ 0.382
        assert!((ep.weight() - PHI_INV * PHI_INV).abs() < 1e-5);
    }

    #[test]
    fn test_cosine_sim_identica() {
        let act = make_activation(50, &[(3, 0.8), (7, 0.6), (15, 0.4)]);
        let ep = Episode::encode(&act, [0.0; 16]).unwrap();
        // Cosine similarity con se stesso deve essere ~1.0
        let sim = ep.cosine_sim(&act);
        assert!(sim > 0.99, "cosine con se stesso = {sim}");
    }

    #[test]
    fn test_cosine_sim_ortogonale() {
        let act_ep = make_activation(50, &[(0, 1.0), (1, 0.8)]);
        let act_now = make_activation(50, &[(25, 1.0), (30, 0.8)]);
        let ep = Episode::encode(&act_ep, [0.0; 16]).unwrap();
        let sim = ep.cosine_sim(&act_now);
        // Vettori ortogonali: cosine = 0
        assert!(sim < 0.01, "cosine ortogonale = {sim}");
    }

    #[test]
    fn test_recall_into_risuona() {
        let mut store = EpisodeStore::new(10);
        // Codifica episodio con parole attive in 5, 10, 15
        let act_past = make_activation(50, &[(5, 0.9), (10, 0.7), (15, 0.5)]);
        store.encode(&act_past, [0.0; 16]);

        // Attivazione parziale corrente: solo parola 5 e 10
        let mut act_now = make_activation(50, &[(5, 0.6), (10, 0.4)]);
        store.recall_into(&mut act_now, RECALL_THRESHOLD);

        // Dopo il recall, la parola 15 deve essere stata parzialmente riattivata
        assert!(act_now[15] > 0.0, "parola 15 deve risuonare dopo recall");
        // Le parole originali non devono essere diminuite
        assert!(act_now[5] >= 0.6 - 1e-5);
    }

    #[test]
    fn test_age_all_e_prune() {
        let mut store = EpisodeStore::new(10);
        let act = make_activation(20, &[(0, 0.5), (1, 0.4)]);
        store.encode(&act, [0.0; 16]);
        assert_eq!(store.len(), 1);

        // Invecchia molte volte — il peso deve scendere sotto MIN_WEIGHT
        // PHI_INV^n × 0.5 < 0.001 → n > log(0.001/0.5) / log(0.618) ≈ 14
        for _ in 0..30 {
            store.age_all();
        }
        assert_eq!(store.len(), 0, "episodio deve essere rimosso dopo decadimento φ");
    }

    #[test]
    fn test_eviction_quando_pieno() {
        let mut store = EpisodeStore::new(3); // capacità minima

        // Codifica 3 episodi con intensità diverse
        let act1 = make_activation(20, &[(0, 0.9)]); // intensità 0.9
        let act2 = make_activation(20, &[(5, 0.5)]); // intensità 0.5
        let act3 = make_activation(20, &[(10, 0.7)]); // intensità 0.7
        store.encode(&act1, [0.0; 16]);
        store.encode(&act2, [0.0; 16]);
        store.encode(&act3, [0.0; 16]);
        assert_eq!(store.len(), 3);

        // Il 4° episodio deve evictare il più debole (act2, intensità 0.5)
        let act4 = make_activation(20, &[(15, 0.8)]); // intensità 0.8
        store.encode(&act4, [0.0; 16]);
        assert_eq!(store.len(), 3, "capacità rispettata");
        // Verifica che il peso totale sia aumentato rispetto a prima dell'eviction
        let has_weak = store.episodes.iter().any(|ep| {
            ep.activation_sparse.iter().any(|&(id, _)| id == 5)
        });
        assert!(!has_weak, "episodio debole deve essere stato evictato");
    }

    #[test]
    fn test_snapshot_roundtrip() {
        let mut store = EpisodeStore::new(10);
        let act = make_activation(30, &[(2, 0.6), (8, 0.4), (20, 0.3)]);
        store.encode(&act, [0.1; 16]);
        store.age_all();

        let snap = store.snapshot();
        let json = serde_json::to_string(&snap).expect("serializzazione ok");
        let snap2: EpisodeSnapshot = serde_json::from_str(&json).expect("deserializzazione ok");

        let mut store2 = EpisodeStore::new(10);
        store2.restore(snap2);
        assert_eq!(store2.len(), store.len());
        assert_eq!(store2.episodes[0].age, store.episodes[0].age);
    }
}
