enum Event {
    Tick,
    SleepRequest { unit_id: u32, duration: u32 },
    Fault { unit_id: u32, fault_code: u32 },
}

struct EventQueue {
    buffer: [Option<Event>; N],
    read: usize,
    write: usize,   
    len: usize,
}
impl EventQueue {
    fn front(&self) -> Option<&Event> {
        if self.len == 0 {
            return None;
        }
        self.buffer[self.read].as_ref()
    }
    fn back(&self) -> Option<&Event> {
        if self.len == 0 {
            return None;
        }
        self.buffer[(self.write + N - 1) % N].as_ref()
    }
    fn push(&mut self, event: Event) {
        if self.len == N {  
            return;
        }
        self.buffer[self.write] = Some(event);
        self.write = (self.write + 1) % N;
        self.len += 1;
    }
    fn pop(&mut self) -> Option<Event> {
        if self.len == 0 {  
            return None;
        }
        let event = self.buffer[self.read].take();
        self.read = (self.read + 1) % N;
        self.len -= 1;
        event
    }
}