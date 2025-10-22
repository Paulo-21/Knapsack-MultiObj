use rand::prelude::*;

pub fn mise_a_jour(
    pareto_front: &mut Vec<(Vec<bool>, Vec<u32>)>,
    x: (Vec<bool>, Vec<u32>),
) -> bool {
    let mut updated = pareto_front.is_empty();
    let mut indices = Vec::new();
    for (k, xp) in pareto_front.iter_mut().enumerate() {
        if xp.1[0] >= x.1[0] && xp.1[1] >= x.1[1] {
            return false;
        } else {
            updated = true;
            if xp.1[0] <= x.1[0] && xp.1[1] <= x.1[1] {
                indices.push(k);
            }
        }
    }
    if updated {
        for (i, idx) in indices.iter().enumerate() {
            pareto_front.remove(idx - i);
        }
        pareto_front.push(x);
    }
    updated
}
pub fn get_rand_sol(
    m: usize,
    w: &[u32],
    p: usize,
    v: &[Vec<u32>],
    max_cap: u32,
) -> Vec<(Vec<bool>, Vec<u32>)> {
    let mut rng = rand::rng();
    let mut ynd: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    let n = w.len();
    for _ in 0..m.pow(2) {
        let mut x_start = vec![false; n as usize];
        let mut indices: Vec<usize> = (0..n).collect();
        indices.shuffle(&mut rng);

        let mut w_total = 0;
        let mut v_start = vec![0; p];

        for i in indices {
            if w_total + w[i] <= max_cap {
                x_start[i] = true;
                w_total += w[i];
                for j in 0..p {
                    v_start[j] += v[i][j as usize];
                }
            }
        }
        let updated = mise_a_jour(&mut ynd, (x_start, v_start));
    }
    ynd
}
/// Compare la proportion de points de YN retrouvés dans YApprox
pub fn proportion(yn: &[Vec<f64>], y_approx: &[(usize, Vec<f64>)]) -> f64 {
    let mut count = 0;

    for y in yn {
        if y_approx.iter().any(|(_, sol)| sol == y) {
            count += 1;
        }
    }

    count as f64 / yn.len() as f64
}

/// Distance euclidienne pondérée entre deux points y1 et y2
pub fn distance_euclidienne(y1: &[f64], y2: &[f64], poids: &[f64]) -> f64 {
    y1.iter()
        .zip(y2.iter())
        .zip(poids.iter())
        .map(|((a, b), w)| (a - b).powi(2) * w)
        .sum::<f64>()
        .sqrt()
}

/// Distance minimale entre un point `y` et les points de `YApprox`
pub fn d_prime(y_approx: &[(usize, Vec<f64>)], y: &[f64], poids: &[f64]) -> f64 {
    y_approx
        .iter()
        .map(|(_, sol)| distance_euclidienne(y, sol, poids))
        .fold(f64::INFINITY, f64::min)
}

/// Calcul de la moyenne des distances minimales (DM)
pub fn dm(yn: &[Vec<f64>], y_approx: &[(usize, Vec<f64>)]) -> f64 {
    let p = yn[0].len();

    // Nadir = min par colonne, Ideal = max par colonne
    let mut nadir = vec![f64::INFINITY; p];
    let mut ideal = vec![f64::NEG_INFINITY; p];

    for y in yn {
        for j in 0..p {
            if y[j] < nadir[j] {
                nadir[j] = y[j];
            }
            if y[j] > ideal[j] {
                ideal[j] = y[j];
            }
        }
    }

    // Poids = 1 / |Ideal - Nadir|
    let poids: Vec<f64> = (0..p)
        .map(|j| 1.0 / (ideal[j] - nadir[j]).abs().max(1e-9)) // évite la division par zéro
        .collect();

    // Moyenne des distances minimales
    let total_d: f64 = yn.iter().map(|y| d_prime(y_approx, y, &poids)).sum();

    total_d / yn.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dm() {
        let yn = vec![vec![1.0, 2.0], vec![2.0, 3.0], vec![3.0, 4.0]];
        let y_approx = vec![(0, vec![1.0, 2.0]), (1, vec![3.0, 4.0])];

        let prop = proportion(&yn, &y_approx);
        let d = dm(&yn, &y_approx);

        println!("proportion = {}", prop);
        println!("DM = {}", d);
    }
}
