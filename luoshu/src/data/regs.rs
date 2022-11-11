use std::net::SocketAddr;

use async_trait::async_trait;
use luoshu_configuration::Configurator;
use luoshu_core::default_namespace;
use luoshu_namespace::Namespace;
use luoshu_registry::{Registry, Service};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::{Frame, Subscribe};
use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

/// 注册中心请求体
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ServiceReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    name: String,
    #[serde(flatten)]
    service: Service,
}

impl ServiceReg {
    /// 注册中心实例化
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

/// 配置中心请求体
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ConfigurationReg {
    #[serde(default = "default_namespace")]
    namespace: String,
    pub(crate) name: String,
    /// 配置内容
    pub config: Value,
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

impl ConfigurationReg {
    /// 配置中心实例化
    pub fn new(namespace: String, name: String, config: Value) -> Self {
        Self {
            namespace,
            name,
            config,
        }
    }
    /// 获取命名空间
    pub fn get_namespace(&self) -> String {
        self.namespace.clone()
    }
    /// 获取订阅名称
    pub fn get_subscribe_str(&self) -> String {
        format!("{}|{}", self.namespace.clone(), self.name.clone())
    }
}

/// 命名空间请求体
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NamespaceReg {
    name: String,
}

impl NamespaceReg {
    /// 命名空间请求体实例化
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<&NamespaceReg> for Namespace {
    fn from(namespace_reg: &NamespaceReg) -> Self {
        Namespace::new(namespace_reg.name.clone())
    }
}

/// 洛书数据层枚举
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum LuoshuDataEnum {
    /// 命名空间
    Namespace(NamespaceReg),
    /// 配置中心
    Configuration(ConfigurationReg),
    /// 注册中心服务
    Service(ServiceReg),
    /// 消息订阅
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

/// 洛书数据层消息处理器接口
#[async_trait]
pub trait LuoshuDataHandle {
    /// 新增数据
    async fn append(&mut self, value: &LuoshuDataEnum, client: Option<SocketAddr>) -> Result<()>;
    /// 删除数据
    async fn remove(&mut self, value: &LuoshuDataEnum) -> Result<()>;
    /// 同步数据
    async fn sync(&mut self, value: &LuoshuDataEnum) -> Result<()>;
    /// 订阅消息
    async fn subscribe(
        &mut self,
        subscribe: Subscribe,
        subscriber_sender: &UnboundedSender<Frame>,
    ) -> Result<()>;
    /// 连接断开
    async fn broken(&mut self, client: SocketAddr) -> Result<()>;
}
