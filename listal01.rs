/*pub struct node<T> {
    pub next: Option<Box<node<T>>>,
    pub prev: Option<Box<node<T>>>,
    pub value: T,
}

pub struct nodeList<T> {
    pub value: T,
    pub left: Option<Box<<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}*/



fn main(){

    println!("hellooo\n");
//DECLARACION DE VARIABLES:
//las variables al ser declaradas toman un tipo de  dato por defecto
//integer = i32, float = f32, char 4bytes y son representados por unicode.  
//aunque el tipo de variable tambien puede ser especificado.
let  sel = true;//variable tipo booleana por defecto    
let  mut cont = 1;//variable tipo integer por defecto, se debe 
//agregar mut para que la variable puede ser modificada mas adelante
let  conteo: f32 = 3.0;//variable tipo float especificada


    if sel == true{
        println!("verdadero\n");

    }

    if cont >0 {
        println!("El valor es mayor que 0\n");

    }

    if conteo == 3.0{
        println!("6\n");

    }

    //ciclos:
    loop {
        cont += 1;//incremento de contador

        if cont == 3 {
            println!("el valor es 3");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}",cont);

        if cont == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

}