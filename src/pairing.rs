use std::collections::HashMap;

use bluer::{Adapter, AdapterEvent, Result};
use dialoguer::Select;
use futures::StreamExt;

use crate::util::get_name;


pub async fn pair_new_device(adapter: &Adapter) -> Result<()> {
    adapter.set_pairable(true).await?;

    let mut pairing_stream = adapter.discover_devices_with_changes().await?;

    let mut found_devices: HashMap<String, String> = HashMap::new();

    while let Some(event) = pairing_stream.next().await {
        match event {
            AdapterEvent::DeviceAdded(address) => {
                match adapter.device(address) {
                    Ok(device) => {
                        let name = get_name(&device).await;
                        found_devices.insert(address.to_string(), name);
                    },
                    Err(..) => {
                        found_devices.insert(address.to_string(), address.to_string());
                    }
                };
            },
            AdapterEvent::DeviceRemoved(address) => {
                found_devices.remove(&address.to_string());
            },
            AdapterEvent::PropertyChanged(_property) => {}
        }

        let names: Vec<String> = found_devices.values().cloned().collect();

        let _selection = Select::new()
            .items(&names)
            .interact_opt();
    }

    Ok(())
}
