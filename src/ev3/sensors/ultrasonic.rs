use crate::device::{ReadOnlyAttributeFile, ReadWriteAttributeFile};

#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "lego-sensor", driver = "lego-ev3-us")]
pub struct UltrasonicSensor {
    mode: ReadWriteAttributeFile,
    #[device(attr_name = "value0")]
    value: ReadOnlyAttributeFile,
}

impl UltrasonicSensor {
    pub fn measure_cm(&mut self) -> anyhow::Result<CmMeasurements> {
        self.mode.set_value("US-DIST-CM")?;
        Ok(CmMeasurements(self))
    }
}

pub struct CmMeasurements<'a>(&'a mut UltrasonicSensor);

impl CmMeasurements<'_> {
    pub fn cm(&mut self) -> anyhow::Result<f32> {
        self.0
            .value
            .value::<u32>()
            .map(|mm| mm as f32 / 10.0)
            .map_err(Into::into)
    }
}
