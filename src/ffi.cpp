#include "ffi.h"

void rust_ffi::FrameworkView::Initialize()
{
    v_table.initialize(framework_view);
}
void rust_ffi::FrameworkView::Load()
{
    v_table.load(framework_view);
}
void rust_ffi::FrameworkView::Run()
{
    v_table.run(framework_view);
}
void rust_ffi::FrameworkView::SetWindow()
{
    v_table.set_window(framework_view);
}
void rust_ffi::FrameworkView::Uninitialize()
{
    v_table.uninitialize(framework_view);
}
