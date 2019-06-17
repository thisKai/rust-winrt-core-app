// #![windows_subsystem = "windows"]

mod framework_view;

use winrt::{
    init_apartment,
    ApartmentType,
    ComPtr,
    windows::{
        applicationmodel::core::{
            IFrameworkViewSource,
            CoreApplication,
        },
        ui::core::{
            CoreWindow,
            CoreProcessEventsOption,
        },
    },
};

use framework_view::{
    FrameworkView,
    FrameworkViewFfi,
    ffi,
};

extern "C" {
    fn create_app(view: FrameworkViewFfi) -> *mut IFrameworkViewSource;
}

struct App;
impl FrameworkView for App {
    fn run(&mut self) {
        let window = CoreWindow::get_for_current_thread().unwrap().unwrap();
        let _ = window.activate();

        let dispatcher = window.get_dispatcher().unwrap().unwrap();
        let _ = dispatcher.process_events(CoreProcessEventsOption::ProcessUntilQuit);
    }
}

fn main() {
    init_apartment(ApartmentType::MTA);

    let view = ffi(App);

    let app = unsafe { ComPtr::wrap(create_app(view)) };

    let _ = CoreApplication::run(&app);
}
