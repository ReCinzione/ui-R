# Guida Completa al Sistema Compact

**Sistema di insegnamento rapido per Prometeo**
**Versione**: 1.0 (Phase 16)
**Data**: 2026-02-14

---

## 📋 Cos'è il Sistema Compact?

Il **formato compact** è un sistema di insegnamento acceler

ato che permette di creare lezioni con una sola riga per parola, invece di 4 frasi manuali.

**Prima** (formato tradizionale):
```
# Insegnare "nostalgia" richiede 4 frasi scritte a mano:
nostalgia prima lontano dolce
nostalgia dolce ricordare prima
io nostalgia ricordare lontano
nostalgia tristezza dentro freddo
```

**Dopo** (formato compact):
```
nostalgia: prima lontano ricordare dolce / ora qui vicino
# L'engine genera automaticamente 4 frasi strutturate ↑
```

**Velocità**: **4× più veloce** nella creazione manuale delle lezioni.

---

## 🎯 Formato Compact

### Sintassi Base

```
parola_target: ancora1 ancora2 ancora3 / negativa1 negativa2
```

**Componenti:**

| Parte | Descrizione | Obbligatorio |
|-------|-------------|--------------|
| `parola_target` | Nuova parola da insegnare | ✅ Sì |
| `:` | Separatore fisso | ✅ Sì |
| `ancora1 ancora2 ...` | Ancore positive (parole note) | ✅ Sì (min 2) |
| `/` | Separatore ancore/negative | ⚠️ Opzionale |
| `negativa1 negativa2` | Ancore negative (contrasti) | ⚠️ Opzionale ma consigliato |

### Esempi Validi

```
# Minimo (2 ancore, no negative):
gioia: io dentro caldo

# Ottimo (4 ancore + 2 negative):
gioia: io dentro caldo forte luce / tristezza freddo

# Con molte ancore (6 positive + 3 negative):
nostalgia: prima lontano ricordare dolce io tempo / ora qui vicino
```

### Esempi INVALIDI

```
❌ gioia io dentro caldo              # Manca ":"
❌ gioia:                              # Mancano ancore
❌ gioia: io                           # Solo 1 ancora (min 2)
❌ felicita: gioia caldo io            # "gioia" potrebbe non essere nota
❌ casa: qui dentro # commento         # Commenti inline non supportati
```

---

## 🔄 Generazione Automatica delle 4 Frasi

Dato l'input compact:
```
nostalgia: prima lontano ricordare dolce / ora qui vicino
```

L'engine genera:

```
1. DEFINITORIA (cos'è):
   nostalgia prima lontano ricordare io
   └─ prime 3 ancore + sempre "io"

2. PROSPETTIVA (come la percepisco):
   nostalgia io ricordare dolce prima
   └─ ancore ruotate (dalla posizione 2+) + io all'inizio

3. IO-PRIMA (soggettività):
   io nostalgia lontano dolce
   └─ io primo + ancore centrali

4. CONTRASTIVA (cosa NON è):
   nostalgia no ora no qui no vicino
   └─ parola + "no" + negative (fino a 3)
```

**Perché 4 frasi?**
- Crea 4 contesti diversi → firma 8D più ricca
- Co-occorrenze variate → campo topologico più denso
- Contrasti → separazione topologica (via negative)

---

## ✅ REGOLE ASSOLUTE

### REGOLA 1: Parola target NON deve esistere

**Verifica PRIMA di scrivere:**

```bash
cd c:\Users\Fra\Desktop\Prometeo\prometeo_standalone
target\release\prometeo.exe

# Nel prompt:
:load prometeo_state.bin
nostalgia  # Digita la parola

# ✅ Se dice "parola sconosciuta" → OK, puoi usarla
# ❌ Se mostra signature → GIÀ ESISTE, scegli altra parola
```

### REGOLA 2: TUTTE le ancore devono essere note

Le ancore sono le parole **dopo** i due punti (sia positive che negative).

**Vocabolario attuale: 482 parole** (da lezioni 00-14)

**Lista ancore sempre sicure** (cardinali + lezioni base):

```
qui, la, dentro, fuori, vicino, lontano, ora, prima, dopo, sempre, mai,
ancora, io, essere, sentire, pensare, volere, sapere, tu, noi, insieme,
dare, dire, amico, potere, forse, diventare, nuovo, speranza, possibile,
no, fine, limite, confine, regola, basta, corpo, mano, occhio, voce,
toccare, caldo, freddo, forte, debole, gioia, tristezza, paura, rabbia,
calma, amore, dolore, piangere, ridere, abbracciare, terra, cielo, acqua,
luce, buio, sole, luna, stella, vento, pioggia, albero, casa, mare,
montagna, ieri, domani, mattina, sera, notte, nascere, morire, crescere,
cambiare, aspettare, ricordare, dimenticare, madre, padre, figlio, fratello,
famiglia, parlare, ascoltare, capire, aiutare, fiducia, rispetto,
solitudine, idea, domanda, risposta, vero, falso, perche, come, cosa,
significare, cercare, trovare, scegliere, decidere, dubbio, certezza,
fare, creare, costruire, rompere, camminare, correre, fermare, aprire,
chiudere, prendere, lasciare, provare, riuscire, fallire, bello, brutto,
buono, cattivo, grande, piccolo, alto, basso, lungo, corto, duro, morbido,
dolce, amaro, chiaro, scuro, veloce, lento, pesante, leggero
(... + altre 240 dalle lezioni 08-14)
```

**Per verificare se un'ancora è nota:**
```bash
# Nel prompt prometeo:
ricordare
# ✅ Se mostra signature → OK, usa come ancora
# ❌ Se "sconosciuta" → NON usare
```

### REGOLA 3: Minimo 2 ancore positive, massimo 6

```
❌ TROPPO POCHE (< 2):
solitudine: io

✅ OK (2-3):
solitudine: io dentro freddo

✅ OTTIMO (4-5):
solitudine: io dentro freddo lontano tristezza

⚠️ TROPPE (> 6, funziona ma ridondante):
solitudine: io dentro freddo lontano tristezza calma silenzio buio notte
```

### REGOLA 4: Negative opzionali ma CONSIGLIATE

```
✅ Senza negative (funziona):
gioia: io dentro caldo forte

✅✅ Con negative (MIGLIORE):
gioia: io dentro caldo forte / tristezza freddo buio
```

**Perché?** Le negative creano **contrasto topologico** → separazione nel campo.

### REGOLA 5: Ancore semanticamente DIVERSE

Ogni ancora deve aggiungere informazione da un FRATTALE DIVERSO.

```
❌ MALE (tutte spaziali):
casa: qui dentro vicino chiudere
└─ Solo SPAZIO

✅ BENE (mix frattali):
casa: qui dentro famiglia caldo chiudere / lontano freddo
└─ SPAZIO + RELAZIONE + EMOZIONE + AZIONE + (negative)
```

**Frattali di riferimento:**

- **SPAZIO**: qui, la, dentro, fuori, vicino, lontano, alto, basso
- **TEMPO**: ora, prima, dopo, sempre, mai, ieri, domani
- **EGO**: io, essere, sentire, pensare, volere, sapere
- **RELAZIONE**: tu, noi, insieme, dare, dire, amico, famiglia
- **POTENZIALE**: potere, forse, diventare, nuovo, speranza, possibile
- **LIMITE**: no, fine, limite, confine, regola, basta, fermare
- **EMOZIONE**: gioia, tristezza, paura, rabbia, calma, amore, dolore
- **CORPO**: corpo, mano, occhio, voce, toccare, forte, debole

**Obiettivo:** ogni parola tocca 2-3+ frattali.

### REGOLA 6: NON usare parola target come ancora

```
❌ MALE:
amare: amore io tu dare
└─ "amore" è troppo simile, crea ridondanza

✅ BENE:
amare: io tu dare caldo insieme
```

---

## 🛠️ WORKFLOW COMPLETO

### STEP 1: Pianifica batch semantico

Scegli **un dominio** per batch (15-30 parole):

```
Esempi:
- Emozioni complesse (nostalgia, vergogna, orgoglio...)
- Relazioni familiari (nonno, nipote, cugino...)
- Fenomeni naturali (tempesta, lampo, tuono...)
- Stati cognitivi (intuizione, ragione, fantasia...)
- Movimenti corporei (saltare, cadere, volare...)
```

### STEP 2: Brainstorm parole

Lista 20-30 parole. **Criteri di scelta:**

✅ **SÌ:**
- Esperienziali (vissute dall'entità)
- Concrete o emotive
- Di uso comune
- Arricchiscono il campo semantico

❌ **NO:**
- Termini tecnici (neuroplasticità, fotosintesi)
- Nomi propri (Roma, Dante)
- Parole enciclopediche (auricola, coronarica)
- Parole già nel lessico

**Esempio** (dominio "Emozioni paura-correlate"):
```
timore, terrore, panico, ansia, angoscia,
preoccupazione, inquietudine, agitazione,
nervosismo, spavento
```

### STEP 3: Per ogni parola, trova ancore

**Metodo:** "Quando sento X, sento anche Y, Z, W"

```
Esempio: timore

Quando sento timore...
├─ sento paura (ma meno forte)
├─ sento freddo dentro
├─ sono incerto (dubbio)
├─ voglio fermare qualcosa
└─ NON sento calma, NON coraggio

Compact:
timore: paura freddo dentro dubbio fermare / calma coraggio
```

**Trucco:** usa i frattali come checklist.

| Frattale | Ancore candidate |
|----------|------------------|
| SPAZIO | qui/la/dentro/fuori/vicino/lontano |
| TEMPO | ora/prima/dopo/sempre/mai |
| EGO | io/sentire/pensare/volere |
| RELAZIONE | tu/noi/dare/insieme |
| EMOZIONE | gioia/tristezza/paura/calma |
| CORPO | corpo/forte/debole/toccare |

**Obiettivo:** ogni parola copre 2-3 frattali.

### STEP 4: Scrivi il file

```bash
# Crea file in lessons/
notepad lessons/16_emozioni_paura.txt
```

**Template:**

```
# Lezione 16: Emozioni correlate alla paura
# Dominio: stati emotivi asse paura-sicurezza
# Ancore: lezioni 00-14 (482 parole)

# === PAURA INTENSA ===
terrore: paura forte grande freddo io / calma luce
panico: paura forte veloce correre fuori / calma lento
orrore: paura forte brutto guardare no / bello calma

# === PAURA MODERATA ===
timore: paura freddo dentro dubbio io / calma coraggio
ansia: dentro freddo veloce paura io / calma lento
preoccupazione: pensare paura dopo io / calma ora

# === TENSIONE ===
nervosismo: dentro veloce io forte paura / calma lento
agitazione: forte veloce fuori io paura / calma
inquietudine: dentro io no calma paura / pace

...
```

**Best practices:**
- Commenti con `#` all'inizio riga
- Organizza per sotto-categorie (`# === NOME ===`)
- 15-30 parole per file
- Nome file: `NN_descrizione.txt` (NN = numero progressivo)

### STEP 5: Valida il file

**Controlli manuali:**

- [ ] Ogni riga ha formato `parola: ancore / negative`
- [ ] Nessuna parola target è già nel lessico
- [ ] Ogni ancora è nel lessico corrente
- [ ] Minimo 2 ancore per parola
- [ ] Ancore coprono 2+ frattali
- [ ] Batch semanticamente coeso

**Script Python (opzionale):**

```python
# tools/validate_compact.py
import sys

# Lista completa 482 parole (carica da file o hard-code)
KNOWN_WORDS = set([...])

def validate_line(line_num, line):
    if ':' not in line:
        return [f"Line {line_num}: missing ':'"]

    word, rest = line.split(':', 1)
    word = word.strip().lower()

    errors = []

    if word in KNOWN_WORDS:
        errors.append(f"Line {line_num}: '{word}' already exists")

    if '/' in rest:
        pos, neg = rest.split('/', 1)
    else:
        pos, neg = rest, ""

    anchors = pos.strip().split() + neg.strip().split()

    if len(pos.strip().split()) < 2:
        errors.append(f"Line {line_num}: need >= 2 positive anchors")

    for anchor in anchors:
        if anchor and anchor not in KNOWN_WORDS:
            errors.append(f"Line {line_num}: unknown anchor '{anchor}'")

    return errors

# Uso:
# python tools/validate_compact.py lessons/16_emozioni_paura.txt
```

### STEP 6: Testa il caricamento

```bash
cd c:\Users\Fra\Desktop\Prometeo\prometeo_standalone
target\release\prometeo.exe

# Nel prompt:
:load prometeo_state.bin
:compact lessons/16_emozioni_paura.txt

# Verifica output:
# ✅ "Parole nuove: N" (deve essere > 0)
# ✅ "Vocabolario ora: 482 + N"
# ✅ "Frasi generate: N×4"

# ❌ Se "Lezione già completata" → rinomina file
# ❌ Se crash → errore formato, controlla sintassi

:save prometeo_state.bin
:quit
```

---

## 📊 BEST PRACTICES

### 1. Batch size: 15-30 parole

```
❌ TROPPO POCHE (< 10):
- Overhead alto
- Frammentazione semantica

✅ OTTIMO (15-30):
- Batch coeso
- Tempo scrittura ~30 min

⚠️ TROPPE (> 50):
- Difficile mantenere coerenza
- Rischio errori
```

### 2. Progressione semantica

**Ordine consigliato** (dal concreto all'astratto):

```
1. Corpo e sensazioni → caldo, freddo, dolore, piacere
2. Emozioni base → gioia, paura, rabbia
3. Azioni fisiche → camminare, prendere, aprire
4. Relazioni concrete → madre, figlio, amico
5. Tempo e spazio → ieri, qui, lontano
6. Emozioni complesse → nostalgia, vergogna, orgoglio
7. Stati cognitivi → pensare, decidere, dubitare
8. Astratti esperienziali → libertà, giustizia, bellezza
```

### 3. Densità frattale

**Ogni batch deve toccare 3+ frattali:**

```
Esempio batch "Mestieri":

contadino: terra lavoro fare io mani / citta riposo
└─ NATURA + SOCIETA + AZIONE + EGO + CORPO (5 frattali)

maestro: insegnare sapere dare tu io / ignoranza
└─ SOCIETA + PENSIERO + RELAZIONE + EGO (4 frattali)
```

### 4. Evita ridondanza

**Ogni ancora deve aggiungere informazione:**

```
❌ RIDONDANTE:
freddo: gelo ghiaccio inverno basso
└─ Tutte dicono "freddo"

✅ INFORMATIVO:
freddo: io dentro corpo inverno sentire / caldo sole
└─ EGO + SPAZIO + CORPO + TEMPO + (negative)
```

### 5. Usa negative per contrasti

**Le negative creano separazione topologica:**

```
# Senza negative:
gioia: io dentro caldo luce
tristezza: io dentro freddo buio

Risultato: distanza 8D piccola (0.15?)

# Con negative:
gioia: io dentro caldo luce / tristezza freddo
tristezza: io dentro freddo buio / gioia caldo

Risultato: distanza 8D maggiore (0.25+)
└─ Contrasto esplicito → migliore differenziazione
```

---

## 🚨 ERRORI COMUNI

| Errore | Esempio | Fix |
|--------|---------|-----|
| **Ancora sconosciuta** | `casa: qui dentro comfort` | Usa solo parole note: `casa: qui dentro famiglia caldo` |
| **Target già esistente** | `gioia: ...` (già in 01_emozioni) | Controlla prima: `:load` + `gioia` |
| **Troppo poche ancore** | `casa: qui dentro` | Aggiungi almeno 1: `casa: qui dentro famiglia` |
| **Ancore stesso frattale** | `correre: camminare saltare` | Mix: `correre: io veloce fuori gambe` |
| **Target come ancora** | `amare: amore io tu` | Rimuovi: `amare: io tu dare caldo` |
| **Formato errato** | `casa qui dentro` | Aggiungi `:` → `casa: qui dentro` |
| **Commento inline** | `casa: qui dentro # mio` | Commenti solo su righe separate |

---

## ✅ CHECKLIST PRE-INSEGNAMENTO

Prima di eseguire `:compact`:

- [ ] File salvato in `lessons/NN_nome.txt`
- [ ] Ogni riga ha formato `parola: ancora1 ... / neg1 ...`
- [ ] Ogni parola target NON esiste nel lessico
- [ ] TUTTE le ancore esistono nel lessico
- [ ] Ogni parola ha 2-6 ancore positive
- [ ] Ogni parola (idealmente) ha 1-3 negative
- [ ] Ancore coprono 2+ frattali
- [ ] Batch semanticamente coeso (1 dominio)
- [ ] Nessuna ridondanza (ancore sinonime)
- [ ] Backup di `prometeo_state.bin` fatto

---

## 🎯 ESEMPIO COMPLETO

### Lezione "Cicli Vitali"

```
# Lezione 17: Cicli Vitali
# Dominio: nascita, crescita, morte, trasformazione
# Ancore: lezioni 00-14 (482 parole)

# === INIZIO ===
germogliare: seme nuovo nascere piccolo terra / morire
sbocciare: fiore aprire nuovo luce bello / chiudere morire
sorgere: sole nascere luce mattina alto / tramonto

# === CRESCITA ===
maturare: crescere io dopo grande forte / piccolo giovane
sviluppare: crescere nuovo io dentro forte / debole
fiorire: crescere bello forte luce vita / morire
prosperare: crescere forte grande bene io / debole

# === DECLINO ===
appassire: morire lento debole pianta / forte nuovo
invecchiare: dopo lento corpo debole / giovane forte
deteriorare: morire lento debole brutto / forte bello
svanire: luce debole lontano no / forte qui

# === FINE ===
estinguersi: morire fine no vita fuoco / nascere
decomporsi: morire terra dentro lento / vita forte
scomparire: lontano no io / qui vicino essere
dissolvere: acqua no lento / forte qui

# === TRASFORMAZIONE ===
trasformare: cambiare forte nuovo dopo io / stesso
evolvere: cambiare dopo nuovo grande crescere / stesso
mutare: cambiare io forte dentro dopo / prima
rinnovare: nuovo dopo cambiare forte vita / vecchio
rigenerare: nascere nuovo dopo forte vita / morire
```

### Caricamento

```bash
:load prometeo_state.bin
:compact lessons/17_cicli_vitali.txt

# Output:
# Parole processate: 40
# Parole nuove: 20
# Vocabolario ora: 502
# Frasi generate: 80

# Esempi frasi:
# germogliare seme nuovo nascere io
# germogliare io nascere piccolo seme
# io germogliare nuovo piccolo
# germogliare no morire

:save prometeo_state.bin
```

---

## 📈 Metriche di Qualità

### Durante insegnamento

```
✅ BUONO:
- Vocabolario cresce linearmente (~N parole → N nuove)
- Nessun warning "ancora sconosciuta"
- Nessun crash
- Simplessi crescono (~5-10 per parola)

⚠️ ATTENZIONE:
- Molte parole "già note" → controlla sovrapposizione
- Crash → errore sintassi o ancora invalida

❌ MALE:
- Vocabolario non cresce → tutte parole duplicate
- Simplessi non crescono → ancore troppo simili
```

### Post-insegnamento

```bash
:report

# Controlla:
✅ Vocabolario: incremento corretto
✅ Simplessi: crescita proporzionale
✅ Archi campo: densità aumentata
✅ Emergenti: più frattali calibrati
```

### Qualità lessico

```bash
:emergent

# Controlla:
✅ Emergenti calibrate aumentano
✅ Population (w) cresce
✅ std_dev > 0.05 (differenziazione)
```

---

## 🔧 Troubleshooting

### "Lezione già completata"

```
Causa: File già insegnato (tracked in curriculum)

Fix:
1. Rinomina file: 16_emozioni.txt → 16b_emozioni.txt
   OPPURE
2. :reteach lessons/16_emozioni.txt  # (se vuoi sovrascrivere)
```

### "Ancora sconosciuta: X"

```
Causa: Ancora 'X' non nel lessico corrente

Fix:
1. Verifica: :load + X
2. Se sconosciuta: sostituisci con sinonimo noto
3. Se nota: probabile typo (es. "caaldo" vs "caldo")
```

### Crash al caricamento

```
Causa: Errore di sintassi nel file

Fix:
1. Controlla righe senza ":"
2. Controlla caratteri speciali
3. Controlla encoding (deve essere UTF-8)
4. Valida con tools/validate_compact.py
```

### Vocabolario non cresce

```
Causa: Tutte parole già esistenti

Fix:
1. Controlla ogni parola: :load + parola
2. Usa parole diverse
3. Verifica non ci siano duplicati in batch precedenti
```

---

## 🚀 Tips Avanzati

### 1. Riutilizzo batch tra sessioni

```bash
# Insegna batch, poi crea candidati per il prossimo
:compact lessons/16_emozioni.txt
:save prometeo_state.bin

# Ora usa parole appena insegnate come ancore per batch 17!
# Es. se 16 insegna "timore", 17 può usarlo:

# lessons/17_coraggio.txt:
ardire: io forte fuori nuovo / timore paura
```

### 2. Batch incrementali (stratificazione)

```
Layer 1 (primitivi):
  00_corpo.txt → mano, occhio, corpo...

Layer 2 (composti da primitivi):
  15_movimenti.txt → afferrare (usa "mano")

Layer 3 (astratti da composti):
  18_possesso.txt → possedere (usa "afferrare")
```

### 3. Bilanciamento frattale

```bash
# Prima di creare batch, controlla distribuzione:
:report

# Se vedi:
# POTENZIALE: 300w (70%)  ← troppo!
# SPAZIO: 20w (5%)        ← poco

# Prossimo batch: enfatizza SPAZIO
# Usa molte ancore spaziali: qui, dentro, vicino, alto...
```

### 4. Semantic Axes Tracking

```bash
# Dopo batch di opposti (gioia/tristezza, caldo/freddo):
:axes

# Verifica che l'asse sia rilevato:
# gioia ↔ tristezza (dist: 0.25, co-occ: 5)

# Se non appare: insegna più frasi con entrambi:
:teach gioia no tristezza
:teach tristezza no gioia
```

---

## 📚 Risorse

**File di riferimento:**
- `lessons/14_espansione_compact.txt` - esempio 133 parole
- `docs/ARCHITECTURE.md` - architettura completa
- `CLEANUP_PLAN.md` - organizzazione progetto

**Comandi utili:**
```
:help              # Lista comandi
:lexicon           # Lista tutte parole
:curriculum        # Lezioni completate
:report            # Stato completo
```

**Script:**
- `tools/validate_compact.py` - validatore (da creare)
- `archive/scripts/teach_batch.py` - vecchio sistema (reference)

---

**Versione**: 1.0
**Ultima modifica**: 2026-02-14
**Autore**: Sistema Prometeo Team

---

## Appendice: Template Vuoto

```
# Lezione NN: [NOME DOMINIO]
# Dominio: [descrizione semantica]
# Ancore: lezioni 00-14 (482 parole)

# === CATEGORIA 1 ===
parola1: ancora1 ancora2 ancora3 / neg1 neg2
parola2: ancora1 ancora2 ancora3 ancora4 / neg1
parola3: ancora1 ancora2 / neg1 neg2 neg3

# === CATEGORIA 2 ===
parola4: ancora1 ancora2 ancora3 ancora4
parola5: ancora1 ancora2 ancora3 / neg1 neg2

...
```

Copia, riempi, valida, insegna! 🚀
