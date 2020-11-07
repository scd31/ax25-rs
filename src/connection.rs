use crate::frame::{Address, Ax25Frame, Disconnect, FrameContent};
use crate::tnc::{Tnc, TncError};
use std::sync::mpsc::Receiver;
use std::time::Instant;

pub struct Connection {
    tnc: Tnc,
    receiver: Receiver<Ax25Frame>,
    state: ConnectionState,
    retry_count: u32,
    us: Address,
    them: Address,
    // Outstanding iframe or p-bit
    t1: Instant,
    // Idle supervision (keep-alive)
    t3: Instant,
}

impl Connection {
    pub fn disconnect(&mut self) -> Result<(), TncError> {
        match self.state {
            ConnectionState::Disconnected => {}
            ConnectionState::AwaitingConnection => {}
            ConnectionState::AwaitingV22Connection => {}
            ConnectionState::Connected => {
                // TODO clear queue
                self.retry_count = 0;
                self.tnc.send_frame(&Ax25Frame {
                    source: self.us.clone(),
                    destination: self.them.clone(),
                    route: vec![],
                    command_or_response: None,
                    content: FrameContent::Disconnect(Disconnect { poll: true }),
                })?;

                self.t1 = Instant::now();
                self.state = ConnectionState::AwaitingRelease
            }
            ConnectionState::AwaitingRelease => {}
            ConnectionState::TimerRecovery => {}
        }

        Ok(())
    }

    pub fn send(&mut self) {}

    fn handle(&mut self) {
        if let Ok(frame) = self.receiver.recv() {
            match self.state {
                ConnectionState::Disconnected => {}
                ConnectionState::AwaitingConnection => {}
                ConnectionState::AwaitingV22Connection => {}
                ConnectionState::Connected => {
                    self.handle_connected(frame);
                }
                ConnectionState::AwaitingRelease => {}
                ConnectionState::TimerRecovery => {}
            }
        }
    }

    fn handle_connected(&mut self, frame: Ax25Frame) {
        match frame.content {
            FrameContent::Information(_) => {}
            FrameContent::ReceiveReady(_) => {}
            FrameContent::ReceiveNotReady(_) => {}
            FrameContent::Reject(_) => {}
            FrameContent::SetAsynchronousBalancedMode(_) => {}
            FrameContent::Disconnect(_) => {
                self.retry_count = 0;
            }
            FrameContent::DisconnectedMode(_) => {}
            FrameContent::UnnumberedAcknowledge(_) => {}
            FrameContent::FrameReject(_) => {}
            FrameContent::UnnumberedInformation(_) => {}
            FrameContent::UnknownContent(_) => {}
        }
    }
}

#[derive(Debug)]
enum ConnectionState {
    Disconnected,
    AwaitingConnection,
    AwaitingV22Connection,
    Connected,
    AwaitingRelease,
    TimerRecovery,
}
