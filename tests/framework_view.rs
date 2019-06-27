use std::sync::Arc;
use winrt_core_app::FrameworkView;
use winrt::{
    *,
    windows::{
        ui::core::CoreWindow,
        applicationmodel::core::CoreApplicationView
    },
};

#[test]
fn initialize_method() {
    struct App;
    impl FrameworkView for App {
        fn initialize(self: Arc<Self>, _application_view: ComPtr<CoreApplicationView>) -> Result<()> {
            Ok(())
        }
    }
}

#[test]
fn load_method() {
    struct App;
    impl FrameworkView for App {
        fn load(self: Arc<Self>, _entry_point: HString) -> Result<()> {
            Ok(())
        }
    }
}

#[test]
fn run_method() {
    struct App;
    impl FrameworkView for App {
        fn run(self: Arc<Self>) -> Result<()> {
            Ok(())
        }
    }
}

#[test]
fn set_window_method() {
    struct App;
    impl FrameworkView for App {
        fn set_window(self: Arc<Self>, _window: ComPtr<CoreWindow>) -> Result<()> {
            Ok(())
        }
    }
}

#[test]
fn uninitialize_method() {
    struct App;
    impl FrameworkView for App {
        fn uninitialize(self: Arc<Self>) -> Result<()> {
            Ok(())
        }
    }
}
