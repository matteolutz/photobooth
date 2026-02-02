use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::mpsc,
};

use std::sync::mpsc::{Receiver, Sender};

use tauri::async_runtime::JoinHandle;

pub trait CommRequest: Send + 'static {
    type Response: Send + 'static;
}

pub struct CommRequestEnvelope {
    type_id: TypeId,
    payload: Box<dyn Any + Send>,
    responder: Sender<Box<dyn Any + Send>>,
}

pub struct CommRequestDispatcher {
    tx: Sender<CommRequestEnvelope>,
}

impl CommRequestDispatcher {
    pub fn new(tx: Sender<CommRequestEnvelope>) -> Self {
        Self { tx }
    }

    fn _send<R>(&self, req: R) -> Receiver<Box<dyn Any + Send + 'static>>
    where
        R: CommRequest,
    {
        let (tx_resp, rx_resp) = mpsc::channel();

        let envelope = CommRequestEnvelope {
            type_id: TypeId::of::<R>(),
            payload: Box::new(req),
            responder: tx_resp,
        };

        self.tx.send(envelope).unwrap();

        rx_resp
    }

    pub fn send<R>(&self, req: R) -> JoinHandle<R::Response>
    where
        R: CommRequest,
    {
        let rx = self._send(req);
        tauri::async_runtime::spawn_blocking(move || {
            let response = rx.recv().expect("Channel has hung up");
            let boxed = response
                .downcast::<R::Response>()
                .expect("Response type mismatch");
            *boxed
        })
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
