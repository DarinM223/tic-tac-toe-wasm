#![feature(test)]

#[macro_use]
extern crate stdweb;
extern crate test;

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
            if (cloned_cell.text_content().is_some()
                && cloned_cell.text_content() != Some("".to_string()))
                || board.borrow().has_won().is_some()
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

    let button = document().query_selector("#reset").unwrap();
    let board = board.clone();
    button.add_event_listener(move |_: ClickEvent| {
        board.borrow_mut().reset();
        clear_board();
    });

    stdweb::event_loop();
}

fn clear_board() {
    let cells = document().query_selector_all("li");
    let possible_classes = ["x", "o"];
    for cell in cells {
        let cell: Element = cell.try_into().unwrap();
        cell.set_text_content("");
        for class in possible_classes.iter() {
            if cell.class_list().contains(class) {
                cell.class_list().remove(class);
            }
        }
    }
}
