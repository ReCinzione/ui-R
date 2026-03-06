"""
Estrae le frasi esempio da kaikki-italian.jsonl.gz per noun/verb/adj.
Output: kaikki_examples.txt — una frase per riga, testo pulito.
"""
import gzip, json, sys, io, re
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

def is_good_sentence(text: str) -> bool:
    """Frasi italiane moderne: alfabeto latino, lunghezza ragionevole."""
    if not text or len(text) < 15 or len(text) > 300:
        return False
    # Rimuove frasi con troppi caratteri non latini (frasi medievali con abbreviazioni)
    alpha = sum(1 for c in text if c.isalpha())
    latin = sum(1 for c in text if c.isalpha() and ord(c) < 256)
    if alpha == 0 or latin / alpha < 0.85:
        return False
    # Scarta frasi che sembrano solo citazioni bibliche/latine (hanno troppe maiuscole)
    words = text.split()
    if len(words) < 3:
        return False
    caps = sum(1 for w in words if w and w[0].isupper() and len(w) > 2)
    if caps / len(words) > 0.6:  # più del 60% delle parole è maiuscolo → canto epico
        return False
    return True

def clean_sentence(text: str) -> str:
    """Pulizia minima: normalizza spazi, rimuove markup."""
    text = re.sub(r'\[.*?\]', '', text)  # rimuove [note a piè]
    text = re.sub(r'\s+', ' ', text).strip()
    return text

print("Estrazione frasi da Kaikki.org...")
sentences = []
seen: set[str] = set()

with gzip.open('kaikki-italian.jsonl.gz', 'rt', encoding='utf-8') as f:
    for line in f:
        e = json.loads(line)
        pos = e.get('pos', '')
        if pos not in ('noun', 'verb', 'adj', 'adv'):
            continue
        for sense in e.get('senses', []):
            for ex in sense.get('examples', []):
                text = ex.get('text', '').strip()
                text = clean_sentence(text)
                if is_good_sentence(text) and text not in seen:
                    seen.add(text)
                    sentences.append(text)

print(f"  Frasi estratte: {len(sentences)}")

with open('kaikki_examples.txt', 'w', encoding='utf-8') as f:
    f.write('\n'.join(sentences) + '\n')

print(f"Scritto: kaikki_examples.txt")
print()
print("--- CAMPIONE (prime 20) ---")
for s in sentences[:20]:
    print(f"  {s}")
