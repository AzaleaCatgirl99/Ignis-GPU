use crate::{Adapter, AdapterInfo, AdapterRequestError, Backend, BackendInfo};

pub trait InstanceInterface {
    fn destroy(&self);

    fn get_backend(&self) -> Backend;
    fn get_backend_info(&self) -> BackendInfo;

    fn request_adapter(&self) -> Result<Adapter, AdapterRequestError>;
    fn get_adapter_info(&self, adapter: &Adapter) -> AdapterInfo;
}
