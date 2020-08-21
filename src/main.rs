use std::io;

fn main() {
    //let mut lat1 = String::new();
    //let mut lon1 = String::new();

    println!("Input latitude and longitude: ");
    let (lat1,lon1) = take_input();
    println!("Coords of point 1 (lat,lon): {} {}", lat1, lon1);

    let d = haversine(lat1,lon1);
    println!("{}",d);

    example_earth();
}

fn take_input() -> (usize,usize){
    let mut val = String::new();
    // Read in latitude and longitude of first point
    io::stdin().read_line(&mut val)
        .expect("Failed to read input.");

    let mut substr_iter = val.split_whitespace();
    let mut next_num = || -> usize {
        substr_iter.next().expect("Not enough input numbers")
                   .parse().expect("Input is not a number")
    };
    let lat1 = next_num();
    let lon1 = next_num();

    (lat1,lon1)
}

fn haversine(lat1: usize, lon1: usize) -> f32 {
    let mut lat = lat1 as f32;
    let mut lon = lon1 as f32;

    lat = lat.to_radians();
    let d = lat+lon;
    d
}

fn example_earth() {

    let earth_radius_kilometer = 6371.0_f64;
    let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34880_f64);
    let (london_latitude_degrees, london_longitude_degrees) = (51.50853_f64, -0.12574_f64);

    let paris_latitude = paris_latitude_degrees.to_radians();
    let london_latitude = london_latitude_degrees.to_radians();

    let delta_latitude = (paris_latitude_degrees - london_latitude_degrees).to_radians();
    let delta_longitude = (paris_longitude_degrees - london_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + paris_latitude.cos() * london_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
        distance
    );

}
