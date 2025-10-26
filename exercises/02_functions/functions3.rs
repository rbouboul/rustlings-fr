fn call_me(num: u8) {
    for i in 0..num {
        println!("Dring ! Appel numéro {}", i + 1);
    }
}

fn main() {
    // TODO: Corrige l'appel de fonction.
    call_me(3);
}
