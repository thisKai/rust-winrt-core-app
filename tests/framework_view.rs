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
        fn initialize(self: Arc<Self>, application_view: ComPtr<CoreApplicationView>) {}
    }
}

#[test]
fn load_method() {
    struct App;
    impl FrameworkView for App {
        fn load(self: Arc<Self>, entry_point: HString) {}
    }
}

#[test]
fn run_method() {
    struct App;
    impl FrameworkView for App {
        fn run(self: Arc<Self>) {}
    }
}

#[test]
fn set_window_method() {
    struct App;
    impl FrameworkView for App {
        fn set_window(self: Arc<Self>, window: ComPtr<CoreWindow>) {}
    }
}

#[test]
fn uninitialize_method() {
    struct App;
    impl FrameworkView for App {
        fn uninitialize(self: Arc<Self>) {}
    }
}
