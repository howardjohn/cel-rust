use crate::FunctionContext;
use std::any::Any;
use std::sync::Arc;

pub mod ip {
    use std::any::Any;
    use crate::ExecutionError;
    use std::fmt::Debug;
    type Result = std::result::Result<Value, ExecutionError>;
    use crate::magic::{Function, This};
    use crate::objects::{opaque, OpaqueValue, ValueType};
    use crate::{FunctionContext, Value};
    use std::str::FromStr;
    use std::sync::Arc;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct IpAddr(std::net::IpAddr);

    impl IpAddr {
        pub fn is_localhost(self: Arc<Self>) -> Result {
            Ok(self.0.is_loopback().into())
        }
    }

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
            .map(|i| Value::Opaque(Arc::new(IpAddr(i))))
    }

    pub fn is_localhost(ftx: &FunctionContext, s: This<Arc<dyn OpaqueValue>>) -> Result {
        Ok(s.0.downcast_ref::<IpAddr>().unwrap().0.is_loopback().into())
    }
    pub fn is_localhost2(ip: Arc<IpAddr>) -> Result {
        Ok(ip.0.is_loopback().into())
    }

    pub fn wrap<F, T>(f: F) -> Function
    where
        F: Fn(Arc<T>) -> Result,
        T: 'static + Send + Sync,
    {
        Box::new(|f: &mut FunctionContext| {
            let this = f.this.clone().unwrap();
            let Value::Opaque(this) = this else {
                panic!("nop")
            };
            let v = this.downcast::<T>().unwrap();
            // f(v)
            todo!()
        })
    }
    fn as_any<T: Any + Send + Sync>(t: Arc<T>)-> Arc<dyn Any + Send + Sync> {
        t
    }
}

