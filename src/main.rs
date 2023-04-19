use serde::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use warp::Filter;

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

// abstract class?
#[derive(Clone)]
struct Store {
    grocery_list: Arc<RwLock<Items>>,
}

// constructor()
impl Store {
    fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn add_grocery_list_item(
    item: Item,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut r = store.grocery_list.write().unwrap();
    r.insert(item.name.clone(), item.quantity);
    Ok(warp::reply::json(&*r))
}

async fn get_grocery_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let result = store.grocery_list.read();
    Ok(warp::reply::json(&*result.unwrap()))
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_grocery_list_item);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_grocery_list);

    let routes = add_items.or(get_items);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
