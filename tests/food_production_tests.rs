use advent_of_code_2023::food_production::MapEntry;

#[test]
fn test_filter_ranges() {
    let mut ranges = vec![(10, 20), (30, 40)];
    let entry = MapEntry::new(25, 15, 10);
    let transformed = entry.filter_ranges(&mut ranges);
    assert_eq!(transformed, vec![(25, 10)]);
    assert_eq!(ranges, vec![(10, 5), (25, 5), (30, 40)]);
}