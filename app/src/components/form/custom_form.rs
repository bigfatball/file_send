use std::ops::Deref;



use gloo::console::log;
use yew::html::onsubmit;
use yew::prelude::*;
use crate::components::form::files_input::FilesInput;
use crate::components::form::custom_button::CustomButton;


//use gloo_file::File;
use web_sys::File;

#[derive(Default, Clone)]
pub struct Data {
    pub username: Vec<File>,

}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(||Data::default());
    
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username|{
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);


        
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event:SubmitEvent|{
        // let mut data = cloned_state.deref().clone();
        // data.count += 1;
        // cloned_state.set(data);
        event.prevent_default();
        let data = cloned_state.deref().clone();

        


        form_onsubmit.emit(data);


    });



    // let cloned_state = state.clone();
    // let form_onsubmit = state.clone();
    // let onsubmit = Callback::from(move |event|{
    //     let data = *cloned_state;

    // });
    
    html! {
        <form onsubmit={onsubmit}>
            <FilesInput name="username" handle_onchange={username_changed}/>
            <CustomButton label = "button_clicked"  />
            
        </form>
    }
}