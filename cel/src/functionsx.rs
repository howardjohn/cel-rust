pub mod ip {
    use std::fmt::Debug;
    use crate::ExecutionError;
    type Result = std::result::Result<Value, ExecutionError>;
    use crate::magic::This;
    use crate::objects::{opaque, OpaqueValue, ValueType};
    use crate::{FunctionContext, Value};
    use std::str::FromStr;
    use std::sync::Arc;

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct IpAddr(std::net::IpAddr);

    impl OpaqueValue for IpAddr {
        // fn to_value(&self, _: ValueType) -> Option<Value> {
        //     Some(Value::String(self.0.to_string().into()))
        // }

        fn runtime_type_name(&self) -> &str {
            "ip_addr"
        }

        fn eq(&self, other: &dyn OpaqueValue) -> bool {
            opaque::eq::<Self>(self, other)
        }

        fn as_debug(&self) -> Option<&dyn Debug> {
            Some(self)
        }

        fn json(&self) -> Option<serde_json::Value> {
            serde_json::to_value(self.0).ok()
        }
    }

    pub fn ip(ftx: &FunctionContext, s: Arc<String>) -> Result {
        std::net::IpAddr::from_str(&s)
          .map_err(|e| ftx.error(e))
          .map(|i| {
              Value::Opaque(Arc::new(IpAddr(i)))
          })
    }

    pub fn is_localhost(ftx: &FunctionContext, s: This<Arc<dyn OpaqueValue>>) -> Result {
        Ok(s.0.downcast_ref::<IpAddr>().unwrap().0.is_loopback().into())
    }
}
