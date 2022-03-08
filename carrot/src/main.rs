extern crate alloc;
use alloc::sync::Arc;
use embedded_svc::storage::Storage;
use log::info;

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("booted");

    let nvs = Arc::new(esp_idf_svc::nvs::EspDefaultNvs::new().unwrap());
    let mut storage =
        esp_idf_svc::nvs_storage::EspNvsStorage::new_default(nvs, "carrot", true).unwrap();

    let mut count: u32 = storage.get("count").unwrap().unwrap_or(0);
    info!("initial count: {}", count);
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        count += 1;
        info!("count: {}", count);
        storage.put("count", &count).unwrap();
    }
}
