#include "App.h"

App::App()
{
    m_view = nullptr;
}
App::App(rust_ffi::FrameworkView view)
{
    m_view = std::make_unique<rust_ffi::FrameworkView>(view);
}

IFrameworkView App::CreateView()
{
    return *this;
}

void App::Initialize(CoreApplicationView const & application_view)
{
    m_view->Initialize(application_view);
}

void App::Load(hstring const& entry_point)
{
    m_view->Load(entry_point);
}

void App::Uninitialize()
{
    m_view->Uninitialize();
}

void App::Run()
{
    m_view->Run();
}

void App::SetWindow(CoreWindow const & window)
{
    m_view->SetWindow(window);
}
