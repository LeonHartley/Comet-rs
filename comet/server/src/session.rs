use GameCodec;

pub enum SessionStatus {
    Idle,
    Active
}

type NetworkWriter = actix::io::FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    id: usize,
    server: Addr<Server>,
    status: SessionState,
    writer: NetworkWriter
}

impl ServerSession {
    pub fn new(id: usize, server: Addr<Server>, writer: NetworkWriter) -> Self {
        Self {
            id,
            server,
            status: SessionStatus::Idle,
            writer
        }
    }
}