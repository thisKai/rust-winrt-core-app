#![windows_subsystem = "windows"]

use winrt::{
    init_apartment,
    ApartmentType,
    ComPtr,
    windows::applicationmodel::core::{
        IFrameworkViewSource,
        CoreApplication,
    },
};

extern "C" {
    fn create_app() -> *mut IFrameworkViewSource;
}

fn main() {
    init_apartment(ApartmentType::MTA);

    let app = unsafe { ComPtr::wrap(create_app()) };

    let _ = CoreApplication::run(&app);
}
