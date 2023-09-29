use crate::ResourceType;

pub trait ResourceCreator {
	fn get_resource_type(&self) -> ResourceType::ResourceType;

	fn create_resource(&self, app_name: &String, app_location: &String);
}