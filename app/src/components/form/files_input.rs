use gloo::console::log;

use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;


use gloo_file::callbacks::FileReader;
//use gloo_file::File;
use web_sys::File; 


// indexeddb
use web_sys::{window, IdbFactory, IdbRequest, IdbDatabase, IdbObjectStore, IdbTransactionMode, IdbTransaction};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;

#[derive(Properties, PartialEq)]

pub struct Props {
    pub name: String,
    // pub result: Vec<File>,
    pub handle_onchange: Callback<Vec<File>>,
}

#[function_component(FilesInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    // let onchange = Callback::from(move |event: Event| {
    //     let value = event
    //         .target()
    //         .unwrap()
    //         .unchecked_into::<HtmlInputElement>()
    //         .value();
    //     handle_onchange.emit(value);
    // });
        
    let onchange = Callback::from(move |event: Event| {
        let mut result = Vec::new();
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Some(files) = input.files() {
        let files = js_sys::try_iter(&files)
            .unwrap()
            .unwrap()
            .map(|v| web_sys::File::from(v.unwrap()))
            .map(File::from);
            result.extend(files);
            

        };
        
        

        handle_onchange.emit(result);
    });

    html!{
        <input type = "file" name={props.name.clone()} onchange={onchange} />
    }

    
    // let onchange = Callback::from(move |event: Event| {
    //     let window =  web_sys::window().unwrap();
    //     log!("abc");
        
    //     let idb_factory  = window.indexed_db().unwrap().unwrap();

    //     let open_request = idb_factory
    //         .open_with_u32(String::from("todo").as_str(), 1)
    //         .unwrap();

    //     let on_upgradeneeded = Closure::once(move |event: &Event| {
    //         let target = event.target().expect("Event should have a target; qed");
    //         let req = target
    //             .dyn_ref::<IdbRequest>()
    //             .expect("Event target is IdbRequest; qed");
            
    //         let result = req
    //             .result()
    //             .expect("IndexedDB.onsuccess should have a valid result; qed");
    //         assert!(result.is_instance_of::<IdbDatabase>());
    //         let db = IdbDatabase::from(result);
    //         let store: IdbObjectStore = db.create_object_store(&String::from("user")).unwrap();
    //         let _index = store
    //             .create_index_with_str(&String::from("name"), &String::from("name"))
    //             .expect("create_index_with_str error");
    //         });


    //     open_request.set_onupgradeneeded(Some(on_upgradeneeded.as_ref().unchecked_ref()));
    //     on_upgradeneeded.forget();

    //     let on_success = Closure::once(move |event: &Event| {
    //         // Extract database handle from the event
    //         let target = event.target().expect("Event should have a target; qed");
    //         let req = target
    //             .dyn_ref::<IdbRequest>()
    //             .expect("Event target is IdbRequest; qed");
        
    //         let result = req
    //             .result()
    //             .expect("IndexedDB.onsuccess should have a valid result; qed");
    //         assert!(result.is_instance_of::<IdbDatabase>());
        
    //         let db = IdbDatabase::from(result);
    //         let transaction = db
    //             .transaction_with_str_and_mode(&String::from("user"), IdbTransactionMode::Readwrite)
    //             .expect("transaction_with_str error");
    //         let store = transaction
    //             .object_store(&String::from("user"))
    //             .expect("store error");
        
    //         let name = JsValue::from_str("_content_element.value().as_str()");
    //         let add_request = store
    //             .add_with_key(&name, &JsValue::from("name"))
    //             .expect("add error");
        
    //         let on_add_error = Closure::once(move |event: &Event| {
    //             log!("写入数据失败");
    //             log!(event.to_string());
    //         });
    //         add_request.set_onerror(Some(on_add_error.as_ref().unchecked_ref()));
    //         on_add_error.forget();
        
    //         let on_add_success = Closure::once(move |event: &Event| {
    //             log!("写入数据成功");
    //         });
    //         add_request.set_onsuccess(Some(on_add_success.as_ref().unchecked_ref()));
    //         on_add_success.forget();
    //     });
    //     open_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    //     on_success.forget();

    // });
    
    

    // html!{
    //          <input type = "file" name={props.name.clone()} onchange={onchange} />
    // }

}

