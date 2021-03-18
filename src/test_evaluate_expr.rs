use super::evaluate_expr;


// Math operations
#[test]
fn testing_integer_addition() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["5", "4", "+", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 9.0);
}

#[test]
fn testing_floating_addition() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["0.1", "0.2", "+", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 0.30000000000000004);
}

#[test]
fn testing_integer_subtraction() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["5", "4", "-", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.0);
}

#[test]
fn testing_floating_subtraction() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["5.4", "4.2", "-", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.2000000000000002);
}

#[test]
fn testing_integer_multiplication() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["5", "4", "*", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 20.0);
}


#[test]
fn testing_floating_multiplication() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["1.2", "0.7", "*", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 0.84);
}

#[test]
fn testing_integer_division() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["10", "2", "/", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 5.0);
}

#[test]
fn testing_floating_division() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["10", "7", "/", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.4285714285714286);
}

#[test]
fn testing_division_by_zero() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["1", "0", "/", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.0);
}

#[test]
fn testing_exponential() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["4", "2", "^", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 16.0);
}

#[test]
fn testing_integer_square_root() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["256","v", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 16.0);
}

#[test]
fn testing_floating_square_root() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["5", "v", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 2.23606797749979);
}

#[test]
fn testing_modulo() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["13", "3", "%", "p"];

    evaluate_expr::evaluate_expression(
        tokens,
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.0);
}

#[test]
fn testing_deg_to_rad() {
    let angle = 180.0;
    let theta = evaluate_expr::deg_to_rad(
        angle
    );
    assert_eq!(theta, std::f64::consts::PI);
}

#[test]
fn testing_rad_to_deg() {
    let angle = std::f64::consts::PI;
    let theta = evaluate_expr::rad_to_deg(
        angle
    );
    assert_eq!(theta, 180.0);
}

#[test]
fn testing_sine() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["90", "sin"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.0);
}

#[test]
fn testing_cosine() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["0", "cos"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), 1.0);
}

#[test]
fn testing_tangent() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["45", "tan"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap().round(), 1.0);
}

#[test]
fn testing_arcsine() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["1", "asin"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), evaluate_expr::deg_to_rad(90.0));
}

#[test]
fn testing_arcsine_out_of_domain() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["-5", "asin"];

    assert!(
        evaluate_expr::evaluate_expression(
            tokens.clone(),
            &mut stack_vec
        ) == false
    );
}

#[test]
fn testing_arccosine() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["0", "acos"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), evaluate_expr::deg_to_rad(90.0));
}

#[test]
fn testing_arccosine_out_of_domain() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["+5", "acos"];

    assert!(
        evaluate_expr::evaluate_expression(
            tokens.clone(),
            &mut stack_vec
        ) == false
    );
}

#[test]
fn testing_arctangent() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["1", "atan"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    assert_eq!(stack_vec.last().cloned().unwrap(), evaluate_expr::deg_to_rad(45.0));
}

#[test]
fn testing_arctangent_out_of_domain() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["270", "atan"];

    assert!(
        evaluate_expr::evaluate_expression(
            tokens.clone(),
            &mut stack_vec
        ) == false
    );
}

// Stack operations
#[test]
fn testing_stack_pushing() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["13", "3", "100.55", "-40", "75"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );

    for item in tokens.iter().zip(stack_vec.iter()) {
        let expected: f64 = item.0.parse::<f64>().unwrap();
        let actual: f64 = *item.1;
        assert_eq!(expected, actual);
    }
}

#[test]
fn testing_stack_erasing() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["13", "3", "100.55", "-40", "75"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );

    evaluate_expr::evaluate_expression(
        vec!["c"],
        &mut stack_vec
    );
    
    assert!(stack_vec.is_empty());
}

#[test]
fn testing_duplicate() {
    let mut stack_vec: Vec<f64> = Vec::new();
    let tokens: Vec<&str> = vec!["1", "d"];

    evaluate_expr::evaluate_expression(
        tokens.clone(),
        &mut stack_vec
    );
    
    assert_eq!(stack_vec, vec![1.0, 1.0]);
}
