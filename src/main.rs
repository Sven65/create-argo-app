use dialoguer::{theme::ColorfulTheme, MultiSelect, Input};
use std::{fs, process::{self}, path::PathBuf, env::{self}};

use crate::ResourceType::CreationGetter;

pub mod ResourceType;
pub mod resources;


fn prepare_fs(app_path: &String) -> std::io::Result<()>{
    fs::create_dir_all(app_path)?;
    Ok(())
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

fn main() {
    let appName: String = Input::with_theme(&ColorfulTheme::default())
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
        ResourceType::ResourceType::Deployment,
        ResourceType::ResourceType::PV,
        ResourceType::ResourceType::PVC,
        ResourceType::ResourceType::Deployment,
        ResourceType::ResourceType::Service,
        ResourceType::ResourceType::IngressRoute,
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
            let resType: ResourceType::ResourceType = selected[selection].try_into().unwrap();

            let creator = resType.to_owned().get_creator();

            let creatorType = creator.get_resource_type();

            creatorType.get_creator().create_resource(&appName, &app_directory);
        }
    }
}
