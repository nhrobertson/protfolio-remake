use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html !{
        <div class="container">
            <div class="about-section">
                <img class="about-image" src="assets/research-pic.png" alt="Me" />
                <div class="about-text">
                    <h3>{"About Me"}</h3>
                    <p>{"Hello! My name is Noah Robertson and I am a computer engineer! I was born and raised in Raleigh NC and plan on staying here for as long as I can. I like to play video games, code, stay active, golf, and One Piece."}</p>
                </div>
            </div>
            <div class="about-section">
                <div class="about-text">
                    <h3>{"Academics"}</h3>
                    <p>{"I graduated from the University of South Carolina in Columbia with a Bachelors of Science in Computer Engineering in May 2024. During my time there I accomplished computer science, electrical engineeing, and embedded systems classes to say the least! I graduated with Cum Laude with a 3.67 GPA in case you were wondering!"}</p>
                </div>
                <img class="about-image" src="assets/grad.png" alt="Me" />
            </div>
            <div class="about-section">
                <img class="about-image" src="assets/hehe.png" alt="Me" />
                <div class="about-text">
                    <h3>{"Current"}</h3>
                    <p>{"Currently, I work part-time for Impact Forensics and NSite.AI providing expert support to various research projects. At NSite, we have developed a pipeline for geo-registrated pointclouds of scences which is able to be transformed into a 3D Gaussian Splat Scene. For more information, please make sure to contact me!"}</p>
                </div>
            </div>
        </div>
    }
}
