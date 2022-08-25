//ownership es como la estrategia que usa Rust para el manejo de la memoria
//Rust no usa un collector de basura o deja en la limpieza explicita en manos del programador
//uno de los fundamentos de ownership es que la memoria se libera cuando se sale del scope en el que se ha declarado.
//En ese aspecto es similar a RAII de C++.
//Ademas de que aborda el problema del double free, cuando se intenta limpiar una misma localidad en memoria por mas de una variable
//ya que en lugar de usar un shallow copy (pase por referencia), invalida la primer referencia y deja solo la ultima asignacion como referencia valida
//de tal manera que solo existe un owner, esto es especialmente optimo en tipos que van al heap.
//en cuanto a las deep copy (pase por valor), se realizan en los tipos escalares, compuestos y llamadas de funciones principalmente
//ya que estos se alojan en el stack y Rust crea una copia cuando se asignan a otro miembro, por lo cual no se invalida la primer referencia, se crea una nueva.


//Recordemos que el Stack y el heap son dos estructuras disponibles en la memoria
//el stack es rapido, pero los datos deben de tener un tamaño fijo y conocido durante el tiempo de compilacion
//usa logica LIFO.
//en caso de que el tamaño del espacio que requiere un dato no es fijo o conocido en tiempo de compilacion y/o
//se definira o sera variable durante el tiempo de ejecucion, la memoria que se utilizara sera la del heap.
//El heap es menos organizado, no tiene una logica LIFO, usa punteros o referencias a los espacios de memoria asignados,
//Ya que el heap funciona mediante un allocator que asigna un espacio lo suficientemente grande para alamacenar la informacion.
//El heap es mas costoso que el stack en rendimienti, pero en el stack solo podemos almacenar informacion de tamaño conocido y fijo.

//Reglas del ownership
//En rust cada valor tiene un dueño
//Solo puede haber un dueño de un valor a la vez
//Cuando el dueño sale de su scope, el valor sera liberado.

fn main() {
    //String es un tipo perfecto para ver el funcionamiento del ownership.
    //los string literales son inmutables, pero las string variables pueden ser mutables

    {
        //scope
        //saludo solo es valido aqui
        let mut saludo = String::from("Hey"); //creacion de un String desde una literal
        saludo.push_str(" crustaceans!"); //al ser mutable se puede cambiar el valor de saludo.
        println!("{}",saludo);

        //en todo el scope podemos usar saludo.
    } // despues de este punto se libera la memoria asignada a saludo. Rust usa la funcion drop automaticamente al finalizar el scope

    //Formas en que las variables y los datos interactuan

    //al asignar una variable a otra es importante saber donde se almacena
    //por que de ellos depende si se copiara o movera

    //copiar

    //en el caso de un tipo escalar, se realiza una copia del valor, por lo que se crea una nueva locacion de memoria (en el stack)
    //se crea un nuevo owner de los datos copiados

    let x = 1;
    let y = x;

    //x es owner de una localidad de memoria, distinta a la localidad de la que y es owner, aunque ambas localidades almacenan el valor (reprentacion) 1.
    //por lo tanto x se puede serguir usando, por que x aun es owner de su informacion
    println!("El valor de x es {x}");

    //mover 

    //cuando las operaciones se realizan sobre tipos mas complejo que se almacenan en el heap
    //la asignacion no copia el valor, lo mueve (invalida la primera referencia)
    let s1 = String::from("Rust is so cool!"); //s1 es el owner de la informacion
    let s2 = s1; //no copia el valor, cambia el owner a s2, por lo que s1 es invalido y ya no se puede usar
    //println!("El valor de s1 es {s1}"); este codigo no compila por que s1 ya no es owner
    println!("El valor de s2 es {s2}");

    //lo que conforma a este tipo de variables es que estan conformadas de 3 partes
    //un puntero (ptr), la longitud (len) y la capacidad (capacity)
    //esta informacion permite encontrar y gestionar la informacion que se encuentra en el heap
    //este grupo de datos se almacena en el stack (ya que su tamaño es conocido y limitado) y
    //la informacion a la que hacen referencia estos datos se alamacena en el heap.

    //los tipos complejos implementan drop para hacer el cambio de owner (mover)
    //los tipos escalares y compuestos (cuando almacenan tipos escalares) implementan la anotacion copy, que hace que la asignacion resulte en una copia trivial (copiar)
    //los tipos que implementan drop, no pueden ser anotados con copy.

    //clonar

    //En caso de que la informacion de una variable se quiera copiar y no mover se puede usar el metodo clone().
    //clone copia la informacion el ownership se asigna a la copia.
    //Se debe de considerar que hacer esto puede ser una operacion costosa para el rendimiento y juega un papel importante la cantidad de memoria

    let s1 = String::from("Hey again rustaceans!");
    let s2 = s1.clone(); //copia el valor de s1 y lo copia en una nueva locacion, el owner de esa copia es s2
    //s1 no ha perdido au ownership, solo fue clonada
    println!("El valor de s1 es {} y el valor de s2 es {}", s1,s2);

    //Ownership y funciones

    //Cuando una variable recibe un valor como argumento, este valor se va a copiar o mover y su nuevo scope sera la funcion
    //con los tipos escalares se copia, por lo que el owner que esta en la funcion tiene una copia
    //para los tipos complejos como String, se realiza una operacion de mover, por lo que el nuevo owner es el que esta en el scope de la funcion,
    //por lo que el primer owner que invalido (el que se mando como argumento).

    //s3 es el owner
    let s3 = String::from("una cadena cualquiera");

    //al mandar s3 como argumento pierde su ownership y se lo pasa al parametro de la funcion (s).
    takes_ownership(s3);

    //aqui ya no podemos usar s3, por que perdio su ownership.
    //println!("{s3}"); //no compila

    //num es el owner de el valor 2 (datos de la representacion)
    let n = 2;

    //al mandar n (tipo escalar) se hace una copia solo del valor y no pierde su ownership
    //el parametro num tiene una copia del valor del argumento n (2), pero esta copia esta en otra locacion de memoria y num es su ownership
    makes_copy(n);

    //como n no perdio su ownership, aun es una variable valida
    println!("El valor de n es {n}");

    //valores de retorno
    //cuando un valor se retorna de una funcion el owner de este valor, pierde su ownership y puede ser asignado desde donde se lamo la funcion

    //s4 toma el ownership del valor retornado por la funcion
    let s4 = gives_ownership(); 

    let s5 = String::from("Let's get rusty!"); //s5 es el owner

    //s5 pierde el ownership y lo gana el parametro de la funcion (sent_string)
    //la funcion retorna esa variable por lo que sent_string pierde el ownership
    //como el valor retornado se asigna a ese s6, esta variable es la que gana el ownership que originalmente tenia s5.
    let s6 = takes_and_gives_ownership(s5);

    //s4 y s6 son owners de valores, pero s5 perdio su ownership por que fue movido.

    //retorno de valores sin perder el ownership de las variables enviadas como parametros
    //Ya que siempre que usemos tipos complejos como argumentos de funciones estos perderan su ownership
    //Rust nos permite retornar tuples, donde se puede retornar el valor del argumento y volver a asignarlo a su owner original.
    let s7 = String::from("Rust is cool");
    //al pasar s7 a la funcion, s7 pierde su ownership
    //pero la funcion retorna ese valor (en la tuple retornada) para ser asignado a s8 (se podria asignar a s7 con shadowing)
    //basta con usar deconstruccion.
    let (s8, len) = calculate_length(s7); //s7 queda sin ownership por que queda en s8 y el resultado se la funcion tiene como owner a len

    println!("El String {} tiene una longitud de {} caracteres", s8, len);
    //Aunque este proceso nos permite no perder el ownership, es poco practico
    //Afortunadamente Rust tiene un mecanismo para no transferir el ownership y es mediante referencias.


    
} // s5 y s7 fueron movidos, por lo que al terminar el scope, a s4,s6,s8 y len se les hace drop.

//En el momento que un string se pasa como argumento, el owner desde donde se llamo la funcion pierde el ownership y pasa a s
fn takes_ownership(s: String){
    println!("s es el owner del String [{s}]");
} //s pierde contexto y se le hace drop

//En este caso al ser el parametro de tipo escalar, cuando pasemos un valor como argumento no se cambia de owner, se hace una copia.
//Por lo que el owner del valor desde donde se llamo la funcion no pierde el ownership.

fn makes_copy(num: u32){
    println!("num tiene el valor de {num}, pero es una copia de la variable envida")
}//pierde contexto num, pero la variable que se uso como argumento sigue disponible desde su contexto.

fn gives_ownership() -> String{
     let dummy_string = String::from("where do I own ?");
     dummy_string //retorna el valor de dummy_string por lo que pierde su owner
}

fn takes_and_gives_ownership(sent_string: String) -> String {
    //la variable que haya mandado como argumento ya perdio el ownership sobre el valor
    //ahora le pertenece a sent_string
    sent_string //retornamos el valor de sent_string, con lo que este pierde el ownership y puede ser recuperado desde donde se llamo la funcion.
}

//indicamos que devolveremos una tuple que devuelve la cadena enviada y el tamaño
fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s,length) // devolvemos el valor que se recibio y el resultado.
}

