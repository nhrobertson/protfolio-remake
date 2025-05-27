use yew::prelude::*;

use crate::components::tabitem::TabItem;
use crate::components::card::Card;

#[function_component]
pub fn Projects() -> Html {
    html! {
        <div class="container">
            <div class="info">
                <h2>{"Projects"}</h2>
                <p>{"Here is a list of non-research related projects that I have worked on. Most can be found on my GitHub."}</p>
            </div>
            <div class="tab-item-container">
                <TabItem title="PondPi">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"PondPi"}</h3>
                            <p>{"This is a simple Raspberry Pi Pico W project which targets controlling a outdoor pond with two pumps, a hose water input, and a waterline detector. This project publishes a live control panel on the device's connected local network. This allows users to set timers for activation of outputs and set thresholds for inputs. These configurations are stored locally on the device. The project is written in MicroPython using Thonny. This is a very basic, non-modular version. I plan on reimplenting in Rust soon, adding modular features!"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <div class="links-card-container">
                            <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" class="card-img" />
                            <a href="https://github.com/nhrobertson/PondPi">{"PondPi Repository"}</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
                 <TabItem title="This Website">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Portfolio-Remake"}</h3>
                            <p>{"This website is one of my most recent projects! I set out to remake my portfolio and add new features to it aswell. This website is created in Rust using the Yew framework. This website is actually hosted locally on a Raspberry Pi 5! I wanted to get some experience with creating all of the features from scratch in Rust (not the actual html interpreter but the components) and learn how to host my own web server!"}</p>
                        </div>
                        <Card title="GitHub Link">
                            <div class="links-card-container">
                            <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" class="card-img" />
                            <a href="https://github.com/nhrobertson/protfolio-remake">{ "Portfolio-Remake Repository" }</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>

                <TabItem title="RustTODO">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Simple Task App In Rust"}</h3>
                            <p>{"This was my first attempt at a full fledged Rust project. The target is to implement a CLI and GUI portions of the project. The tasks are stored locally use SQLite. This project is still being worked on."}</p>
                        </div> 
                        <Card title="GitHub Link">
                            <div class="links-card-container">
                            <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" class="card-img" />
                            <a href="https://github.com/nhrobertson/RustTODO">{ "RustTODO Repository" }</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="RustLS">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Simple Replication of LS in Rust"}</h3>
                            <p>{"This was a very basic project in Rust that I used to learn the fundamentals. It replicates the Unix 'ls' CLI command and displays repositories asked, color coded."}</p>
                        </div> 
                        <Card title="GitHub Link">
                            <div class="links-card-container">
                            <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" class="card-img" />
                            <a href="https://github.com/nhrobertson/RustLS">{ "RustLS Repository" }</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Welcome Together">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Welcome Together Web Application"}</h3>
                            <p>{"Welcome Together is a web application created using React, JavaScript, and handles it's backend and hosting through Firebase. This project is a full fledged event social media web app which features accounts, creation of event objects, notifications, and much more. However this project is not maintained anymore and that has resulted in the Search feature being deprecated."}</p>
                        </div>
                        <Card title="Welcome Together Link">
                            <div class="links-card-container">
                            <img src="https://welcome-together.web.app/static/media/logo.2dcd0910daabc69bad6d.png" class="card-img" />
                            <a href="https://welcome-together.web.app">{"Link"}</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Old Portoflio">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Original Portfolio"}</h3>
                            <p>{"My original portfolio was created right after my graduation from UofSC. It uses React, JavaScript and hosted on Firebase. It features some basic features and uses React features."}</p>
                        </div>
                        <Card title="GitHub Link">
                            <div class="links-card-container">
                            <img src="https://github.githubassets.com/assets/GitHub-Mark-ea2971cee799.png" class="card-img" />
                            <a href="https://github.com/nhrobertson/personalportfolio">{"Original Portfolio Repository"}</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="Impact Forensics">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"Work Related Projects"}</h3>
                            <p>{"I have done various projects with Impact including low level binary data decoders, embedded device dissection projects. Make sure to check out the research page to see more!"}</p>
                        </div>
                        <Card title="Impact Forensics">
                            <div class="links-card-container">
                            <img src="https://impact-forensics.com/wp-content/uploads/2024/08/Impact-Forensics-Logo-Inverted-for-Dark-BG.webp" class="card-img" />
                            <a href="https://impact-forensics.com/">{"Impact Forensics Website"}</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
                <TabItem title="The Digital Custodian">
                    <div class="tabitem-body-container">
                        <div class="tab-item-body">
                            <h3>{"monday.com Marketplace Application"}</h3>
                            <p>{"I worked with the Digital Custodian to create a monday.com Marketplace Application that tracks item's Lead Time, Cycle Time, and Date Completed and is able to display them in Graphs. Contact The Digital Custodian for more information!"}</p>
                        </div>
                        <Card title="The Digital Custodian">
                            <div class="links-card-container">
                            <img src="https://images.squarespace-cdn.com/content/v1/5f22db65a463c942e8f49804/1597345969276-F16V72V4BT7ZZ7I6ZRVK/Digital+Custodian+Main+Logo.png" class="card-img" />
                            <a href="https://www.thedigitalcustodian.com/">{"The Digital Custodian Website"}</a>
                            </div>
                        </Card>
                    </div>
                </TabItem>
            </div>
        </div>
    }
}
