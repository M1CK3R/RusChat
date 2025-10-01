# Sintaxis | Rust

## Varibles
En Rust las varibles son inmutables (constantes) por defecto. Si quiero cambiarla o que se cambie en algun momento se debe de dejar indicado 

```rust

// Por ejemplo
let x = 5;  // Este es inmutable (constante)
let mut y = 10; // Este es un mutable (cambiante)

println!("x = {}, y = {}", x , y); 
// Esto imprime "x = 5, y = 10"
// Nota: {} es la forma en la que se pueden imprimir las variables que uno le indique, se imprimen en el orden que se le pasen

y = 80;
// Esto ahora cambia su valor a 80
println!("Ahora y = {}", y);

```

## Tipos de datos
El Rust es un lenguaje fuertemente tipado pero se entienden el tipo en la mayoria de casos

```rust

// Todos son inmutables en este caso
let a: i32 = 42;      // entero de 32 bits
let b: f64 = 3.14;    // número de punto flotante
let c: bool = true;   // booleano
let d: char = 'R';    // carácter
let e = "texto";      // &str (string)

```

## Condicionales
Igual que en JS, solo que sin poner entre parentesis () la condicion

```rust

let a = 2;

if a % 2 == 0 {
    println!("Paaar");
} else {
    println!("Impar");
}

```

## Match
Es practicamente un Switch mejorado

```rust

    match numero {
        1 => println!("Uno"),
        2 | 3 => println!("Dos o tres"),
        4..=10 => println!("Entre 4 y 10"),
        _ => println!("Otro número"), // _ es el "default"
    }

```


## Bucles
Rust, ademas de tener los bucles traadicionales de while y for, tiene un bucle infinito "loop"

```rust

let mut contador = 0;

    // loop infinito
    loop {
        contador += 1;
        if contador == 3 {
            println!("Salgo del loop");
            break;
        }
    }

    // while
    let mut n = 3;
    while n > 0 {
        println!("n = {}", n);
        n -= 1;
    }

    // for
    for i in 0..5 { // rango de 0 a 4
        println!("i = {}", i);
    }

```

## Funciones
Se definen en una misma linea los parametros y el tipo de retorno que van a tener las funciones

```rust

fn suma(a: i32, b: i32) -> i32 {
    a + b // La ultima expresion es el valor de retorno (sin el ;)
}

```

## Metodos y estructuras
En Rust la POO es algo diferente, ya que no existen las clases como tal, sino que se tienen que definir las estructuras y se tiene que indicar que se van a implementar un metodo para estas estructuras

```rust

// Se crea la estructura de datos para persona
struct Persona {
    nombre: String,
    edad: u8,
}

// Se implementa el metodo para saludar de la persona
impl Persona {
    fn saludar(&self) {
        println!("Hola, me llamo {} y tengo {} años", self.nombre, self.edad);
    }
}

// Y algo asi es que se deberia de declarar e invocar estas estructuras
fn main() {
    let p = Persona {
        nombre: String::from("Ana"),
        edad: 25,
    };

    p.saludar();
}

```

## Algunos tipos de estructuras de datos nativas
Se cuenta con algunas estructuras de datos nativas del lenguaje, Tuplas, Arrays y Slices

```rust

// Ejemplo de array estatico que de un tipo especifico [Tipo a guardar; Tamaño]
let array: [i32; 5] = [1, 2, 3, 4, 5]

// EJemplo de una tupla con diferentes tipos de datos

let tupla = (5u22, 1u8, true, "max")

```