use advent_of_code_2023::camel_cards::{Hand, HandRank};

#[test]
fn test_hand_rank() {
    let test_cases = vec![
        ("AAAAA 100", HandRank::FiveOfAKind('A')),
        ("AAAA1 100", HandRank::FourOfAKind('A')),
        ("AAA11 100", HandRank::FullHouse('A', '1')),
        ("AA111 100", HandRank::FullHouse('1', 'A')),
        ("A1111 100", HandRank::FourOfAKind('1')),
        ("11111 100", HandRank::FiveOfAKind('1')),
        ("1111A 100", HandRank::FourOfAKind('1')),
        ("111AA 100", HandRank::FullHouse('1', 'A')),
        ("11AAA 100", HandRank::FullHouse('A', '1')),
        ("1AAAA 100", HandRank::FourOfAKind('A')),
        ("23456 100", HandRank::HighCard),
        ("22345 100", HandRank::OnePair('2')),
        ("22344 100", HandRank::TwoPairs('4', '2')),
        ("22333 100", HandRank::FullHouse('3', '2')),
        ("22233 100", HandRank::FullHouse('2', '3')),
        ("22222 100", HandRank::FiveOfAKind('2')),
        ("222AA 100", HandRank::FullHouse('2', 'A')),
        ("22AAA 100", HandRank::FullHouse('A', '2')),
        ("2AAAA 100", HandRank::FourOfAKind('A')),
        ("AA234 100", HandRank::OnePair('A')),
    ];

    for (input, expected) in test_cases {
        let hand = Hand::from_string(input).unwrap();
        assert_eq!(hand.get_rank(), &expected);
    }

}
