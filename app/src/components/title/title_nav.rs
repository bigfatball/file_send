use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    
}


#[function_component(TitleNAV)]
pub fn custom_button(props: &Props) -> Html {
    
    html! {
        <nav class="navbar navbar-dark bg-dark">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">{&props.label}</a>
            </div>
        </nav>
    }
}