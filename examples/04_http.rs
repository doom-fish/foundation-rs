#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;
    use std::sync::mpsc;
    use std::time::Duration;

    use foundation::network::{DelegateQueue, Event, Request, Session, SessionConfig};

    let queue = DelegateQueue::new("foundation.http-example")?;
    let session = Session::new(&queue, &SessionConfig::default())?;
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com/".into());
    let mut request = Request::new(url);
    request.deadline = Some(Duration::from_secs(30));
    request
        .headers
        .push(("Accept-Encoding".into(), "identity".into()));
    let (sender, receiver) = mpsc::channel();
    let task = session.data_task(&request, move |event| {
        let _ = sender.send(event);
    })?;
    task.resume();

    for event in receiver {
        match event {
            Event::Response(response) => eprintln!("HTTP {}", response.status),
            Event::Data(bytes) => std::io::stdout().write_all(&bytes)?,
            Event::Redirect { decision, .. } => decision.refuse(),
            Event::Complete(result) => {
                result?;
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {}
