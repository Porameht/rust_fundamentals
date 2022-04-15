#![allow (unused)]

struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64
}

struct Segment {
    start: Waypoint,
    end: Waypoint
}

fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let mut kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111
    };

    let mut kslc = Waypoint{
        name: "KSLC".to_string(),
        ..kcle
    };

}