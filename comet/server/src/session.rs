

pub enum SessionStatus {
    Idle,
    Active
}

pub struct ServerSession {
    id: usize,
    server: Addr<Server>,
    status: SessionState
}