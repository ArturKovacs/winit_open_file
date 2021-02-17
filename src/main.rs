use std::path::{Path, PathBuf};
use std::sync::Arc;

use log::{info, trace, LevelFilter, SetLoggerError};
use syslog::{BasicLogger, Facility, Formatter3164};

use winit::{dpi::LogicalSize, event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, window::{Window, WindowBuilder}};

#[cfg(target_os = "macos")]
use winit::platform::macos::{EventLoopWindowTargetExtMacOS, FileOpenResult};

fn open_file(window: &Window, path: &Path) {
    // TODO place your file opening logic here.

    // WARNING:
    // This example only has a single window where only
    // the "last" file's path is shown even if multiple files
    // were selected at once.
    //
    // This is not an idiomatic way to handle file open
    // requests on macOS. Instead, applications usually open a new
    // window for each file or open all files in a single window.

    let filename = path.as_os_str();
    let filename = filename.to_owned().into_string().unwrap();
    let title = format!("> Opened: '{}'", filename);
    window.set_title(&title);
}

fn set_file_open_callback(window: &Arc<Window>, el_win_target: &EventLoopWindowTarget<()>, enable: bool) {
    #[cfg(target_os = "macos")]
    {
        if enable {
            let window = window.clone();
            el_win_target.set_file_open_callback(Some(Box::new(move |paths: Vec<PathBuf>| {
                for path in paths.iter() {
                    open_file(&*window, path.as_ref());
                }
                FileOpenResult::Success
            }) as _));
        } else {
            el_win_target.set_file_open_callback(None);
        }
    }
}

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "winit_open".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(LevelFilter::Trace))
        .unwrap();

    info!("hello world");

    let event_loop = EventLoop::new();

    // Systems other than macOS provide the file paths
    // as a program argument.
    let file_path;
    if let Some(arg) = std::env::args().skip(1).next() {
        file_path = Some(arg);
    } else {
        file_path = None;
    }

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Loading")
            .with_inner_size(LogicalSize::new(400.0, 200.0))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap(),
    );

    if let Some(file_path) = file_path {
        open_file(&window, file_path.as_ref());
    }

    // WARNING
    // Normally this boolean is not needed, it is only used to demonstrate
    // that the callback can be set to `None` while the application is running
    // and that restores the default behavior.
    let mut has_callback = true;
    set_file_open_callback(&window, &*event_loop, has_callback);

    event_loop.run(move |event, el_win_target, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(vkey),
                            state: ElementState::Released,
                            ..
                        },
                    ..
                } => {
                    match vkey {
                        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                        VirtualKeyCode::Space => {
                            has_callback = !has_callback;
                            set_file_open_callback(&window, el_win_target, has_callback);
                        }
                        _ => ()
                    }
                }
                _ => (),
            },
            _ => (),
        };
    });
}
