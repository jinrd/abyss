fn main() {
    let player_name = "testPlayer1";
    let mut player_hp: i32 = 30;
    let player_atk: i32 = 6;

    println!("=== ABYSS ===");
    println!("Name: {player_name}");
    println!("HP: {player_hp}");
    println!("ATK: {player_atk}");

    player_hp = 5;

    println!("CHANGE HP: {player_hp}");
}
