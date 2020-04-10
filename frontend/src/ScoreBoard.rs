use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct ScoreBoardModel {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    // pub route: Option<ARoute>,
}

pub enum Msg {}

impl Component for ScoreBoardModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ScoreBoardModel { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round"> </hr>

                <div id="game-stream">
                <table>
                    <tr>
                        <th>{"Game-ID"}</th>
                        <th>{"Game Type"}</th>
                        <th>{"Player1"}</th>
                        <th>{"Player2"}</th>
                        <th>{"Winner"}</th>
                        <th>{"When Played"}</th>
                    </tr>
                    // <tr ng-repeat="game in games">
                    //     <td>{{ $index + 1 }}</td>
                    //     <td>{{game.gameType}}</td>
                    //     <td>{{game.Player1Name}}</td>
                    //     <td>{{game.Player2Name}}</td>
                    //     <td>{{game.WinnerName}}</td>
                    //     <td>{{game.GameDate | date:"h:mma 'on' MMM d, y"}}</td>
                    // </tr>
                    // TODO: backend probably
                </table>

                </div>
            </div>
        }
    }
}
