use crate::{resource_type, get_current_working_dir};

use super::resource_creator::ResourceCreator;

use std::fs::{File, self};
use std::io::prelude::*;

use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use serde::Serialize;


#[derive(Serialize)]
pub struct PVCContext {
    app_name: String,
    storage_capacity: String,
}


pub struct PVCCreator {

}

impl ResourceCreator for PVCCreator {
	fn get_resource_type(&self) -> crate::resource_type::ResourceType {
		return resource_type::ResourceType::PVC;
	}

	fn get_template_content(&self) -> String {
		let current_dir = get_current_working_dir().unwrap();
		let current_dir = current_dir.as_os_str().to_str().unwrap();

		let current_path = format!("{}/src/data/templates", current_dir);

		let file_path = format!("{}/pvc.yml.template", current_path);

		let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

		contents
	}

	fn create_resource(&mut self, app_name: &String, app_location: &String) -> std::io::Result<()> {
		println!("Creating PVC.");
		
		let mut templates = tinytemplate::TinyTemplate::new();

		let storage_capacity: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter storage capacity")
        .interact_text()
        .unwrap();


		let content = self.get_template_content();

		templates.add_template("pvc-template", &content).unwrap();

		let context = PVCContext {
			app_name: app_name.to_string(),
			storage_capacity: storage_capacity.to_string(),
		};

		

		let rendered = templates.render("pvc-template", &context).unwrap();
    	
		let mut file = File::create(format!("{}/pvc.yml", app_location))?;

		file.write_all(rendered.as_bytes())?;

		Ok(())
	}
}