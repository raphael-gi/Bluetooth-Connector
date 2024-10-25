use bluer::{Adapter, Device};
use dialoguer::Select;

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    let addresses = adapter.device_addresses().await?;
    let devices: Vec<Device> = addresses.iter().map(|address| adapter.device(*address).unwrap()).collect();
    let mut connected_devices: Vec<Device> = Vec::new();

    let mut options = vec![String::from("Connect")];

    for device in &devices {
        if let Ok(connected) = device.is_connected().await {
            if !connected {
                continue;
            }
            connected_devices.push(device.clone());
            let name = match device.name().await {
                Ok(Some(name)) => name,
                _ => device.address().to_string()
            };
            options.push(format!("Disconnect: {}", name));
        }
    }

    let selected = Select::new()
        .items(&options)
        .default(0)
        .interact_opt()
        .unwrap();

    if selected.is_none() {
        return Ok(());
    }

    match selected.unwrap() {
        0 => display_devices(devices).await?,
        _ => {
            let device = match connected_devices.get(selected.unwrap() - 1) {
                Some(device) => device.clone(),
                None => panic!("Index of connected device out of range")
            };
            let _ = disconnect_from_device(device).await;
            return Ok(());
        }
    }

    Ok(())
}

async fn display_devices(devices: Vec<Device>) -> bluer::Result<()> {
    let mut names: Vec<String> = Vec::new();

    for device in &devices {
        let name = match device.name().await {
            Ok(Some(name)) => name,
            _ => device.address().to_string()
        };
        names.push(name);
    }

    let selected = Select::new()
        .default(0)
        .items(&names)
        .interact()
        .unwrap();

    match devices.get(selected) {
        Some(device) => connect_to_device(device).await?,
        None => panic!("Invalid selection")
    }

    Ok(())
}

async fn connect_to_device(device: &Device) -> bluer::Result<()> {
    match device.connect().await {
        Ok(_) => println!("Connected to: {:?}", device.name().await?),
        Err(_) => println!("Failed to connect to device")
    }

    Ok(())
}

async fn disconnect_from_device(device: Device) -> bluer::Result<()> {
    device.disconnect().await
}

