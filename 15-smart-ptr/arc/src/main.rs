use std::collections::HashMap;
use std::sync::{RwLock, Arc};

#[derive(Clone)]
pub struct Notifier {
    id: i32,
    nodes_state: Arc<RwLock<HashMap<i32, bool>>>,
}

impl Notifier {
    fn new(id: i32, nodes_state: Arc<RwLock<HashMap<i32, bool>>>) -> Self {
        Self {
            id,
            nodes_state,
        }
    }
}

impl Notifier {
    fn state_changed(&mut self, status: bool) {
        let mut map = self.nodes_state.write().unwrap();
        *map.entry(self.id).or_insert(status) = status;
    }
}

fn main() {
    let nodes_state: Arc<RwLock<HashMap<i32, bool>>> =  Arc::new(RwLock::new(HashMap::new()));

    let state1 = nodes_state.clone();
    let state2 = nodes_state.clone();

    let state3 = nodes_state.clone();
    let mut notifier3 = Notifier::new(3.into(), state3);

    {
        let mut map = state1.write().unwrap();
        *map.entry(1.into()).or_insert(true) = true;
    }
    {
        let mut map = state2.write().unwrap();
        *map.entry(2.into()).or_insert(true) = true;
    }
    notifier3.state_changed(true);

    for (k, v) in nodes_state.read().unwrap().iter() {
        println!("{}: {}", k, v);
    }

    notifier3.state_changed(false);
    println!("{} -> {}", 3, nodes_state.read().unwrap()[&3]);

}
