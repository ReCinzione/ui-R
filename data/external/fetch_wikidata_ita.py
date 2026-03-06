#!/usr/bin/env python3
"""
Scarica triple semantiche italiane da Wikidata SPARQL.

Strategia:
  - Query mirate per categorie italiane importanti
  - Relazioni: IS_A (P279/P31), HAS (P527), PART_OF (P361),
    SIMILAR_TO (P5973 sinonimi), OPPOSITE (P1889 diverso-da)
  - Output: data/kg/wikidata_ita.tsv

Uso:
  python3 data/external/fetch_wikidata_ita.py
  # poi:
  cargo run --release --bin import-kg
  cargo run --release --bin rebuild-semantic-topology
"""

import requests
import time
import os
import sys
from pathlib import Path

SPARQL_URL = "https://query.wikidata.org/sparql"
HEADERS    = {"User-Agent": "Prometeo-KG/1.0 (educational semantic project; contact: local)"}

OUTPUT_DIR = Path(__file__).parent.parent / "kg"
OUTPUT_DIR.mkdir(exist_ok=True)
OUTPUT_FILE = OUTPUT_DIR / "wikidata_ita.tsv"


# ═══════════════════════════════════════════════════════════════════════════
# SPARQL helpers
# ═══════════════════════════════════════════════════════════════════════════

def sparql(query: str, delay: float = 1.5) -> list:
    """Esegue SPARQL, ritorna lista di dizionari {var: value}."""
    try:
        resp = requests.get(
            SPARQL_URL,
            params={"query": query, "format": "json"},
            headers=HEADERS,
            timeout=60,
        )
        resp.raise_for_status()
        bindings = resp.json()["results"]["bindings"]
        time.sleep(delay)
        return [{k: v["value"] for k, v in row.items()} for row in bindings]
    except Exception as e:
        print(f"  [ERRORE] {e}", file=sys.stderr)
        time.sleep(3)
        return []


def label(uri: str) -> str:
    """Estrae la label leggibile da un URI Wikidata: /c/it/cane → cane."""
    return uri.split("/")[-1].lower().replace("_", " ").strip()


# ═══════════════════════════════════════════════════════════════════════════
# QUERY TEMPLATES
# ═══════════════════════════════════════════════════════════════════════════

def q_subclass_ita(category_qid: str, limit: int = 500) -> str:
    """Tutte le sottoclassi dirette di una categoria con label italiana."""
    return f"""
SELECT DISTINCT ?itemLabel ?superLabel WHERE {{
  ?item wdt:P279 wd:{category_qid} .
  ?item rdfs:label ?itemLabel . FILTER(LANG(?itemLabel) = "it")
  wd:{category_qid} rdfs:label ?superLabel . FILTER(LANG(?superLabel) = "it")
}} LIMIT {limit}"""


def q_instance_ita(category_qid: str, limit: int = 500) -> str:
    """Tutte le istanze dirette di una categoria con label italiana."""
    return f"""
SELECT DISTINCT ?itemLabel ?superLabel WHERE {{
  ?item wdt:P31 wd:{category_qid} .
  ?item rdfs:label ?itemLabel . FILTER(LANG(?itemLabel) = "it")
  wd:{category_qid} rdfs:label ?superLabel . FILTER(LANG(?superLabel) = "it")
}} LIMIT {limit}"""


def q_has_part_ita(subject_qid: str, limit: int = 200) -> str:
    """Parti di un oggetto (subject HAS part)."""
    return f"""
SELECT DISTINCT ?subjectLabel ?partLabel WHERE {{
  wd:{subject_qid} wdt:P527 ?part .
  wd:{subject_qid} rdfs:label ?subjectLabel . FILTER(LANG(?subjectLabel) = "it")
  ?part rdfs:label ?partLabel . FILTER(LANG(?partLabel) = "it")
}} LIMIT {limit}"""


def q_part_of_ita(object_qid: str, limit: int = 200) -> str:
    """Cose che sono parte di un oggetto (subject PART_OF object)."""
    return f"""
SELECT DISTINCT ?partLabel ?wholeLabel WHERE {{
  ?part wdt:P361 wd:{object_qid} .
  ?part rdfs:label ?partLabel . FILTER(LANG(?partLabel) = "it")
  wd:{object_qid} rdfs:label ?wholeLabel . FILTER(LANG(?wholeLabel) = "it")
}} LIMIT {limit}"""


# ═══════════════════════════════════════════════════════════════════════════
# CATEGORIE DA SCARICARE
# ═══════════════════════════════════════════════════════════════════════════

# (nome_italiano, QID, tipo: "subclass"|"instance"|"both")
IS_A_CATEGORIES = [
    # ── Esseri viventi ────────────────────────────────────────────────────
    ("animale",        "Q729",    "subclass"),
    ("mammifero",      "Q7377",   "subclass"),
    ("uccello",        "Q5113",   "subclass"),
    ("pesce",          "Q152",    "subclass"),
    ("rettile",        "Q10811",  "subclass"),
    ("insetto",        "Q1390",   "subclass"),
    ("pianta",         "Q756",    "subclass"),
    ("albero",         "Q10884",  "subclass"),
    ("fungo",          "Q764",    "subclass"),
    ("essere_vivente", "Q3327",   "subclass"),
    ("vertebrato",     "Q25241",  "subclass"),
    # ── Corpo umano ───────────────────────────────────────────────────────
    ("organo_umano",   "Q712378", "instance"),
    ("ossa",           "Q265868", "subclass"),
    ("muscolo",        "Q16521",  "instance"),
    # ── Concetti astratti ─────────────────────────────────────────────────
    ("emozione",       "Q9415",   "subclass"),
    ("sentimento",     "Q16748",  "subclass"),
    ("colore",         "Q1075",   "instance"),
    ("lingua",         "Q34770",  "instance"),
    # ── Istituzioni / Luoghi ─────────────────────────────────────────────
    ("stato",          "Q6256",   "instance"),   # nazione
    ("città",          "Q515",    "instance"),
    ("continente",     "Q5107",   "instance"),
    ("oceano",         "Q9430",   "instance"),
    ("montagna",       "Q8502",   "subclass"),
    ("fiume",          "Q4022",   "subclass"),
    # ── Oggetti / Artefatti ───────────────────────────────────────────────
    ("strumento_musicale", "Q34379", "subclass"),
    ("veicolo",        "Q42889",  "subclass"),
    ("edificio",       "Q41176",  "subclass"),
    ("cibo",           "Q2095",   "subclass"),
    ("bevanda",        "Q40050",  "subclass"),
    ("sport",          "Q349",    "subclass"),
    # ── Persone / Professioni ─────────────────────────────────────────────
    ("professione",    "Q28640",  "subclass"),
    ("scienziato",     "Q901",    "instance"),
    ("artista",        "Q483501", "subclass"),
    # ── Fenomeni naturali ─────────────────────────────────────────────────
    ("fenomeno_meteo", "Q14914980", "subclass"),
    ("astro",          "Q6999",   "subclass"),  # corpo celeste
    ("elemento_chimico", "Q11344", "instance"),
]

# Oggetti di cui scaricare le parti (HAS relazione)
HAS_PARTS_OF = [
    ("corpo_umano",    "Q23852"),  # corpo umano ha: cuore, polmoni, cervello...
    ("cervello",       "Q1073"),
    ("occhio",         "Q430"),
    ("casa",           "Q3947"),
    ("automobile",     "Q1420"),
    ("computer",       "Q68"),
    ("libro",          "Q571"),
    ("albero",         "Q10884"),
]

# Oggetti di cui scaricare i "parti di" (PART_OF)
PART_OF_OBJECTS = [
    ("sistema_solare", "Q544"),
    ("terra",          "Q2"),
    ("europa",         "Q46"),
    ("italia",         "Q38"),
    ("universo",       "Q1"),
]


# ═══════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════

def clean_word(s: str) -> str:
    """Normalizza una label per il KG: minuscolo, no spazi extra."""
    return s.lower().strip().replace("  ", " ").replace(" ", "_")


def is_valid(w: str) -> bool:
    """Filtra parole non italiane o non significative."""
    if len(w) < 2 or len(w) > 40: return False
    if any(c.isdigit() for c in w): return False
    # Rimuovi nomi propri evidenti (iniziano con maiuscola dopo clean)
    return True


def main():
    triples: list[tuple[str, str, str]] = []
    seen = set()

    def add(subj: str, rel: str, obj: str):
        subj = clean_word(subj)
        obj  = clean_word(obj)
        if not (is_valid(subj) and is_valid(obj)): return
        key = (subj, rel, obj)
        if key not in seen:
            seen.add(key)
            triples.append(key)

    print("=== Fetch Wikidata Italiano ===")

    # ── IS_A da categorie ──────────────────────────────────────────────────
    print(f"\n[IS_A] Scaricando {len(IS_A_CATEGORIES)} categorie...")
    for cat_name, qid, typ in IS_A_CATEGORIES:
        print(f"  {cat_name} ({qid})...", end="", flush=True)

        if typ in ("subclass", "both"):
            rows = sparql(q_subclass_ita(qid, 400))
            for r in rows:
                item  = r.get("itemLabel", "")
                super_ = r.get("superLabel", cat_name)
                if item: add(item, "IS_A", super_)
            print(f" subclass={len(rows)}", end="", flush=True)

        if typ in ("instance", "both"):
            rows = sparql(q_instance_ita(qid, 400))
            for r in rows:
                item  = r.get("itemLabel", "")
                super_ = r.get("superLabel", cat_name)
                if item: add(item, "IS_A", super_)
            print(f" instance={len(rows)}", end="", flush=True)

        print()

    # ── HAS (parti di oggetti) ────────────────────────────────────────────
    print(f"\n[HAS] Scaricando parti di {len(HAS_PARTS_OF)} oggetti...")
    for obj_name, qid in HAS_PARTS_OF:
        print(f"  {obj_name}...", end="", flush=True)
        rows = sparql(q_has_part_ita(qid, 150))
        for r in rows:
            subject = r.get("subjectLabel", obj_name)
            part    = r.get("partLabel", "")
            if part: add(subject, "HAS", part)
        print(f" {len(rows)} parti")

    # ── PART_OF ───────────────────────────────────────────────────────────
    print(f"\n[PART_OF] Scaricando contenuti di {len(PART_OF_OBJECTS)} oggetti...")
    for obj_name, qid in PART_OF_OBJECTS:
        print(f"  {obj_name}...", end="", flush=True)
        rows = sparql(q_part_of_ita(qid, 150))
        for r in rows:
            part  = r.get("partLabel", "")
            whole = r.get("wholeLabel", obj_name)
            if part: add(part, "PART_OF", whole)
        print(f" {len(rows)} elementi")

    # ── SIMILAR_TO da sinonimi Wikidata Lexeme ────────────────────────────
    # (Questa query è più pesante, facoltativa)
    print("\n[SIMILAR_TO] Sinonimi da Wikidata Lexemes (italiano)...")
    syn_query = """
SELECT DISTINCT ?wordA ?wordB WHERE {
  ?lexA dct:language wd:Q652 ;
        wikibase:lemma ?wordA ;
        ontolex:sense/^skos:closeMatch/ontolex:sense/^ontolex:sense ?lexB .
  ?lexB wikibase:lemma ?wordB .
  FILTER(LANG(?wordA) = "it" && LANG(?wordB) = "it")
  FILTER(?wordA != ?wordB)
} LIMIT 500"""
    rows = sparql(syn_query, delay=2.0)
    for r in rows:
        wa = r.get("wordA", "")
        wb = r.get("wordB", "")
        if wa and wb:
            add(wa, "SIMILAR_TO", wb)
            add(wb, "SIMILAR_TO", wa)
    print(f"  {len(rows)} sinonimi")

    # ── Salva ─────────────────────────────────────────────────────────────
    print(f"\n=== Totale triple: {len(triples)} ===")
    print(f"Salvo in: {OUTPUT_FILE}")

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        f.write("# Wikidata Italia — triple semantiche\n")
        f.write("# Auto-generato da fetch_wikidata_ita.py\n")
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
