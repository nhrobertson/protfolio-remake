use yew::prelude::*;

use crate::components::tabitem::TabItem;
use crate::components::card::Card;

#[function_component]
pub fn Research() -> Html {
    html! {
        <div class="container">
            <h2>{"Research"}</h2>
            <p>{"Below you can find some research projects I have been involved with!"}</p>
            <div class="tab-item-container">
                <TabItem title="McNair Junior Fellows">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Research into Finite State Machines and Neural Networks"}</h3>
                            <p>{"During my time at South Carolina, between my Sophomore to Junior Year summer, I was awarded the McNair Junior Fellows grant. I conducted research into various forms of AI. This research fellowship resulted in a co-authorship on a published paper!"}</p>
                        </div>
                        <Card title="An Approach to Mitigate CNN Complexity on Domain-Specific Architectures">
                            <p>{"Here you can find the paper that I helped co-author during my time with McNair Junior Fellows"}</p>
                            <a href="https://www.researchgate.net/publication/379369922_An_Approach_to_Mitigate_CNN_Complexity_on_Domain-Specific_Architectures">{"Link"}</a> 
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Data Recovery From Video Event Recorder Imaging - AJR Publication">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Data Recovery From Video Event Recorder Imaging"}</h3>
                            <p>{"In this publication, you can find work that I did with my colleagues into dissection of Dash Camera devices with the intent of recovering dash cam event video files. We succesfully recovered these files and I assisted in writing some helpful applications in C# for the team."}</p>
                        </div>
                        <Card title="AJR Publication">
                            <p>{"Look out for the section in the May/June 2025 publication for AJR!"}</p>
                            <a href="">{"Coming Soon"}</a>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Coming Soon....">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Look out for a publication in SAE come next year!"}</h3>
                            <p>{"Being worked on..."}</p>
                        </div>
                        <Card title="Nothin yet!">
                            <p>{"Didn't ya hear! Nothing quite yet check back soon!"}</p>
                            <a href="">{"..."}</a>
                        </Card>
                    </div>
                </TabItem>
            </div>
        </div>
    }
}
