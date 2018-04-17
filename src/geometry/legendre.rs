use ::geometry::{Polynomial, Polynomial2d};
use na::Vector2;

pub fn build_interpolating_poly(knots: Vec<f64>, values: Vec<f64>) -> Polynomial {
    if knots.len() != values.len() {
        panic!("Knots and values of different length!");
    }

    let mut output = Polynomial::zero();

    for (index, value) in values.into_iter().enumerate() {
        let factor = value * get_knot_scalar_product_for_knot(index, knots.clone());
        let poly = get_knot_poly_product_for_knot(index, knots.clone());
        println!("factor: {:?}, poly: {:?}", factor, poly);
        output = output + factor * poly;
    }

    output
}

pub fn build_interpolating_poly2d(knots: Vec<f64>, values: Vec<Vector2<f64>>) -> Polynomial2d {
    let values1 = values.iter().map(|val| {val.x}).collect();
    let values2 = values.iter().map(|val| {val.y}).collect();
    
    let poly1 = build_interpolating_poly(knots.clone(), values1);
    let poly2 = build_interpolating_poly(knots, values2);

    Polynomial2d::new_from_1d_polys(poly1, poly2)
}

fn get_knot_scalar_product_for_knot(index: usize, knots: Vec<f64>) -> f64 {
    knots.iter().enumerate().fold(1.0, |acc, (i, knot)| {
        if i != index {
            return acc / (knots[index] - knot);
        } else {
            return acc;
        }
    })
}

fn get_knot_poly_product_for_knot(index: usize, knots: Vec<f64>) -> Polynomial {
    knots.iter().enumerate().fold(Polynomial::identity(), |acc, (i, knot)| {
        if i != index {
            return acc * Polynomial::new_x_minus(*knot)
        } else {
            return acc;
        }
    })
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use ::geometry::build_interpolating_poly;

    #[test]
    fn interpolation_test_linear() {
        let error_margin = 0.00001;

        let knots = vec![0.0, 1.0];
        let values = vec![0.0, 1.0];

        let interpolating_poly = build_interpolating_poly(knots.clone(), values.clone());
        println!("{:?}", interpolating_poly);

        assert!((interpolating_poly.get_point(knots[0]) - values[0]).abs() < error_margin);
        assert!((interpolating_poly.get_point(knots[1]) - values[1]).abs() < error_margin);
    }

    #[test]
    fn interpolation_test_quadratic() {
        let error_margin = 0.00001;

        let knots = vec![0.0, 0.5, 1.0];
        let values = vec![0.0, 1.0, 0.0];

        let interpolating_poly = build_interpolating_poly(knots.clone(), values.clone());
        println!("{:?}", interpolating_poly);

        assert!((interpolating_poly.get_point(knots[0]) - values[0]).abs() < error_margin);

        println!("{:?}", interpolating_poly.get_point(knots[1]));
        assert!((interpolating_poly.get_point(knots[1]) - values[1]).abs() < error_margin);
        assert!((interpolating_poly.get_point(knots[2]) - values[2]).abs() < error_margin);
    }

    #[test]
    fn interpolation_test_cubic() {
        let error_margin = 0.00001;

        let knots = vec![0.0, 0.33, 0.66, 1.0];
        let values = vec![-PI.cos(), (-2.0 * PI / 3.0).cos(), (PI / 3.0).cos(), 0.0f64.cos()];

        let interpolating_poly = build_interpolating_poly(knots.clone(), values.clone());
        println!("Poly Details: {:?}", interpolating_poly);

        assert!((interpolating_poly.get_point(knots[0]) - values[0]).abs() < error_margin);
        assert!((interpolating_poly.get_point(knots[1]) - values[1]).abs() < error_margin);
        assert!((interpolating_poly.get_point(knots[2]) - values[2]).abs() < error_margin);
        assert!((interpolating_poly.get_point(knots[3]) - values[3]).abs() < error_margin);
    }
}