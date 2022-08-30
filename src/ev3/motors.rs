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

        impl $crate::motor::tacho::TachoMotorInterface for $ident {
            fn command(
                &mut self,
                value: $crate::motor::tacho::Command,
            ) -> ::anyhow::Result<()> {
                self.0.command(value)
            }

            fn count_per_rot(&self) -> $crate::motor::tacho::TachoCounts {
                self.0.count_per_rot()
            }

            fn duty_cycle(
                &mut self,
            ) -> ::anyhow::Result<$crate::percentage::SignedPercentage> {
                self.0.duty_cycle()
            }

            fn duty_cycle_sp(
                &mut self,
            ) -> ::anyhow::Result<$crate::percentage::SignedPercentage> {
                self.0.duty_cycle_sp()
            }

            fn set_duty_cycle_sp(
                &mut self,
                value: $crate::percentage::SignedPercentage,
            ) -> ::anyhow::Result<()> {
                self.0.set_duty_cycle_sp(value)
            }

            fn polarity(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::Polarity> {
                self.0.polarity()
            }

            fn set_polarity(
                &mut self,
                value: $crate::motor::Polarity,
            ) -> ::anyhow::Result<()> {
                self.0.set_polarity(value)
            }

            fn position(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::tacho::TachoCounts> {
                self.0.position()
            }

            fn set_position(
                &mut self,
                value: $crate::motor::tacho::TachoCounts,
            ) -> ::anyhow::Result<()> {
                self.0.set_position(value)
            }

            fn position_sp(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::tacho::TachoCounts> {
                self.0.position_sp()
            }

            fn set_position_sp(
                &mut self,
                value: $crate::motor::tacho::TachoCounts,
            ) -> ::anyhow::Result<()> {
                self.0.set_position_sp(value)
            }

            fn max_speed(&self) -> $crate::motor::tacho::TachoCounts {
                self.0.max_speed()
            }

            fn state(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::tacho::State> {
                self.0.state()
            }

            fn speed(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::tacho::TachoCounts> {
                self.0.speed()
            }

            fn speed_sp(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::tacho::TachoCounts> {
                self.0.speed_sp()
            }

            fn set_speed_sp(
                &mut self,
                value: $crate::motor::tacho::TachoCounts,
            ) -> ::anyhow::Result<()> {
                self.0.set_speed_sp(value)
            }

            fn stop_action(
                &mut self,
            ) -> ::anyhow::Result<$crate::motor::tacho::StopAction> {
                self.0.stop_action()
            }

            fn set_stop_action(
                &mut self,
                value: $crate::motor::tacho::StopAction,
            ) -> ::anyhow::Result<()> {
                self.0.set_stop_action(value)
            }
        }

        impl<SpeedUnit> $crate::motor::Run<SpeedUnit> for $ident
        where
            SpeedUnit: $crate::motor::tacho::TachoMotorSpeedUnit,
        {
            fn run(&mut self, speed: SpeedUnit) -> ::anyhow::Result<()> {
                self.0.run(speed)
            }
        }

        impl $crate::motor::IsRunning for $ident {
            fn is_running(&mut self) -> ::anyhow::Result<bool> {
                self.0.is_running()
            }
        }

        impl $crate::motor::IsHolding for $ident {
            fn is_holding(&mut self) -> ::anyhow::Result<bool> {
                self.0.is_holding()
            }
        }

        impl $crate::motor::Coast for $ident {
            fn coast(&mut self) -> ::anyhow::Result<()> {
                self.0.coast()
            }
        }

        impl $crate::motor::Brake for $ident {
            fn brake(&mut self) -> ::anyhow::Result<()> {
                self.0.brake()
            }
        }

        impl $crate::motor::Hold for $ident {
            fn hold(&mut self) -> ::anyhow::Result<()> {
                self.0.hold()
            }
        }
    };
}

tacho_motor!(LargeMotor, "lego-ev3-l-motor");
tacho_motor!(MediumMotor, "lego-ev3-m-motor");
