use crate::app::*;
use std::collections::VecDeque;
use zoon::*;

// ------ route_history ------

#[static_ref]
fn route_history() -> &'static Mutable<VecDeque<Route>> {
    Mutable::new(VecDeque::new())
}

fn push_to_route_history(route: Route) {
    let mut history = route_history().lock_mut();
    if history.len() == 2 {
        history.pop_back();
    }
    history.push_front(route);
}

pub fn previous_route() -> Option<Route> {
    route_history().lock_ref().get(1).cloned()
}

// ------ router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route: Option<Route>| async move {
        let route = match route {
            Some(route) => {
                push_to_route_history(route.clone());
                route
            }
            None => Route::Home,
        };

        match route {
            Route::Home => {
            }
        }
    })
}


#[route]
#[derive(Copy, Clone)]
pub enum Route {
    #[route()]
    Home,
}
