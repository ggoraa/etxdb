use tokio::sync::mpsc;

// shamelessly stolen from https://stackoverflow.com/a/73463169/18430533

pub struct InteractiveStdin {
    chan: mpsc::Receiver<std::io::Result<String>>,
}

impl InteractiveStdin {
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel(16);
        std::thread::spawn(move || {
            for line in std::io::stdin().lines() {
                if send.blocking_send(line).is_err() {
                    return;
                }
            }
        });
        InteractiveStdin { chan: recv }
    }

    /// Get the next line from stdin.
    ///
    /// Returns `Ok(None)` if stdin has been closed.
    ///
    /// This method is cancel safe.
    pub async fn next_line(&mut self) -> std::io::Result<Option<String>> {
        self.chan.recv().await.transpose()
    }
}

impl Default for InteractiveStdin {
    fn default() -> Self {
        Self::new()
    }
}
