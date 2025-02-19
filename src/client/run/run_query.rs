use std::io::{BufRead, BufReader};
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::request::client::ClientRequest;
use dizi_lib::response::server::ServerBroadcastEvent;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::util::request::send_client_request;

pub fn run_query(context: &mut AppContext, query: String) -> DiziResult<()> {
    // server listener
    {
        let stream = context.clone_stream()?;
        let event_tx = context.events.event_tx.clone();

        let _ = thread::spawn(move || {
            let cursor = BufReader::new(stream);
            for line in cursor.lines().flatten() {
                event_tx.send(AppEvent::Server(line));
            }
        });

        // request for server state
        let request = ClientRequest::ServerQuery {
            query: query.clone(),
        };
        send_client_request(context, &request)?;

        // request for server state
        /*
        let request = ClientRequest::PlayerState;
        send_client_request(context, &request)?;
        */
    }

    loop {
        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Server(message) => {
                let server_broadcast_event: ServerBroadcastEvent = serde_json::from_str(&message)?;
                match server_broadcast_event {
                    ServerBroadcastEvent::ServerQuery { query } => {
                        println!("{}", query);
                        break;
                    }
                    ServerBroadcastEvent::PlayerState { mut state } => {
                        if !state.playlist_ref().is_empty() {
                            state.playlist_mut().set_cursor_index(Some(0));
                        }
                        let res = state.query(&query)?;
                        println!("{}", res);
                        break;
                    }
                    ServerBroadcastEvent::ServerError { msg } => {
                        println!("{}", msg);
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}
