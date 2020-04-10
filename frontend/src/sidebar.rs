use crate::{
    display_window::DisplayWindow,
    page::{Page, PageProps},
};

use yew::{html::ChildrenWithProps, prelude::*, virtual_dom::VNode, Properties};
use yew_router::{agent::RouteRequest::GetCurrentRoute, matcher::RouteMatcher, prelude::*};

pub struct Sidebar {
    router_agent: Box<dyn Bridge<RouteAgent>>,
    route: Option<Route>,
    props: SidebarProps,
}

#[derive(Properties, Clone)]
pub struct SidebarProps {
    children: ChildrenWithProps<Page>,
}

pub enum Msg {
    UpdateRoute(Route),
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = SidebarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::UpdateRoute);
        let router_agent = RouteAgent::bridge(callback);
        Sidebar {
            router_agent,
            route: None,
            props,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.router_agent.send(GetCurrentRoute);
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(route) => {
                self.route = Some(route);
            }
        }
        true
    }

    fn view(&self) -> VNode {
        if let Some(route) = &self.route {
            let active_markdown_uri: Option<String> = self
                .props
                .children
                .iter()
                .filter_map(|child| {
                    if child.props.page_url == route.to_string() {
                        Some(child.props.uri)
                    } else {
                        None
                    }
                })
                .next();

            let list_items = self
                .props
                .children
                .iter()
                .map(|child| render_page_list_item(child.props, route));
            return html! {
                <>
                    <nav class="w3-sidenav w3-red w3-collapse w3-top w3-large w3-padding" style="z-index:3;width:350px;font-weight:bold" id="mySidenav">
                        <a href="javascript:void(0)" class="w3-padding-xlarge w3-hide-large w3-display-topleft w3-hover-white" style="width:100%">{"Close Menu"}</a>
                        <div class="w3-container">
                            <h3 class="w3-padding-64"><b>{"Play"}<br></br> {"Connect4 / TOOT-OTTO"}</b></h3>
                        </div>

                        {for list_items}
                    </nav>
                    <div style="overflow-y: auto; padding-left: 390px">
                    {
                        html !{
                            <DisplayWindow uri=active_markdown_uri />
                        }
                    }
                    </div>
                </>
            };
        } else {
            return html! {};
        }
    }
}

fn render_page_list_item(props: PageProps, route: &Route) -> Html {
    return html! {
        <div class="w3-padding w3-hover-white">
            <RouterAnchor<String> route=props.page_url.clone()> {&props.title} </RouterAnchor<String>>
        </div>
    };
}
