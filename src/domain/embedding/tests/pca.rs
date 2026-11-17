use super::*;

/// Points spread along `(1, 1, 0)/√2` with a small spread along `z`.
fn line_with_noise() -> Vec<Vec<f32>> {
    (0..9)
        .map(|i| {
            let t = i as f32 - 4.0;
            let wobble = if i % 2 == 0 { 0.1 } else { -0.1 };
            vec![5.0 + t, 5.0 + t, wobble]
        })
        .collect()
}

#[test]
fn the_first_component_follows_the_spread_and_keeps_the_order() {
    let projection = project_2d(&line_with_noise());
    let [first, second] = &projection.components;
    let expected = std::f32::consts::FRAC_1_SQRT_2;
    assert!((first[0].abs() - expected).abs() < 1e-3 && (first[1].abs() - expected).abs() < 1e-3, "{first:?}");
    assert!(first[2].abs() < 1e-3);
    assert!(dot(first, second).abs() < 1e-3, "components are orthogonal");
    assert!(second[2].abs() > 0.99, "the second follows the wobble: {second:?}");
    let xs: Vec<f32> = projection.points.iter().map(|p| p[0]).collect();
    let increasing = xs.windows(2).all(|w| w[1] > w[0]);
    let decreasing = xs.windows(2).all(|w| w[1] < w[0]);
    assert!(increasing || decreasing, "order along the line is kept: {xs:?}");
    let spread = (xs[8] - xs[0]).abs();
    assert!((spread - 8.0 * std::f32::consts::SQRT_2).abs() < 1e-2, "distances are kept: {spread}");
}

#[test]
fn the_projection_is_centered_and_degenerate_input_stays_at_the_origin() {
    let projection = project_2d(&line_with_noise());
    let mean_x: f32 = projection.points.iter().map(|p| p[0]).sum::<f32>() / projection.points.len() as f32;
    assert!(mean_x.abs() < 1e-4);
    assert!(project_2d(&[]).points.is_empty());
    let single = project_2d(&[vec![1.0, 2.0, 3.0]]);
    assert_eq!(single.points, vec![[0.0, 0.0]]);
    let same = project_2d(&[vec![1.0, 1.0], vec![1.0, 1.0]]);
    assert!(same.points.iter().all(|p| p[0].abs() < 1e-6 && p[1].abs() < 1e-6));
}
