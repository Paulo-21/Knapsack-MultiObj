mod indicator;
mod local_search;
mod parser;
mod utils;

use local_search::pls;
use parser::{read_file, read_points};
use utils::plot_points;

fn main() {
    println!("TME enssemble non-dominé");
    let num_instance = 0;
    let n = 100;
    let p = 2;

    let filename_dat = format!("Data/{}_items/2KP{}-TA-{}.dat", n, n, num_instance);
    let (w, v, max_cap) = read_file(filename_dat);

    // Lecture des points non dominés
    let filename_eff = format!("Data/{}_items/2KP{}-TA-{}.eff", n, n, num_instance);
    //let yn = read_points(filename_eff, p);
    //println!("{:?}", yn);
    //plot_points(&yn, "points.png").expect("RE");*/
    let m = 100;
    //let pareto_front = indicator::get_rand_sol(m, &w, p, &v, max_cap);
    //println!("{:?}", x);
    pls(m, &w, p, &v, max_cap);
}
