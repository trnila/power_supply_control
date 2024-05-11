use dioxus::prelude::*;
use dioxus::{
    core_macro::rsx,
    dioxus_core::Element,
    hooks::{use_context, use_signal},
    signals::Signal,
};
use serialport::UsbPortInfo;

use crate::config::{AppConfig, ChannelConfig, PowerSupplyConfig};

fn format_usb_port(port: &UsbPortInfo) -> String {
    [
        Some(format!("{:04x}:{:04x}", port.vid, port.pid)),
        port.manufacturer.clone(),
        port.product.clone(),
        port.serial_number.clone(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<String>>()
    .join(", ")
}

fn scan_usb() -> Vec<UsbPortInfo> {
    serialport::available_ports()
        .unwrap()
        .into_iter()
        .filter_map(|p| match p.port_type {
            serialport::SerialPortType::UsbPort(usbinfo) => Some(usbinfo),
            _ => None,
        })
        .collect()
}

pub fn AddDeviceComponent() -> Element {
    let mut appconfig = use_context::<Signal<AppConfig>>();
    let mut ports = use_signal(scan_usb);

    rsx! {
        form {
            class: "input-group",
            onsubmit: move |evt| {
                let index:usize = evt.data.values()["index"].as_value().parse().unwrap();
                let port = &ports.read()[index];
                appconfig.write().data.power_supply.push(PowerSupplyConfig{
                    vid: port.vid,
                    pid: port.pid,
                    serial_number: port.serial_number.clone(),
                    id: port.serial_number.clone().unwrap(),
                    name: "Power Supply MX100QP".to_string(),
                    channels: (1..=4).map(|ch| {
                        ChannelConfig{
                            name: format!("Channel {ch}"),
                            voltage: 0.0,
                            current: 0.0
                        }
                    }).collect(),
                });
                appconfig.write().save();
            },

            button {
                class: "btn btn-sm btn-secondary",
                prevent_default: "onclick",
                onclick: move |_| *ports.write() = scan_usb(),
                "Rescan USB"
            }
            select {
                class: "form-control form-control-sm",
                name: "index",
                for (i, port) in ports.read().iter().enumerate() {
                    option {
                        value: format!("{i}"),
                        "{format_usb_port(port)}"
                    }
                }
            }
            button {
                class: "btn btn-sm btn-success",
                "Add"
            }
        }
    }
}
