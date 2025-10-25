pub fn pls3(
    m: usize,
    w: &[u32],
    _p: usize,
    v: &[Vec<u32>],
    max_cap: u32,
) -> Vec<(Vec<bool>, Vec<u32>)> {
    println!("PLS2 Start");
    let mut weight_sorted_idx: Vec<usize> = (0..w.len()).collect();
    weight_sorted_idx.sort_by_key(|&i| w[i]);

    let mut pareto_front = gen_init_pop(w, v, max_cap, m);
    let mut all_pprime = Vec::new();
    let mut pop = pareto_front.clone();
    let mut pop_aux: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    println!("Start with {} pt", pop.len());
    while !pop.is_empty() {
        for p in pop.iter_mut() {
            get_voisins(p, w, v, max_cap, &weight_sorted_idx, &mut all_pprime);
            while let Some(pp) = all_pprime.pop() {
                if !(p.1[0] >= pp.1[0] && p.1[1] >= pp.1[1])
                    && mise_a_jour2(&mut pareto_front, pp.clone())
                {
                    mise_a_jour2(&mut pop_aux, pp);
                }
            }
        }
        println!("{}", pop_aux.len());
        std::mem::swap(&mut pop, &mut pop_aux);
        pop_aux.clear();
    }
    return pareto_front;
}
fn get_voisins(
    x: &mut (Vec<bool>, Vec<u32>),
    w: &[u32],
    v: &[Vec<u32>],
    max_cap: u32,
    weight_sorted_idx: &Vec<usize>,
    voisins: &mut Vec<(Vec<bool>, Vec<u32>)>,
) {
    let (take, profit) = x;
    let mut tot_weight = 0;
    for (ti, wi) in take.iter().zip(w) {
        tot_weight += wi * (*ti as u32);
    }

    for k in 0..take.len() {
        if take[k] {
            take[k] = false;
            profit[0] -= v[k][0];
            profit[1] -= v[k][1];
            tot_weight -= w[k];
            for s in weight_sorted_idx {
                if *s != k && !take[*s] && tot_weight + w[*s] <= max_cap {
                    profit[0] += v[*s][0];
                    profit[1] += v[*s][1];
                    take[*s] = true;
                    voisins.push((take.clone(), profit.clone()));
                    take[*s] = false;
                    profit[0] -= v[*s][0];
                    profit[1] -= v[*s][1];
                } else if tot_weight + w[*s] > max_cap {
                    break;
                }
            }
            take[k] = true;
            profit[0] += v[k][0];
            profit[1] += v[k][1];
            tot_weight += w[k];
        }
    }
}
pub fn mise_a_jour2(
    pareto_front: &mut Vec<(Vec<bool>, Vec<u32>)>,
    xpt: (Vec<bool>, Vec<u32>),
) -> bool {
    let x = [xpt.1[0], xpt.1[1]];
    let mut k = match pareto_front.binary_search_by(|pt| pt.1[0].cmp(&x[0])) {
        Ok(j) => j,
        Err(index) => index,
    };

    if k < pareto_front.len() {
        if x[0] == pareto_front[k].1[0] {
            if x[1] > pareto_front[k].1[1] {
                //Changer si meilleur que pareil
                pareto_front[k] = xpt;
            } else {
                return false;
            }
        } else if x[0] < pareto_front[k].1[0] {
            if x[1] > pareto_front[k].1[1] {
                pareto_front.insert(k, xpt);
            } else {
                return false;
            }
        }
    } else {
        pareto_front.insert(k, xpt);
    }

    let mut done = k > 0;
    k -= done as usize;
    while done {
        if pareto_front[k].1[1] <= x[1] {
            pareto_front.remove(k);
        } else {
            break;
        }
        done = k > 0;
        k -= done as usize;
    }
    true
}
fn gen_init_pop(w: &[u32], v: &[Vec<u32>], max_cap: u32, m: usize) -> Vec<(Vec<bool>, Vec<u32>)> {
    let mut pareto_front: Vec<(Vec<bool>, Vec<u32>)> = Vec::new();
    for i in 0..=m {
        let q = (1. / m as f32) * i as f32;
        let greedy_sol = gen_sol_q(w, v, max_cap, q);
        mise_a_jour2(&mut pareto_front, greedy_sol);
    }
    pareto_front
}
fn gen_sol_q(w: &[u32], v: &[Vec<u32>], max_cap: u32, q: f32) -> (Vec<bool>, Vec<u32>) {
    let obj: Vec<usize> = (0..w.len()).collect();
    let mut sol = vec![false; w.len()];
    let mut a = Vec::with_capacity(w.len());
    for ((wi, vi), xi) in w.iter().zip(v).zip(obj) {
        a.push((wi, vi, xi));
    }
    a.sort_unstable_by(|(wa, va, _), (wb, vb, _)| {
        ((q * (va[0]) as f32 + (1. - q) * va[1] as f32) / (**wa as f32))
            .partial_cmp(&((q * (vb[0]) as f32 + (1. - q) * vb[1] as f32) / (**wb as f32)))
            .unwrap()
    });
    let mut tot_weight: u32 = 0;
    let mut v2: Vec<u32> = vec![0; v[0].len()];
    for (wi, vi, xi) in a.iter().rev() {
        if **wi + tot_weight <= max_cap {
            tot_weight += **wi;
            for (k, value) in v2.iter_mut().zip(*vi) {
                *k += value;
            }
            sol[*xi] = true;
        }
    }
    (sol, v2)
}
fn _print_neightboor(
    k: usize,
    front: &[(Vec<bool>, Vec<u32>)],
    n: usize,
    x: &(Vec<bool>, Vec<u32>),
    res: bool,
) {
    for j in k - n / 2..=k + n / 2 {
        if j == front.len() {
            break;
        }
        print!("{:?} ", front[j].1);
    }
    print!("| {:?} -> ", x.1);
    println!("{res}");
}
