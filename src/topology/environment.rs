/// Contesto ambientale — bias implicito sulle 8 dimensioni cognitive.
///
/// Non produce parole. Non modifica il lessico. Non compare mai nelle risposte.
/// Agisce come i ritmi circadiani: condiziona il campo senza essere pensato.
///
/// Il bias è piccolo (max ±0.05 per dimensione) e additivo rispetto alla firma
/// del campo attivo. Non è una forza — è un clima.

use std::time::{SystemTime, UNIX_EPOCH};

/// Snapshot ambientale calcolato a ogni interazione.
#[derive(Debug, Clone)]
pub struct Environment {
    /// Unix timestamp al momento del campionamento (UTC, secondi)
    pub unix_ts: u64,
    /// Ora del giorno in UTC [0.0, 24.0)
    pub hour: f64,
    /// Giorno dell'anno [0, 364]
    pub day_of_year: u16,
    /// Secondi dall'ultima interazione (silenzio attivo)
    pub silence_secs: f64,
    /// Giorni di vita dell'istanza (da instance_born)
    pub instance_age_days: f64,
}

impl Default for Environment {
    fn default() -> Self {
        // Valori neutri: mezzogiorno, estate, nessun silenzio
        Self {
            unix_ts: 0,
            hour: 12.0,
            day_of_year: 180,
            silence_secs: 0.0,
            instance_age_days: 0.0,
        }
    }
}

impl Environment {
    /// Campiona l'ambiente corrente.
    pub fn now(silence_secs: f64, instance_born: u64) -> Self {
        let unix_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let secs_today = unix_ts % 86400;
        let hour = secs_today as f64 / 3600.0;
        let day_of_year = ((unix_ts / 86400) % 365) as u16;
        let instance_age_days = if instance_born > 0 {
            unix_ts.saturating_sub(instance_born) as f64 / 86400.0
        } else {
            0.0
        };

        Self { unix_ts, hour, day_of_year, silence_secs, instance_age_days }
    }

    /// Bias implicito sulle 8 dimensioni primitive.
    ///
    /// Ordine: Agency, Permanenza, Intensita, Tempo, Confine, Complessita, Definizione, Valenza.
    ///
    /// I valori sono piccoli (max ±0.05) e non vengono mai convertiti in parole.
    /// Sono un condizionamento del campo, non un contenuto.
    pub fn dimension_bias(&self) -> [f64; 8] {
        let c = self.circadian_bias();
        let s = self.seasonal_bias();
        std::array::from_fn(|i| c[i] + s[i])
    }

    /// Profondità del silenzio [0.0, 1.0].
    /// Satura a 30 minuti — oltre, il campo è completamente raffreddato.
    /// Usata dal ciclo REM per modulare la profondità del sogno.
    pub fn silence_depth(&self) -> f64 {
        (self.silence_secs / 1800.0).min(1.0)
    }

    // ─────────────────────────────────────────────────────────────────────
    // Bias circadiano — ritmo 24h, max ±0.05 per dimensione.
    //
    // Struttura:
    //   mezzogiorno → Agency alta, Definizione alta, Intensita alta
    //   sera        → Valenza lievemente alta, Complessita cresce
    //   notte       → Permanenza alta, Complessita alta, Confine più definito
    //   alba        → Tempo transitorio (massimo alle transizioni)
    // ─────────────────────────────────────────────────────────────────────
    fn circadian_bias(&self) -> [f64; 8] {
        use std::f64::consts::PI;
        // phase: 0 = mezzanotte, PI = mezzogiorno
        let phase = 2.0 * PI * self.hour / 24.0;
        // cos_p: +1 = mezzogiorno, -1 = mezzanotte
        let cos_p = (phase - PI).cos();

        [
            0.04 * cos_p,                                        // Agency: alta di giorno
            -0.02 * cos_p,                                       // Permanenza: alta di notte
            0.025 * (phase * 0.75 - PI * 0.25).cos().max(0.0),  // Intensita: picco ~ore 9
            0.02 * (2.0 * phase).sin().abs(),                    // Tempo: alba e tramonto
            -0.015 * cos_p,                                      // Confine: più netto di notte
            -0.03 * cos_p,                                       // Complessita: serale/notturna
            0.025 * (phase * 0.75).cos().max(0.0),               // Definizione: mattina
            0.02 * (phase - PI * 1.25).cos(),                    // Valenza: lievemente serale
        ]
    }

    // ─────────────────────────────────────────────────────────────────────
    // Bias stagionale — ritmo annuale, max ±0.02 per dimensione.
    //
    //   estate (giugno ~giorno 172): Agency, Intensita, Definizione più alte
    //   inverno (dicembre ~giorno 355): Permanenza, Complessita più alte
    // ─────────────────────────────────────────────────────────────────────
    fn seasonal_bias(&self) -> [f64; 8] {
        use std::f64::consts::PI;
        // cos_s: +1 = estate (giugno), -1 = inverno (dicembre)
        let season = 2.0 * PI * self.day_of_year as f64 / 365.0;
        let cos_s = (season - PI * 0.95).cos();

        [
             0.015 * cos_s,  // Agency: estate
            -0.008 * cos_s,  // Permanenza: inverno
             0.010 * cos_s,  // Intensita: estate
             0.0,            // Tempo: invariante stagionalmente
             0.0,            // Confine: invariante
            -0.012 * cos_s,  // Complessita: inverno (introversione)
             0.008 * cos_s,  // Definizione: estate (chiarezza)
             0.0,            // Valenza: invariante
        ]
    }
}
