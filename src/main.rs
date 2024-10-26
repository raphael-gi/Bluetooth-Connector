use bluer::{Device, Error, ErrorKind, Result};
use dialoguer::{Confirm, Select};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
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
            options.push(format!("Disconnect: {}", get_name(device).await));
        }
    }

    loop {
        let selection = Select::new()
            .items(&options)
            .default(0)
            .interact_opt();

        let selected = match selection {
            Ok(Some(selected)) => selected,
            Ok(None) => return error(ErrorKind::Failed, ""),
            Err(_) => return error(ErrorKind::Failed, "Failed to select an option")
        };

        let res = match selected {
            0 => display_devices(devices.clone()).await,
            _ => {
                let device = match connected_devices.get(selected - 1) {
                    Some(device) => device.clone(),
                    None => panic!("Index of connected device out of range")
                };
                disconnect_from_device(device).await
            }
        };

        if res.is_ok() {
            break;
        }
    }

    Ok(())
}

async fn display_devices(devices: Vec<Device>) -> Result<()> {
    let mut names: Vec<String> = Vec::new();

    for device in &devices {
        names.push(get_name(device).await);
    }

    let selection = Select::new()
        .default(0)
        .items(&names)
        .interact_opt();

    let selected = match selection {
        Ok(Some(selected)) => selected,
        Ok(None) => return error(ErrorKind::Failed, ""),
        Err(..) => return error(ErrorKind::Failed, "Failed to select an option")
    };

    match devices.get(selected) {
        Some(device) => connect_to_device(device).await?,
        None => panic!("Invalid selection")
    }

    Ok(())
}

async fn connect_to_device(device: &Device) -> Result<()> {
    let device_name = get_name(device).await;
    loop {
        let connection_res = device.connect().await;
        if connection_res.is_ok() {
            break;
        }

        let confirmation = Confirm::new()
            .with_prompt(format!("Connection to device {} failed. Try again?", device_name))
            .interact();

        match confirmation {
            Ok(response) => {
                if !response {
                    break;
                }
            },
            Err(..) => return error(ErrorKind::Failed, "")
        }
    }

    Ok(())
}

async fn get_name(device: &Device) -> String {
    match device.name().await {
        Ok(Some(name)) => name,
        _ => device.address().to_string()
    }
}

async fn disconnect_from_device(device: Device) -> Result<()> {
    device.disconnect().await
}

fn error(kind: ErrorKind, message: &str) -> Result<()> {
    Err(Error { kind, message: message.to_string() })
}

