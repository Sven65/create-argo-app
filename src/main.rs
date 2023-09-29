use dialoguer::{theme::ColorfulTheme, MultiSelect, Input};

use crate::ResourceType::CreationGetter;

pub mod ResourceType;
pub mod resources;

fn main() {
    let appName: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter app name")
        .interact_text()
        .unwrap();

    let appDirectory: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter app location")
        .interact_text()
        .unwrap();

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
        println!("You selected these things:");
        for selection in selections {
            let resType: ResourceType::ResourceType = selected[selection].try_into().unwrap();

            let creator = resType.to_owned().get_creator();

            let creatorType = creator.get_resource_type();

            creatorType.get_creator().create_resource(&appName, &appDirectory);

            println!("  {}", selected[selection]);
        }
    }
}
