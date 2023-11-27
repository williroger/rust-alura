fn soma(a:i32, b:i32) -> i32 {
    println!("{} + {} = {}", a, b, a+b);
    a + b //a expresao que quero retornar sem o ponto e virgula
}

fn escopo() {
   // println!("Ola mundo!");
    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    let a = 123;
    fn  segundo_escopo(){
        let a = 456;
        println!("Var a do segundo Escopo = {}", a)
    }
    println!("Var a do escopo = {}", a);
    segundo_escopo();
}
fn main(){
    escopo();
    println!("Soma = {}", soma(2, 3));
}