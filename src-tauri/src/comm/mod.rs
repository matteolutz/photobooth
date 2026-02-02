use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::mpsc::{self, Receiver},
};

use tokio::sync::oneshot;

pub trait CommRequest: Send + 'static {
    type Response: Send + 'static;
}

pub struct CommRequestEnvelope {
    type_id: TypeId,
    payload: Box<dyn Any + Send>,
    responder: oneshot::Sender<Box<dyn Any + Send>>,
}

pub struct CommRequestDispatcher {
    tx: mpsc::Sender<CommRequestEnvelope>,
}

impl CommRequestDispatcher {
    pub fn new(tx: mpsc::Sender<CommRequestEnvelope>) -> Self {
        Self { tx }
    }

    /// Sends a request and returns a future that resolves to the response.
    /// This integrates with Tauri's async runtime and supports proper async/await.
    pub async fn send<R>(&self, req: R) -> Result<R::Response, String>
    where
        R: CommRequest,
    {
        let (tx_resp, rx_resp) = oneshot::channel();

        let envelope = CommRequestEnvelope {
            type_id: TypeId::of::<R>(),
            payload: Box::new(req),
            responder: tx_resp,
        };

        self.tx
            .send(envelope)
            .map_err(|_| "Failed to send request to camera thread".to_string())?;

        let response = rx_resp
            .await
            .map_err(|_| "Camera thread did not respond".to_string())?;

        let boxed = response
            .downcast::<R::Response>()
            .map_err(|_| "Response type mismatch".to_string())?;

        Ok(*boxed)
    }
}

#[derive(Copy, Clone)]
pub struct CommRequestHandlerContext {}

pub struct CommRequestHandler {
    rx: Receiver<CommRequestEnvelope>,
    handlers: HashMap<
        TypeId,
        Box<dyn Fn(Box<dyn Any + Send>, CommRequestHandlerContext) -> Box<dyn Any + Send> + Send>,
    >,
}

impl CommRequestHandler {
    pub fn new(rx: Receiver<CommRequestEnvelope>) -> Self {
        Self {
            rx,
            handlers: HashMap::new(),
        }
    }

    pub fn register<R, F>(&mut self, handler: F)
    where
        R: CommRequest,
        F: Fn(R, CommRequestHandlerContext) -> R::Response + Send + 'static,
    {
        let type_id = TypeId::of::<R>();

        let wrapper = move |boxed: Box<dyn Any + Send>,
                            context: CommRequestHandlerContext|
              -> Box<dyn Any + Send> {
            let req = *boxed.downcast::<R>().unwrap();
            let res = handler(req, context);
            Box::new(res)
        };

        self.handlers.insert(type_id, Box::new(wrapper));
    }

    pub fn handle_all(&self, context: CommRequestHandlerContext) {
        for env in self.rx.try_iter() {
            let handler = self.handlers.get(&env.type_id).expect(
                format!("No handler registered for request type {:?}", env.type_id).as_str(),
            );

            let res = handler(env.payload, context);
            let _ = env.responder.send(res);
        }
    }
}
