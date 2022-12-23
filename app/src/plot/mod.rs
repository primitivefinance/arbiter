use plotters::prelude::*;
use chrono::{Utc, TimeZone};

pub fn plot(
    time_data: Vec<f64>,
    price_data: Vec<f64>,
    filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Deal with the time data
    if time_data.last() == None {
        println!("Time data is empty!");
        panic!()
    }
    let t_start = time_data[0];
    let t_end = *time_data.last().unwrap();
    let x_spec = t_start as f32..t_end as f32;

    // Deal with the price data
    if price_data.last() == None {
        println!("Time data is empty!");
        panic!()
    }
    let (price_low, price_high) = price_data
        .iter()
        .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), price_data| {
            (price_data.min(m), price_data.max(n))
        });
    // let price_low = price_data.iter().copied().fold(f64::INFINITY,f64::min);
    // let price_high = price_data.iter().copied().fold(f64::NAN, f64::max);
    let y_spec = price_low as f32..price_high as f32;
    println!("Price_low: {price_low:#?}");
    println!("Price_high: {price_high:#?}");

    // TESTING PLOTTING BELOW
    let root_area = BitMapBackend::new("./sample.png", (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let start_date = Utc.ymd(2019, 10, 1);
  let end_date = Utc.ymd(2019, 10, 18);

  let DATA: [f64; 14] = [ 137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.10, 138.24, 135.67, 137.12, 138.12];

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("MSFT daily close price", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 130.0..145.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        LineSeries::new(
            (0..).zip(DATA.iter()).map(|(idx, price)| {
                let day = (idx / 5) * 7 + idx % 5 + 1;
                let date = Utc.ymd(2019,10, day);
                (date, *price)
            }),
            &BLUE,
        )
    ).unwrap();



    // let root_area = BitMapBackend::new("./sample.png", (600, 400)).into_drawing_area();
    // root_area.fill(&WHITE).unwrap();

    // let mut ctx = ChartBuilder::on(&root_area)
    //     .set_label_area_size(LabelAreaPosition::Left, 40)
    //     .set_label_area_size(LabelAreaPosition::Bottom, 40)
    //     .caption("Scatter Demo", ("sans-serif", 40))
    //     .build_cartesian_2d(0..10, 0..50)
    //     .unwrap();

    // ctx.configure_mesh().draw().unwrap();
    // let font = ("san-serif", 20);
    // let mut chart = ChartBuilder::on(&root_area)
    //     .caption("kinetic energy and potential", font.into_font())
    //     .build_cartesian_2d(x_spec, y_spec)?;
    // chart.configure_mesh().draw()?;

    //    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    //   let data: &[f64] = &time_data;
    // println!("DATA = {data:#?}");
    // let series = LineSeries::new(time_data, &RGBColor(255, 0, 0));
    // chart.draw_series(series);
    // let data = [time_data, price_data];

    // let root_area = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();

    // root_area.fill(&WHITE)?;

    // //let root_area = root_area.titled("GBM Simulation", ("serif", 40))?;
    // let root_area = BitMapBackend::new("images/2.7.png", (600, 400))
    // .into_drawing_area();
    // root_area.fill(&WHITE).unwrap();

    // let (upper, _lower) = root_area.split_vertically(512);

    // let x_axis = (-3.4f32..3.4).step(0.1);

    // let mut cc = ChartBuilder::on(&upper)
    //     .margin(5)
    //     .set_all_label_area_size(50)
    //     .caption("Prices", ("serif", 24))
    //     .build_cartesian_2d(x_spec, y_spec)?;

    // cc.configure_mesh()
    //     .x_labels(20)
    //     .y_labels(10)
    //     .disable_mesh()
    //     .x_label_formatter(&|v| format!("{:.1}", v))
    //     .y_label_formatter(&|v| format!("{:.1}", v))
    //     .draw()?;

    // cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))?
    //     .label("Prices")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filename);
    Ok(())
}
