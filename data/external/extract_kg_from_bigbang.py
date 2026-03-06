#!/usr/bin/env python3
"""
Estrae triple KG dalle lezioni BigBang.

Formato BigBang:
  ancora: parola1 parola2 parola3 [/ contrario1 contrario2]

Regole di estrazione:
  ancora + parole_prima_slash  → SIMILAR_TO (bidirezionale)
  ancora + parole_dopo_slash   → OPPOSITE_OF (bidirezionale)

Output: data/kg/bigbang_kg.tsv
"""

from pathlib import Path
import re

LESSONS = Path(__file__).parent / "bigbang_lessons.txt"
OUTPUT  = Path(__file__).parent.parent / "kg" / "bigbang_kg.tsv"

# Caratteri validi per una parola italiana
VALID = re.compile(r'^[a-zàèéìíîòóùúüä\-]+$')

def is_word(w: str) -> bool:
    return bool(w) and 2 <= len(w) <= 30 and bool(VALID.match(w))

def clean(w: str) -> str:
    return w.strip().lower()


def main():
    triples: set[tuple[str, str, str]] = set()

    with open(LESSONS, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"): continue
            if ":" not in line: continue

            anchor_raw, rest = line.split(":", 1)
            anchor = clean(anchor_raw)
            if not is_word(anchor): continue

            # Separa simili e opposti
            if "/" in rest:
                similar_part, opposite_part = rest.split("/", 1)
            else:
                similar_part, opposite_part = rest, ""

            similar_words  = [clean(w) for w in similar_part.split()  if is_word(clean(w))]
            opposite_words = [clean(w) for w in opposite_part.split() if is_word(clean(w))]

            # SIMILAR_TO — bidirezionale
            for w in similar_words:
                if w != anchor:
                    triples.add((anchor, "SIMILAR_TO", w))
                    triples.add((w, "SIMILAR_TO", anchor))

            # OPPOSITE_OF — bidirezionale
            for w in opposite_words:
                if w != anchor:
                    triples.add((anchor, "OPPOSITE_OF", w))
                    triples.add((w, "OPPOSITE_OF", anchor))

            # Tra parole dello stesso cluster (solo simili, non ridondanti)
            # Limite: max 3 coppie per cluster per evitare esplosione
            for i, a in enumerate(similar_words[:4]):
                for b in similar_words[i+1:4]:
                    if a != b:
                        triples.add((a, "SIMILAR_TO", b))
                        triples.add((b, "SIMILAR_TO", a))

    print(f"Triple estratte: {len(triples)}")

    similar_count  = sum(1 for _, r, _ in triples if r == "SIMILAR_TO")
    opposite_count = sum(1 for _, r, _ in triples if r == "OPPOSITE_OF")
    print(f"  SIMILAR_TO:  {similar_count}")
    print(f"  OPPOSITE_OF: {opposite_count}")

    with open(OUTPUT, "w", encoding="utf-8") as f:
        f.write("# KG estratto dalle lezioni BigBang di Prometeo\n")
        f.write("# Relazioni: SIMILAR_TO (cluster Kaikki), OPPOSITE_OF (antonimi)\n")
        f.write("# soggetto\tRELAZIONE\toggetto\n\n")
        for (s, r, o) in sorted(triples):
            f.write(f"{s}\t{r}\t{o}\n")

    print(f"Salvato: {OUTPUT}")


if __name__ == "__main__":
    main()
