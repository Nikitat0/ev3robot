macro_rules! tacho_motor {
    ($ident:ident, $driver:literal) => {
        #[derive(Debug, FindableDevice)]
        #[findable_device(class = "tacho-motor", driver = $driver)]
        pub struct $ident($crate::motor::tacho::TachoMotor);

        impl $crate::device::Device for $ident {
            fn open<P>(device_node: P) -> ::anyhow::Result<Self>
            where
                P: ::std::convert::AsRef<::std::path::Path>,
            {
                Ok(Self($crate::motor::tacho::TachoMotor::open(device_node)?))
            }
        }
    };
}

tacho_motor!(LargeMotor, "lego-ev3-l-motor");
tacho_motor!(MediumMotor, "lego-ev3-m-motor");
