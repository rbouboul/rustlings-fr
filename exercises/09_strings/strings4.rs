// Les appels à cette fonction devraient être remplacés par des appels à `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
   println!("{arg}");
}

fn string(arg: String) {
   println!("{arg}");
}

// TODO: Voici plusieurs valeurs - certaines sont `String`, d'autres sont `&str`.
// Ta tâche est de remplacer `placeholder(…)` par soit `string_slice(…)`
// soit `string(…)` selon ce que tu penses que chaque valeur est.
fn main() {
   string_slice("blue");

   string("red".to_string());

   string(String::from("hi"));

   string("rust is fun!".to_owned());

   string("nice weather".into());

   string(format!("Interpolation {}", "Station"));

   // ATTENTION: Ceci est une indexation d'octets, pas une indexation de caractères.
   // L'indexation de caractères peut être faite en utilisant `s.chars().nth(INDEX)`.
   string_slice(&String::from("abc")[0..1]);

   string_slice("  hello there ".trim());

   string("Happy Monday!".replace("Mon", "Tues"));

   string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
