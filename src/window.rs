use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
      window::Window, platform::run_return::EventLoopExtRunReturn,
    },
    webview::{WebViewBuilder, WebView},
};

pub struct MDWindow {
    webview: WebView,
    event_loop: EventLoop<()>,
}

impl MDWindow {
    pub fn new(event_loop: EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Hello World")
            .build(&event_loop).unwrap();

        let _webview = WebViewBuilder::new(window).unwrap()
            .with_url("localhost:7878").unwrap()
            .build().unwrap();

        Self {
            webview: _webview,
            event_loop,
        }
    }

    pub async fn handle_window(&mut self) {
        self.event_loop.run_return(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
        
        match event {
            Event::WindowEvent {
            event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
            }
        });
    }
}