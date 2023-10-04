use crate::{resource_type, get_current_working_dir};

use super::resource_creator::ResourceCreator;

use std::fs::{File, self};
use std::io::prelude::*;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Confirm};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct DeploymentContext {
    app_name: String,
	containers: Vec<Container>,
	volumes: Vec<Volume>,
}

#[derive(Serialize, Clone)]
pub struct Container {
	image: String,
	volume_mounts: Vec<VolumeMount>,
	name: String,
}

#[derive(Serialize, Clone)]
pub struct Resource {
	memory: String,
	cpu: String,
}

#[derive(Serialize, Clone)]
pub struct ContainerResources {
	requests: Resource,
	limits: Resource,
}

#[derive(Serialize, Clone)]
pub struct VolumeMount {
	mount_path: String,
	name: String,
}

// todo: make this work with more than just PVC
#[derive(Serialize, Clone)]
pub struct Volume {
	name: String,
	pvc_claim_name: Option<String>,
}


pub struct DeploymentCreator {

}

impl DeploymentCreator {
	pub fn get_volume_mounts(&self, mounts: &mut Vec<VolumeMount>) -> Vec<VolumeMount> {
		let mount_path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter mount path")
        .interact_text()
        .unwrap();

		let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter volume name")
        .interact_text()
        .unwrap();

		let confirmation = Confirm::new()
        .with_prompt("Do you want to add more mounts?")
        .interact()
        .unwrap();

		let new_mount = VolumeMount {
			mount_path,
			name,
		};

		mounts.push(new_mount);

		if confirmation {
			self.get_volume_mounts(mounts)
		} else {
			mounts.to_vec()
		}
	}

	pub fn get_volumes(&self, volumes: &mut Vec<Volume>) -> Vec<Volume> {
		let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter volume name")
        .interact_text()
        .unwrap();

		let pvc_claim_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter PVC Claim name")
        .interact_text()
        .unwrap();

		let confirmation = Confirm::new()
        .with_prompt("Do you want to add more volumes?")
        .interact()
        .unwrap();

		let new_volume = Volume {
			pvc_claim_name: Some(pvc_claim_name),
			name,
		};

		volumes.push(new_volume);

		if confirmation {
			self.get_volumes(volumes)
		} else {
			volumes.to_vec()
		}
	}

	pub fn get_containers(&self, containers: &mut Vec<Container>) -> Vec<Container> {

		let container_image: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter container image")
        .interact_text()
        .unwrap();


		let container_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter container name")
        .interact_text()
        .unwrap();

		let mount_volume_confirmation = Confirm::new()
        .with_prompt("Do you need any mounted volumes?")
        .interact()
        .unwrap();


		let mounts = match mount_volume_confirmation {
			true => self.get_volume_mounts(&mut vec![]),
			false => vec![]
		};

		let confirmation = Confirm::new()
        .with_prompt("Do you want to add more containers?")
        .interact()
        .unwrap();

		let new_container = Container {
			image: container_image,
			name: container_name,
			volume_mounts: mounts,
		};

		containers.push(new_container);

		if confirmation {
			self.get_containers(containers)
		} else {
			containers.to_vec()
		}
	}
}

impl ResourceCreator for DeploymentCreator {
	fn get_resource_type(&self) -> crate::resource_type::ResourceType {
		return resource_type::ResourceType::Deployment;
	}

	fn get_template_content(&self) -> String {
		let current_dir = get_current_working_dir().unwrap();
		let current_dir = current_dir.as_os_str().to_str().unwrap();

		let current_path = format!("{}/src/data/templates", current_dir);

		let file_path = format!("{}/deployment.yml.template", current_path);

		let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

		contents
	}

	fn create_resource(&mut self, app_name: &String, app_location: &String) -> std::io::Result<()> {
		println!("Creating Deployment");

		let mut templates = tinytemplate::TinyTemplate::new();


		let content = self.get_template_content();

		templates.add_template("deployment-template", &content).unwrap();

		let volume_confirmation = Confirm::new()
        .with_prompt("Do you need any volumes?")
        .interact()
        .unwrap();

		let volumes = match volume_confirmation {
			true => self.get_volumes(&mut vec![]),
			false => vec![]
		};


		let context = DeploymentContext {
			app_name: app_name.to_string(),
			containers: self.get_containers(&mut vec![]),
			volumes,
		};

		

		let rendered = templates.render("deployment-template", &context).unwrap();
    	
		let mut file = File::create(format!("{}/deployment.yml", app_location))?;

		file.write_all(rendered.as_bytes())?;

		Ok(())
	}
}