use bluer::Device;


pub async fn get_name(device: &Device) -> String {
    match device.name().await {
        Ok(Some(name)) => name,
        _ => device.address().to_string()
    }
}
