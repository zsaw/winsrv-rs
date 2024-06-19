# Winsrv

Simple implementation of windows service

## Example

On Cargo.toml:

```toml
[dependencies]
winsrv = { version = "0.0.1" }
```

Then, on your main.rs:

```rust
use std::sync::mpsc::{Receiver, RecvTimeoutError::Disconnected};
use std::time::Duration;
use winsrv::{run_service, SERVICE_CONTROL_SHUTDOWN, SERVICE_CONTROL_STOP};

fn srvmain(receiver: Receiver<u32>) {
    loop {
        match receiver.recv_timeout(Duration::from_secs(1)) {
            Ok(ctl) => match ctl {
                SERVICE_CONTROL_STOP | SERVICE_CONTROL_SHUTDOWN => break,
                _ => continue,
            },
            Err(err) => {
                if err == Disconnected {
                    break;
                }
            }
        };

        // Your code ...
    }
}

fn main() {
    run_service("Demo", srvmain);
}
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/zsaw/winsrv-rs/blob/main/LICENSE
