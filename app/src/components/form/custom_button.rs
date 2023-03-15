use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    
}


#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
    
    html! {
        <button id= "bn1" class="btn btn-primary">{&props.label}</button>
    }
}