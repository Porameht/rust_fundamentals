#![allow(unused_variables)]
fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("DBQ", 41.51030, -83.88080),
        ("VIGGR", 42.55520, -93.12410),
        ("FOD", 42.61110, -94.29480),
        ("ONL", 42.47050, -98.68690),
        ("KSLC", 40.7861, -111.9822),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(waypoint.clone());
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radian = waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - waypoint.1).to_radians();
                let delta_longtitude = (previous_waypoint_value.2 - waypoint.2).to_radians();

                let inner_central_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                        * waypoint_radian.cos()
                        * f64::powi((delta_longtitude / 2.0).sin(), 2);
                let central_angle = 2.0 * inner_central_angle.sqrt().asin();
                let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;

                total_distance += distance;
                previous_waypoint = Option::from(waypoint.clone());

                println!(
                    "The distance between {} and {} is {:.1} kilometers",
                    previous_waypoint_value.0, waypoint.0, distance
                );
            }
        }
    }

    // println!("{}", inner_central_angle);
    // println!("{}", central_angle);
}
