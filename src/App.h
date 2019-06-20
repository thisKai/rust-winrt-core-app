#pragma once
#include "pch.h"
#include "ffi.h"

using namespace winrt;
using namespace Windows::ApplicationModel::Core;
using namespace Windows::UI::Core;

struct App : implements<App, IFrameworkViewSource, IFrameworkView>
{
    std::unique_ptr<rust_ffi::FrameworkView> m_view;

    App();
    App(rust_ffi::FrameworkView view);

    IFrameworkView CreateView();

    void Initialize(CoreApplicationView const &);

    void Load(hstring const&);

    void Uninitialize();

    void Run();

    void SetWindow(CoreWindow const & window);
};
