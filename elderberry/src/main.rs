use log::info;
use std::cell::RefCell;
use std::thread;

thread_local! {
    static FOO: RefCell<Bar> = RefCell::new(Bar::default());
}

#[derive(Debug, Default)]
struct Bar {
    v: u32,
}

impl Drop for Bar {
    fn drop(&mut self) {
        info!("drop FOO: {}", self.v);
    }
}

fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("booted");

    FOO.with(|f| {
        info!("FOO: {}", f.borrow().v);
        f.borrow_mut().v = 2;
    });

    let t1 = thread::spawn(move || {
        FOO.with(|f| {
            info!("FOO: {}", f.borrow().v);
            f.borrow_mut().v = 3;
        })
    });

    t1.join().unwrap();
    info!("exit main");
}
