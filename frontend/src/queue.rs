use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};
use yew::prelude::*;
use serde_json::{to_string, to_vec, json};
//use yewdux::prelude::*;

use common::data_types::{
//     Algorithm,
//     Dataset,
//     ExperimentType,
//     Parameter,
//     Platform,
//     ProfilingConfiguration,
    Job,
    QueueMessage
};

#[function_component]
pub fn Queue() -> Html {
    let queue: Vec<Job> = Vec::new();
    let ws = match WebSocket::open("ws://localhost:3000/api/queue") { // `ws://` is required, otherwise there's an error.
        Ok(ws) => ws,
        Err(e) => {
            log!(format!("Error opening websocket: {:?}", e));
            panic!();
        }
    };
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        let message = json!(QueueMessage::RequestQueue);
        log!("sending first...");
        write.send(Message::Text(to_string(&message).unwrap())).await.unwrap();
        log!("Done!\nSecond...");
        write.send(Message::Bytes(to_vec(&message).unwrap())).await.unwrap();
        log!("Done!");
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            log!(format!("1. {:?}", msg));
        }
        log!("WebSocket Closed");
    });
    html! {
        <div>
            <h2>{"Queue"}</h2>

        </div>
    }
}
