use crate::device::{ReadOnlyAttributeFile, ReadWriteAttributeFile};

#[derive(Debug, Device, FindableDevice)]
#[findable_device(class = "lego-sensor", driver = "lego-ev3-us")]
pub struct UltrasonicSensor {
    mode: ReadWriteAttributeFile,
    #[device(attr_name = "value0")]
    value: ReadOnlyAttributeFile,
}

impl UltrasonicSensor {
    pub fn cm_distance_mode(&mut self) -> anyhow::Result<CmDistanceMode> {
        self.mode.set_value("US-DIST-CM")?;
        Ok(CmDistanceMode(self))
    }
}

pub struct CmDistanceMode<'a>(&'a mut UltrasonicSensor);

impl CmDistanceMode<'_> {
    pub fn mm(&mut self) -> anyhow::Result<i32> {
        Ok(self.0.value.value()?)
    }

    pub fn cm(&mut self) -> anyhow::Result<i32> {
        self.mm().map(|it| it / 10)
    }
}
