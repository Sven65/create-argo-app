use crate::{resource_type, get_current_working_dir};

use super::resource_creator::ResourceCreator;

use std::fs::{File, self};
use std::io::prelude::*;

use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use serde::Serialize;


#[derive(Serialize)]
pub struct PVContext {
    app_name: String,
    storage_capacity: String,
    storage_class_name: String,
}


pub struct PVCreator {

}

impl ResourceCreator for PVCreator {
	fn get_resource_type(&self) -> crate::resource_type::ResourceType {
		return resource_type::ResourceType::PV;
	}

	fn get_template_content(&self) -> String {
		let current_dir = get_current_working_dir().unwrap();
		let current_dir = current_dir.as_os_str().to_str().unwrap();

		let current_path = format!("{}/src/data/templates", current_dir);

		let file_path = format!("{}/pv.yml.template", current_path);

		let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

		contents
	}

	fn create_resource(&mut self, app_name: &String, app_location: &String) -> std::io::Result<()> {
		println!("Creating PV.");

		let mut templates = tinytemplate::TinyTemplate::new();

		let storage_capacity: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage capacity")
        .interact_text()
        .unwrap();

		let storage_class_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage class name")
        .interact_text()
        .unwrap();


		let content = self.get_template_content();

		templates.add_template("pv-template", &content).unwrap();

		let context = PVContext {
			app_name: app_name.to_string(),
			storage_capacity: storage_capacity.to_string(),
			storage_class_name: storage_class_name.to_string()
		};

		

		let rendered = templates.render("pv-template", &context).unwrap();
    	
		let mut file = File::create(format!("{}/pv.yml", app_location))?;

		file.write_all(rendered.as_bytes())?;

		Ok(())
	}
}