#!/usr/bin/env python3
"""
fetch_leipzig.py — Scarica corpus italiano Leipzig e prepara per Prometeo.

Leipzig Corpora: frasi reali da giornali/Wikipedia/web italiani.
Formato output: una frase per riga, pronto per teach_corpus.

Uso:
    py -3 fetch_leipzig.py                         # news 100K + wiki 100K (default)
    py -3 fetch_leipzig.py --sources news          # solo news
    py -3 fetch_leipzig.py --sources wiki          # solo wiki
    py -3 fetch_leipzig.py --sources news wiki web # tutti e tre
    py -3 fetch_leipzig.py --size 300K             # 300K frasi per sorgente
    py -3 fetch_leipzig.py --local file.txt        # usa file già scaricato
"""

import sys
import os
import re
import argparse
import tarfile
import urllib.request

# ─── Configurazione ───────────────────────────────────────────────────────────

BASE_URL = "https://downloads.wortschatz-leipzig.de/corpora"

CORPORA = {
    ("news", "10K"):   "ita_news_2023_10K.tar.gz",
    ("news", "30K"):   "ita_news_2023_30K.tar.gz",
    ("news", "100K"):  "ita_news_2023_100K.tar.gz",
    ("news", "300K"):  "ita_news_2023_300K.tar.gz",
    ("wiki", "10K"):   "ita_wikipedia_2021_10K.tar.gz",
    ("wiki", "100K"):  "ita_wikipedia_2021_100K.tar.gz",
    ("wiki", "300K"):  "ita_wikipedia_2021_300K.tar.gz",
    ("web",  "100K"):  "ita-it_web_2013_100K.tar.gz",
    ("web",  "300K"):  "ita-it_web_2013_300K.tar.gz",
}

OUTPUT_FILE = "corpus_italiano.txt"

# ─── Filtri qualità ───────────────────────────────────────────────────────────

MIN_WORDS = 5
MAX_WORDS = 60

IT_MARKERS = {"di", "la", "il", "che", "è", "in", "un", "e", "non", "per",
              "del", "al", "le", "da", "con", "una", "lo", "si", "ha", "i"}

def is_italian(sentence: str) -> bool:
    words = sentence.lower().split()
    if not words:
        return False
    italian_words = sum(1 for w in words if w.strip(".,;:!?\"'") in IT_MARKERS)
    return italian_words >= max(1, len(words) // 8)

def clean_sentence(line: str) -> str | None:
    """Pulisce una riga del corpus Leipzig. Formato: '<id>\t<frase>'"""
    if "\t" in line:
        line = line.split("\t", 1)[1]
    line = line.strip()
    if not line or line.startswith("#"):
        return None
    if "http://" in line or "https://" in line:
        return None
    line = re.sub(r"[\x00-\x1f\x7f-\x9f]", " ", line)
    line = re.sub(r"\s+", " ", line).strip()
    words = line.split()
    if len(words) < MIN_WORDS or len(words) > MAX_WORDS:
        return None
    if not is_italian(line):
        return None
    num_count = sum(1 for w in words if re.match(r"^\d+[\.,]?\d*$", w))
    if num_count > len(words) * 0.3:
        return None
    return line

def process_sentences_file(filepath: str) -> list:
    """Legge file Leipzig e restituisce frasi pulite."""
    sentences = []
    skipped = 0
    with open(filepath, "r", encoding="utf-8", errors="replace") as f:
        for line in f:
            cleaned = clean_sentence(line)
            if cleaned:
                sentences.append(cleaned)
            else:
                skipped += 1
    print(f"  Frasi valide: {len(sentences):,}  |  Scartate: {skipped:,}")
    return sentences

def download_corpus(source: str, size: str, dest_dir: str) -> str | None:
    """Scarica il corpus Leipzig e restituisce il path del file frasi."""
    key = (source, size)
    if key not in CORPORA:
        print(f"Corpus ({source}, {size}) non disponibile.")
        print(f"Opzioni: {list(CORPORA.keys())}")
        return None

    filename = CORPORA[key]
    url = f"{BASE_URL}/{filename}"
    local_tar = os.path.join(dest_dir, filename)

    if not os.path.exists(local_tar):
        print(f"Download [{source} {size}]: {url}")
        try:
            def progress(count, block_size, total_size):
                if total_size > 0:
                    pct = min(100, count * block_size * 100 // total_size)
                    mb_done = count * block_size // 1024 // 1024
                    mb_total = total_size // 1024 // 1024
                    sys.stdout.write(f"\r  {pct}% ({mb_done}MB / {mb_total}MB)   ")
                    sys.stdout.flush()
            urllib.request.urlretrieve(url, local_tar, progress)
            print()
        except Exception as e:
            print(f"\nErrore download: {e}")
            print(f"Scarica manualmente da: {url}")
            print(f"Salva in: {local_tar}")
            return None
    else:
        print(f"File gia presente: {local_tar}")

    print("Estrazione...")
    sentences_file = None
    with tarfile.open(local_tar, "r:gz") as tar:
        for member in tar.getmembers():
            if member.name.endswith("-sentences.txt"):
                tar.extract(member, dest_dir)
                sentences_file = os.path.join(dest_dir, member.name)
                print(f"  Estratto: {member.name}")
                break

    return sentences_file

def main():
    parser = argparse.ArgumentParser(description="Scarica corpus Leipzig IT per Prometeo")
    parser.add_argument("--sources", nargs="+", choices=["news", "wiki", "web"],
                        default=["news", "wiki"],
                        help="Sorgenti da scaricare e unire (default: news wiki)")
    parser.add_argument("--size", choices=["10K", "30K", "100K", "300K"], default="100K",
                        help="Dimensione per sorgente (default: 100K)")
    parser.add_argument("--local", metavar="FILE",
                        help="Usa file gia scaricato (singola sorgente)")
    parser.add_argument("--output", default=OUTPUT_FILE,
                        help=f"File output (default: {OUTPUT_FILE})")
    args = parser.parse_args()

    script_dir = os.path.dirname(os.path.abspath(__file__))
    output_path = os.path.join(script_dir, args.output)

    print("=" * 60)
    print("  FETCH LEIPZIG — Corpus italiano per Prometeo")
    print("=" * 60)

    all_sentences = []

    if args.local:
        # Modalità file locale singolo
        print(f"File locale: {args.local}")
        sentences = process_sentences_file(args.local)
        all_sentences.extend(sentences)
    else:
        # Scarica ogni sorgente richiesta
        sources_label = " + ".join(f"{s} {args.size}" for s in args.sources)
        print(f"Sorgenti: {sources_label}\n")

        for source in args.sources:
            print(f"--- {source.upper()} ---")
            sentences_file = download_corpus(source, args.size, script_dir)
            if not sentences_file:
                print(f"  SKIP: {source} non scaricabile, continuo.")
                continue
            print(f"Processamento {source}...")
            sentences = process_sentences_file(sentences_file)
            all_sentences.extend(sentences)
            print()

    if not all_sentences:
        print("ERRORE: nessuna frase valida trovata.")
        sys.exit(1)

    # Deduplicazione globale (mantieni ordine)
    print(f"Totale frasi pre-dedup: {len(all_sentences):,}")
    seen = set()
    unique = []
    for s in all_sentences:
        key = s.lower()
        if key not in seen:
            seen.add(key)
            unique.append(s)
    print(f"Dopo deduplicazione:    {len(unique):,} frasi uniche")

    # Scrivi output
    sources_tag = "+".join(args.sources) if not args.local else "locale"
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(f"# Corpus italiano Leipzig — {sources_tag} {args.size}\n")
        f.write(f"# Frasi: {len(unique):,}\n")
        f.write(f"# Pronto per teach_corpus\n")
        f.write("#\n")
        for s in unique:
            f.write(s + "\n")

    print(f"\nSalvato: {output_path}")
    print(f"  {len(unique):,} frasi — pronto per teach_corpus")
    print(f"\nProssimo passo:")
    print(f"  cargo run --release --bin teach-corpus")

if __name__ == "__main__":
    main()
