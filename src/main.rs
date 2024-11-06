use plotters::prelude::*;
use std::env;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let pointsfile: String;
    let routefile: String;
    let outfile: String;
    let mut cities: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let mut route: Vec<usize> = Vec::<usize>::new();

    if args.len() == 4 {
        pointsfile = args[1].clone();
        routefile = args[2].clone();
        outfile = args[3].clone();
    } else {
        pointsfile = match read_filename("points data") {
            Ok(name) => name,
            Err(_) => return,
        };

        routefile = match read_filename("route data") {
            Ok(name) => name,
            Err(_) => return,
        };

        outfile = match read_filename("output") {
            Ok(name) => name,
            Err(_) => return,
        };
    }

    for result in BufReader::new(File::open(pointsfile.trim()).unwrap()).lines() {
        let line = match result {
            Ok(line) => line,
            Err(_) => continue,
        };
        let v: Vec<i32> = line.split(',').map(|k| k.parse().unwrap()).collect();
        let x = v[0];
        let y = v[1];
        cities.push((x, y));
    }

    for result in BufReader::new(File::open(routefile.trim()).unwrap()).lines() {
        let line = match result {
            Ok(line) => line,
            Err(_) => continue,
        };
        let r = match line.parse() {
            Ok(val) => val,
            Err(_) => continue,
        };
        route.push(r);
    }

    let mut root = BitMapBackend::new(outfile.trim(), (1920, 1080)).into_drawing_area();
    let _ = root.fill(&WHITE);
    root = root.margin(2, 2, 2, 2);

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0i32..1024i32, 0i32..1024i32)
        .unwrap();
    match chart.draw_series(PointSeries::of_element(
        cities.clone(),
        3,
        &BLACK,
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
        },
    )) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    let mut route2 = Vec::<(i32, i32)>::new();
    for r in route {
        route2.push(cities[r - 1]);
    }
    match chart.draw_series(LineSeries::new(route2, &BLACK)) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }

    match root.present() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    };
}

fn read_filename(whatfile: &str) -> Result<String, std::io::Error> {
    let mut filename = String::new();
    println!("please enter file path of {}:", whatfile);
    let ret = stdin().read_line(&mut filename);
    match ret {
        Ok(_) => {
            return Ok(filename);
        }
        Err(err) => {
            eprintln!("{}", err);
            return Err(err);
        }
    };
}
