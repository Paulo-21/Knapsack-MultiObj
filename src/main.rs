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
    let yn = read_points(filename_eff, p);
    //println!("{:?}", yn);
    let m = 10000000;
    let approx_yn = pls(m, &w, p, &v, max_cap);
    let approx_pt: Vec<Vec<u32>> = approx_yn.iter().map(|(_, y)| y.clone()).collect();
    plot_points(&yn, &approx_pt, "points.png").expect("RE");
}
