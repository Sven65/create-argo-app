use crate::ResourceType;

use super::resource_creator::ResourceCreator;

pub struct PVCreator {
		
}

impl ResourceCreator for PVCreator {
	fn get_resource_type(&self) -> crate::ResourceType::ResourceType {
		return ResourceType::ResourceType::PV;
	}

	fn create_resource(&self, app_name: &String, app_location: &String) {
		println!("Creating pv! for app {} in location {}", app_name, app_location)
	}
}