use yew::prelude::*;
use yew_hooks::prelude::*;




use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct User {
    userid: i32,
    id: i32,
    title: String,
    body: String,
}

//!have 2 function
//! 1: makes the api call
//! 2: calls function one and displays 
//! it on the page since function 1 has to be a tokio task
