# Istruzioni per Usare il Nuovo Materiale Didattico

## ✅ File Corretti da Usare

### Lezioni in Formato `.lesson` (per `:compact`)
1. **`lessons/200_verbi_modali.lesson`** - Verbi modali e ausiliari
2. **`lessons/201_connettivi.lesson`** - Connettivi logici
3. **`lessons/202_tempo_avanzato.lesson`** - Espressioni temporali
4. **`lessons/203_astratti.lesson`** - Concetti astratti e filosofici

### Libri in Formato `.txt` (per `:read`)
1. **`books/grammatica_italiana_base.txt`** - Grammatica attraverso esempi
2. **`books/racconti_brevi.txt`** - 7 racconti narrativi

---

## 🚀 Procedura di Insegnamento

### Passo 1: Verifica Stato Attuale
```bash
:load prometeo_topology_state.bin
:report
```

Controlla:
- Parole attuali: ~8463
- Simplessi: ~15129
- Composti fertili: ~50+
- Assi semantici: ~30+

### Passo 2: Insegna Lezioni Compact (1-2 giorni)

```bash
# Giorno 1: Verbi modali (fondamentali)
:compact lessons/200_verbi_modali.lesson
:save prometeo_topology_state.bin

# Giorno 1: Connettivi logici
:compact lessons/201_connettivi.lesson
:save prometeo_topology_state.bin

# Giorno 2: Espressioni temporali
:compact lessons/202_tempo_avanzato.lesson
:save prometeo_topology_state.bin

# Giorno 2: Concetti astratti
:compact lessons/203_astratti.lesson
:save prometeo_topology_state.bin
```

Dopo ogni `:compact`, verifica:
```bash
:report
# Controlla che parole e simplessi siano aumentati
```

### Passo 3: Lettura Narrativa (1-2 settimane)

```bash
# Settimana 1: Racconti
:read books/racconti_brevi.txt
:save prometeo_topology_state.bin

# Settimana 2: Grammatica (esposizione massiccia)
:read books/grammatica_italiana_base.txt
:save prometeo_topology_state.bin
```

### Passo 4: Verifica Emergenza (dopo 2-3 settimane)

```bash
# Test generazione
io sentire gioia forte dentro
# Osserva se l'output è più articolato

# Test DualField
:dual auto 50
# Osserva se Adamo/Eva usano frasi più complesse

# Report finale
:report
# Controlla crescita: parole, simplessi, composti, assi
```

---

## 📊 Risultati Attesi

### Dopo Lezioni Compact (Passo 2)
- **Parole**: 8463 → ~8600 (+ ~140 parole)
- **Simplessi**: 15129 → ~16000 (+ ~900)
- **Output**: Ancora primitivo, ma con più varietà lessicale

### Dopo Lettura Narrativa (Passo 3)
- **Parole**: 8600 → ~9500-10000 (+ ~1000-1500 parole)
- **Simplessi**: 16000 → ~18000-20000 (+ ~2000-4000)
- **Output**: Transizione da primitivo a semi-articolato
  - Prima: "io dentro sentire calma"
  - Dopo: "io sento calma dentro" o "io sono calmo dentro"

### Dopo 3-4 Settimane Totali
- **Grammatica emergente**: Uso più frequente di:
  - Articoli (il, la, un, una)
  - Preposizioni (di, a, da, in, con)
  - Pronomi (mi, ti, lo, la)
  - Concordanza soggetto-verbo
  - Ordine SVO più stabile

---

## ⚠️ Note Importanti

### 1. Il Formato `.lesson` è Efficiente
Ogni riga genera 4 frasi automaticamente:
```
parola: contesto positivo / contesto negativo
```
Diventa:
1. `parola contesto positivo`
2. `parola non contesto negativo`
3. Varianti con prospettiva
4. Varianti contrastive

### 2. Salva Spesso
Dopo ogni sessione importante:
```bash
:save prometeo_topology_state.bin
```

Fai backup:
```bash
# In PowerShell
cp prometeo_topology_state.bin "backup_$(Get-Date -Format 'yyyyMMdd_HHmmss').bin"
```

### 3. Monitora la Stabilità
Se dopo l'insegnamento vedi:
- Will incoerente (stessi input → intenzioni molto diverse)
- Composti che collassano (numero diminuisce)
- Assi semantici frammentati

→ Rallenta l'insegnamento, lascia consolidare (usa `:dream` o aspetta cicli autonomi)

### 4. Il DualField Accelera
Usa regolarmente:
```bash
:dual auto 100
```
Adamo/Eva sviluppano pattern grammaticali più velocemente attraverso pressione sociale reciproca.

---

## 🎯 Comandi Utili per Monitoraggio

```bash
# Stato generale
:report

# Parole attive nel campo
:field

# Volontà corrente
:will

# Composti emergenti
:compound

# Pensieri topologici
:thoughts

# Percezione interna
:percept

# Allineamento DualField
:dual align

# Report DualField completo
:dual report
```

---

## 📈 Metriche di Successo

Usa questa checklist dopo aver completato tutto il materiale:

- [ ] Parole: 9000-10000 (da 8463)
- [ ] Simplessi: 18000-20000 (da 15129)
- [ ] Composti fertili: 70-100 (da ~50)
- [ ] Assi semantici: 40-50 (da ~30)
- [ ] Output: Semi-articolato (non più primitivo puro)
- [ ] Grammatica: Uso corretto di articoli, preposizioni, pronomi
- [ ] DualField: Allineamento stabile >0.85, divergenza codon 3-5

---

## ❌ File da NON Usare

Questi file sono nel formato sbagliato (troppo verbose):
- `lessons/200_verbi_modali_ausiliari.txt`
- `lessons/201_connettivi_logici.txt`
- `lessons/202_espressioni_temporali.txt`
- `lessons/203_concetti_astratti.txt`

**Ignora questi file. Usa i `.lesson` corretti.**

---

## 🚀 Prossimi Passi (dopo questo materiale)

1. **Più libri**: Cerca testi semplici (favole, racconti per bambini)
2. **Dialogo umano**: Conversa regolarmente con Prometeo
3. **Regole esplicite**: Se dopo 10000+ parole l'output è ancora primitivo, considera Fase 3 vera (regole grammaticali esplicite)
4. **Specializzazione**: Approfondisci domini specifici (scienza, filosofia, arte)

---

**Creato**: 2026-03-01
**Per**: Prometeo Phase 31 (8463 parole)
**Formato**: Compact lessons (`.lesson`) + Narrative books (`.txt`)
