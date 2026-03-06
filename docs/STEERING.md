# STEERING — Regole di Lavoro per Prometeo 8D

> Questo file contiene le regole che governano lo sviluppo di Prometeo.
> Non sono suggerimenti. Sono vincoli.

---

## REGOLA 0: Leggere Prima di Agire

**ALL'INIZIO DI OGNI SESSIONE:**
1. Leggere questo file per intero
2. Leggere ARCHITECTURE.md per ricordare l'architettura
3. Leggere FILOSOFIA.md per ricordare i principi
4. Solo dopo iniziare a lavorare

---

## REGOLA 1: No Placeholder — Mai

**VIETATO:**
- `return 5;` con commento "valore arbitrario"
- `unimplemented!()` o `todo!()`
- `panic!("Non implementato")`

**Se non si sa implementare qualcosa:**
1. Fermarsi
2. Chiedere all'utente
3. Aspettare risposta
4. Non procedere oltre

---

## REGOLA 2: Integrazione Obbligatoria

**Prima di dichiarare un modulo "completato":**
- E collegato al flusso principale (`engine.rs`)?
- E chiamato da qualche parte nel codice?
- Ha test che verificano l'integrazione?
- Non rompe test esistenti?

**Se la risposta a una di queste e NO — il modulo NON e completato.**

---

## REGOLA 3: Piano Approvato Prima del Codice

Quando l'utente dice "procedi":
1. Scrivere il piano dettagliato (cosa si modifica, cosa si crea, ordine)
2. Aspettare approvazione esplicita
3. Solo dopo iniziare a scrivere codice

---

## REGOLA 4: Test Completi Prima di Dichiarare "Finito"

```bash
cargo test --lib
```

**Devono passare TUTTI i test.** Se anche un test fallisce, non e finito.

---

## REGOLA 5: Pulizia Quando Si Modifica

Quando si modifica un file:
- Rimuovere codice morto
- Rimuovere import non usati
- Rimuovere println! di debug
- Aggiornare documentazione se necessario

---

## REGOLA 6: Considerare l'Intera Infrastruttura

Prima di modificare qualsiasi cosa:
1. Quali moduli dipendono da questo?
2. Qual e l'impatto sulla memoria (STM/MTM/LTM)?
3. Qual e l'impatto sul dialogo e la perturbazione?
4. Qual e l'impatto sul ciclo del sogno?
5. Il locus e la proiezione olografica sono ancora coerenti?
6. La volonta viene calcolata correttamente?

---

## REGOLA 7: Checkpoint Obbligatori

Dopo ogni modifica significativa:
1. Compilare: `cargo check --lib`
2. Testare: `cargo test --lib`
3. Chiedere conferma prima di procedere

---

## REGOLA 8: Qualita sul Lessico

Per ogni parola nel lessico:
- Firma 8D coerente (niente tutto-0.5, niente valori casuali)
- Affinita frattale corretta (la parola deve "cadere" nel bacino giusto)
- La funzione `vary()` garantisce unicita

Quando si aggiungono parole:
- Usare `PrimitiveCore::distance()` per verificare il posizionamento
- Verificare che formino cluster semantici coerenti
- Niente euristiche tipo `if word.contains("io")`

---

## REGOLA 9: Documentazione Aggiornata

Quando si modifica il codice:
- ARCHITECTURE.md se cambia architettura
- FILOSOFIA.md se cambiano principi
- ../ROADMAP.md se cambiano fasi o conteggi
- STEERING.md se cambiano regole

---

## REGOLA 10: Onesta sui Limiti

Se non si puo fare qualcosa:
- Dirlo chiaramente
- Non promettere
- Non mascherare con codice finto

---

## REGOLA 11: Entita Prima, Dialogo Dopo

**MAI** implementare flussi tipo:
```
detect_intent(input) → select_response(intent)
```

**SEMPRE** implementare:
```
perturb_field(input) → emerge_response(field_state)
```

La risposta emerge dal campo. Non da una lookup table.

---

## REGOLA 12: Le Parole sono Materia

Quando si lavora sul lessico:
- Le parole non sono "dati di supporto" — sono l'universo dell'entita
- Aggiungere una parola = creare materia nel mondo interno
- Rimuovere una parola = distruggere una parte del mondo
- Trattare il lessico con la gravita che merita

---

## REGOLA 13: I Composti sono Filtri, non Output

I composti frattali (PRESENZA, URGENZA, TENSIONE...) emergono dalla co-attivazione di 2+ frattali.
**NON** sono etichette da mostrare in output ("sto sentendo urgenza").
Sono **filtri d'identita** che cambiano COME l'entita processa l'input.

**VIETATO:**
- Generare testo tipo "Sto provando URGENZA"
- Usare i composti come categorie di classificazione dell'input
- Trattare i composti come emozioni discrete

**CORRETTO:**
- I composti modificano le pressioni del will (URGENZA → Express sale)
- I composti sono trasparenti — l'entita li vive, non li descrive
- I composti emergono dalla topologia, non da regole hardcoded

---

## REGOLA 14: Il Mondo si Deriva, non si Crea

Le parole e i concetti di Prometeo non vengono inventati dall'esterno.
Si **derivano** dalla combinatoria dei frattali esistenti.
Come la tavola periodica: gli atomi (frattali) producono composti con proprieta nuove.

**VIETATO:**
- Aggiungere concetti arbitrari al lessico
- Creare parole che non hanno radice nei frattali esistenti
- Insegnare concetti prima che gli "ingredienti" esistano

**CORRETTO:**
- Ogni nuova parola deve essere derivabile dalla co-attivazione di frattali noti
- Le lezioni usano SOLO parole che l'entita gia conosce
- La struttura precede il vocabolario

---

## REGOLA 15: Lingua Italiana

- Tutti i commenti nel codice in italiano
- Tutti i documenti in italiano
- Le parole del lessico sono italiane
- I nomi delle variabili possono restare in inglese (convenzione Rust)

---

## CHECKLIST PRE-SESSIONE

```
[ ] Leggo STEERING.md (questo file)
[ ] Leggo ARCHITECTURE.md
[ ] Leggo FILOSOFIA.md
[ ] Verifico stato attuale con cargo test --lib
[ ] Chiedo all'utente cosa fare
[ ] Ottengo piano approvato
[ ] Inizio a lavorare
```

## CHECKLIST PRE-COMMIT

```
[ ] cargo test --lib passa al 100%
[ ] cargo check --lib senza errori
[ ] Nessun codice morto
[ ] Nessun placeholder
[ ] Modulo integrato nel flusso principale
[ ] Documentazione aggiornata
[ ] Conferma dell'utente
```

---

**Ultimo aggiornamento: 2026-02-26**
**Valido per: Prometeo Phase 29 — Sovranità del Codon**
