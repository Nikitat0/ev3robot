use std::convert::Infallible;
use std::fmt::{self, Debug, Display, Formatter};
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct Port(Inner);

#[derive(Clone)]
enum Inner {
    Static(&'static str),
    Dynamic(Arc<str>),
}

impl Port {
    pub fn new<T: Into<Port>>(name: T) -> Self {
        name.into()
    }

    pub const fn new_static(name: &'static str) -> Port {
        Port(Inner::Static(name))
    }
}

impl<T: Into<Arc<str>>> From<T> for Port {
    fn from(v: T) -> Self {
        Port(Inner::Dynamic(v.into()))
    }
}

impl FromStr for Port {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(s.into())
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for Port {}

impl AsRef<str> for Port {
    fn as_ref(&self) -> &str {
        match self {
            Port(Inner::Static(port)) => port,
            Port(Inner::Dynamic(port)) => port,
        }
    }
}

impl Debug for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Port").field(&self.as_ref()).finish()
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

macro_rules! port {
    ($ident:ident, $port:literal) => {
        pub const $ident: $crate::port::Port =
            $crate::port::Port::new_static($port);
    };
}

pub(crate) use port;
