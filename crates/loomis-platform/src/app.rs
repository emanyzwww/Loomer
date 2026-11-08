//! Event loop and window lifecycle.
//!
//! [`run`] creates the event loop, opens one window, and forwards backend activity to an
//! [`AppHandler`], which `loomis-runtime` implements. This layer drives the handler; it never
//! interprets what the handler does with the events.
//!
//! # Loop rules
//!
//! - The loop waits for events (`ControlFlow::Wait`) instead of polling, so an idle window does not
//!   consume CPU.
//! - The window is created in `resumed`, which some platforms call more than once; a guard keeps
//!   creation to a single window.
//! - `resumed` returns `()`, so failures cannot use `?`; a failed window creation is reported and
//!   the loop is stopped.

use crate::error::Error;
use crate::event::{self, Event};
use crate::window::Window;
use loomis_core::window::WindowOptions;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{WindowAttributes, WindowId};

/// Callbacks implemented by the runtime.
///
/// The platform layer calls these in response to backend activity. Methods run on the event loop
/// thread, so they must return quickly: blocking here freezes the window.
pub trait AppHandler {
    /// Called once, right after the window is created.
    fn created(&mut self, window: &Window);
    /// Called for every translated [`Event`].
    fn event(&mut self, event: Event);
    /// Called when the platform asks for a frame.
    fn redraw(&mut self, window: &Window);
}

/// Creates the event loop, opens one window, and blocks until the loop exits.
///
/// Blocks the calling thread for the lifetime of the window. Returns after the window is closed or
/// the loop is stopped.
///
/// # Errors
///
/// Returns [`Error::EventLoop`] when the event loop cannot be created or run. Window creation
/// happens inside the loop, where a `Result` cannot be returned; a failure there stops the loop.
pub fn run(handler: Box<dyn AppHandler>, options: WindowOptions) -> Result<(), Error> {
    let event_loop = EventLoop::new()?;
    // Without Wait the loop spins and burns a core.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = PlatformApp {
        handler,
        options,
        window: None,
        scale: 1.0,
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}

/// State owned by the event loop.
struct PlatformApp {
    /// Handler that receives lifecycle callbacks.
    handler: Box<dyn AppHandler>,
    /// Description of the window to create.
    options: WindowOptions,
    /// Set once the window exists; also guards against creating a second one.
    window: Option<Window>,
    /// Scale factor sampled in `resumed`, used to report logical sizes.
    scale: f32,
}

impl ApplicationHandler for PlatformApp {
    /// Creates the window on the first call; later calls are ignored.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // resumed may be called more than once; create the window only once.
        if self.window.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title(self.options.title.as_str())
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.options.size.width as f64,
                self.options.size.height as f64,
            ));

        // resumed returns (), so `?` is unavailable; report and stop instead.
        let Ok(inner) = event_loop.create_window(attrs) else {
            eprintln!("[loomis] failed to create window");
            event_loop.exit();
            return;
        };

        self.scale = inner.scale_factor() as f32;
        let window = Window { inner };
        self.handler.created(&window);
        self.window = Some(window);
    }

    /// Routes backend events: close stops the loop, redraw reaches the handler, everything else is
    /// translated first.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    self.handler.redraw(window);
                }
            }
            _ => {
                // Translate into Loomis events so winit types stay in this crate.
                if let Some(ev) = event::translate(&event, self.scale) {
                    self.handler.event(ev);
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
        }
    }
}
