#include "pch.h"
#include "App.h"

using namespace winrt;

using namespace Windows;
using namespace Windows::ApplicationModel::Core;
using namespace Windows::Foundation::Numerics;
using namespace Windows::UI;
using namespace Windows::UI::Core;
using namespace Windows::UI::Composition;

namespace abi {
    using namespace ABI::Windows::ApplicationModel::Core;
};

struct App : implements<App, IFrameworkViewSource, IFrameworkView>
{
    std::unique_ptr<rust_ffi::FrameworkView> m_view;

    App()
    {
        m_view = nullptr;
    }
    App(rust_ffi::FrameworkView view)
    {
        m_view = std::make_unique<rust_ffi::FrameworkView>(view);
    }

    IFrameworkView CreateView()
    {
        return *this;
    }

    void Initialize(CoreApplicationView const &)
    {
        m_view->Initialize();
    }

    void Load(hstring const&)
    {
        m_view->Load();
    }

    void Uninitialize()
    {
        m_view->Uninitialize();
    }

    void Run()
    {
        m_view->Run();
    }

    void SetWindow(CoreWindow const & window)
    {
        m_view->SetWindow();
    }
};

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
