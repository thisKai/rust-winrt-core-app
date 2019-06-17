use std::ffi::c_void;

pub trait FrameworkView {
    fn initialize(&mut self) {}
    fn load(&mut self) {}
    fn run(&mut self) {}
    fn set_window(&mut self) {}
    fn uninitialize(&mut self) {}
}

macro_rules! vtable_methods {
    ($fn_name: ident) => {
        extern "C" fn $fn_name(framework_view: *mut c_void) {
            let framework_view = framework_view as *mut Box<FrameworkView>;
            let framework_view = unsafe { &mut *framework_view };
            framework_view.$fn_name()
        }
    };
    ($($fn_name: ident,)+) => {
        $(
            vtable_methods!($fn_name);
        )+
    };
}

#[repr(C)]
pub struct FrameworkViewFfi {
    data: *mut c_void,
    v_table: FrameworkViewVTable,
}
#[repr(C)]
pub struct FrameworkViewVTable {
    initialize: extern "C" fn(*mut c_void),
    load: extern "C" fn(*mut c_void),
    run: extern "C" fn(*mut c_void),
    set_window: extern "C" fn(*mut c_void),
    uninitialize: extern "C" fn(*mut c_void),
}


pub fn ffi<A: FrameworkView + 'static>(framework_view: A) -> FrameworkViewFfi {
    let data: *mut Box<dyn FrameworkView> = Box::into_raw(Box::new(Box::new(framework_view)));
    let data = data as *mut c_void;

    vtable_methods![
        initialize,
        load,
        run,
        set_window,
        uninitialize,
    ];

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
