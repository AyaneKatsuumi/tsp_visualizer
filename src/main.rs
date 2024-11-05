use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    //let mut cities: Vec<(usize, isize, isize)> = Vec::<(usize, isize, isize)>::new();
    //let mut route: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();
    let mut cities: Vec<(i32, i32)> = Vec::<(i32, i32)>::new();
    let mut route: Vec<(usize)> = Vec::<(usize)>::new();

    let filename = match read_filename("points data") {
        Ok(name) => name,
        Err(_) => return,
    };

    let mut i: usize = 1;
    for result in BufReader::new(File::open(filename.trim()).unwrap()).lines() {
        let line = match result {
            Ok(line) => line,
            Err(_) => continue,
        };
        let v: Vec<i32> = line.split(',').map(|k| k.parse().unwrap()).collect();
        let x = v[0];
        let y = v[1];
        //cities.push((i, x, y));
        cities.push((x, y));
        i += 1;
    }

    /*// debug
    for city in cities.iter() {
        println!("{} {} {}", city.0, city.1, city.2);
    }
    */

    let filename = match read_filename("route data") {
        Ok(filename) => filename,
        Err(_) => return,
    };

    i = 1;
    for result in BufReader::new(File::open(filename.trim()).unwrap()).lines() {
        let line = match result {
            Ok(line) => line,
            Err(_) => continue,
        };
        let r = match line.parse() {
            Ok(val) => val,
            Err(_) => continue,
        };
        //route.push((i, r));
        route.push(r);
        i += 1;
    }

    /*// debug
    for r in route {
        println!("{} {}", r.0, r.1);
    }
    */

    let mut root = BitMapBackend::new("dots_and_lines.png", (1920, 1080)).into_drawing_area();
    let _ = root.fill(&WHITE);
    root = root.margin(2, 2, 2, 2);
    /*let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    ));

    let dot = |x: isize, y: isize| {
        return EmptyElement::<(isize, isize), BitMapBackend>::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled());
    };*/

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
