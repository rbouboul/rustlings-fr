struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
    // TODO: Ajoute les champs attendus par le test `regular_structs`.
    // Quels types devraient avoir les champs ? Quelles sont les valeurs minimales et maximales pour les couleurs RGB ?
}

struct ColorTupleStruct(u8, u8, u8/* TODO: Ajoute les champs attendus par le test `tuple_structs` */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instancie une struct régulière.
        // let green =
        let green = ColorRegularStruct{red: 0, green: 255, blue: 0};

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instancie une tuple struct.
        // let green =
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instancie une unit struct. 
        // let unit_struct =
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
