pub mod axon {
    tonic::include_proto!("axon");
}

use axon::Empty;
use axon::system_control_client::SystemControlClient;
use tonic::transport::Channel;

pub struct AxonClient {
    client: SystemControlClient<Channel>,
}

impl AxonClient {
    pub async fn connect(dst: String) -> Result<Self, tonic::transport::Error> {
        let client = SystemControlClient::connect(dst).await?;
        Ok(Self { client })
    }

    pub async fn get_stats(&mut self) -> Result<axon::SystemStats, tonic::Status> {
        let request = tonic::Request::new(Empty {});
        let response = self.client.get_stats(request).await?;
        Ok(response.into_inner())
    }
}
