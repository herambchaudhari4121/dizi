use dizi_commands::error::DiziResult;

use crate::context::AppContext;
use crate::util::search::SearchPattern;

use super::cursor_move;
use super::search_glob;
use super::search_string;

pub fn search_next(context: &mut AppContext) -> DiziResult<()> {
    if let Some(search_context) = context.get_search_context() {
        let index = match search_context {
            SearchPattern::Glob(s) => {
                search_glob::search_glob_fwd(context.curr_list_ref().unwrap(), s)
            }
            SearchPattern::String(s) => {
                search_string::search_string_fwd(context.curr_list_ref().unwrap(), s)
            }
        };
        if let Some(index) = index {
            let _ = cursor_move::cursor_move(index, context);
        }
    }
    Ok(())
}

pub fn search_prev(context: &mut AppContext) -> DiziResult<()> {
    if let Some(search_context) = context.get_search_context() {
        let index = match search_context {
            SearchPattern::Glob(s) => {
                search_glob::search_glob_rev(context.curr_list_ref().unwrap(), s)
            }
            SearchPattern::String(s) => {
                search_string::search_string_rev(context.curr_list_ref().unwrap(), s)
            }
        };
        if let Some(index) = index {
            let _ = cursor_move::cursor_move(index, context);
        }
    }
    Ok(())
}
