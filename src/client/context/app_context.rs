use std::io;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use crate::config;
use crate::context::MessageQueue;
use crate::event::{AppEvent, Events};
use crate::fs::DirList;
use crate::history::{DirectoryHistory, History};
use crate::util::search::SearchPattern;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuitType {
    DoNot,
    Normal,
    Server,
}

pub struct AppContext {
    pub quit: QuitType,
    // event loop querying
    pub events: Events,
    // server unix socket
    pub stream: UnixStream,
    // app config
    config: config::AppConfig,

    _cwd: PathBuf,
    // directory history
    history: History,
    // context related to searching
    search_context: Option<SearchPattern>,
    // message queue for displaying messages
    message_queue: MessageQueue,
}

impl AppContext {
    pub fn new(config: config::AppConfig, cwd: PathBuf, stream: UnixStream) -> Self {
        let events = Events::new();
        let event_tx = events.event_tx.clone();

        Self {
            quit: QuitType::DoNot,
            stream,
            events,
            _cwd: cwd,
            history: History::new(),
            search_context: None,
            message_queue: MessageQueue::new(),
            config,
        }
    }

    pub fn flush_stream(&mut self) -> io::Result<()> {
        const NEWLINE: &[u8] = &['\n' as u8];
        self.stream.write(NEWLINE)?;
        Ok(())
    }

    // event related
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.events.event_tx.clone()
    }

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut config::AppConfig {
        &mut self.config
    }

    pub fn message_queue_ref(&self) -> &MessageQueue {
        &self.message_queue
    }
    pub fn message_queue_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
    }

    pub fn get_search_context(&self) -> Option<&SearchPattern> {
        self.search_context.as_ref()
    }
    pub fn set_search_context(&mut self, pattern: SearchPattern) {
        self.search_context = Some(pattern);
    }

    pub fn history_ref(&self) -> &History {
        &self.history
    }
    pub fn history_mut(&mut self) -> &mut History {
        &mut self.history
    }

    pub fn cwd(&self) -> &Path {
        &self._cwd
    }
    pub fn set_cwd(&mut self, path: &Path) {
        self._cwd = path.to_path_buf();
    }

    pub fn curr_list_ref(&self) -> Option<&DirList> {
        self.history.get(self.cwd())
    }
    pub fn curr_list_mut(&mut self) -> Option<&mut DirList> {
        self.history.get_mut(self._cwd.as_path())
    }
}
