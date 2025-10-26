fn main() {
   // TODO: Crée un tableau appelé `a` avec au moins 100 éléments.
   // let a = ???
   let a = [6; 100];

   if a.len() >= 100 {
       println!("Wow, c'est un grand tableau !");
   } else {
       println!("Bah, je mange des tableaux comme ça au petit-déjeuner.");
       panic!("Tableau pas assez grand, plus d'éléments nécessaires");
   }
}
