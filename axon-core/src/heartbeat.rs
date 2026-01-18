use crate::client::axon::{Pulse, PulseAck, life_support_server::LifeSupport};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct HeartbeatService;

#[tonic::async_trait]
impl LifeSupport for HeartbeatService {
    async fn heartbeat(&self, request: Request<Pulse>) -> Result<Response<PulseAck>, Status> {
        // In a real system, we'd update a "Last Seen" map here
        // For MVP, just acknowledging keeps the agent alive
        let _pulse = request.into_inner();

        Ok(Response::new(PulseAck { alive: true }))
    }
}
