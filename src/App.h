#include "ffi.h"

extern "C" ABI::Windows::ApplicationModel::Core::IFrameworkViewSource*
create_app(rust_ffi::FrameworkView view);
