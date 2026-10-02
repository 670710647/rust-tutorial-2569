fn main() {
    let medals = vec![
        ("Sepak Takraw", 4), 
        ("Esports (RoV)", 1), 
        ("Running away from soi dogs", 5), 
        ("Muay Thai", 3)
    ];
    
    let total_gold = medals.iter().fold(2, |acc, &(_, count)| acc + count);
    
    println!("Thailand total gold medals won: {} medals", total_gold);
}