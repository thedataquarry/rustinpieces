#![cfg(test)]

#[test]
fn test_revenue_range() {
    let (revenue_lower, revenue_upper) = super::calculate_range("$1.5M-$2.5M");
    assert!(revenue_lower > 0.0);
    assert!(revenue_lower < revenue_upper);
}

#[test]
fn test_run() {
    let result = super::run();
    assert_eq!(result[0].annual_revenue_lower, 10000000.0);
    assert_eq!(result[0].annual_revenue_upper, 20000000.0);
    assert_eq!(result[1].annual_revenue_lower, 7500000.0);
    assert_eq!(result[1].annual_revenue_upper, 8500000.0);
    assert_eq!(result[2].annual_revenue_lower, 500000.0);
    assert_eq!(result[2].annual_revenue_upper, 1000000.0);
    assert_eq!(result[3].annual_revenue_lower, 800000000.0);
    assert_eq!(result[3].annual_revenue_upper, 1000000000.0);
}
