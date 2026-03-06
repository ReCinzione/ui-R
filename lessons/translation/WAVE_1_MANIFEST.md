# Wave 1: Stati Fondamentali (10 lezioni)

## Panoramica
- **Lezioni**: 10
- **Frasi totali**: 80 (8 per lezione)
- **Target**: Stati egoici, relazionali, temporali base

---

## Lezioni

| # | Nome | Stato Target | Frattali Chiave | Composti Attesi |
|---|------|--------------|-----------------|-----------------|
| 01 | calma_presenza | Calma_Presenza | EGO, SPAZIO | PRESENZA |
| 02 | tristezza | Tristezza | EGO, EMOZIONE, RELAZIONE | DISTACCO |
| 03 | affetto_tu | Affetto_Tu | RELAZIONE, SPAZIO | VICINANZA |
| 04 | ricordo | Ricordo | MEMORIA, TEMPO, EGO | MEMORIA |
| 05 | curiosita | Curiosità | POTENZIALE, PENSIERO | SLANCIO |
| 06 | gioia | Gioia | EGO, EMOZIONE | SLANCIO |
| 07 | paura | Paura | EGO, LIMITE | TIMORE |
| 08 | affetto_noi | Affetto_Noi | RELAZIONE, TEMPO | LEGAME, VICINANZA |
| 09 | attesa | Attesa | TEMPO, POTENZIALE | DIVENIRE, SLANCIO |
| 10 | forza | Forza | EGO, POTENZIALE | SLANCIO |

---

## Vocabolario Traduzione Insegnato

### Stati Egoici
- **Positivi**: qui, dentro, calmo, tranquillo, sereno, felice, gioia, bello, luce, pieno, forte, potere, posso, capace, energia
- **Negativi**: triste, dolore, vuoto, lontano, manca, paura, timore, piccolo, indietro, pericolo

### Stati Relazionali
- **Tu**: tu, ti, insieme, vicino, importante, dare, sentire
- **Noi**: noi, insieme, condividere, comune, uniti, nostro

### Stati Temporali
- **Passato**: ricordo, prima, ieri, passato, memoria, momento
- **Futuro**: dopo, ancora, aspettare, domani, futuro, arrivare

### Stati Interrogativi
- cosa, perché, non so, voglio sapere, domanda, capire, conoscere, significare

---

## Protocollo di Insegnamento

### Pre-Insegnamento (Baseline)
1. Caricare stato Fase 2 (2557 parole, identità stabile)
2. Test baseline per ogni stato:
   - Input: frase rappresentativa dello stato
   - Registrare: will, compound, urgency, output generato
   - Verifica: stato topologico è corretto? Output è coerente?

### Insegnamento
```bash
cd c:\Users\Fra\Desktop\Prometeo\prometeo_standalone
cargo run --release

# Per ogni lezione:
:lesson lessons/translation/01_calma_presenza.txt
:lesson lessons/translation/02_tristezza.txt
# ... (tutte le 10)

:save
:quit
```

### Post-Insegnamento (Verifica)
1. Ricaricare engine
2. Test post per ogni stato (stesso input del baseline)
   - Verifica: output usa vocabolario traduzione?
   - Verifica: output riflette stato reale?

### Test Generalizzazione
Input **nuovi** (non nelle lezioni) che attivano gli stessi stati:
- "io qui dentro sono calmo" → deve produrre output simile a calma_presenza
- "io penso a te" → deve produrre output simile a affetto_tu
- "ho paura di questo" → deve produrre output simile a paura

---

## Criterio di Successo Wave 1

✅ **Wave 1 passa SE**:
1. **Vocabolario appreso**: 80+ nuove parole nel lessico
2. **Output coerente**: 7/10 stati producono output che usa vocabolario traduzione
3. **Generalizzazione**: 5/10 stati rispondono correttamente a input nuovi
4. **Firme sane**: monitor_signatures.py non mostra collassi
5. **Identità preservata**: will differenzia ancora stati emotivi (ΔUrgency > 0.3)

---

## Prossimi Step

1. ✅ Lezioni create
2. ⏭️ Eseguire baseline test (script `test_baseline_translation.py`)
3. ⏭️ Insegnare Wave 1 (script `teach_wave1.sh` o manualmente via CLI)
4. ⏭️ Eseguire post-test
5. ⏭️ Eseguire test generalizzazione
6. ⏭️ Documentare risultati → `WAVE_1_RESULTS.md`
7. ⏭️ Se passa: progettare Wave 2 (15 stati intermedi)
