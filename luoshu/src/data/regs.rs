use async_trait::async_trait;
use luoshu_configuration::Configurator;
use luoshu_core::default_namespace;
use luoshu_namespace::Namespace;
use luoshu_registry::{Registry, Service};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::Frame;
use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

#[derive(Clone, Deserialize, Serialize)]
pub struct ServiceReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    #[serde(flatten)]
    service: Service,
}

impl ServiceReg {
    pub fn new(namespace: String, name: String, service: Service) -> Self {
        Self {
            namespace,
            name,
            service,
        }
    }
}

impl From<&ServiceReg> for Registry {
    fn from(service_reg: &ServiceReg) -> Self {
        let mut registry = Registry::new(
            Some(service_reg.namespace.clone()),
            service_reg.name.clone(),
        );
        registry
            .register_service(service_reg.service.clone())
            .unwrap();
        registry
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigurationReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    pub(crate) name: String,
    config: Value,
}

impl From<&ConfigurationReg> for Configurator {
    fn from(configuration_reg: &ConfigurationReg) -> Self {
        let mut configuration = Configurator::new(Some(configuration_reg.namespace.clone()));
        configuration
            .set_configuration(
                configuration_reg.name.clone(),
                configuration_reg.config.clone(),
            )
            .unwrap();
        configuration
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct NamespaceReg {
    pub name: String,
}

impl From<&NamespaceReg> for Namespace {
    fn from(namespace_reg: &NamespaceReg) -> Self {
        Namespace::new(namespace_reg.name.clone())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum LuoshuDataEnum {
    Namespace(NamespaceReg),
    Configuration(ConfigurationReg),
    Service(ServiceReg),
    Subscribe(String),
}

impl From<NamespaceReg> for LuoshuDataEnum {
    fn from(namespace: NamespaceReg) -> Self {
        Self::Namespace(namespace)
    }
}

impl From<ConfigurationReg> for LuoshuDataEnum {
    fn from(configuration: ConfigurationReg) -> Self {
        Self::Configuration(configuration)
    }
}

impl From<ServiceReg> for LuoshuDataEnum {
    fn from(service: ServiceReg) -> Self {
        Self::Service(service)
    }
}

#[async_trait]
pub trait LuoshuDataHandle {
    async fn append(&mut self, value: &LuoshuDataEnum) -> Result<()>;
    async fn remove(&mut self, value: &LuoshuDataEnum) -> Result<()>;
    async fn sync(&mut self, value: &LuoshuDataEnum) -> Result<()>;
    async fn subscribe(&mut self, value: String, client: UnboundedSender<Frame>) -> Result<()>;
}
