#![windows_subsystem = "windows"]

use winrt_core_app::{
    FrameworkView,
    FrameworkViewSource,
};

use std::sync::Arc;
use winrt::{
    *,
    windows::{
        applicationmodel::core::CoreApplication,
        ui::core::{
            CoreWindow,
            CoreProcessEventsOption,
        },
    },
};

#[derive(Default)]
struct App;

impl FrameworkView for App {
    fn run(self: Arc<Self>) -> Result<()> {
        let window = CoreWindow::get_for_current_thread()?.unwrap();
        let _ = window.activate();

        let dispatcher = window.get_dispatcher()?.unwrap();
        dispatcher.process_events(CoreProcessEventsOption::ProcessUntilQuit)?;

        Ok(())
    }
}

fn main() {
    init_apartment(ApartmentType::MTA);

    let app = App::default().com();

    let _ = CoreApplication::run(&app);
}
