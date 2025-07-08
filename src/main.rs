use std::{io::{self,Write}, path::Path, str::FromStr};
use umya_spreadsheet::{new_file,Spreadsheet};

// modulos

mod funciones;

fn main() {

    print!("Ingrese el ID del proyecto: ");
    io::stdout().flush().unwrap();
    let mut proyecto_id = String::new();
    io::stdin().read_line(&mut proyecto_id).unwrap();
    let proyecto_id = proyecto_id.trim().to_string();

    print!("Ingrese el nombre de la red VPC: ");
    io::stdout().flush().unwrap();
    let mut red_vpc = String::new();
    io::stdin().read_line(&mut red_vpc).unwrap();
    let red_vpc = red_vpc.trim().to_string();

    println!("Ingrese el directorio a guardar el archivo excel junto el nombre del libro");
    io::stdout().flush().unwrap();
    let mut directorio = String::new();
    io::stdin().read_line(&mut directorio).unwrap();
    let ubicacion = Path::new(directorio.trim());

    // creacion del excel
    // let direccion_directorio = std::path::Path::new(directorio);

    let nombre_libro: String = format!("Servicios_red_{}_{}",proyecto_id,red_vpc);
    let mut book = new_file();
    


    // llamamos las funciones para buscar los recursos y añadirles al libro de excel
        
    funciones::write_compute_sheet(&mut book,&proyecto_id,&red_vpc);
    funciones::write_gke_sheet(&mut book, &proyecto_id,&red_vpc);

    // Escribimos el libro de excel

    match umya_spreadsheet::writer::xlsx::write(&book, ubicacion) {
        Ok(_) => println!("Libro de excel guardado excitosamente en: {}", ubicacion.display()),
        Err(e) => eprintln!("Error al guardar el libro: {}", e),
    }


    println!("Archivo Excel generado: recursos_gcp.xlsx")
}
