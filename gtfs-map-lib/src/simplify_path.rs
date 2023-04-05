use std::f64;

use crate::path::Point;

fn shortest_distance_to_segment(point: &Point, line: &(&Point, &Point)) -> f64 {
    // from http://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    let (x1, mut y1) = (line.0.lat, line.0.lon);
    let (x2, mut y2) = (line.1.lat, line.1.lon);
    let (x0, mut y0) = (point.lat, point.lon);

    // scale longitude to match latitude for small distances
    // from http://en.wikipedia.org/wiki/Geographical_distance#Spherical_Earth_projected_to_a_plane
    let lon_factor = f64::cos((f64::consts::PI / 180.0) * x0);
    y1 *= lon_factor;
    y2 *= lon_factor;
    y0 *= lon_factor;

    let divisor = f64::sqrt(f64::powi(y2-y1, 2) + f64::powi(x2-x1, 2));
    if divisor == 0. {
        f64::INFINITY
    } else {
        f64::abs((y2-y1)*x0 - (x2-x1)*y0 + x2*y1 - y2*x1) / divisor
    }
}

pub fn simplify_path(path: &[Point]) -> Vec<Point> {
    // adapted from http://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
    let epsilon = 0.00001;

    let mut dmax: f64 = 0.;
    let mut index = 0;
    let end = path.len() - 1;
    for i in 1..end {
        let d = shortest_distance_to_segment(&path[i], &(&path[0], &path[end]));
        if d > dmax {
            index = i;
            dmax = d;
        }
    }
    let mut ret: Vec<Point> = Vec::new();
    if dmax > epsilon {
        let rec_results1 = simplify_path(&path[0..(index+1)]);
        let rec_results2 = simplify_path(&path[index..(end+1)]);
        let slice: &[Point] = &rec_results1[0..(rec_results1.len() - 1)];
        ret.extend(slice.iter().cloned());
        ret.extend(rec_results2.iter().cloned());
    } else {
        ret.push(path[0].clone());
        ret.push(path[end].clone());
    }
    ret
}
