use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
      window::Window,
    },
    webview::{WebViewBuilder, WebView},
};

struct window {
    window: Window,
    webview: WebView,
    event_loop: EventLoop,
}

impl window {
    pub fn create_window(&self, event_loop: &EventLoop<()>) -> wry::Result<Self> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Hello World")
            .build(&event_loop)?;
        let _webview = WebViewBuilder::new(window)?
            .with_url("localhost:7878")?
            .build()?;
        
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
        
        match event {
            Event::WindowEvent {
            event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
            }
        });
    
        Ok(Self {
            window,
            webview: _webview,
        })
    }
}