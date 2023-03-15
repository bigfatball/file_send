use yew::prelude::*;
use crate::components::title::title_nav::TitleNAV;


#[function_component(TitleMain)]
pub fn custom_button() -> Html {
    
    html! {
        <TitleNAV label = "example"  />
    }
}