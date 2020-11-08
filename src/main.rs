

use std::rc::Rc;
use std::path::{Path, PathBuf};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};

#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopWindowTargetExtMacOS;

fn open_file(window: &Window, path: &Path) {
    // TODO place your file opening logic here.
    let filename = path.as_os_str();
    let filename = filename.to_owned().into_string().unwrap();
    window.set_title(filename.as_str());
}

fn main() {
    let event_loop = EventLoop::new();

    // Systems other than macOS provide the file paths
    // as a program argument.
    let file_path;
    if let Some(arg) = std::env::args().skip(1).next() {
        file_path = Some(arg);
    } else {
        file_path = None;
    }

    let window = Rc::new(WindowBuilder::new()
        .with_title("Loading")
        .with_inner_size(LogicalSize::new(400.0, 200.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap());

    if let Some(file_path) = file_path {
        open_file(&window, file_path.as_ref());
    }

    #[cfg(target_os = "macos")]
    {
        let window = window.clone();
        event_loop.set_open_files_callback(Some(move |files: &[PathBuf]| {
            if let Some(filename) = files.iter().next() {
                open_file(&window, filename.as_ref());
            }
        }));
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        };
    });
}
