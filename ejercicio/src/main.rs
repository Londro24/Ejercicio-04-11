use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

//creación de estrcutura
#[derive(Default)]
struct Persona{
    nombre: String,
    edad: u8,
    rut: String,
    mascota: [Mascota; 5]
}

//creación de sub-estrcutura
#[derive(Default)]
struct Mascota{
    nombre: String, 
    tipo: String,
    color: String,
}


fn read_file(mut f: &File){ //Muestra el texto del archivo
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}


fn create_blank_file(p: &Path){ //Crea el archivo si no existe
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}


/* //Añade texto
fn add_new_content(mut f: &File){ 
    f.write_all(b"Nuevo texto\n");
} 
*/


fn open_file_to_append(p: &Path) -> File{ //Abre el archivo para escribir en el
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    return file
}


fn open_file_to_read(p: &Path){ //Verifica si existe
    if Path::new(p).exists(){
        let file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
        read_file(&file)
    } else {
        create_blank_file(p);
    }
}


fn crear_personas() -> [Persona;3] { // Crea el array de personas
    // Primera persona
    let m1 = Mascota{nombre:"Nora".to_string(),
                                tipo:"Perro".to_string(),
                                color:"Negro".to_string()
    };

    let mut p1 = Persona{nombre:"Sofia".to_string(),
                                    edad: 18,
                                    rut: "77777777-7".to_string(),
                                    mascota: Default::default()
    };

    p1.mascota[0] = m1;

    // Segunda Persona
    let m1 = Mascota{nombre:"Goten".to_string(),
                                tipo:"Gato".to_string(),
                                color:"Cafe".to_string()
    };

    let mut p2 = Persona{nombre:"Alejandro".to_string(),
                                    edad: 19,
                                    rut: "22222222-2".to_string(),
                                    mascota: Default::default()
    };

    p2.mascota[0] = m1;

    // Tercera Persona
    let m1 = Mascota{nombre:"Blue".to_string(),
                                tipo:"Pajaro".to_string(),
                                color:"Azul".to_string()
    };

    let m2 = Mascota{nombre:"Green".to_string(),
                                tipo:"Pajaro".to_string(),
                                color:"Verde".to_string()
    };

    let mut p3 = Persona{nombre:"Marcelo".to_string(),
                                    edad: 19,
                                    rut: "55555555-5".to_string(),
                                    mascota: Default::default()
    };

    p3.mascota[0] = m1;
    p3.mascota[1] = m2;

    let registro: [Persona;3] = [p1, p2, p3];

    return registro;
}


fn registrar_persona(p: &Path, registro: [Persona;3]){ //Genera datos
    for persona in 0..3{
        let mut temp = format!("{}:{}:{}", registro[persona].nombre, 
        registro[persona].edad, 
        registro[persona].rut
        );

        for mascota in 0..5 {
            if mascota != 4 && registro[persona].mascota[mascota].nombre != registro[persona].mascota[mascota + 1].nombre{
                temp = format!("{}:{}:{}:{}", temp,
                registro[persona].mascota[mascota].nombre, 
                registro[persona].mascota[mascota].tipo, 
                registro[persona].mascota[mascota].color, 
                );
            } else {
                temp = format!("{}:\n", temp);
                break;
            }

        }
    
        let mut file = open_file_to_append(p);
        file.write_all(temp.as_bytes());
    }
}


fn main(){
    let path = Path::new("ejemplo.txt");
    open_file_to_read(path);
    //let file = open_file_to_append(path);
    //add_new_content(&file);
    let registro: [Persona;3] = crear_personas();
    registrar_persona(path, registro);
}
