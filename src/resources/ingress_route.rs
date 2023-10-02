use crate::{resource_type, get_current_working_dir};

use super::resource_creator::ResourceCreator;

use std::fs::{File, self};
use std::io::prelude::*;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Confirm};
use serde::Serialize;

#[derive(Serialize)]
pub struct IngressRouteContext {
    app_name: String,
	entrypoint: String,
	routes: Vec<Route>,
	tls: bool,
}

#[derive(Serialize, Clone, Copy)]
pub struct RouteService {
	svc_port: u16,
}

#[derive(Serialize, Clone)]
pub struct Route {
	route_host_match: String,
	services: Vec<RouteService>,
}

pub struct IngressRouteCreator {

}

impl IngressRouteCreator {
	fn get_entrypoint(&self) -> String {
		let name: String = Input::new()
        .with_prompt("Enter entrypoint")
        .interact_text()
        .unwrap();

		name
	}

	fn get_services(&self, services: &mut Vec<RouteService>) -> Vec<RouteService> {
		let svc_port: u16 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter service port")
        .interact_text()
        .unwrap();


		let confirmation = Confirm::new()
        .with_prompt("Do you want to add more services?")
        .interact()
        .unwrap();

		let new_service = RouteService {
			svc_port,
		};

		services.push(new_service);

		if confirmation {
			self.get_services(services)
		} else {
			services.to_vec()
		}
	}

	fn get_routes(&self, routes: &mut Vec<Route>) -> Vec<Route> {
		let route_host_match: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter host match")
        .interact_text()
        .unwrap();

		
		let new_route = Route {
			route_host_match,
			services: self.get_services(&mut vec![]),
		};

		let confirmation = Confirm::new()
        .with_prompt("Do you want to add more ports?")
        .interact()
        .unwrap();


		routes.push(new_route);

		if confirmation {
			self.get_routes(routes)
		} else {
			routes.to_vec()
		}
	}
}

impl ResourceCreator for IngressRouteCreator {
	fn get_resource_type(&self) -> crate::resource_type::ResourceType {
		return resource_type::ResourceType::IngressRoute;
	}

	fn get_template_content(&self) -> String {
		let current_dir = get_current_working_dir().unwrap();
		let current_dir = current_dir.as_os_str().to_str().unwrap();

		let current_path = format!("{}/src/data/templates", current_dir);

		let file_path = format!("{}/ingress-route.yml.template", current_path);

		let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

		contents
	}

	fn create_resource(&mut self, app_name: &String, app_location: &String) -> std::io::Result<()> {
		println!("Creating Ingress Route.");

		let mut templates = tinytemplate::TinyTemplate::new();


		let content = self.get_template_content();

		templates.add_template("ingress-route-template", &content).unwrap();


		let context = IngressRouteContext {
			app_name: app_name.to_string(),
			entrypoint: self.get_entrypoint(),
			routes: self.get_routes(&mut vec![]),
			tls: false
		};

		

		let rendered = templates.render("ingress-route-template", &context).unwrap();
    	
		let mut file = File::create(format!("{}/ingress-route.yml", app_location))?;

		file.write_all(rendered.as_bytes())?;

		Ok(())
	}
}