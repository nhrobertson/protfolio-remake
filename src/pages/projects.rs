use yew::prelude::*;

use crate::components::tabitem::TabItem;
use crate::components::card::Card;

#[function_component]
pub fn Projects() -> Html {
    html! {
        <div class="container">
            <h2>{"Projects"}</h2>
            <p>{"Here is a list of non-research related projects that I have worked on. Most can be found on my GitHub."}</p>
            <div class="tab-item-container">
                <TabItem title="PondPi">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"PondPi"}</h3>
                            <p>{"TODO"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="RustTODO">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Simple Task App In Rust"}</h3>
                            <p>{"TODO"}</p>
                        </div> 
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="This Website">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Portfolio-Remake"}</h3>
                            <p>{"TODO"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Welcome Together">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Welcome Together Web Application"}</h3>
                            <p>{"TODO"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Old Portoflio">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Original Portfolio"}</h3>
                            <p>{"TODO"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Impact Forensics">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Work Related Projects"}</h3>
                            <p>{"TODO"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="The Digital Custodian">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"monday.com Marketplace Application"}</h3>
                            <p>{"TODO"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <a href=""></a>
                        </Card>
                    </div>
                </TabItem>
            </div>
        </div>
    }
}
