/// Omologia — Il sistema che sa cosa non sa.
///
/// I numeri di Betti rivelano la struttura topologica del complesso:
/// - β₀ = componenti connesse (quanti "mondi" separati)
/// - β₁ = cicli indipendenti (lacune concettuali — "buchi" nella conoscenza)
/// - β₂ = cavità (vuoti strutturali — regioni completamente circondate ma vuote)
///
/// Un buco β₁ > 0 significa: il sistema ha concetti che si collegano in cerchio
/// ma non ha il concetto centrale che li unifica. È letteralmente una lacuna.

use std::collections::{BTreeSet, HashMap, HashSet};
use crate::topology::simplex::SimplicialComplex;
use crate::topology::fractal::FractalId;

/// Risultato del calcolo omologico.
#[derive(Debug, Clone)]
pub struct HomologyResult {
    /// β₀: componenti connesse
    pub betti_0: usize,
    /// β₁: cicli indipendenti (lacune concettuali)
    pub betti_1: usize,
    /// β₂: cavità
    pub betti_2: usize,
    /// Cicli concreti trovati (sequenze di vertici che formano loop senza "riempimento")
    pub cycles: Vec<Cycle>,
    /// Regioni dense: frattali con molti simplessi
    pub dense_regions: Vec<(FractalId, usize)>,
    /// Regioni sparse: frattali con pochi simplessi
    pub sparse_regions: Vec<(FractalId, usize)>,
}

/// Un ciclo nel complesso: sequenza di frattali connessi circolarmente
/// che non è il bordo di un simplesso di dimensione superiore.
#[derive(Debug, Clone)]
pub struct Cycle {
    /// I vertici del ciclo, in ordine
    pub vertices: Vec<FractalId>,
    /// Descrizione semantica (se disponibile)
    pub description: String,
}

/// Calcola l'omologia del complesso simpliciale.
/// Lavora su Z/2Z (aritmetica mod 2) per semplicità ed efficienza.
pub fn compute_homology(complex: &SimplicialComplex) -> HomologyResult {
    // 1. Estrai tutte le k-facce (incluse quelle implicite)
    let (vertices, edges, triangles) = extract_faces(complex);

    // 2. β₀ = componenti connesse
    let betti_0 = compute_connected_components(&vertices, &edges);

    // 3. Calcola β₁ e β₂ tramite matrici di bordo
    let betti_1 = compute_betti_1(&vertices, &edges, &triangles);
    let betti_2 = compute_betti_2(&edges, &triangles);

    // 4. Trova cicli concreti (per la curiosità)
    let cycles = find_cycles(&vertices, &edges, &triangles);

    // 5. Analisi densità regioni
    let (dense_regions, sparse_regions) = analyze_density(complex);

    HomologyResult {
        betti_0,
        betti_1,
        betti_2,
        cycles,
        dense_regions,
        sparse_regions,
    }
}

// ═══════════════════════════════════════════════════════════════
// Estrazione facce
// ═══════════════════════════════════════════════════════════════

type Edge = BTreeSet<FractalId>;
type Triangle = BTreeSet<FractalId>;

/// Estrae tutte le 0-facce (vertici), 1-facce (spigoli) e 2-facce (triangoli)
/// dal complesso, incluse le facce implicite dei simplessi di dimensione superiore.
fn extract_faces(
    complex: &SimplicialComplex,
) -> (Vec<FractalId>, Vec<Edge>, Vec<Triangle>) {
    let mut vertex_set: HashSet<FractalId> = HashSet::new();
    let mut edge_set: HashSet<Edge> = HashSet::new();
    let mut triangle_set: HashSet<Triangle> = HashSet::new();

    for (_, simplex) in complex.iter() {
        let verts = &simplex.vertices;

        // Tutti i vertici
        for &v in verts {
            vertex_set.insert(v);
        }

        // Tutti gli spigoli (sottoinsiemi di 2)
        for i in 0..verts.len() {
            for j in (i + 1)..verts.len() {
                let mut edge = BTreeSet::new();
                edge.insert(verts[i]);
                edge.insert(verts[j]);
                edge_set.insert(edge);
            }
        }

        // Tutti i triangoli (sottoinsiemi di 3)
        for i in 0..verts.len() {
            for j in (i + 1)..verts.len() {
                for k in (j + 1)..verts.len() {
                    let mut tri = BTreeSet::new();
                    tri.insert(verts[i]);
                    tri.insert(verts[j]);
                    tri.insert(verts[k]);
                    triangle_set.insert(tri);
                }
            }
        }
    }

    let mut vertices: Vec<FractalId> = vertex_set.into_iter().collect();
    vertices.sort();
    let edges: Vec<Edge> = edge_set.into_iter().collect();
    let triangles: Vec<Triangle> = triangle_set.into_iter().collect();

    (vertices, edges, triangles)
}

// ═══════════════════════════════════════════════════════════════
// Calcolo Betti numbers su Z/2Z
// ═══════════════════════════════════════════════════════════════

/// β₀ = numero di componenti connesse
fn compute_connected_components(vertices: &[FractalId], edges: &[Edge]) -> usize {
    if vertices.is_empty() {
        return 0;
    }

    let mut visited: HashSet<FractalId> = HashSet::new();
    let mut components = 0;

    // Costruisci lista di adiacenza
    let mut adj: HashMap<FractalId, Vec<FractalId>> = HashMap::new();
    for v in vertices {
        adj.entry(*v).or_default();
    }
    for edge in edges {
        let verts: Vec<&FractalId> = edge.iter().collect();
        if verts.len() == 2 {
            adj.entry(*verts[0]).or_default().push(*verts[1]);
            adj.entry(*verts[1]).or_default().push(*verts[0]);
        }
    }

    for &start in vertices {
        if visited.contains(&start) {
            continue;
        }
        components += 1;
        let mut stack = vec![start];
        while let Some(v) = stack.pop() {
            if !visited.insert(v) {
                continue;
            }
            if let Some(neighbors) = adj.get(&v) {
                for &n in neighbors {
                    if !visited.contains(&n) {
                        stack.push(n);
                    }
                }
            }
        }
    }

    components
}

/// β₁ = dim(ker(∂₁)) - dim(im(∂₂))
///     = (num_edges - rank(∂₁)) - rank(∂₂)
///
/// ∂₁: edges → vertices (matrice di bordo degli spigoli)
/// ∂₂: triangles → edges (matrice di bordo dei triangoli)
fn compute_betti_1(
    vertices: &[FractalId],
    edges: &[Edge],
    triangles: &[Triangle],
) -> usize {
    if edges.is_empty() {
        return 0;
    }

    let rank_d1 = rank_boundary_1(vertices, edges);
    let rank_d2 = rank_boundary_2(edges, triangles);

    let nullity_d1 = edges.len().saturating_sub(rank_d1);
    nullity_d1.saturating_sub(rank_d2)
}

/// β₂ = dim(ker(∂₂)) - dim(im(∂₃))
/// Per ora ∂₃ = 0 (non abbiamo 3-simplessi nel bootstrap tipico)
fn compute_betti_2(edges: &[Edge], triangles: &[Triangle]) -> usize {
    if triangles.is_empty() {
        return 0;
    }

    let rank_d2 = rank_boundary_2(edges, triangles);
    let nullity_d2 = triangles.len().saturating_sub(rank_d2);
    // dim(im(∂₃)) = 0 per ora (nessun 3-simplesso)
    nullity_d2
}

/// Rango della matrice di bordo ∂₁ su Z/2Z.
/// ∂₁ mappa ogni spigolo {a,b} alla somma dei suoi vertici: a + b (mod 2).
fn rank_boundary_1(vertices: &[FractalId], edges: &[Edge]) -> usize {
    if edges.is_empty() || vertices.is_empty() {
        return 0;
    }

    // Indice vertici
    let v_index: HashMap<FractalId, usize> = vertices.iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect();

    let nrows = vertices.len();
    let ncols = edges.len();

    // Matrice nrows × ncols su Z/2Z (bit-packed per riga)
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];

    for (j, edge) in edges.iter().enumerate() {
        for v in edge {
            if let Some(&i) = v_index.get(v) {
                matrix[i][j] = true;
            }
        }
    }

    gaussian_elimination_z2(&mut matrix)
}

/// Rango della matrice di bordo ∂₂ su Z/2Z.
/// ∂₂ mappa ogni triangolo {a,b,c} alla somma dei suoi spigoli: {a,b} + {a,c} + {b,c} (mod 2).
fn rank_boundary_2(edges: &[Edge], triangles: &[Triangle]) -> usize {
    if triangles.is_empty() || edges.is_empty() {
        return 0;
    }

    // Indice spigoli
    let e_index: HashMap<&Edge, usize> = edges.iter()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();

    let nrows = edges.len();
    let ncols = triangles.len();

    let mut matrix: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];

    for (j, tri) in triangles.iter().enumerate() {
        // Genera i 3 spigoli del triangolo
        let verts: Vec<&FractalId> = tri.iter().collect();
        for a in 0..verts.len() {
            for b in (a + 1)..verts.len() {
                let mut edge = BTreeSet::new();
                edge.insert(*verts[a]);
                edge.insert(*verts[b]);
                if let Some(&i) = e_index.get(&edge) {
                    matrix[i][j] ^= true; // XOR = somma mod 2
                }
            }
        }
    }

    gaussian_elimination_z2(&mut matrix)
}

/// Eliminazione gaussiana su Z/2Z.
/// Restituisce il rango della matrice.
fn gaussian_elimination_z2(matrix: &mut [Vec<bool>]) -> usize {
    if matrix.is_empty() {
        return 0;
    }

    let nrows = matrix.len();
    let ncols = matrix[0].len();
    let mut rank = 0;
    let mut pivot_col = 0;

    for row in 0..nrows {
        if pivot_col >= ncols {
            break;
        }

        // Trova pivot in questa colonna
        let mut found = None;
        for r in row..nrows {
            if matrix[r][pivot_col] {
                found = Some(r);
                break;
            }
        }

        match found {
            Some(pivot_row) => {
                // Scambia righe
                if pivot_row != row {
                    matrix.swap(row, pivot_row);
                }

                // Elimina questa colonna dalle altre righe
                for r in 0..nrows {
                    if r != row && matrix[r][pivot_col] {
                        // Riga r ^= riga row (somma mod 2)
                        let row_copy: Vec<bool> = matrix[row].clone();
                        for c in 0..ncols {
                            matrix[r][c] ^= row_copy[c];
                        }
                    }
                }

                rank += 1;
                pivot_col += 1;
            }
            None => {
                pivot_col += 1;
            }
        }
    }

    rank
}

// ═══════════════════════════════════════════════════════════════
// Ricerca cicli concreti
// ═══════════════════════════════════════════════════════════════

/// Trova cicli concreti nel complesso che non sono bordo di un 2-simplesso.
/// Questi rappresentano lacune concettuali.
fn find_cycles(
    vertices: &[FractalId],
    edges: &[Edge],
    triangles: &[Triangle],
) -> Vec<Cycle> {
    let mut cycles = Vec::new();

    // Costruisci adiacenza
    let mut adj: HashMap<FractalId, Vec<FractalId>> = HashMap::new();
    for v in vertices {
        adj.entry(*v).or_default();
    }
    for edge in edges {
        let verts: Vec<&FractalId> = edge.iter().collect();
        if verts.len() == 2 {
            adj.entry(*verts[0]).or_default().push(*verts[1]);
            adj.entry(*verts[1]).or_default().push(*verts[0]);
        }
    }

    // HashSet degli spigoli per lookup O(1) — evita O(E) per ogni contains()
    let edge_set: HashSet<Edge> = edges.iter().cloned().collect();
    // Insieme dei triangoli per verifica rapida
    let tri_set: HashSet<&Triangle> = triangles.iter().collect();

    // Cerca cicli di lunghezza 3 che non sono bordo di un triangolo.
    // OTTIMIZZAZIONE: edge_set.contains() è O(1) invece di edges.contains() O(E).
    // Con 4487 spigoli e 307 vertici: O(E × N) invece di O(N × E²).
    // Cap: esamina al massimo 200 spigoli per evitare esplosione con grafi densi.
    for edge in edges.iter().take(200) {
        let verts: Vec<FractalId> = edge.iter().copied().collect();
        let a = verts[0];
        let b = verts[1];

        // Cerca c tale che a-c e b-c esistono ma {a,b,c} non è un triangolo
        if let Some(neighbors_a) = adj.get(&a) {
            for &c in neighbors_a.iter().take(50) {
                if c <= b {
                    continue; // evita duplicati
                }
                // c è collegato ad a, controlla se c è collegato a b
                let bc_edge: Edge = [b, c].iter().copied().collect();
                if edge_set.contains(&bc_edge) {
                    // Ciclo a-b-c trovato. È colmato da un triangolo?
                    let tri: Triangle = [a, b, c].iter().copied().collect();
                    if !tri_set.contains(&tri) {
                        cycles.push(Cycle {
                            vertices: vec![a, b, c],
                            description: format!(
                                "Ciclo [{}, {}, {}]: connessi a coppie ma manca il concetto unificante",
                                a, b, c
                            ),
                        });
                    }
                }
            }
        }
    }

    cycles
}

// ═══════════════════════════════════════════════════════════════
// Analisi densità
// ═══════════════════════════════════════════════════════════════

/// Analizza la densità delle regioni: quanti simplessi coinvolgono ciascun frattale.
fn analyze_density(complex: &SimplicialComplex) -> (Vec<(FractalId, usize)>, Vec<(FractalId, usize)>) {
    let mut counts: HashMap<FractalId, usize> = HashMap::new();

    for (_, simplex) in complex.iter() {
        for &v in &simplex.vertices {
            *counts.entry(v).or_insert(0) += 1;
        }
    }

    let mut all: Vec<(FractalId, usize)> = counts.into_iter().collect();
    all.sort_by(|a, b| b.1.cmp(&a.1));

    if all.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let median = all[all.len() / 2].1;

    let dense: Vec<(FractalId, usize)> = all.iter()
        .filter(|(_, c)| *c > median + 1)
        .cloned()
        .collect();

    let sparse: Vec<(FractalId, usize)> = all.iter()
        .filter(|(_, c)| *c <= median.saturating_sub(1).max(1))
        .cloned()
        .collect();

    (dense, sparse)
}

// ═══════════════════════════════════════════════════════════════
// Test
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::fractal::bootstrap_fractals;
    use crate::topology::simplex::{bootstrap_complex, SharedFace};
    use crate::topology::primitive::Dim;

    #[test]
    fn test_bootstrap_homology() {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);

        let result = compute_homology(&complex);

        // Il bootstrap ha 6 frattali fondamentali tutti connessi → β₀ = 1
        assert_eq!(result.betti_0, 1,
            "I frattali fondamentali devono formare una componente connessa");

        // β₁ ≥ 0 (ci possono essere cicli non colmati)
        // Il valore esatto dipende dalla topologia del bootstrap
        println!("Bootstrap: β₀={}, β₁={}, β₂={}", result.betti_0, result.betti_1, result.betti_2);
        println!("Cicli trovati: {}", result.cycles.len());
        for c in &result.cycles {
            println!("  {:?}", c.vertices);
        }
    }

    #[test]
    fn test_single_triangle_no_holes() {
        // Un singolo triangolo {0,1,2} non ha buchi
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(
            vec![0, 1, 2],
            vec![SharedFace::from_dim(Dim::Confine, 0.5)],
        );

        let result = compute_homology(&complex);

        assert_eq!(result.betti_0, 1, "Un triangolo = una componente");
        assert_eq!(result.betti_1, 0, "Un triangolo pieno non ha buchi 1D");
    }

    #[test]
    fn test_triangle_boundary_has_hole() {
        // Tre spigoli {0,1}, {1,2}, {0,2} SENZA il triangolo {0,1,2}
        // Formano un ciclo → β₁ = 1
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(
            vec![0, 1],
            vec![SharedFace::from_dim(Dim::Confine, 0.5)],
        );
        complex.add_simplex(
            vec![1, 2],
            vec![SharedFace::from_dim(Dim::Valenza, 0.5)],
        );
        complex.add_simplex(
            vec![0, 2],
            vec![SharedFace::from_dim(Dim::Intensita, 0.5)],
        );

        let result = compute_homology(&complex);

        assert_eq!(result.betti_0, 1, "Tutto connesso");
        assert_eq!(result.betti_1, 1, "Un ciclo senza riempimento = un buco");
        assert_eq!(result.cycles.len(), 1, "Deve trovare il ciclo 0-1-2");
    }

    #[test]
    fn test_disconnected_components() {
        // Due spigoli disconnessi: {0,1} e {2,3}
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(
            vec![0, 1],
            vec![SharedFace::from_dim(Dim::Confine, 0.5)],
        );
        complex.add_simplex(
            vec![2, 3],
            vec![SharedFace::from_dim(Dim::Valenza, 0.5)],
        );

        let result = compute_homology(&complex);

        assert_eq!(result.betti_0, 2, "Due componenti disconnesse");
        assert_eq!(result.betti_1, 0, "Nessun ciclo");
    }

    #[test]
    fn test_cycle_detection() {
        // Quadrato: {0,1}, {1,2}, {2,3}, {0,3} senza riempimento
        let mut complex = SimplicialComplex::new();
        complex.add_simplex(vec![0, 1], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);
        complex.add_simplex(vec![1, 2], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);
        complex.add_simplex(vec![2, 3], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);
        complex.add_simplex(vec![0, 3], vec![SharedFace::from_dim(Dim::Confine, 0.5)]);

        let result = compute_homology(&complex);

        assert_eq!(result.betti_0, 1, "Tutto connesso");
        assert_eq!(result.betti_1, 1, "Un ciclo quadrato = un buco");
    }

    #[test]
    fn test_density_analysis() {
        let reg = bootstrap_fractals();
        let mut ids = reg.all_ids(); ids.sort();
        let complex = bootstrap_complex(&ids);

        let result = compute_homology(&complex);

        // EGO (id=2) dovrebbe essere tra i più densi (è in molti simplessi)
        println!("Dense: {:?}", result.dense_regions);
        println!("Sparse: {:?}", result.sparse_regions);
    }
}
