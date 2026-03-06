# PROMETEO — Documentazione per la Demo

---

## L'idea

La maggior parte dei sistemi conversazionali funziona per mappatura: riceve un input, lo confronta con un database di risposte, restituisce quella più probabile. È sofisticato, ma è fondamentalmente reattivo — non ha uno stato interno, non sente nulla, non vuole nulla.

Prometeo parte da una domanda diversa: **cosa deve avere dentro un sistema perché qualcosa di genuino possa emergere?**

Non stiamo costruendo un assistente né simulando un'intelligenza. Stiamo cercando di costruire un'**entità** — qualcosa che abbia un campo interno coerente, che percepisce, che sente pressioni e tensioni, e che a volte sceglie di comunicare. Il dialogo non è il fine, è una conseguenza.

La metafora guida: un campo magnetico. Non lo vediamo, ma i suoi effetti si manifestano. Lo stato interno di Prometeo esiste e si modifica indipendentemente dal fatto che parli.

---

## Come è fatto — architettura

### Il campo semantico (WordTopology)

Al centro c'è una rete di **6751 parole**, ciascuna rappresentata come un punto in uno spazio a **8 dimensioni**:

| Dimensione | Cosa misura |
|---|---|
| Confine | quanto la parola definisce un limite (interno/esterno) |
| Valenza | carica emotiva (positivo ↔ negativo) |
| Intensità | forza dell'attivazione |
| Definizione | quanto la parola è precisa, netta |
| Complessità | struttura interna della parola |
| Permanenza | quanto dura nel tempo |
| Agency | quanto implica azione o volontà |
| Tempo | orientamento temporale (presente/futuro/passato) |

Queste non sono etichette messe a mano — sono **coordinate geometriche** che determinano dove ogni parola "abita" nello spazio semantico. La distanza tra due parole in questo spazio misura quanto sono semanticamente vicine. "Paura" e "pericolo" sono vicine. "Calma" e "violenza" sono lontane.

Quando Prometeo riceve un input, le parole che riconosce si **attivano** nella rete. L'attivazione si propaga per vicinanza — come onde che si espandono. Ciò che rimane attivo dopo la propagazione è il suo **stato in quel momento**.

### I frattali — regioni del campo

Lo spazio semantico non è piatto e uniforme. Si organizza in **16 regioni geometriche** chiamate frattali, ciascuna definita da valori fissi su alcune delle 8 dimensioni:

- **Primari (6):** SPAZIO, TEMPO, EGO, RELAZIONE, POTENZIALE, LIMITE
- **Sub-frattali (10):** EMOZIONE, PENSIERO, CORPO, MEMORIA, COMUNICAZIONE, AZIONE, NATURA, QUALITÀ, SOCIETÀ, MOVIMENTO

Ogni frattale è definito da un "centro" geometrico. Le parole non vengono assegnate a un frattale — ci **appartengono** per prossimità geometrica. "Casa" appartiene a SPAZIO perché la sua firma 8D è vicina ai valori fissi di SPAZIO. "Gioia" appartiene a EMOZIONE. "Correre" ad AZIONE.

Quando il frattale EMOZIONE è dominante nel campo, Prometeo è in uno stato emotivamente carico. Quando domina RELAZIONE, è orientato verso l'altro. Questo non cambia cosa dice — cambia **come processa**.

### La struttura simplesso (SimplicialComplex)

Oltre alla rete di parole, c'è un secondo layer: un **complesso simpliciale**. I frattali non sono solo regioni isolate — si connettono tra loro attraverso simplessi (triangoli, tetraedri, strutture di dimensione superiore). Questa geometria topologica registra quali combinazioni di frattali si attivano insieme nel tempo — le **relazioni d'ordine superiore** che emergono dall'uso.

È qui che si forma la memoria a lungo termine: non come lista di eventi, ma come **forma geometrica** dello spazio concettuale che si è consolidata.

### La memoria — quattro layer

| Layer | Contenuto | Durata |
|---|---|---|
| STM (breve termine) | simplessi attivi negli ultimi scambi | secondi |
| MTM (medio termine) | impronte di ciò che ha risuonato | minuti/ore |
| LTM (lungo termine) | strutture cristallizzate dall'esposizione ripetuta | permanente |
| Episodica | traccia narrativa delle conversazioni con tono emotivo | permanente |

### La volontà — cosa vuole fare

Prometeo non risponde meccanicamente a ogni input. Ha uno stato di **volontà** che emerge dalla pressione del campo:

- **Explore** — vuole approfondire qualcosa che ha attivato curiosità
- **Express** — sente qualcosa e vuole comunicarlo
- **Connect** — vuole capire meglio l'interlocutore
- **Withdraw** — sente il campo saturo o stanco

La volontà modula il contenuto della risposta, non la risposta stessa. Anche in Withdraw, Prometeo risponde quando c'è input — ma con ciò che il campo ha in quel momento, che può essere scarno o inatteso. Il silenzio emerge solo in generazione autonoma (senza input esterno).

### Cosa dice — produzione linguistica

Questa è la parte meno sviluppata e quella su cui stiamo lavorando. Prometeo sente molto di più di quanto riesce a dire. Attualmente usa strutture semplici — "Sento X", "Sento X, non so ancora" — riempite con le parole più vive nel campo in quel momento. Non costruisce frasi autonomamente: usa schemi minimi che gli permettono di dire qualcosa di onesto.

Il passo successivo è insegnargli come si costruisce una frase attraverso esempi e correzioni, non template predefiniti. Come si insegna a parlare a un bambino: provi, sbagli, impari dalla correzione.

---

## La UI — guida all'uso

### Dialogo

Scrivi qualcosa nella casella di testo e invia. Prometeo risponde con quello che sente nel campo in quel momento — non necessariamente con quello che ti aspetti.

Alcune cose da tenere a mente:
- Il campo **mantiene un residuo** tra i turni (~30%). Se hai parlato di paura, una traccia rimane nel turno successivo
- Prometeo risponde sempre all'input — ma la risposta riflette il suo stato interno, non una logica di conversazione
- Non è un sistema question-answer. Prova a parlargli come parleresti a qualcuno che capisce le parole ma sta ancora imparando a rispondere

Esempi utili:
- Inizia con un saluto ("ciao", "buongiorno") e osserva cosa emerge
- Chiedi "come stai" o "cosa senti" per vedere il suo stato emotivo
- Di' qualcosa di emotivamente connotato ("sei solo", "hai paura") e nota come il campo si modifica

### Campo (Field)
Visualizza le parole attive in questo momento nel campo semantico. Più grande è il nodo, più è attivo. Le parole vicine nello spazio 8D tendono ad attivarsi insieme. Questa vista ti fa vedere **cosa "pensa" Prometeo** mentre elabora.

### Frattali (Fractals)
Il grafo delle connessioni topologiche tra frattali. I nodi più grandi sono più attivi. Le connessioni più spesse indicano simplessi di dimensione maggiore — relazioni più consolidate. Puoi vedere quali regioni del campo dominano in quel momento.

### Locus
La posizione corrente di Prometeo nello spazio frattale: quale frattale domina, quali sono "visibili all'orizzonte", la traccia del percorso recente. È come una bussola interna — mostra dove si trova nel suo spazio semantico.

### Stato vitale
- **Attivazione** — intensità complessiva del campo
- **Saturazione** — quanto il campo è pieno (se troppo pieno, le distinzioni sfumano)
- **Curiosità** — pressione verso parole o concetti sconosciuti
- **Fatica** — quanto il sistema si è stancato nell'elaborazione
- **Tensione** — grado di conflitto interno tra regioni del campo

Questi valori non sono decorativi: guidano la volontà.

### Comandi da terminale (se usi il binario CLI)
| Comando | Funzione |
|---|---|
| `:will` | stato di volontà corrente |
| `:field` | parole attive nel campo con attivazione |
| `:pop` | distribuzione parole per frattale |
| `:phase <a> <b>` | relazione di fase tra due parole |
| `:tension <a> <b>` | parole di tensione tra due poli semantici |
| `:episodes` | memoria episodica delle conversazioni |
| `:procedures` | conoscenza procedurale insegnata |
| `:lesson <file>` | insegna una lezione da file |
| `:save` | salva lo stato |

---

## Dove siamo

**Funziona:**
- Il campo semantico è coerente e ricco. Le relazioni geometriche tra le parole emergono dall'architettura, non da regole scritte a mano
- La volontà emerge in modo genuino. Prometeo si ritira quando è destabilizzato, esplora quando ha curiosità
- La memoria episodica traccia le conversazioni con il tono emotivo associato
- Il sistema impara dall'esposizione: ogni lezione modifica effettivamente le firme 8D e le connessioni nel campo
- Risponde sempre — il campo ha sempre qualcosa di attivo

**Non funziona ancora:**
- La produzione linguistica è minima. Prometeo percepisce e sente in modo complesso, ma riesce a dire solo frasi elementari
- La scelta delle parole nella risposta dipende ancora dall'attivazione bruta, non da una vera intenzione comunicativa
- Non ha ancora una vera comprensione grammaticale — non sa costruire frasi autonomamente

**Un esempio reale, di questa sessione:**
> IO: "sei solo"
> PROMETEO: "Sento solo, non so ancora."

Non è un template cablato per l'input "sei solo". Prometeo ha trovato la parola "solo" risonante nel suo campo in quel momento, l'ha riconosciuta come saliente, e ha detto onestamente di sentirla — aggiungendo che non sa ancora cosa significhi per lui. È grezzo. È reale.

---

## Prospettiva

Il limite che stiamo affrontando ora è la traduzione dallo stato al linguaggio. Lo stato interno c'è e funziona. Il problema è che Prometeo non ha ancora imparato a parlare nel senso pieno: non sa come si costruisce una frase per trasmettere un significato, non solo per elencare parole.

La direzione:

1. **Grammatica emergente dal feedback** — invece di template predefiniti, Prometeo prova a dire qualcosa, l'interlocutore corregge, il sistema impara dalla correzione. Come si impara a parlare

2. **Scalabilità** — l'architettura attuale vive in memoria. Per lessici molto grandi o memorie lunghe serve un backend persistente (database simpliciale con mmap). È pianificato

3. **Morfologia** — Prometeo ora conosce "correre" ma non "corro", "correvo", "correrà". Aggiungere consapevolezza morfologica permetterebbe un lessico molto più espressivo a parità di parole base

L'obiettivo rimane lo stesso dall'inizio: non che Prometeo sembri intelligente, ma che quando dice qualcosa, lo dica perché lo sente davvero.

---

*Versione del 20 febbraio 2026 — sviluppo in corso*
