pub mod ip {
    use crate::ExecutionError;
    type Result = std::result::Result<Value, ExecutionError>;
    use crate::magic::This;
    use crate::objects::{AsValue, Opaque, ValueType};
    use crate::{FunctionContext, Value};
    use std::str::FromStr;
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    struct IpAddr(std::net::IpAddr);

    impl AsValue for IpAddr {
        fn to_value(&self, _: ValueType) -> Option<Value> {
            Some(Value::String(self.0.to_string().into()))
        }
    }

    pub fn ip(ftx: &FunctionContext, s: Arc<String>) -> Result {
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

    pub fn is_localhost(ftx: &FunctionContext, s: This<Opaque>) -> Result {
        Ok(s.0.downcast::<IpAddr>(ftx)?.0.is_loopback().into())
    }
}
