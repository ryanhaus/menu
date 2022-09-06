extern crate env_logger;
extern crate ws;
extern crate serde_json;

use std::thread;
use serde_json::Value as Json;
use ws::util::Token;

use crate::config;

pub fn create_ws_thread() {
    thread::spawn(move ||
        ws::listen(
            format!("{}:{}", config::IP_ADDR, config::WS_PORT),
            |out| { 
                Socket {
                    out: out,
                    room: String::from(""), 
                    socket_index: -1
                } 
            })
            .unwrap()
    );
}

#[derive(Debug)]
pub struct Socket {
    pub out: ws::Sender,
    pub room: String,
    socket_index: i32
}

pub static mut SOCKETS_IN_ROOMS: Vec<*mut Socket> = Vec::<*mut Socket>::new();

impl ws::Handler for Socket {
    fn on_open(&mut self, _shake: ws::Handshake) -> ws::Result<()> {
        self.out.timeout(5000, Token(1))
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // attempt to parse message as JSON
        if let Ok(json) = serde_json::from_str::<Json>(msg.as_text()?) {
            // attempt to get the message type
            let m_type_val = json.get("message_type");
            if m_type_val == None {
                return self.out.close_with_reason(
                    ws::CloseCode::Invalid,
                    "Packet was JSON, but did not contain a message type"
                )
            }

            // attempt to get message type value as a string
            let message_type = m_type_val.unwrap().as_str();
            if message_type == None {
                return self.out.close_with_reason(
                    ws::CloseCode::Invalid,
                    "JSON object contained a message type, but could not be parsed as a string"
                )
            }

            // match the message type
            match message_type.unwrap() {
                "GIVE_PAGE" => { // GIVE_PAGE: tells the server which page (monitor) this socket is from
                    // attempt to get the page value
                    let page_val = json.get("page");
                    if page_val == None {
                        return self.out.close_with_reason(
                            ws::CloseCode::Invalid,
                            "Packet of type 'GIVE_PAGE' did not contain a page value"
                        )
                    }

                    // attempt to get the page as a string
                    let page = page_val.unwrap().as_str();
                    if page == None {
                        return self.out.close_with_reason(
                            ws::CloseCode::Invalid,
                            "Packet of type 'GIVE_PAGE' did contain a page value, but it could not be parsed as a string"
                        )
                    }

                    self.room = String::from(page.unwrap());

                    unsafe {
                        // set the socket index in the vector
                        self.socket_index = SOCKETS_IN_ROOMS.len() as i32;
                        
                        // add to the vector
                        SOCKETS_IN_ROOMS.push(&mut *self);
                    }

                    Ok(())
                },

                // not one of the given message types
                _ => return self.out.close_with_reason(
                    ws::CloseCode::Invalid,
                    "Message type is not recognized"
                )
            }
        } else {
            return self.out.close_with_reason(
                ws::CloseCode::Invalid,
                "Received content could not be parsed as a JSON object"
            )
        }
    }

    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        if event != Token(1) {
            return self.out.close_with_reason(ws::CloseCode::Invalid, "Invalid timeout token");
        }

        self.out.timeout(5000, Token(1))
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        if self.socket_index == -1 { return; }

        unsafe {
            for i in self.socket_index as usize + 1 .. SOCKETS_IN_ROOMS.len() {
                if let Some(socket_pref) = SOCKETS_IN_ROOMS.get(i) {
                    let socket = &mut **socket_pref;

                    socket.socket_index -= 1;
                }
            }

            SOCKETS_IN_ROOMS.remove(self.socket_index as usize);
        }
    }
}