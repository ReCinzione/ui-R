use prometeo::topology::PartOfSpeech;
use prometeo::topology::persistence::{PrometeoState, WordSnapshot};

fn pos_label(pos: &Option<PartOfSpeech>) -> &'static str {
    match pos {
        None => "-",
        Some(PartOfSpeech::Verb)      => "V",
        Some(PartOfSpeech::Noun)      => "N",
        Some(PartOfSpeech::Adjective) => "Adj",
        Some(PartOfSpeech::Adverb)    => "Adv",
        Some(PartOfSpeech::Pronoun)   => "Pro",
    }
}

fn main() {
    let path = std::path::Path::new("prometeo_topology_state.bin");
    if !path.exists() {
        eprintln!("File .bin non trovato");
        std::process::exit(1);
    }

    let state = match PrometeoState::load_from_binary(path) {
        Ok(s) => s,
        Err(e) => { eprintln!("Errore caricamento: {}", e); std::process::exit(1); }
    };

    let words: &Vec<WordSnapshot> = &state.lexicon.words;
    let total = words.len();
    let pct = |n: usize| 100.0 * n as f64 / total as f64;

    println!("=== ANALISI LESSICO PROMETEO ===");
    println!("Totale parole: {}", total);

    // POS distribution
    let mut n_none = 0usize;
    let mut n_verb = 0usize;
    let mut n_noun = 0usize;
    let mut n_adj  = 0usize;
    let mut n_adv  = 0usize;
    let mut n_pron = 0usize;
    for w in words {
        match &w.pos {
            None => n_none += 1,
            Some(PartOfSpeech::Verb)      => n_verb += 1,
            Some(PartOfSpeech::Noun)      => n_noun += 1,
            Some(PartOfSpeech::Adjective) => n_adj  += 1,
            Some(PartOfSpeech::Adverb)    => n_adv  += 1,
            Some(PartOfSpeech::Pronoun)   => n_pron += 1,
        }
    }
    println!("\n--- POS ---");
    println!("None:      {} ({:.1}%)", n_none, pct(n_none));
    println!("Verb:      {} ({:.1}%)", n_verb, pct(n_verb));
    println!("Noun:      {} ({:.1}%)", n_noun, pct(n_noun));
    println!("Adjective: {} ({:.1}%)", n_adj,  pct(n_adj));
    println!("Adverb:    {} ({:.1}%)", n_adv,  pct(n_adv));
    println!("Pronoun:   {} ({:.1}%)", n_pron, pct(n_pron));

    // Qualità dati
    let mut punct_words: Vec<&str>      = Vec::new();
    let mut apostrophe_words: Vec<&str> = Vec::new();
    let conj_forms = ["vedo","sento","penso","sono","ho","ha","fa","va","viene",
        "vuole","dice","vedi","senti","pensi","siamo","abbiamo","fanno","vanno",
        "sta","sto","sei","era","dalla","della","nella","sulla","degli"];
    let mut conj_found: Vec<&str> = Vec::new();

    for ws in words {
        let w: &str = &ws.word;
        let has_apos  = w.contains('\'');
        let has_punct = !has_apos && w.chars().any(|c| ":.,;!?()\"«»—–".contains(c));
        if has_apos   { apostrophe_words.push(w); }
        if has_punct  { punct_words.push(w); }
        if conj_forms.contains(&w) { conj_found.push(w); }
    }

    println!("\n--- QUALITA DATI ---");
    println!("Con punteggiatura:       {} ({:.1}%)", punct_words.len(), pct(punct_words.len()));
    println!("Con apostrofo:           {} ({:.1}%)", apostrophe_words.len(), pct(apostrophe_words.len()));
    let clean = total.saturating_sub(punct_words.len() + apostrophe_words.len());
    println!("Pulite:                  {} ({:.1}%)", clean, pct(clean));
    println!("Forme coniugate spurie:  {} {:?}", conj_found.len(), conj_found);

    print!("\nCampione punteggiatura: ");
    for w in punct_words.iter().take(15) { print!("'{}'  ", w); }
    println!();
    print!("\nCampione contrazioni: ");
    for w in apostrophe_words.iter().take(15) { print!("'{}'  ", w); }
    println!();

    // Parole con : (dizionario malformato)
    let colon_words: Vec<&str> = words.iter()
        .filter(|ws| ws.word.ends_with(':') || ws.word.starts_with(':'))
        .map(|ws| ws.word.as_str())
        .collect();
    println!("\nParole con ':' (malformate): {} ({:.1}%)", colon_words.len(), pct(colon_words.len()));
    print!("Campione: ");
    for w in colon_words.iter().take(10) { print!("'{}'  ", w); }
    println!();

    // Top per esposizione
    let mut by_exp: Vec<(&str, u64, f64, &str)> = words.iter()
        .map(|ws| (ws.word.as_str(), ws.exposure_count, ws.stability, pos_label(&ws.pos)))
        .collect();
    by_exp.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\n--- TOP 35 PER ESPOSIZIONE ---");
    for (w, exp, stab, pos) in by_exp.iter().take(35) {
        println!("  {:28} exp={:5}  stab={:.2}  pos={}", w, exp, stab, pos);
    }

    // Distribuzione esposizioni
    println!("\n--- DISTRIBUZIONE ESPOSIZIONI ---");
    println!("exp=1:    {}", words.iter().filter(|ws| ws.exposure_count == 1).count());
    println!("exp=2:    {}", words.iter().filter(|ws| ws.exposure_count == 2).count());
    println!("exp<=5:   {}", words.iter().filter(|ws| ws.exposure_count <= 5).count());
    println!("exp 6-20: {}", words.iter().filter(|ws| ws.exposure_count > 5 && ws.exposure_count <= 20).count());
    println!("exp>20:   {}", words.iter().filter(|ws| ws.exposure_count > 20).count());
    println!("exp>100:  {}", words.iter().filter(|ws| ws.exposure_count > 100).count());
    println!("exp>500:  {}", words.iter().filter(|ws| ws.exposure_count > 500).count());

    // Distribuzione stabilità
    println!("\n--- DISTRIBUZIONE STABILITA ---");
    println!("stab>=0.9:    {}", words.iter().filter(|ws| ws.stability >= 0.9).count());
    println!("stab 0.5-0.9: {}", words.iter().filter(|ws| ws.stability >= 0.5 && ws.stability < 0.9).count());
    println!("stab<0.5:     {}", words.iter().filter(|ws| ws.stability < 0.5).count());

    // Co-occorrenze: parole con e senza
    let with_co = words.iter().filter(|ws| !ws.co_occurrences.is_empty()).count();
    let avg_co = words.iter().map(|ws| ws.co_occurrences.len()).sum::<usize>() as f64 / total as f64;
    println!("\n--- CO-OCCORRENZE ---");
    println!("Parole con co-occ:  {} ({:.1}%)", with_co, pct(with_co));
    println!("Media co-occ/parola: {:.1}", avg_co);

    // Taggabili per suffisso (solo untagged)
    let sfx_noun: &[&str] = &["zione","tà","ità","mento","tore","tura","anza","enza","ismo","ista","ume","aggio"];
    let sfx_adj:  &[&str] = &["oso","osa","ale","ico","ica","ibile","abile","ivo","iva","ante","ente"];
    let sfx_adv:  &[&str] = &["mente"];
    let mut tn = 0usize;
    let mut ta = 0usize;
    let mut td = 0usize;
    for ws in words {
        if ws.pos.is_some() { continue; }
        let wc: String = ws.word.chars().filter(|c| c.is_alphabetic()).collect();
        if wc.len() < 5 { continue; }
        if sfx_adv.iter().any(|s| wc.ends_with(s))       { td += 1; }
        else if sfx_noun.iter().any(|s| wc.ends_with(s)) { tn += 1; }
        else if sfx_adj.iter().any(|s| wc.ends_with(s))  { ta += 1; }
    }
    println!("\n--- TAGGABILI PER SUFFISSO (untagged) ---");
    println!("Noun (stima):  {}", tn);
    println!("Adj  (stima):  {}", ta);
    println!("Adv  (stima):  {}", td);
    let ttot = tn + ta + td;
    println!("Totale:        {} / {} ({:.1}%)", ttot, n_none, pct(ttot));

    // Affinità frattali: quante parole hanno affinità definite
    let with_aff = words.iter().filter(|ws| !ws.fractal_affinities.is_empty()).count();
    println!("\n--- AFFINITA FRATTALI ---");
    println!("Parole con affinita: {} ({:.1}%)", with_aff, pct(with_aff));
    let avg_aff = words.iter().map(|ws| ws.fractal_affinities.len()).sum::<usize>() as f64 / total as f64;
    println!("Media affinita/parola: {:.1}", avg_aff);
}
