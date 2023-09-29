use core::fmt;

use crate::resources::resource_creator::ResourceCreator;
use crate::resources::pv::PVCreator;

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    PV,
    PVC,
    Deployment,
    Service,
    IngressRoute,
    HPA,
}

pub trait CreationGetter {
	fn get_creator(&self) -> Box<dyn ResourceCreator + 'static>;
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceType::PV => write!(f, "PV"),
            ResourceType::PVC => write!(f, "PVC"),
            ResourceType::Deployment => write!(f, "Deployment"),
            ResourceType::Service => write!(f, "Service"),
			ResourceType::IngressRoute => write!(f, "IngressRoute"),
			ResourceType::HPA => write!(f, "HPA"),
        }
    }
}

impl CreationGetter for ResourceType {
	fn get_creator(&self) -> Box<dyn ResourceCreator + 'static> {
		match self {
			ResourceType::PV => Box::new(PVCreator {}),
			_ => panic!("This creator is not implemented.")
		}
	}
}