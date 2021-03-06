#pragma once
#include "pch.h"

using namespace winrt;
using namespace Windows::ApplicationModel::Core;
using namespace Windows::UI::Core;

namespace abi {
    using namespace ABI::Windows::ApplicationModel::Core;
    using namespace ABI::Windows::UI::Core;
};

namespace rust_ffi {
    template<typename... Args>
    using FrameworkViewOverrideMethod = void (*) (void * framework_view, Args... args);

    struct FrameworkView {
        void Initialize(CoreApplicationView const & application_view);
        void Load(hstring const & entry_point);
        void Run();
        void SetWindow(CoreWindow const & window);
        void Uninitialize();
    private:
        void* framework_view;
        struct {
            FrameworkViewOverrideMethod<abi::ICoreApplicationView *> initialize;
            FrameworkViewOverrideMethod<HSTRING> load;
            FrameworkViewOverrideMethod<> run;
            FrameworkViewOverrideMethod<abi::ICoreWindow *> set_window;
            FrameworkViewOverrideMethod<> uninitialize;
        } v_table;
    };
};

extern "C" abi::IFrameworkViewSource* create_app(rust_ffi::FrameworkView view);
