#![windows_subsystem = "windows"]

use winrt_core_app::{
    FrameworkView,
    FrameworkViewSource,
    com::{ComProp, ComIterMirror},
};

use std::sync::Arc;
use winrt::{
    *,
    windows::{
        foundation::{
            TypedEventHandler,
            numerics::{
                Vector2,
                Vector3,
            },
        },
        applicationmodel::core::CoreApplication,
        ui::{
            Color,
            core::{
                CoreWindow,
                CoreProcessEventsOption,
                PointerEventArgs,
            },
            composition::{
                Compositor,
                CompositionTarget,
                Visual,
                VisualCollection,
                CompositionObject,
            },
        },
    },
};
use atomic::{Atomic, Ordering};

#[derive(Default)]
struct App {
    target: ComProp<CompositionTarget>,
    visuals: ComProp<VisualCollection>,
    visuals_mirror: ComIterMirror<Visual>,
    selected: ComProp<Visual>,
    last: Atomic<usize>,
    offset: Atomic<(f32, f32)>,
}

impl App {
    fn set_target(&self, value: ComPtr<CompositionTarget>) {
        self.target.set(Some(value));
    }
    fn visuals(&self) -> Option<ComPtr<VisualCollection>> {
        self.visuals.get()
    }
    fn set_visuals(&self, value: ComPtr<VisualCollection>) {
        self.visuals.set(Some(value));
    }
    fn selected(&self) -> Option<ComPtr<Visual>> {
        self.selected.get()
    }
    fn select(&self, value: ComPtr<Visual>) {
        self.selected.set(Some(value));
    }
    fn deselect(&self) {
        self.selected.set(None);
    }
    fn last(&self) -> usize {
        self.last.load(Ordering::SeqCst)
    }
    fn increment_last(&self) {
        self.last.store(self.last() + 1, Ordering::SeqCst);
    }
    fn offset(&self) -> (f32, f32) {
        self.offset.load(Ordering::SeqCst)
    }
    fn set_offset(&self, x: f32, y: f32) {
        self.offset.store((x, y), Ordering::SeqCst);
    }
    fn add_visual(&self, x: f32, y: f32) {
        let compositor = self
            .visuals()
            .as_ref()
            .unwrap()
            .query_interface::<CompositionObject>()
            .unwrap()
            .get_compositor()
            .unwrap()
            .unwrap();
        let visual = compositor.create_sprite_visual().unwrap().unwrap();

        let colors = [
            Color { A: 0xDC, R: 0x5B, G: 0x9B, B: 0xD5 },
            Color { A: 0xDC, R: 0xED, G: 0x7D, B: 0x31 },
            Color { A: 0xDC, R: 0x70, G: 0xAD, B: 0x47 },
            Color { A: 0xDC, R: 0xFF, G: 0xC0, B: 0x00 },
        ];

        let next = self.last() % colors.len();
        let brush = compositor.create_color_brush_with_color(colors[next]).unwrap().unwrap();
        self.increment_last();
        let _ = visual.set_brush(&*brush.query_interface().unwrap());

        let block_size = 100.0;

        let visual = visual.query_interface::<Visual>().unwrap();

        let _ = visual.set_size(Vector2 {
            X: block_size,
            Y: block_size,
        });
        let _ = visual.set_offset(Vector3 {
            X: x - block_size / 2.0,
            Y: y - block_size / 2.0,
            Z: 0.0,
        });
        if let Some(visuals) = &self.visuals() {
            let _ = visuals.insert_at_top(&visual);
            self.visuals_mirror.push(visual.clone());
        }

        self.select(visual);
        self.set_offset(-block_size / 2.0, -block_size / 2.0);
    }
}
impl FrameworkView for App {
    fn run(self: Arc<Self>) -> Result<()> {
        let window = CoreWindow::get_for_current_thread()?.unwrap();
        let _ = window.activate();

        let dispatcher = window.get_dispatcher()?.unwrap();
        let _ = dispatcher.process_events(CoreProcessEventsOption::ProcessUntilQuit);
        Ok(())
    }
    fn set_window(self: Arc<Self>, window: ComPtr<CoreWindow>) -> Result<()> {
        let compositor = Compositor::new();
        let root = compositor.create_container_visual()?.unwrap();
        let target = compositor.create_target_for_current_view()?.unwrap();
        let _ = target.set_root(&*root.clone().query_interface().unwrap());
        self.set_target(target);
        self.set_visuals(root.get_children()?.unwrap());

        let this = self.clone();
        let pointer_pressed_handler = TypedEventHandler::new(move |_sender, args: *mut PointerEventArgs| {
            let args = unsafe { &*args };

            let point = args
                .get_current_point()?
                .unwrap()
                .get_position()
                .unwrap();
            let visuals = this
                .visuals()
                .unwrap();
            let visuals_vec = this.visuals_mirror.vec();

            for visual in visuals_vec.iter() {
                let offset = visual.get_offset().unwrap();
                let size = visual.get_size().unwrap();
                let hit_test = point.X >= offset.X &&
                    point.X < offset.X + size.X &&
                    point.Y >= offset.Y &&
                    point.Y < offset.Y + size.Y;

                if hit_test {
                    this.select(visual.clone());
                    this.set_offset(offset.X - point.X, offset.Y - point.Y);
                }
            }
            if let Some(selected) = this.selected() {
                let _ = visuals.remove(&selected);
                let _ = visuals.insert_at_top(&selected);
            } else {
                this.add_visual(point.X, point.Y);
            }
            Ok(())
        });

        let this = self.clone();
        let pointer_moved_handler = TypedEventHandler::new(move |_sender, args: *mut PointerEventArgs| {
            if let Some(selected) = &this.selected() {
                let args = unsafe { &*args };

                let point = args
                    .get_current_point()?
                    .unwrap()
                    .get_position()
                    .unwrap();

                let (x, y) = this.offset();
                let _ = selected.set_offset(Vector3 {
                    X: point.X + x,
                    Y: point.Y + y,
                    Z: 0.0,
                });
            }
            Ok(())
        });

        let this = self.clone();
        let pointer_released_handler = TypedEventHandler::new(move |_sender, _args: *mut PointerEventArgs| {
            this.deselect();
            Ok(())
        });

        window.add_pointer_pressed(&pointer_pressed_handler)?;
        window.add_pointer_moved(&pointer_moved_handler)?;
        window.add_pointer_released(&pointer_released_handler)?;
        Ok(())
    }
}

fn main() {
    init_apartment(ApartmentType::MTA);

    let app = App::default().com();

    let _ = CoreApplication::run(&app);
}
