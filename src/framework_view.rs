use std::{
    ffi::c_void,
    sync::Arc,
};
use winrt::{
    *,
    windows::{
        ui::core::CoreWindow,
        applicationmodel::core::{
            IFrameworkViewSource,
            CoreApplicationView,
        }
    },
};

type HSTRING = <HString as RtType>::Abi;

extern "C" {
    fn create_app(view: FrameworkViewFfi) -> *mut IFrameworkViewSource;
}

pub trait FrameworkView {
    fn initialize(self: Arc<Self>, application_view: ComPtr<CoreApplicationView>) -> Result<()> { Ok(()) }
    fn load(self: Arc<Self>, entry_point: HString) -> Result<()> { Ok(()) }
    fn run(self: Arc<Self>) -> Result<()> { Ok(()) }
    fn set_window(self: Arc<Self>, window: ComPtr<CoreWindow>) -> Result<()> { Ok(()) }
    fn uninitialize(self: Arc<Self>) -> Result<()> { Ok(()) }
}

pub trait FrameworkViewSource {
    fn com(self) -> ComPtr<IFrameworkViewSource>;
}
impl<T: FrameworkView + 'static> FrameworkViewSource for T {
    fn com(self) -> ComPtr<IFrameworkViewSource> {
        let view = ffi(self);

        unsafe { ComPtr::wrap(create_app(view)) }
    }
}

fn this(ptr: *mut c_void) -> Arc<FrameworkView> {
    let ptr = ptr as *mut Arc<FrameworkView>;
    let ptr = unsafe { &mut *ptr };
    ptr.clone()
}

#[repr(C)]
pub struct FrameworkViewFfi {
    data: *mut c_void,
    v_table: FrameworkViewVTable,
}
#[repr(C)]
pub struct FrameworkViewVTable {
    initialize: extern "C" fn(*mut c_void, *mut CoreApplicationView),
    load: extern "C" fn(*mut c_void, entry_point: HSTRING),
    run: extern "C" fn(*mut c_void),
    set_window: extern "C" fn(*mut c_void, *mut CoreWindow),
    uninitialize: extern "C" fn(*mut c_void),
}


pub fn ffi<A: FrameworkView + 'static>(framework_view: A) -> FrameworkViewFfi {
    let data: *mut Arc<dyn FrameworkView> = Box::into_raw(Box::new(Arc::new(framework_view)));
    let data = data as *mut c_void;

    extern "C" fn initialize(ptr: *mut c_void, application_view: *mut CoreApplicationView) {
        let application_view = unsafe { ComPtr::wrap(application_view) };
        let _ = this(ptr).initialize(application_view);
    }
    extern "C" fn load(ptr: *mut c_void, entry_point: HSTRING) {
        let entry_point = unsafe { HString::wrap(entry_point) };
        let _ = this(ptr).load(entry_point);
    }
    extern "C" fn run(ptr: *mut c_void) {
        let _ = this(ptr).run();
    }
    extern "C" fn set_window(ptr: *mut c_void, window: *mut CoreWindow) {
        let window = unsafe { ComPtr::wrap(window) };
        let _ = this(ptr).set_window(window);
    }
    extern "C" fn uninitialize(ptr: *mut c_void) {
        let _ = this(ptr).uninitialize();
    }

    FrameworkViewFfi {
        data,
        v_table: FrameworkViewVTable {
            initialize,
            load,
            run,
            set_window,
            uninitialize,
        },
    }
}
