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
