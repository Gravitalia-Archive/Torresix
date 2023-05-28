use tonic::{transport::Server, Request, Response, Status};

use torresix::torre_server::{Torre, TorreServer};
use torresix::{TorreRequest, TorreReply};

mod model;
pub mod torresix {
    tonic::include_proto!("torresix");
}

pub struct TorreSix {
    mobilenet: model::TensorflowModel,
    grenade: model::TensorflowModel
}

#[tonic::async_trait]
impl Torre for TorreSix {
    async fn torre_predict(
        &self,
        request: Request<TorreRequest>,
    ) -> Result<Response<TorreReply>, Status> {
        let body = request.into_inner();

        let result: String = match body.model {
            0 => {
                model::mobilenet::predict(self.mobilenet.clone(), body.data).unwrap_or_default()
            },
            1 => {
                model::grenade::predict(self.grenade.clone(), body.data).unwrap_or_default()
            },
            _ => {
                return Ok(Response::new(TorreReply {
                    model: body.model,
                    message: format!("Unknown model {}", body.model),
                    error: true
                }));
            }
        };

        Ok(Response::new(TorreReply {
            model: 0,
            message: result,
            error: false
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(TorreServer::new(TorreSix { mobilenet: model::mobilenet::init()?, grenade: model::grenade::init()? }))
        .serve(addr)
        .await?;

    Ok(())
}
