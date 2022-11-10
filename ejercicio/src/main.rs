use std::io::stdin;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Default)]
struct Persona{ 
    nombre: String,
    edad: u8,
    rut: String,
    mascota: [Mascota; 5]
}
// Creación de estrcutura

#[derive(Default)]
struct Mascota{
    nombre: String, 
    tipo: String,
    color: String,
}
// Creación de sub-estrcutura

fn read_file(mut file: &File){
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}
// Muestra el texto del archivo

fn have_file(mut file: &File) -> String{
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    return text
}
// Toma el texto del archivo

fn create_blank_file(path: &Path){ 
    let _file = File::create(path).expect("El archivo no pudo crearse");
}
// Crea el archivo si no existe

fn open_file_to_append(path: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(path){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };
    //
    return file
}
// Abre el archivo para escribir en el

fn open_file_to_read(path: &Path){ 
    if Path::new(path).exists(){
        let file = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file)
    } else {
        create_blank_file(path);
        println!("Se creo el archivo");
    }
}
// Verifica si existe

fn open_file_to_have(path: &Path) -> String{ 
    if Path::new(path).exists(){
        let file = match File::open(&path){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        let text = have_file(&file);
        return text;
    } else {
        create_blank_file(path);
        return "Se creo el archivo".to_string();
    }
}
// Verifica si existe2

fn create_personas() -> [Persona;3]{ 
    // Primera persona
    let m1 = Mascota{nombre:"Nora".to_string(),
        tipo:"Perro".to_string(),
        color:"Negro".to_string()
    };
    //
    let mut p1 = Persona{nombre:"Sofia".to_string(),
        edad: 18,
        rut: "77777777-7".to_string(),
        mascota: Default::default()
    };
    //
    p1.mascota[0] = m1;
    //
    // Segunda Persona
    let m1 = Mascota{nombre:"Goten".to_string(),
        tipo:"Gato".to_string(),
        color:"Cafe".to_string()
    };
    //
    let mut p2 = Persona{nombre:"Alejandro".to_string(),
        edad: 19,
        rut: "22222222-2".to_string(),
        mascota: Default::default()
    };
    //
    p2.mascota[0] = m1;
    //
    // Tercera Persona
    let m1 = Mascota{nombre:"Blue".to_string(),
        tipo:"Pajaro".to_string(),
        color:"Azul".to_string()
    };
    //
    let m2 = Mascota{nombre:"Green".to_string(),
        tipo:"Pajaro".to_string(),
        color:"Verde".to_string()
    };
    //
    let mut p3 = Persona{nombre:"Marcelo".to_string(),
        edad: 19,
        rut: "55555555-5".to_string(),
        mascota: Default::default()
    };
    //
    p3.mascota[0] = m1;
    p3.mascota[1] = m2;
    //
    let registro: [Persona;3] = [p1, p2, p3];
    //
    return registro;
}
// Crea el array de personas

fn register_persona(path: &Path, registro: [Persona;3]){
    for persona in 0..3{
        let mut temp = format!("{}:{}:{}",  registro[persona].nombre, 
                                                    registro[persona].edad, 
                                                    registro[persona].rut
        );
        //
        for mascota in 0..5 {
            if mascota != 4 && registro[persona].mascota[mascota].nombre != registro[persona].mascota[mascota + 1].nombre{
                temp = format!("{}:{}:{}:{}",   temp,
                                                registro[persona].mascota[mascota].nombre, 
                                                registro[persona].mascota[mascota].tipo, 
                                                registro[persona].mascota[mascota].color, 
                );
            } else {
                temp = format!("{}:\n", temp);
                break;
            }
        }
        //
        let mut file = open_file_to_append(path);
        file.write_all(temp.as_bytes()).unwrap();
    }
}
// Genera datos

fn add_texto(path: &Path) -> bool{
    open_file_to_read(path);
    let registro: [Persona;3] = create_personas();
    register_persona(path, registro);
    println!("Se han registrado datos");
    return true
}
// Para agregar texto

fn match_persona(line: &str) -> Persona{
    let mut p_temp:Persona = Default::default();
    let mut contador = 0;
    //
    for word in line.split(":"){
        if contador == 0 {
            p_temp.nombre = word.to_string();
        } else if contador == 1 {
            p_temp.edad = word.parse().unwrap();
        } else if contador == 2 {
            p_temp.rut = word.to_string();
        } else if contador == 3 {
            p_temp.mascota[0].nombre = word.to_string();
        } else if contador == 4 {
            p_temp.mascota[0].tipo = word.to_string();
        } else if contador == 5 {
            p_temp.mascota[0].color = word.to_string();
        } else if contador == 6 {
            p_temp.mascota[1].nombre = word.to_string();
        } else if contador == 7 {
            p_temp.mascota[1].tipo = word.to_string();
        } else if contador == 8 {
            p_temp.mascota[1].color = word.to_string();
        }
        contador += 1;
    }
    //
    return p_temp;
}
//Asigna los datos del archivo a la array

fn make_array(text: &str) -> [Persona;3]{
    let mut p1:Persona = Default::default();
    let mut p2:Persona = Default::default();
    let mut p3:Persona = Default::default();
    let mut contador = 0;
    //
    for line in text.split("\n"){
        let p_temp:Persona = match_persona(line);
        //
        if contador == 0 {
            p1 = p_temp;
        } else if contador == 1{
            p2 = p_temp;
        } else if contador == 2 {
            p3 = p_temp;
        }
        //
        contador += 1;
    }

    return [p1,p2,p3];
}
//Hace array

fn menu_2() -> String{
    println!("Elija una persona\n[1] Sofía\n[2] Alejandro\n[3] Marcelo");
    let mut numero = String::new();
    stdin().read_line(&mut numero).unwrap();
    return numero;
}
//menu para elegir las personas

fn show_persona(num: &str, array:[Persona;3]) -> [Persona;3]{
    if num == "1"{
        println!("{} tiene un {} llamado {}", array[0].nombre, array[0].mascota[0].tipo, array[0].mascota[0].nombre)
    } else if num == "2" {
        println!("{} tiene un {} llamado {}", array[1].nombre, array[1].mascota[0].tipo, array[1].mascota[0].nombre)
    } else if num == "3" {
        println!("{} tiene un {} llamado {} y un {} llamado {}", array[2].nombre, array[2].mascota[0].tipo, array[2].mascota[0].nombre, array[2].mascota[1].tipo, array[2].mascota[1].nombre)
    }
    return array;
}
//Muestra en consola los datos de la array

fn read_texto(path: &Path) -> bool {
    let mut _romper = false;
    let text = open_file_to_have(path);
    //
    loop{
        let _array:[Persona;3] = make_array(&text);
        println!("{}", _array[2].nombre);
        let num = menu_2();
        _romper = match num.trim() {
            "1"|"2"|"3" => true,
            _ => break
        };
        //
        let _array = show_persona(&num.trim(), _array);
        break;
    }
    //
    return true
}
// Para leer texto

fn menu() -> String{
    println!("Elija un valor.\n[1] Agregar texto\n[2] Leer texto");
    let mut numero = String::new();
    stdin().read_line(&mut numero).unwrap();
    return numero;
}
// Menu para decidir que hacer

fn main(){
    let path = Path::new("ejemplo.txt");
    let mut _romper = false;
    println!("Bienvenido.");
    //
    loop {
        let numero = menu();
        _romper = match numero.trim() {
            "1" => add_texto(path),
            "2" => read_texto(path),
            _ => false
        };
        //
        if _romper {
            break;
        }
    }
}
