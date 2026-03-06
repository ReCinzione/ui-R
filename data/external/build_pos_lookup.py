"""
Costruisce pos_lookup.tsv da:
  1. Morph-it! (current_version/morph-it_048.txt) — 505K forme, lemma+tag morfologico
  2. Word Frequency Lists ITA (nouns.csv, verbs.csv, adjectives.csv) — lemmi da corpus large

Output: pos_lookup.tsv  →  lemma TAB POS
POS values: V / N / Adj / Adv / Pro

Priorità: frequency lists (corpus-based) > Morph-it (morphological)
Per lemmi con più POS in Morph-it: N > Adj > V > Adv > Pro (euristica conservativa)
"""

import csv
import re

# ── Mapping tag Morph-it → nostro POS ──────────────────────────────────────
# Tag format examples: NOUN-M:s  ADJ:pos+m+s  VER:inf  ADV  PRO-PERS:m+s
MORPHIT_PREFIX_MAP = [
    ('NOUN', 'N'),
    ('ADJ',  'Adj'),
    ('VER',  'V'),
    ('ADV',  'Adv'),
    ('PRO',  'Pro'),
]
# Tags da ignorare (articoli, preposizioni, congiunzioni, punteggiatura, ecc.)
SKIP_PREFIXES = ('ART', 'PRE', 'CON', 'DET', 'SENT', 'PON', 'SYM', 'INT',
                 'NUM', 'ABL', 'NPR', 'WH', 'PREP', 'NOM')

def morphit_tag_to_pos(tag):
    for prefix, pos in MORPHIT_PREFIX_MAP:
        if tag.startswith(prefix):
            return pos
    return None

def is_good_lemma(lemma):
    """Solo lemmi alfabetici, almeno 2 char, senza apostrofi o trattini interni."""
    return len(lemma) >= 2 and all(c.isalpha() for c in lemma)

# Priorità numerica: più basso = più prioritario (si mantiene il più basso)
POS_PRIORITY = {'N': 1, 'Adj': 2, 'V': 3, 'Adv': 4, 'Pro': 5}

lemma_pos = {}

# ── Step 1: Morph-it! ────────────────────────────────────────────────────────
print("Parsing Morph-it!...")
morphit_count = 0
with open('current_version/morph-it_048.txt', encoding='latin-1') as f:
    for line in f:
        line = line.rstrip('\n')
        parts = line.split('\t')
        if len(parts) != 3:
            continue
        _form, lemma, tag = parts
        lemma = lemma.lower().strip()
        if not is_good_lemma(lemma):
            continue
        pos = morphit_tag_to_pos(tag)
        if pos is None:
            continue
        # Mantieni il POS a priorità più alta (numericamente più bassa)
        existing = lemma_pos.get(lemma)
        if existing is None or POS_PRIORITY.get(pos, 99) < POS_PRIORITY.get(existing, 99):
            lemma_pos[lemma] = pos
            morphit_count += 1

print(f"  Lemmi da Morph-it: {len(lemma_pos)} (da {morphit_count} assegnamenti)")

# ── Step 2: Word Frequency Lists — Nouns ────────────────────────────────────
print("Parsing nouns.csv...")
noun_count = 0
with open('nouns.csv', encoding='latin-1') as f:
    reader = csv.DictReader(f)
    seen = set()
    for row in reader:
        lemma = row['lemma'].strip('"').lower().strip()
        if not is_good_lemma(lemma) or lemma in seen:
            continue
        seen.add(lemma)
        lemma_pos[lemma] = 'N'   # corpus > morfologia
        noun_count += 1
print(f"  Lemmi Noun: {noun_count} unici")

# ── Step 3: Word Frequency Lists — Adjectives ───────────────────────────────
print("Parsing adjectives.csv...")
adj_count = 0
with open('adjectives.csv', encoding='latin-1') as f:
    reader = csv.DictReader(f)
    seen = set()
    for row in reader:
        lemma = row['lemma'].strip('"').lower().strip()
        if not is_good_lemma(lemma) or lemma in seen:
            continue
        seen.add(lemma)
        # Non sovrascriviamo un N con Adj (corpus noun più affidabile)
        if lemma_pos.get(lemma) != 'N':
            lemma_pos[lemma] = 'Adj'
            adj_count += 1
print(f"  Lemmi Adj: {adj_count} unici (non-Noun)")

# ── Step 4: Word Frequency Lists — Verbs ────────────────────────────────────
print("Parsing verbs.csv...")
verb_count = 0
with open('verbs.csv', encoding='latin-1') as f:
    reader = csv.DictReader(f)
    seen = set()
    for row in reader:
        lemma = row['lemma'].strip('"').lower().strip()
        if not is_good_lemma(lemma) or lemma in seen:
            continue
        seen.add(lemma)
        # Solo infiniti italiani riconoscibili come lemma (non forme coniugate)
        if lemma.endswith(('are', 'ere', 'ire')) and len(lemma) >= 5:
            # Non sovrascriviamo N/Adj con V
            if lemma_pos.get(lemma) not in ('N', 'Adj'):
                lemma_pos[lemma] = 'V'
                verb_count += 1
print(f"  Lemmi Verb (infiniti): {verb_count} unici (non-N/Adj)")

# ── Output ───────────────────────────────────────────────────────────────────
total = len(lemma_pos)
print(f"\nTotale lemmi nel lookup: {total}")
print(f"  N:   {sum(1 for p in lemma_pos.values() if p == 'N')}")
print(f"  V:   {sum(1 for p in lemma_pos.values() if p == 'V')}")
print(f"  Adj: {sum(1 for p in lemma_pos.values() if p == 'Adj')}")
print(f"  Adv: {sum(1 for p in lemma_pos.values() if p == 'Adv')}")
print(f"  Pro: {sum(1 for p in lemma_pos.values() if p == 'Pro')}")

with open('pos_lookup.tsv', 'w', encoding='utf-8') as f:
    for lemma, pos in sorted(lemma_pos.items()):
        f.write(f"{lemma}\t{pos}\n")

print(f"\nScritto: pos_lookup.tsv ({total} righe)")
