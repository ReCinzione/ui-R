"""
Genera bigbang_lessons.txt per il Big Bang lessicale di Prometeo.

Fonti:
  1. nouns.csv, verbs.csv, adjectives.csv  → top lemmi per frequenza (ItWaC corpus)
  2. kaikki-italian.jsonl.gz               → ancore semantiche (related + synonyms)

Output: bigbang_lessons.txt  →  lemma: ancora1 ancora2 ... (max 6)
Formato compatibile con teach_compact_file() di Prometeo (min 2 ancore).

Strategia ancore v2 — da stella a rete:
  1. Dirette Kaikki:  parole nella famiglia semantica/morfologica di W
  2. Sorelle Kaikki:  altri lemmi del BigBang che condividono le stesse ancore Kaikki
                      → crea connessioni trasversali tra parole nello stesso cluster
  3. Morfologiche:    lemmi del BigBang con stesso prefisso 4-char (famiglia derivazionale)
  4. Fallback POS:    parole generiche per POS (solo se <2 ancore trovate)

Risultato: ogni parola è connessa non solo ai suoi anchor, ma anche alle "sorelle"
che condividono gli stessi anchor → topologia a rete invece che a stella.
"""

import csv
import gzip
import json
import sys
import io
from collections import defaultdict

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

# ── Configurazione ────────────────────────────────────────────────────────────
TOP_NOUNS = 6000
TOP_VERBS = 5000
TOP_ADJS  = 4000
MIN_ANCHORS = 2
MAX_ANCHORS = 6
MAX_ANCHOR_LEN = 20

# Fallback generici — usati SOLO come ultima risorsa
FALLBACK_NOUN = ['cosa', 'parte', 'modo', 'vita', 'tempo', 'corpo', 'mondo', 'mente']
FALLBACK_VERB = ['essere', 'fare', 'avere', 'vedere', 'sentire', 'andare', 'venire', 'stare']
FALLBACK_ADJ  = ['grande', 'forte', 'vivo', 'vero', 'nuovo', 'antico', 'libero', 'buono']
ALL_FALLBACKS = set(FALLBACK_NOUN + FALLBACK_VERB + FALLBACK_ADJ)


def is_good_word(w: str) -> bool:
    return 3 <= len(w) <= MAX_ANCHOR_LEN and all(c.isalpha() for c in w)


# ── Step 1: Kaikki.org → indici forward + reverse ────────────────────────────
print("Caricamento Kaikki.org JSONL...")

# word → lista ordinata di ancore dirette (related + synonyms da senses)
kaikki_fwd: dict[str, list[str]] = {}
# word → lista di antonimi
kaikki_neg: dict[str, list[str]] = {}

with gzip.open('kaikki-italian.jsonl.gz', 'rt', encoding='utf-8') as f:
    for i, line in enumerate(f):
        entry = json.loads(line)
        if entry.get('pos', '') not in ('noun', 'adj', 'adv', 'verb'):
            continue
        word = entry.get('word', '').lower().strip()
        if not is_good_word(word):
            continue

        candidates: list[str] = []
        antonyms:   list[str] = []

        for r in entry.get('related', []):
            w = r.get('word', '').lower().strip()
            if w:
                candidates.append(w)
        for sense in entry.get('senses', []):
            for r in sense.get('related', []):
                w = r.get('word', '').lower().strip()
                if w:
                    candidates.append(w)
            for s in sense.get('synonyms', []):
                w = s.get('word', '').lower().strip()
                if w:
                    candidates.append(w)
            for a in sense.get('antonyms', []):
                w = a.get('word', '').lower().strip()
                if w:
                    antonyms.append(w)

        anchors: list[str] = []
        seen: set[str] = set()
        for w in candidates:
            if is_good_word(w) and w != word and w not in seen:
                seen.add(w)
                anchors.append(w)

        if word not in kaikki_fwd:
            kaikki_fwd[word] = anchors
            kaikki_neg[word] = antonyms
        else:
            existing = set(kaikki_fwd[word])
            for a in anchors:
                if a not in existing:
                    kaikki_fwd[word].append(a)
                    existing.add(a)

        if (i + 1) % 100000 == 0:
            print(f"  ...{i+1:,} voci, {len(kaikki_fwd):,} con ancore")

print(f"  Voci Kaikki con ancore: {len(kaikki_fwd):,}")


# ── Step 2: Carica liste di frequenza ─────────────────────────────────────────
def load_freq_list(path: str, top_n: int, verb_only: bool = False) -> list[str]:
    lemma_freq: dict[str, int] = {}
    with open(path, encoding='latin-1') as f:
        reader = csv.DictReader(f)
        for row in reader:
            lemma = row['lemma'].strip('"').lower().strip()
            if not is_good_word(lemma):
                continue
            if verb_only and not (lemma.endswith(('are', 'ere', 'ire')) and len(lemma) >= 5):
                continue
            try:
                freq = int(row['Freq'])
            except (ValueError, KeyError):
                continue
            if lemma not in lemma_freq or freq > lemma_freq[lemma]:
                lemma_freq[lemma] = freq
    return [w for w, _ in sorted(lemma_freq.items(), key=lambda x: -x[1])[:top_n]]


print("Caricamento liste di frequenza...")
nouns = load_freq_list('nouns.csv',      TOP_NOUNS)
verbs = load_freq_list('verbs.csv',      TOP_VERBS, verb_only=True)
adjs  = load_freq_list('adjectives.csv', TOP_ADJS)

# Set di tutti i lemmi BigBang — usato per filtrare sorelle e morfologiche
bigbang_set: set[str] = set(nouns + verbs + adjs)
print(f"  Nomi: {len(nouns)},  Verbi: {len(verbs)},  Aggettivi: {len(adjs)}")
print(f"  Totale BigBang: {len(bigbang_set):,}")


# ── Step 3: Indice reverse — ancora → lemmi BigBang che la usano ──────────────
# Permette di trovare le "sorelle": altri lemmi che condividono la stessa ancora Kaikki
print("Costruzione indice reverse...")

anchor_to_bigbang: dict[str, list[str]] = defaultdict(list)
for word in bigbang_set:
    for anchor in kaikki_fwd.get(word, []):
        # Solo ancore "ricche" (non i fallback generici) per il reverse index
        if anchor not in ALL_FALLBACKS and is_good_word(anchor):
            anchor_to_bigbang[anchor].append(word)

# Limita a max 50 parole per ancora (evita anchor-hub che connettono tutto)
for anchor in list(anchor_to_bigbang.keys()):
    if len(anchor_to_bigbang[anchor]) > 50:
        # Mantieni solo le parole più frequenti (già ordinate per frequenza)
        anchor_to_bigbang[anchor] = anchor_to_bigbang[anchor][:50]

print(f"  Anchor nel reverse index: {len(anchor_to_bigbang):,}")


# ── Step 4: Indice morfologico — prefisso 4 → lemmi BigBang ───────────────────
prefix_to_bigbang: dict[str, list[str]] = defaultdict(list)
for word in bigbang_set:
    if len(word) >= 5:  # prefisso significativo solo per parole ≥5 char
        prefix_to_bigbang[word[:4]].append(word)

# Limita e filtra prefissi con troppe parole (prefissi troppo generici)
for prefix in list(prefix_to_bigbang.keys()):
    group = prefix_to_bigbang[prefix]
    if len(group) > 30 or len(group) < 2:
        del prefix_to_bigbang[prefix]

print(f"  Gruppi morfologici (prefisso 4): {len(prefix_to_bigbang):,}")


# ── Step 5: Funzione ancore v2 ────────────────────────────────────────────────
def get_anchors_v2(word: str, fallback: list[str]) -> tuple[list[str], list[str]]:
    """
    Costruisce le ancore per word usando 3 livelli:
    1. Dirette Kaikki: anchor del dizionario (stessa famiglia semantica)
    2. Sorelle:        altri BigBang words che condividono le stesse ancore Kaikki
    3. Morfologiche:   BigBang words con stesso prefisso 4-char
    """
    result: list[str] = []
    seen: set[str] = {word}

    # Livello 1 — Kaikki dirette (max 4)
    direct = [a for a in kaikki_fwd.get(word, [])
              if is_good_word(a) and a not in seen]
    for a in direct[:4]:
        result.append(a)
        seen.add(a)

    # Livello 2 — Sorelle via anchor reverse (parole BigBang che condividono ancore)
    # Cerca sorelle attraverso le prime 3 ancore dirette
    sisters: list[str] = []
    for anchor in direct[:3]:
        for sister in anchor_to_bigbang.get(anchor, []):
            if sister not in seen and sister in bigbang_set:
                sisters.append(sister)
                seen.add(sister)
    # Prende fino a 2 sorelle (non di più, per non diluire il segnale)
    for s in sisters[:2]:
        if len(result) < MAX_ANCHORS:
            result.append(s)

    # Livello 3 — Morfologiche (stesso prefisso 4-char, solo se ancora pochi anchor)
    if len(result) < MIN_ANCHORS and len(word) >= 5:
        prefix = word[:4]
        for morph in prefix_to_bigbang.get(prefix, []):
            if morph not in seen and morph in bigbang_set:
                result.append(morph)
                seen.add(morph)
                if len(result) >= MIN_ANCHORS:
                    break

    # Livello 4 — Fallback generico (solo se ancora insufficienti)
    if len(result) < MIN_ANCHORS:
        for fb in fallback:
            if fb not in seen:
                result.append(fb)
                seen.add(fb)
                if len(result) >= MIN_ANCHORS:
                    break

    # Negativi da Kaikki
    neg = [a for a in kaikki_neg.get(word, []) if is_good_word(a)][:3]

    return result[:MAX_ANCHORS], neg


# ── Step 6: Genera lessons ────────────────────────────────────────────────────
print("Generazione lessons...")

lines: list[str] = [
    "# ============================================================",
    "# BigBang lessicale Prometeo v2 — logica cluster-based",
    "# Ancore: dirette Kaikki + sorelle + morfologiche + fallback",
    "# ============================================================",
]

stats = {'noun': 0, 'verb': 0, 'adj': 0, 'skipped': 0,
         'has_sisters': 0, 'has_morpho': 0, 'pure_fallback': 0}
all_written: set[str] = set()


def write_section(label: str, lemmas: list[str], fallback: list[str], key: str):
    lines.append(f"\n# --- {label} ---")
    for lemma in lemmas:
        if lemma in all_written:
            continue
        pos_anchors, neg_anchors = get_anchors_v2(lemma, fallback)
        if len(pos_anchors) < MIN_ANCHORS:
            stats['skipped'] += 1
            continue

        # Statistiche qualità
        direct_k = kaikki_fwd.get(lemma, [])
        sisters = [a for a in pos_anchors if a in bigbang_set and a not in direct_k]
        morpho = [a for a in pos_anchors
                  if len(lemma) >= 5 and a.startswith(lemma[:4]) and a not in direct_k]
        only_fallback = all(a in ALL_FALLBACKS for a in pos_anchors)
        if sisters:
            stats['has_sisters'] += 1
        if morpho:
            stats['has_morpho'] += 1
        if only_fallback:
            stats['pure_fallback'] += 1

        if neg_anchors:
            lines.append(f"{lemma}: {' '.join(pos_anchors)} / {' '.join(neg_anchors)}")
        else:
            lines.append(f"{lemma}: {' '.join(pos_anchors)}")
        all_written.add(lemma)
        stats[key] += 1


write_section("NOMI",       nouns, FALLBACK_NOUN, 'noun')
write_section("VERBI",      verbs, FALLBACK_VERB, 'verb')
write_section("AGGETTIVI",  adjs,  FALLBACK_ADJ,  'adj')


# ── Step 7: Scrivi output ─────────────────────────────────────────────────────
with open('bigbang_lessons.txt', 'w', encoding='utf-8') as f:
    f.write('\n'.join(lines) + '\n')

total = stats['noun'] + stats['verb'] + stats['adj']
print(f"\n--- RISULTATI BIGBANG v2 ---")
print(f"  Nomi:              {stats['noun']}")
print(f"  Verbi:             {stats['verb']}")
print(f"  Aggettivi:         {stats['adj']}")
print(f"  Totale lessons:    {total}")
print(f"  Saltati:           {stats['skipped']}")
print(f"  Con sorelle:       {stats['has_sisters']} ({100*stats['has_sisters']//total}%)")
print(f"  Con morfologiche:  {stats['has_morpho']} ({100*stats['has_morpho']//total}%)")
print(f"  Solo fallback:     {stats['pure_fallback']} ({100*stats['pure_fallback']//total}%)")
print(f"\nScritto: bigbang_lessons.txt")

# Campione qualitativo
print("\n--- CAMPIONE ---")
shown = 0
for line in lines:
    if line.startswith('#') or not line.strip():
        continue
    print(f"  {line}")
    shown += 1
    if shown >= 25:
        break
