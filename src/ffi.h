#pragma once
#include "pch.h"

using namespace winrt;
namespace abi {
    using namespace ABI::Windows::ApplicationModel::Core;
};

namespace rust_ffi {
    template<typename... Args>
    using FrameworkViewOverrideMethod = void (*) (void * framework_view, Args... args);

    struct FrameworkView {
        void Initialize();
        void Load();
        void Run();
        void SetWindow();
        void Uninitialize();
    private:
        void* framework_view;
        struct {
            FrameworkViewOverrideMethod<> initialize;
            FrameworkViewOverrideMethod<> load;
            FrameworkViewOverrideMethod<> run;
            FrameworkViewOverrideMethod<> set_window;
            FrameworkViewOverrideMethod<> uninitialize;
        } v_table;
    };
};

extern "C" abi::IFrameworkViewSource* create_app(rust_ffi::FrameworkView view);
