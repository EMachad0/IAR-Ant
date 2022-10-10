use plotters::prelude::*;

pub fn pickup_probability(similarity: f64) -> f64 {
    let k = 100. / 40.;
    let p = similarity * k - 0.1 * k;
    let p = 1. - p;
    p.clamp(0.0, 1.0)
}

pub fn drop_probability(similarity: f64) -> f64 {
    let k = 100. / 40.;
    let p = similarity * k - 0.1 * k;
    p.clamp(0.0, 1.0)
}

pub fn draw_probability_function() {
    let root_drawing_area =
        BitMapBackend::new("assets/img/probability_function.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .margin(15)
        .caption("Probability Function", ("Arial", 30))
        .set_all_label_area_size(50)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .unwrap();

    // Axis
    ctx.configure_mesh()
        .x_desc("Ratio between Items and Total Cells")
        .y_desc("Probability")
        .axis_desc_style(("Jetbrains Mono", 20))
        .draw()
        .unwrap();

    // Drop Probability
    let style = ShapeStyle {
        color: RED.into(),
        filled: false,
        stroke_width: 5,
    };
    ctx.draw_series(LineSeries::new(
        (0..=100)
            .map(|x| x as f64 / 100.0)
            .map(|x| (x, drop_probability(x))),
        style,
    ))
    .unwrap()
    .label("Drop Probability")
    .legend(|(x, y)| {
        PathElement::new(
            vec![(x, y), (x + 20, y)],
            ShapeStyle {
                color: RED.into(),
                filled: false,
                stroke_width: 5,
            },
        )
    });

    // Pickup Probability
    let style = ShapeStyle {
        color: BLUE.into(),
        filled: false,
        stroke_width: 5,
    };
    ctx.draw_series(LineSeries::new(
        (0..=100)
            .map(|x| x as f64 / 100.0)
            .map(|x| (x, pickup_probability(x))),
        style,
    ))
    .unwrap()
    .label("Pickup Probability")
    .legend(|(x, y)| {
        PathElement::new(
            vec![(x, y), (x + 20, y)],
            ShapeStyle {
                color: BLUE.into(),
                filled: false,
                stroke_width: 5,
            },
        )
    });

    // Label
    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("Jetbrains Mono", 20))
        .draw()
        .unwrap();
}
