use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};

use tap::prelude::*;

use super::DeviceAttribute;
use crate::port::Port;

pub fn find_device_nodes_by_class(class: &str) -> Vec<PathBuf> {
    let class_path = PathBuf::from("/sys/class").tap_mut(|it| it.push(class));
    read_dir(class_path)
        .map(|dir_entries| {
            dir_entries.filter_map(Result::ok).map(|it| it.path()).collect()
        })
        .unwrap_or_default()
}

pub fn device_node_driver_name<P: AsRef<Path>>(
    device_node: P,
) -> io::Result<String> {
    String::of_device(device_node, "driver_name")
}

pub fn device_node_port<P: AsRef<Path>>(device_node: P) -> io::Result<Port> {
    Port::of_device(device_node, "address")
}
