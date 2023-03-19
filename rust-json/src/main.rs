#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;
use maud::html;
use rocket::{Build, Rocket, routes, State};
use rocket::response::content::{RawHtml, RawJson};
use serde::{Deserialize, Serialize};


// #[get("/")]
// fn index(state: &State<RJState>) -> RawHtml<String> {
//
//     state.refresh_count.fetch_add(1,Ordering::Relaxed);
//     RawHtml(String::from(
//         html! {
//         ("Oatmeal, are you crazy?")
//             ("<script>alert(\"XSS\")</script>")
//     }))
// }

// #[get("/costi")]
// fn costis_page(state: &State<RJState>) -> RawHtml<String> {
//     let a = state.refresh_count.fetch_add(1,Ordering::Relaxed);
//
//     RawHtml(format!("<h1>{a}</h1>"))
// }

#[get("/read_question/<question_number>")]
fn read_question(question_number: usize , state: &State<RJState>) -> RawJson<String> {
    let game = state.game.read().unwrap();
    let question_text = match game.questions.get(question_number) {
        None => {
            Question{
                question_text: "No question found".to_string(),
                answers: vec!["OOPS".to_string()],
                correct_answer_index: 0,
            }
        }
        Some(quest) => {
            quest.clone()
        }
    };
    let json_question = serde_json::to_string(&question_text).unwrap();
    RawJson(format!("{}",json_question))
}

#[get("/choose_answer/<answer>")]
fn choose_answer(answer: usize ,state: &State<RJState>) -> RawHtml<String> {
    // get ref to game

    let game_ref = state.game.read().unwrap();
    let current = game_ref.current_question;
    let question = game_ref.questions.get(current);

    // if case for if answer == questions answer
    if answer == question.unwrap().correct_answer_index {
        return RawHtml(format!("<h1>Correct!</h1>")); // TODO: use json here
    }else{
        return RawHtml(format!("<h1>Incorrect</h1>"));
    }

    // RawHtml(format!("<h1>{a}</h1>"))
}

#[get("/debuggame")]
fn debuggame(state: &State<RJState>) -> RawHtml<String> {
    let lock = state.game.read().unwrap();
    RawHtml(format!("{:?}", lock))
}

#[launch]
fn rocket() -> Rocket<Build> {

    let mut g = Game{
        id: 0,
        current_question: 0,
        questions: vec![Question{
            question_text: "lmao?".to_string(),
            answers: vec!["1".to_string(),"2".to_string()],
            correct_answer_index: 0,
        }],
        users: vec![],
    };

    g.start_game();

    rocket::build()
        .manage(RJState{ refresh_count: Default::default(), game: RwLock::new(g) })
        .mount("/", routes![debuggame,read_question,choose_answer])
}

// posts

// accounts

// users

// state
pub struct RJState {
    refresh_count: AtomicU32,
    game: RwLock<Game>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Game {
    id: u32,
    current_question: usize,
    questions: Vec<Question>,
    users: Vec<User>,
}


impl Game {
    fn start_game(&mut self) {
        println!("game started");
    }

    // fn create_game() -> Self {
    //     Game
    // }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    score: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Question {
    question_text: String,
    answers: Vec<String>,
    correct_answer_index: usize,
}