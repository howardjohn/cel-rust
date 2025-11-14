use crate::{ExecutionError, FunctionContext, Value};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use crate::objects::{AsValue, Opaque};

type Resultx = std::result::Result<Value, ExecutionError>;

pub mod ip {
    use crate::functionsx::Resultx;
    use crate::magic::This;
    use crate::objects::{AsValue, Opaque, ValueType};
    use crate::{FunctionContext, Value};
    use antlr4rust::TidExt;
    use std::any::Any;
    use std::str::FromStr;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    struct IpAddr(std::net::IpAddr);

    impl AsValue for IpAddr {
        fn to_value(&self, _: ValueType) -> Option<Value> {
            Some(Value::String(self.0.to_string().into()))
        }

        fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
            self
        }
    }

    pub fn ip(ftx: &FunctionContext, s: Arc<String>) -> Resultx {
        std::net::IpAddr::from_str(&s)
            .map_err(|e| ftx.error(e))
            .map(|i| {
                Opaque {
                    name: "ip".to_string(),
                    data: Arc::new(IpAddr(i)),
                }
                .into()
            })
    }

    pub fn is_localhost(ftx: &FunctionContext, s: This<Opaque>) -> Resultx {
        let d: Arc<dyn AsValue + 'static> = s.0.data.clone();
        let t =
            d.as_any().clone()
                .downcast::<IpAddr>()
                .map_err(|_| ftx.error("inappropriate type"))?;
        Ok(t.0.is_loopback().into())
    }
}
