use na::Vec2;

/// A convex polygon for collision detection
pub struct ConPoly {
    pub corners: Vec<Vec2<f64>>,
}

impl ConPoly {
    /// Length <=> x-axis, Width <=> y-axis
    pub fn new_from_rect(length:f64, width:f64, pos: Vec2<f64>) -> ConPoly {
        let hwidth = length / 2.0;
        let hheight = width / 2.0;

        // Order: Top-left, bottom-left, bottom-right, top-right
        let x_pos = vec![-hwidth, -hwidth, hwidth, hwidth];
        let y_pos = vec![hheight, -hheight, -hheight, hheight];
        let corners: Vec<Vec2<f64>> = x_pos.iter().zip(y_pos.iter()).map(|(&x, &y)| {Vec2::new(x, y) + pos}).collect();
        ConPoly { corners: corners }
    }
}
