#![windows_subsystem = "windows"]

extern "C" {
    fn start_app();
}

fn main() {
    winrt::init_apartment(winrt::ApartmentType::MTA);
    unsafe { start_app(); }
}
