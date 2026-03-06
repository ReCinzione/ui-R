#!/usr/bin/env python3
"""
Scarica triple semantiche italiane da ConceptNet 5 REST API.

ConceptNet è un knowledge graph multilingue curato (MIT License).
API: https://api.conceptnet.io/c/it/{parola}

Strategia:
  1. Parte dal vocabolario italiano di base (parole comuni)
  2. Per ogni parola: scarica tutte le relazioni
  3. Converte al formato KG di Prometeo (TSV)
  4. Espande ricorsivamente 1 livello

Relazioni ConceptNet → KG Prometeo:
  /r/IsA         → IS_A
  /r/HasA        → HAS
  /r/PartOf      → PART_OF
  /r/UsedFor     → USED_FOR
  /r/Causes      → CAUSES
  /r/Antonym     → OPPOSITE_OF
  /r/RelatedTo   → SIMILAR_TO (solo se stesso lemma italiano)
  /r/SimilarTo   → SIMILAR_TO
  /r/CapableOf   → DOES

Output: data/kg/conceptnet_ita.tsv

Uso:
  python3 data/external/fetch_conceptnet_ita.py
"""

import requests
import time
import json
from pathlib import Path

OUTPUT_DIR  = Path(__file__).parent.parent / "kg"
OUTPUT_DIR.mkdir(exist_ok=True)
OUTPUT_FILE = OUTPUT_DIR / "conceptnet_ita.tsv"

API_BASE = "https://api.conceptnet.io"
HEADERS  = {"User-Agent": "Prometeo-KG/1.0 (educational; contact: local)"}

# ── Mappa ConceptNet → formato KG Prometeo ────────────────────────────────
REL_MAP = {
    "/r/IsA":        "IS_A",
    "/r/HasA":       "HAS",
    "/r/PartOf":     "PART_OF",
    "/r/UsedFor":    "USED_FOR",
    "/r/Causes":     "CAUSES",
    "/r/Antonym":    "OPPOSITE_OF",
    "/r/SimilarTo":  "SIMILAR_TO",
    "/r/RelatedTo":  "SIMILAR_TO",
    "/r/CapableOf":  "DOES",
    "/r/MadeOf":     "HAS",
    "/r/HasProperty": "HAS",
    "/r/CausesDesire": "CAUSES",
}

# ── Vocabolario di partenza ───────────────────────────────────────────────
# Parole italiane comuni — espandiamo da queste
SEED_WORDS = [
    # Identità / sé
    "io", "me", "sé", "identità", "coscienza", "mente", "anima", "spirito",
    # Esseri
    "persona", "uomo", "donna", "bambino", "umano", "essere",
    # Corpo
    "corpo", "testa", "cuore", "mano", "occhio", "bocca", "piede", "braccio",
    "cervello", "sangue", "pelle", "osso", "muscolo", "stomaco",
    # Emozioni
    "amore", "paura", "gioia", "tristezza", "rabbia", "felicità", "dolore",
    "speranza", "nostalgia", "pace", "ansia", "calma", "orgoglio", "vergogna",
    # Azioni fondamentali
    "fare", "essere", "avere", "andare", "venire", "vedere", "sapere", "pensare",
    "sentire", "parlare", "mangiare", "dormire", "vivere", "morire", "nascere",
    "camminare", "correre", "toccare", "ascoltare", "leggere", "scrivere",
    # Natura
    "sole", "luna", "terra", "acqua", "fuoco", "aria", "vento", "pioggia",
    "cielo", "mare", "montagna", "albero", "fiore", "erba", "foresta",
    # Tempo
    "tempo", "giorno", "notte", "ora", "momento", "passato", "futuro",
    "mattina", "sera", "anno", "estate", "inverno", "primavera", "autunno",
    # Animali
    "cane", "gatto", "cavallo", "uccello", "pesce", "leone", "lupo", "orso",
    "elefante", "aquila", "serpente", "farfalla",
    # Concetti
    "vita", "morte", "verità", "libertà", "giustizia", "bellezza", "bontà",
    "male", "bene", "realtà", "sogno", "memoria", "pensiero", "idea",
    # Relazioni
    "amico", "nemico", "famiglia", "madre", "padre", "figlio", "fratello",
    # Comunicazione
    "parola", "lingua", "voce", "suono", "silenzio", "domanda", "risposta",
    "ciao", "saluto", "nome",
    # Luoghi
    "casa", "città", "paese", "mondo", "universo", "spazio", "luogo",
    # Oggetti
    "libro", "luce", "porta", "strada", "ponte",
    # Qualità
    "grande", "piccolo", "forte", "debole", "bello", "brutto",
    "vecchio", "nuovo", "vero", "falso", "caldo", "freddo",
    # Cibo
    "cibo", "pane", "acqua", "vino", "frutto", "verdura",
]


def fetch_word(word: str) -> list[tuple[str, str, str]]:
    """Scarica tutte le relazioni per una parola. Ritorna (subj, rel, obj)."""
    url = f"{API_BASE}/c/it/{word.lower().replace(' ', '_')}?limit=50"
    try:
        resp = requests.get(url, headers=HEADERS, timeout=10)
        if resp.status_code != 200:
            return []
        data = resp.json()
    except Exception:
        return []

    triples = []
    for edge in data.get("edges", []):
        rel_uri = edge.get("rel", {}).get("@id", "")
        rel = REL_MAP.get(rel_uri)
        if not rel:
            continue

        start = edge.get("start", {})
        end   = edge.get("end", {})

        start_lang = start.get("language", "")
        end_lang   = end.get("language", "")

        # Solo relazioni dove almeno un lato è italiano
        if start_lang != "it" and end_lang != "it":
            continue

        start_label = start.get("label", "").lower().strip()
        end_label   = end.get("label", "").lower().strip()

        if not start_label or not end_label:
            continue

        # Filtra: niente di troppo lungo, niente caratteri strani
        if len(start_label) > 30 or len(end_label) > 30:
            continue

        triples.append((start_label, rel, end_label))

    return triples


def clean(w: str) -> str:
    return w.lower().strip().replace(" ", "_")


def main():
    triples: set[tuple[str, str, str]] = set()

    print("=== Fetch ConceptNet Italiano ===")
    print(f"Parole seed: {len(SEED_WORDS)}")
    print()

    # ── Giro 1: parole seed ───────────────────────────────────────────────
    expanded_words = set()
    for i, word in enumerate(SEED_WORDS):
        print(f"  [{i+1}/{len(SEED_WORDS)}] {word}...", end="", flush=True)
        new = fetch_word(word)
        for (s, r, o) in new:
            triples.add((clean(s), r, clean(o)))
            # Aggiungi le parole italiane trovate per il secondo giro
            for w in [s, o]:
                if len(w.split()) <= 2 and len(w) <= 25:
                    expanded_words.add(w.lower())
        print(f" {len(new)} relazioni")
        time.sleep(0.5)  # gentile con l'API

    print(f"\nTriple dopo giro 1: {len(triples)}")
    print(f"Parole nuove da espandere: {len(expanded_words - set(SEED_WORDS))}")

    # ── Giro 2: espansione (max 100 parole nuove) ─────────────────────────
    new_words = list(expanded_words - set(w.lower() for w in SEED_WORDS))[:100]
    print(f"\nGiro 2: {len(new_words)} parole...")
    for i, word in enumerate(new_words):
        if i % 10 == 0:
            print(f"  {i}/{len(new_words)}...", flush=True)
        new = fetch_word(word)
        for (s, r, o) in new:
            triples.add((clean(s), r, clean(o)))
        time.sleep(0.4)

    print(f"\nTriple totali: {len(triples)}")

    # ── Salva ─────────────────────────────────────────────────────────────
    print(f"Salvo in: {OUTPUT_FILE}")
    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        f.write("# ConceptNet Italia — triple semantiche\n")
        f.write("# Auto-generato da fetch_conceptnet_ita.py\n")
        f.write("# Formato: soggetto\tRELAZIONE\toggetto\n\n")
        for (s, r, o) in sorted(triples):
            f.write(f"{s}\t{r}\t{o}\n")

    print("✓ Fatto.")
    print()
    print("Ora esegui:")
    print("  cargo run --release --bin import-kg")
    print("  cargo run --release --bin rebuild-semantic-topology")


if __name__ == "__main__":
    main()
