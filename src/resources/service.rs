use crate::{resource_type, get_current_working_dir};

use super::resource_creator::ResourceCreator;

use std::fmt::Display;
use std::fs::{File, self};
use std::io::prelude::*;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Confirm, Select};
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub enum Protocol {
	TCP,
	UDP,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

#[derive(Serialize)]
pub struct ServiceContext {
    app_name: String,
	ports: Vec<ServicePort>,
}

#[derive(Serialize, Clone, Copy)]
pub struct ServicePort {
	port: u16,
	target_port: u16,
	protocol: Protocol,
}


pub struct ServiceCreator {
	pub ports: Vec<ServicePort>
}

impl ServiceCreator {
	fn port_entry (&mut self) {
		let port: u16 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter port")
        .interact_text()
        .unwrap();

		let target_port: u16 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter target port")
        .interact_text()
        .unwrap();

		let protocols = vec![Protocol::TCP, Protocol::UDP];

		let protocol_selection = Select::new()
			.with_prompt("Please select protocol.")
			.items(&protocols)
			.interact()
			.unwrap();

		let protocol = protocols[protocol_selection];

		let new_port = ServicePort {
			port,
			target_port,
			protocol
		};

		self.ports.push(new_port);


		let confirmation = Confirm::new()
        .with_prompt("Do you want to add more ports?")
        .interact()
        .unwrap();

		if confirmation {
			self.port_entry();
		}
	}
}

impl ResourceCreator for ServiceCreator {
	fn get_resource_type(&self) -> crate::resource_type::ResourceType {
		return resource_type::ResourceType::Service;
	}

	fn get_template_content(&self) -> String {
		let current_dir = get_current_working_dir().unwrap();
		let current_dir = current_dir.as_os_str().to_str().unwrap();

		let current_path = format!("{}/src/data/templates", current_dir);

		let file_path = format!("{}/service.yml.template", current_path);

		let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

		contents
	}

	fn create_resource(&mut self, app_name: &String, app_location: &String) -> std::io::Result<()> {
		println!("Creating Service.");

		let mut templates = tinytemplate::TinyTemplate::new();


		let content = self.get_template_content();

		templates.add_template("service-template", &content).unwrap();

		self.port_entry();
		let context = ServiceContext {
			app_name: app_name.to_string(),
			ports: self.ports.clone(),
		};

		

		let rendered = templates.render("service-template", &context).unwrap();
    	
		let mut file = File::create(format!("{}/service.yml", app_location))?;

		file.write_all(rendered.as_bytes())?;

		Ok(())
	}
}