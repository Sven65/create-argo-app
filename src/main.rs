use dialoguer::{theme::ColorfulTheme, MultiSelect, Input};
use std::{fs, process::{self}, path::PathBuf, env::{self}};

pub mod resource_type;
pub mod resources;
use crate::resource_type::CreationGetter;

fn prepare_fs(app_path: &String) -> std::io::Result<()>{
    fs::create_dir_all(app_path)?;
    Ok(())
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn main() {
    let app_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter app name")
        .interact_text()
        .unwrap();

    let app_directory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter app location")
        .interact_text()
        .unwrap();



    //let app_directory = format!("./{}", app_directory);

    println!("Creating apps in {}", app_directory);

    let selected = &[
        resource_type::ResourceType::Deployment,
        resource_type::ResourceType::PV,
        resource_type::ResourceType::PVC,
        resource_type::ResourceType::HPA,
        resource_type::ResourceType::Service,
        resource_type::ResourceType::IngressRoute,
    ];

    let defaults = &[false, false, false, false];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you need?")
        .items(&selected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You need to select what you need.");
    } else {
        match prepare_fs(&app_directory) {
            Err(err) => {
                println!("Failed to create files. {}", err);
                process::exit(1);
            }
            _ => {}
        }

        for selection in selections {
            let res_type: resource_type::ResourceType = selected[selection].try_into().unwrap();

            let creator = res_type.to_owned().get_creator();

            let creator_type = creator.get_resource_type();

            creator_type.get_creator().create_resource(&app_name, &app_directory).unwrap();
        }
    }
}
