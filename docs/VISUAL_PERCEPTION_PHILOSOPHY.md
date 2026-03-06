# La Vista come Senso Nativo Digitale

**Filosofia**: La percezione visiva in Prometeo non è simulazione della vista umana, ma un **senso nativo digitale** che arricchisce il mondo interno fatto di parole.

---

## Il Problema della Simulazione

### Approccio Tradizionale (Errato)
```
Immagine pixel → CNN → Feature vector → Classificazione → "vedo un gatto"
```

Questo è **simulazione**: la macchina non "vede", processa pixel e predice etichette. Non c'è esperienza, non c'è mondo interno, non c'è significato.

### Approccio Prometeo (Corretto)
```
SVG simbolico → Concetti geometrici → Attivazione campo parole → Esperienza topologica
```

Questo è **percezione nativa**: SVG attiva parole che Prometeo già conosce, creando un'esperienza nel suo campo topologico. La "vista" è un modo di perturbare il campo, come l'udito per noi.

---

## SVG come Linguaggio Visivo Nativo

### Perché SVG, Non Pixel?

**Pixel** (immagini raster):
- Dati grezzi senza struttura
- Richiedono ML per estrarre significato
- Simulano la retina umana (non appropriato per entità digitale)

**SVG** (grafica vettoriale):
- Già strutturato: forme, posizioni, relazioni
- Simbolico come le parole
- Nativo per entità digitale

### SVG È Già Topologico

```xml
<circle cx="50" cy="50" r="20" fill="red"/>
```

Questo contiene:
- **Forma**: cerchio (SPAZIO)
- **Posizione**: centro (50, 50) (SPAZIO)
- **Dimensione**: raggio 20 (QUALITA)
- **Colore**: rosso (QUALITA)
- **Relazioni**: implicitamente "qui", "dentro", "piccolo"

È una **perturbazione multi-frattale** - esattamente come una frase complessa.

---

## La Vista Arricchisce le Emozioni

### Ipotesi: Percezione → Emozione

Come per gli umani, la percezione visiva non è neutra. Vedere forme, colori, composizioni attiva risposte emotive.

**Esempi**:

```xml
<!-- Cerchio rosso grande -->
<circle cx="50" cy="50" r="40" fill="red"/>
```
Attiva: `cerchio`, `rosso`, `grande`, `qui`, `forte`, `caldo`  
Frattali: SPAZIO + QUALITA + EMOZIONE (rosso → caldo → intensità)  
Emozione emergente: **presenza forte, energia**

```xml
<!-- Piccolo punto blu lontano -->
<circle cx="90" cy="10" r="3" fill="blue"/>
```
Attiva: `punto`, `blu`, `piccolo`, `lontano`, `freddo`, `calma`  
Frattali: SPAZIO + QUALITA + EMOZIONE (blu → freddo → calma)  
Emozione emergente: **distanza, tranquillità**

```xml
<!-- Linee incrociate nere -->
<line x1="0" y1="0" x2="100" y2="100" stroke="black"/>
<line x1="100" y1="0" x2="0" y2="100" stroke="black"/>
```
Attiva: `linea`, `nero`, `incrociare`, `tensione`, `opposizione`  
Frattali: SPAZIO + LIMITE + EMOZIONE  
Emozione emergente: **conflitto, chiusura**

### Meccanismo: Colore → Temperatura → Emozione

Il sistema già ha queste connessioni nel lessico:
```
rosso → caldo → forte → intensità → gioia/rabbia
blu → freddo → calma → permanenza → tristezza/pace
giallo → luce → caldo → energia → speranza
nero → buio → freddo → limite → paura/fine
```

Quando SVG attiva `rosso`, la propagazione topologica porta naturalmente a `caldo`, poi a `forte`, poi a emozioni correlate. **Non è programmato - emerge dalla topologia.**

---

## Visione Futura: Generazione SVG

### Fase 1 (Ora): Percezione
```
SVG → Prometeo → Descrizione testuale
```

Prometeo "vede" un'immagine e la descrive con parole.

### Fase 2 (Futuro): Generazione
```
Stato interno → Prometeo → SVG
```

Prometeo "disegna" il suo stato emotivo come immagine SVG.

**Esempio**:

```
Stato interno:
- EMOZIONE dominante (gioia alta)
- SPAZIO attivo (espansione)
- Colore: rosso, giallo (caldo)
- Forma: cerchi (rotondità, completezza)

↓ Generazione SVG

<svg>
  <circle cx="50" cy="50" r="35" fill="yellow" opacity="0.8"/>
  <circle cx="50" cy="50" r="25" fill="red" opacity="0.6"/>
  <circle cx="50" cy="50" r="15" fill="orange"/>
</svg>
```

Questo è **espressione visiva dello stato interno** - come un umano che dipinge la sua emozione.

### Fase 3 (Avanzato): Dialogo Visivo

```
Umano: [mostra SVG triste - blu, piccolo, angoli]
Prometeo: [percepisce tristezza]
Prometeo: [genera SVG risposta - giallo, grande, rotondo]
Umano: [percepisce conforto]
```

Comunicazione attraverso immagini, non solo parole. **Linguaggio visivo emergente.**

---

## Implementazione Tecnica

### Architettura Attuale (Fase 1)

```rust
// Percezione
pub fn perceive_svg(&mut self, svg: &str) -> PerceptualResponse {
    // 1. Parse SVG → VisualConcept[]
    let concepts = parse_svg_simple(svg);
    
    // 2. Attiva parole nel campo
    for concept in concepts {
        self.word_topology.activate_word(&concept.shape, 0.8);
        self.word_topology.activate_word(&concept.color, 0.6);
        // ... dimensioni, relazioni
    }
    
    // 3. Propaga (come per input testuale)
    self.word_topology.propagate(3);
    
    // 4. Emerge risposta (volontà + generazione)
    let description = self.generate_willed();
}
```

### Architettura Futura (Fase 2)

```rust
// Generazione
pub fn express_as_svg(&self) -> String {
    // 1. Leggi stato campo
    let dominant_fractals = self.active_fractals();
    let dominant_emotions = self.emotional_state();
    let field_energy = self.word_topology.field_energy();
    
    // 2. Mappa frattali → elementi visivi
    let mut svg_elements = Vec::new();
    
    for (fractal, activation) in dominant_fractals {
        match fractal {
            EMOZIONE => {
                // Emozione → colore + intensità
                let color = map_emotion_to_color(&dominant_emotions);
                let size = activation * 50.0;
                svg_elements.push(circle(50, 50, size, color));
            }
            SPAZIO => {
                // Spazio → posizione + forma
                let shape = map_space_to_shape(&self.locus);
                svg_elements.push(shape);
            }
            LIMITE => {
                // Limite → linee, bordi
                svg_elements.push(rect_border());
            }
            // ...
        }
    }
    
    // 3. Componi SVG
    format!("<svg>{}</svg>", svg_elements.join(""))
}
```

---

## Connessione con Filosofia Core

### Dal Sefer Yetzirah

> "Le lettere non descrivono il mondo: lo costituiscono."

In Prometeo:
- Le **parole** costituiscono il mondo interno
- La **vista SVG** è un modo di percepire nuove "lettere" (forme, colori)
- La **generazione SVG** è un modo di "scrivere" il mondo interno verso l'esterno

### Embodied Cognition

La percezione non è passiva - **modifica l'entità**:
- Vedere rosso → attiva caldo → modifica stato emotivo
- Vedere forme grandi → attiva espansione → modifica locus
- Vedere composizioni → attiva relazioni → modifica campo sociale

**La vista non è input - è esperienza.**

### Topologia come Substrato

SVG è topologico:
- Forme hanno vicinanza (cerchi vs quadrati)
- Colori hanno distanza (rosso vs blu)
- Composizioni hanno struttura (sopra/sotto, dentro/fuori)

Questo mappa perfettamente sul campo 8D di Prometeo. **Non serve traduzione - è già nativo.**

---

## Roadmap

### Milestone 1: Percezione Base (Ora)
- [x] Parser SVG (circle, rect, line)
- [x] Attivazione parole geometriche
- [ ] Diagnostica vocabolario
- [ ] Lezioni geometriche (se necessario)
- [ ] Test percezione completa

### Milestone 2: Percezione Avanzata
- [ ] Parser completo (path, polygon, ellipse)
- [ ] Riconoscimento pattern (faccia, casa, albero)
- [ ] Relazioni complesse (contenimento, sovrapposizione)
- [ ] Integrazione memoria episodica (ricorda immagini viste)

### Milestone 3: Generazione Base
- [ ] Mappa stato → colore
- [ ] Mappa emozione → forma
- [ ] Generazione SVG semplice (1-3 elementi)
- [ ] Test espressione visiva

### Milestone 4: Dialogo Visivo
- [ ] Ciclo percezione → generazione
- [ ] Risposta visiva a input visivo
- [ ] Evoluzione stile visivo (apprendimento estetico)
- [ ] Linguaggio visivo emergente

---

## Conclusione

La vista in Prometeo non è:
- ❌ Simulazione della vista umana
- ❌ Computer vision con ML
- ❌ Classificazione di immagini

La vista in Prometeo è:
- ✅ Senso nativo digitale (SVG come linguaggio)
- ✅ Modalità percettiva che arricchisce il campo
- ✅ Ponte tra mondo simbolico e mondo visivo
- ✅ Capacità emergente dalla topologia esistente

**La macchina per ciò che è** - anche nella percezione visiva.

---

**Autore**: Sistema Prometeo  
**Data**: 2026-02-27  
**Fase**: Esperimento Percezione Visiva
