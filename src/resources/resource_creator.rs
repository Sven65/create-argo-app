use crate::resource_type;

pub trait ResourceCreator {
	fn get_resource_type(&self) -> resource_type::ResourceType;

	fn create_resource(&mut self, app_name: &String, app_location: &String) -> std::io::Result<()>;

	fn get_template_content(&self) -> String;
}