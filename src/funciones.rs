use serde::Deserialize;
use std::fmt::format;
use std::io::{self, Write};
use std::process::Command;
use umya_spreadsheet::Spreadsheet;

// ----------------------------- NETWORK INTERFACE  --------------------------------
#[derive(Debug, serde::Deserialize)]
struct NetworkInterface {
    network: Option<String>,
    subnetwork: Option<String>,
    network_ip: Option<String>,
}

// ----------------------------- COMPUTE ENGINE  --------------------------------
#[derive(Debug, serde::Deserialize)]
struct ComputeInstance {
    name: String,
    zone: String,
    network: String,
    subnetwork: String,
    network_ip: String,
    network_interfaces: Vec<NetworkInterface>,
}

// ----------------------------- GKE CLUSTER  --------------------------------

#[derive(Debug, serde::Deserialize)]
struct GKECluster {
    name: String,
    location: String,
    network: String,
    subnetwork: String,
}


// ----------------------------- Subnetwork   --------------------------------

#[derive(Debug, serde::Deserialize)]
struct Subnetwork {
    name: String,
    range: String,
    region: String,
}

// ----------------------------- FUNCIONES  --------------------------------

fn get_gcloud_output(args: &[&str]) -> Option<String> {
        let output = Command::new("gcloud")
        .args(args)
        .output()
        .expect("Failed to run gcloud");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // checar si la api esta deshabilitada
        
        let api_disabled_messages = [
            "has not been used in project",
            "is disabled",
            "API not enabled",
            "no ha sido utilizada en el proyecto",
            "está deshabilitada",
            "API no habilitada",
        ];

        let permission_denied_messages = [
            "403 Forbidden",
            "Permission denied",
            "The caller does not have permission",
            "Permiso denegado",
            "El llamador no tiene permiso",
            "insufficent permissions",
            "permisos insuficientes",
            "required 'serviceusage.services.use' permission",
            "requiere el permiso 'serviceusage_services.use'"
        ];

        let mut handled = false;

        if api_disabled_messages.iter().any(|msg| stderr.contains(msg)) {
            println!("API deshabilitada para: gcloud {}", args.join(" "));
            println!(" -> Este servicio será omitido automáticamente.");
            handled = true;
        }
        else if permission_denied_messages.iter().any(|msg| stderr.contains(msg)) {
            println!("Accesso denegado (403 Forbidden) o permisos insuficientes para: gcloud {}", args.join(" "));
            println!(" -> Este servicio será omitido o no se podrá acceder. Por favor, verifica tus permisos.");
            handled = true;
        }

        if handled {
            return None;
        } else {
            println!("Error inesperado ejecutando gcloud {}: {}", args.join(" "), stderr);
            return None;
        }
    }

    Some(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn write_compute_sheet(book: &mut Spreadsheet, proyecto: &str, vpc: &str) {
    let filtro = format!("--filter=networkInterfaces.network:{}", vpc);
    let filtro_proyecto = format!("--project={}",proyecto); 
    let args = vec![
         "compute",
        "instances",
        "list",
        &filtro,
        &filtro_proyecto,
        "--format=json",
    ];

    let json = get_gcloud_output(&args).unwrap();
    let instances: Vec<ComputeInstance> = serde_json::from_str(&json).unwrap();

    let sheet_name = "ComputeEngine";
    book.new_sheet(sheet_name);

    let sheet_to_modify = book.get_sheet_by_name_mut(sheet_name).unwrap();
    sheet_to_modify.get_cell_mut((0,0)).set_value("Name");
    sheet_to_modify.get_cell_mut((1,0)).set_value("Zone");
    sheet_to_modify.get_cell_mut((2,0)).set_value("Network");
    sheet_to_modify.get_cell_mut((3,0)).set_value("Subnetwork");
    sheet_to_modify.get_cell_mut((4,0)).set_value("Network Ip");



    for (i, instance) in instances.iter().enumerate() {
            let fila = (i + 1) as u32;
            sheet_to_modify.get_cell_mut((0,fila)).set_value(instance.name.as_str());
            sheet_to_modify.get_cell_mut((1,fila)).set_value(instance.zone.as_str());
            sheet_to_modify.get_cell_mut((2,fila)).set_value(instance.network.as_str());
            sheet_to_modify.get_cell_mut((3,fila)).set_value(instance.subnetwork.as_str());
            sheet_to_modify.get_cell_mut((4,fila)).set_value(instance.network_ip.as_str());
    };
}

pub fn write_gke_sheet(book: &mut Spreadsheet, proyecto: &str, vpc:  &str) {
    let filtro = format!("--filter=networkInterfaces.network:{}", vpc);
    let filtro_proyecto = format!("--project={}",proyecto);
    let args = vec![
        "container",
        "clusters",
        "list",
        &filtro,
        &filtro_proyecto,
        "--format=json",
    ];

    let default_json_output = "[]".to_string(); // O "{}" si esperas un objeto
    let json = get_gcloud_output(&args).unwrap_or(default_json_output);

    let clusters: Vec<GKECluster> = serde_json::from_str(&json).unwrap();

    let sheet_name = "GKE";
    book.new_sheet(sheet_name);
    
    let sheet_to_modify = book.get_sheet_by_name_mut(sheet_name).unwrap();
    sheet_to_modify.get_cell_mut((0,0)).set_value("Name");
    sheet_to_modify.get_cell_mut((1,0)).set_value("Location");
    sheet_to_modify.get_cell_mut((2,0)).set_value("Network");
    sheet_to_modify.get_cell_mut((3,0)).set_value("Subnetwork");



    for (i, cluster) in clusters.iter().enumerate() {
            let fila = (i + 1) as u32;
            sheet_to_modify.get_cell_mut((0,fila)).set_value(cluster.name.as_str());
            sheet_to_modify.get_cell_mut((1,fila)).set_value(cluster.location.as_str());
            sheet_to_modify.get_cell_mut((2,fila)).set_value(cluster.network.as_str());
            sheet_to_modify.get_cell_mut((3,fila)).set_value(cluster.subnetwork.as_str());
    };
}
    
pub fn write_subnetwork_sheet(book: &mut Spreadsheet, proyecto: String, vpc:  &str) {
    let filtro = format!("--filter=networkInterfaces.network:{}", vpc);
    let filtro_proyecto = format!("--project={}",proyecto);
    let args = vec![
        "container",
        "clusters",
        "list",
        &filtro,
        &filtro_proyecto,
        "--format=json",
    ];
    let json = get_gcloud_output(&args).unwrap();
    let clusters: Vec<GKECluster> = serde_json::from_str(&json).unwrap();

    let sheet_name = "GKE";
    book.new_sheet(sheet_name);
    
    let sheet_to_modify = book.get_sheet_by_name_mut(sheet_name).unwrap();
    sheet_to_modify.get_cell_mut((0,0)).set_value("Name");
    sheet_to_modify.get_cell_mut((1,0)).set_value("Location");
    sheet_to_modify.get_cell_mut((2,0)).set_value("Network");
    sheet_to_modify.get_cell_mut((3,0)).set_value("Subnetwork");



    for (i, cluster) in clusters.iter().enumerate() {
            let fila = (i + 1) as u32;
            sheet_to_modify.get_cell_mut((0,fila)).set_value(cluster.name.as_str());
            sheet_to_modify.get_cell_mut((1,fila)).set_value(cluster.location.as_str());
            sheet_to_modify.get_cell_mut((2,fila)).set_value(cluster.network.as_str());
            sheet_to_modify.get_cell_mut((3,fila)).set_value(cluster.subnetwork.as_str());
    };

}

// gcloud compute networks subnets list --network vpc




