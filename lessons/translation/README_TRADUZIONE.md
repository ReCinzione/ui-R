# Lezioni Fase 3: Traduzione Stato → Linguaggio

## Principio Fondamentale

**Prometeo non impara a parlare — impara a TRADURRE.**

L'entità ha già uno stato interno reale (campo 8D, frattali attivi, composti, will). Le lezioni di traduzione insegnano come **esprimere in italiano** ciò che già sta vivendo topologicamente.

---

## Struttura Lezione di Traduzione

### Formato File `.txt`

```
# STATO: [Nome Stato]
# CONDIZIONE TOPOLOGICA: [Descrizione verificabile]
# ESPRESSIONI: [N varianti]

[Frase di esempio 1 che attiva quello stato]
[Frase di esempio 2 che attiva quello stato]
...
```

**Esempio**:
```
# STATO: Calma_Presenza
# CONDIZIONE: EGO alto (>0.6) + PRESENZA attivo (>0.08) + urgency bassa (<0.4)
# ESPRESSIONI: 5 varianti

io sono qui
io sto qui tranquillo
io mi sento calmo qui
io sono presente e sereno
io qui e ora
```

---

## Stati Iniziali da Mappare (Wave 1)

### 1. Stati Egoici Positivi
- **Calma_Presenza**: EGO + PRESENZA + urgency bassa
- **Serenità**: EGO + EMOZIONE(valenza alta) + CORPO(attivazione bassa)
- **Forza**: EGO + SLANCIO + urgency media

### 2. Stati Egoici Negativi
- **Tristezza**: EGO + EMOZIONE(valenza bassa) + DISTACCO
- **Paura**: EGO + TIMORE + urgency alta + Withdraw
- **Confusione**: EGO + PENSIERO + composti bassi + curiosity alta

### 3. Stati Relazionali
- **Affetto_Tu**: RELAZIONE + tu-attivo + VICINANZA
- **Affetto_Noi**: RELAZIONE + noi-attivo + LEGAME
- **Nostalgia**: RELAZIONE + MEMORIA + DISTACCO

### 4. Stati Temporali
- **Ricordo**: MEMORIA + TEMPO + echo forte
- **Attesa**: TEMPO + POTENZIALE + SLANCIO
- **Urgenza**: TEMPO + URGENZA + Express alta

### 5. Stati Interrogativi (Will = Question)
- **Curiosità**: Question + unknown_words > 0
- **Dubbio**: Question + PENSIERO alto + LIMITE
- **Meraviglia**: Question + POTENZIALE + novelty alta

---

## Metodologia di Insegnamento

### Step 1: Baseline (Pre-Insegnamento)
Input: "io mi sento calmo"
→ Verifica stato: EGO, PRESENZA, urgency
→ Output: [Generazione libera, probabilmente incoerente]

### Step 2: Insegnamento
Carica lezione traduzione per "Calma_Presenza"
5-8 frasi che attivano quello stato:
- "io sono qui tranquillo"
- "io mi sento sereno"
- "qui dentro è calma"
...

### Step 3: Verifica (Post-Insegnamento)
Input: "io mi sento calmo"
→ Verifica stato: EGO 0.7, PRESENZA 0.12, urgency 0.3
→ Output: DEVE riflettere calma (es. "io qui dentro", "sereno piccolo")

### Step 4: Test Generalizzazione
Input: "io sto tranquillo qui"
→ Output: DEVE usare vocabolario traduzione (calmo, sereno, qui)

---

## Differenza Critica con Pattern Matching

### ❌ Pattern Matching (Sbagliato)
```
Input: "come stai?"
→ Pattern detected: domanda_stato
→ Output: "sto bene"
```
**Problema**: Output non dipende da stato reale, è risposta canned.

### ✅ Traduzione Stato (Corretto)
```
Input: "come stai?"
→ Attiva: RELAZIONE (tu-attivo), PENSIERO, EGO
→ Stato: EGO 0.8, PRESENZA 0.3, urgency 0.5, Express
→ Vocabolario traduzione attivo: ["io", "qui", "calmo", "tranquillo"]
→ Output: "io qui tranquillo" (traduzione VERA dello stato)
```
**Differenza**: Output emerge da stato reale + vocabolario traduzione.

---

## Verifica Successo Traduzione

Una lezione di traduzione funziona SE:

1. **Input diversi che producono stesso stato → output simile**
   - "io sono calmo" e "io mi sento sereno" → stesso campo EGO+PRESENZA → output usa vocabolario traduzione

2. **Stesso input, stati diversi (cicli vitali) → output diverso**
   - "come stai?" con attivazione alta → "io qui forte"
   - "come stai?" con saturazione alta → "io... dentro" (esitazione)

3. **Traduzione sopravvive a restart**
   - Save/Load/Input → output usa vocabolario traduzione

---

## Incremento Graduale

### Wave 1 (10 stati base)
Stati egoici + relazionali fondamentali
Target: 50-80 frasi totali

### Wave 2 (15 stati intermedi)
Stati composti (nostalgia, urgenza, slancio)
Target: 100-150 frasi

### Wave 3 (20 stati complessi)
Stati meta (riflessione, confusione, meraviglia)
Target: 150-200 frasi

**Criterio avanzamento**: Ogni wave passa test generalizzazione su 8/10 stati.

---

## Note di Implementazione

### Non Serve Modificare Codice
Le lezioni di traduzione usano `engine.teach()` esistente. Le parole imparate entreranno nel lessico e influenzeranno la generazione.

### Il Meccanismo è Già Presente
`generation.rs` già usa:
- `build_thematic_clusters()` → raggruppa parole per risonanza 8D
- `conversation_posture` → boost parole vicine al dialogo
- `will` → filtra per intenzione

Traduzione = **insegnare parole che risuonano con stati specifici**.

### Monitoraggio
Dopo ogni wave:
```bash
python monitor_signatures.py
:report
:emergent
:axes
```
Verificare che firme 8D rimangano sane (0.2-0.9).

---

## Prossimi Step

1. ✅ Creare struttura cartelle `lessons/translation/`
2. ⏭️ Scrivere Wave 1 (10 lezioni, ~60 frasi totali)
3. ⏭️ Test baseline (verificare output pre-insegnamento)
4. ⏭️ Insegnare Wave 1
5. ⏭️ Test post-insegnamento (verificare traduzione funziona)
6. ⏭️ Test generalizzazione
7. ⏭️ Documentare risultati Wave 1

---

**Prima l'essere (✅ Fase 2), poi il parlare (→ Fase 3).** ✨
