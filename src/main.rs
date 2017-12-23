#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod game;

use std::cell::RefCell;
use std::rc::Rc;

use game::{Board, CellMarking, Move};
use stdweb::unstable::TryInto;
use stdweb::web::{document, Element, IElement, IEventTarget, INode};
use stdweb::web::event::ClickEvent;

fn main() {
    stdweb::initialize();

    let board = Rc::new(RefCell::new(Board::new()));

    let cells = document().query_selector_all("li");
    for (i, cell) in cells.iter().enumerate() {
        let cell: Element = cell.try_into().unwrap();
        let board = board.clone();
        let cloned_cell = cell.clone();
        cell.add_event_listener(move |_: ClickEvent| {
            if cloned_cell.text_content().is_some()
                && cloned_cell.text_content() != Some("".to_string())
            {
                return;
            }

            cloned_cell.set_text_content("X");
            cloned_cell.class_list().add("x");

            let board_move = Move {
                position: board.borrow().index_to_pos(i),
                marking: CellMarking::X,
            };
            board.borrow_mut().apply_move(&board_move);

            let response = board.borrow_mut().next_move(CellMarking::O);
            let response_text = format!("{:?}", response);
            js! {
                alert(@{response_text});
            }
            if let Some(resp_move) = response {
                board.borrow_mut().apply_move(&resp_move);
                let index = board.borrow().cell_index(&resp_move.position);
                let response_cell = document()
                    .query_selector(&format!("#cell-{}", index))
                    .unwrap();
                response_cell.set_text_content("O");
                response_cell.class_list().add("o");
            }
        });
    }

    stdweb::event_loop();
}
