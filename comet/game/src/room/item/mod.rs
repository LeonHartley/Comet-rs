// use actix::{Actor, Context};
// use actix::Addr;
// use actix::Handler;
// use actix::Message;
// use model::room::map::Pos;
// use player::Player;
// use room::item::event::ItemEvent;
// use serde::{Derialize, Serialize};
// use std::sync::Arc;

// pub mod event;
// pub mod factory;

// pub struct ItemData<T> where T: Sized + Clone + Serialize {
//     data: T,
//     position: Pos,
// }

// pub trait ItemEventHandler<T> where T: Sized + Clone + Serialize {
//     fn on_interact(player: Option<Addr<Player>>, is_wired: bool) {}

//     fn data(&self) -> &ItemData<T>;
// }

// #[derive(Serialize, Deserialize)]
// pub struct BasicItemState {
//     pub state: i32,
//     pub data: String,
// }

// pub struct Item {
//     data: ItemData<BasicItemState>
// }

// impl ItemEventHandler<BasicItemState> for Item {
//     fn data(&self) -> &ItemData<BasicItemState> {
//         &self.data
//     }
// }

// impl Actor for Item {
//     type Context = Context<Self>;
// }

// impl Handler<ItemEvent> for Item {
//     type Result = ();

//     fn handle(&mut self, msg: ItemEvent, ctx: &Context<Self>) {
//         match msg {
//             ItemEvent::Interact { player, is_wired } => self.on_interact(player, is_wired),
//             //ItemEvent::Move { pos: Pos } =>
//         }
//     }
// }