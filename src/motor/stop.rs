use derive_more::Display;

pub trait Stop<StopAction> {
    fn stop(stop_action: StopAction);
}

macro_rules! stop_action {
    ($name:ident, $lit:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
        #[display(fmt = "{}", "AsRef::<str>::as_ref(self)")]
        pub struct $name;

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                $lit
            }
        }
    };
}

stop_action!(Brake, "brake");
stop_action!(Coast, "coast");
stop_action!(Hold, "hold");
