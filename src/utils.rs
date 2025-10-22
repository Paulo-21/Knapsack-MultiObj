use plotters::prelude::*;
use std::error::Error;

/// Trace un nuage de points (scatter plot) pour YN
pub fn plot_points(
    yn: &[Vec<u32>],
    approx_yn: &[Vec<u32>],
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    // On crée la surface de dessin (ici, une image PNG)
    let root = BitMapBackend::new(output_file, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    // Déterminer les bornes (xmin, xmax, ymin, ymax)
    let (xmin, xmax) = yn
        .iter()
        .flat_map(|v| v.get(0))
        .fold((u32::MAX, u32::MIN), |(min, max), &x| {
            (min.min(x), max.max(x))
        });
    let (ymin, ymax) = yn
        .iter()
        .flat_map(|v| v.get(1))
        .fold((u32::MAX, u32::MIN), |(min, max), &y| {
            (min.min(y), max.max(y))
        });
    // Crée le graphique
    let mut chart = ChartBuilder::on(&root)
        .caption("Front de pareto", ("sans-serif", 25))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(xmin..xmax, ymin..ymax)?;

    // Ajoute une grille (équivalent de plt.grid())
    chart
        .configure_mesh()
        .x_desc("Axe X")
        .y_desc("Axe Y")
        .draw()?;

    // Ajoute les points bleus (équivalent de plt.scatter)
    chart.draw_series(
        yn.iter()
            .map(|v| Circle::new((v[0], v[1]), 3, BLUE.filled())),
    )?;
    chart.draw_series(
        approx_yn
            .iter()
            .map(|v| Circle::new((v[0], v[1]), 2, RED.filled())),
    )?;

    // Sauvegarde l’image
    root.present()?;
    println!("Graphique sauvegardé dans '{}'", output_file);

    Ok(())
}
