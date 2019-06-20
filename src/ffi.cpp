#include "ffi.h"
#include "App.h"

void rust_ffi::FrameworkView::Initialize(CoreApplicationView const & application_view)
{
    com_ptr<abi::ICoreApplicationView> abi_application_view { application_view.as<abi::ICoreApplicationView>() };
    v_table.initialize(framework_view, abi_application_view.detach());
}
void rust_ffi::FrameworkView::Load()
{
    v_table.load(framework_view);
}
void rust_ffi::FrameworkView::Run()
{
    v_table.run(framework_view);
}
void rust_ffi::FrameworkView::SetWindow(CoreWindow const & window)
{
    com_ptr<abi::ICoreWindow> abi_window { window.as<abi::ICoreWindow>() };
    v_table.set_window(framework_view, abi_window.detach());
}
void rust_ffi::FrameworkView::Uninitialize()
{
    v_table.uninitialize(framework_view);
}

com_ptr<abi::IFrameworkViewSource> create_app_cpp(rust_ffi::FrameworkView view)
{
    auto app = make<App>(view);
    auto fwvs = app.as<IFrameworkViewSource>();
    com_ptr<abi::IFrameworkViewSource> ptr {
        fwvs.as<abi::IFrameworkViewSource>()
    };
    return ptr;
}

extern "C" abi::IFrameworkViewSource* create_app(rust_ffi::FrameworkView view)
{
    return create_app_cpp(view).detach();
}
