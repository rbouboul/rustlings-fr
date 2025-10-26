// Ceci est un quiz pour les sections suivantes:
// - Variables
// - Functions (Fonctions)
// - If
//
// Marie achète des pommes. Le prix d'une pomme est calculé comme suit:
// - Une pomme coûte 2 rustbucks.
// - Cependant, si Marie achète plus de 40 pommes, le prix de chaque pomme dans
// la commande entière est réduit à seulement 1 rustbuck!

// TODO: Écris une fonction qui calcule le prix d'une commande de pommes en fonction
// de la quantité achetée.
// fn calculate_price_of_apples(???) -> ??? { ??? }
fn calculate_price_of_apples(apples_number: i32) -> i32 {
    if apples_number <= 40 {
        apples_number * 2
    } else {
        apples_number
    }
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
    println!("{}", calculate_price_of_apples(35))
}

// Ne modifie pas les tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
