

use gloo::console::log;
use js_sys::Uint8Array;
use serde::{Serialize, Deserialize};
use web_sys::EventTarget;
use yew::prelude::*;

mod components;
use components::form::custom_form::CustomForm;
use crate::components::form::custom_form::Data;
use crate::components::title::title_main::TitleMain;

//use gloo_file::File;



// indexeddb




// websocket
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket, DragEvent, Element, window};
use web_sys::{File , Blob, FileReader};
use chrono::prelude::*;





#[derive(Serialize, Deserialize)]
struct MyObject {
    //username: Vec<File>,
    favorite_language: String,
}




#[function_component(App)]
pub fn app() -> Html {
    

    let custom_form_submit = Callback::from(|data:Data|{
        let value = data.username;
        let data = &value[0].clone();

        let file = value[0].clone();
        let file_size = data.size();
        //let file = web_sys::Files::

        log!("file size: ",data.size()*2.0);
        log!("file name: ", data.name() );

        
        let windows =  web_sys::window().expect("global window does not exists"); 
        let document = windows.document().expect("expecting a document on window");
        
        //let element = document.create_element("p").unwrap();
        //element.set_inner_html("Hello World from WebAssemblyMan!");


        let element = document.get_element_by_id("test").unwrap();

                // Cast the element to a web_sys::Element so we can change its style
        let element = element.dyn_into::<Element>().unwrap();

        element.set_inner_html("Hello World from WebAssemblyMan!");


        start_websocket(file, data.name());

        

        
    });


    let Drag_Over_Handler = Callback::from(move |event: DragEvent| {
        
        event.prevent_default();
        log!("abc");
       
       
    });

    let Drag_Handler = Callback::from(move |event: DragEvent| {
        
        event.prevent_default();
        log!("a");
       
       
    });

    html!{
        <>
            <div ondragover={Drag_Over_Handler} ondrop={Drag_Handler}>
                <TitleMain />

            
                <CustomForm onsubmit={custom_form_submit} />
                <label> </label>
                <div class="progress">
  <div class="progress-bar" role="progressbar" aria-valuenow="0" aria-valuemin="0" aria-valuemax="100" id = "bar"></div>
</div>
            </div>

            <p id = "test"></p>
        </>

        
    
    }
}


pub fn start_websocket(file:File,file_name: String) -> Result<(), JsValue> {

    

    /*
    
    this is connect 2
    */
    let ws = WebSocket::new("ws://192.168.248.130:8080/")?;
    let cloned_ws = ws.clone();

    // set websocket data type to binary
    cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
    

    

    let clone_ws =  ws.clone();

    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            log!("message event, received arraybuffer: {:?}", abuf);
   
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            log!("message event, received blob: {:?}", blob);

        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {


            let event_type  = txt.slice(0,4);
            log!(&event_type);

            if event_type == "name" {
                log!("onmessage name");

                let file_clone2 = file.clone();
                let file_size:f64 = file_clone2.size() as f64;
        //let blob_num:f64 = file_size / 524_288_00.0;
        
        //let blob_num:f64 = (file_size / 500.0) + 1.0;

        //let blob_num:f64 = (file_size / 104_857_600.0) + 1.0;

        let blob_num:f64 = 2.0;
        let blob_num = blob_num as i64;

        if blob_num == 0 {
            clone_ws.send_with_blob(&file_clone2).unwrap();
        } else {
          


            

            /*
            
                add data in blob
            */
            

            let  a= JsValue::from(file_clone2);
            let arr_str = js_sys::Array::new();
            arr_str.set(0,JsValue::from_str("user1"));
            let blob_header = Blob::new_with_str_sequence(&arr_str).unwrap();

            let b = JsValue::from(blob_header);
            let arr_blob = js_sys::Array::new();
            arr_blob.set(0,b);
            arr_blob.set(1,a);
            let chunk = Blob::new_with_blob_sequence(&arr_blob).unwrap();
            //let data = JsValue::from_ 
            //clone_ws.send_with_u8_array(data);


            
            clone_ws.send_with_blob(&chunk).unwrap();

            
        }
            }else if event_type == "bars"{

                let windows =  web_sys::window().expect("global window does not exists"); 
                let document = windows.document().expect("expecting a document on window");

                // Get the element by its ID
                let element = document.get_element_by_id("bar").unwrap();

                // Cast the element to a web_sys::Element so we can change its style
                let element = element.dyn_into::<Element>().unwrap();

                // Change the CSS properties
                let len = txt.length();
                let data = txt.slice(4,len);
                log!(&data);
                let width = format!("width: {}%", data);
                
                element.set_attribute("style", &width);
            
            }else{
                let a = clone_ws.buffered_amount();
                log!("message event, received Text: {:?} ... {}", txt, a);
            }
            
            
        } else {
            log!("message event, received Unknown: {:?}", e.data());
        }

        
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    
    
    let file_name_clone = file_name.clone();

    
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        // get open time and log this
        let start_time = Utc::now().to_string();
        let message = format!("Websocket open success, {}!", start_time);
        log!("{}",message);
        

        // let window = web_sys::window().expect("no global `window` exists");
        // let document = window.document().expect("should have a document on window");
        // let body = document.body().expect("document should have a body");

        // let div = document
        //     .get_element_by_id("log")
        //     .expect("should have log_area");

        // let val = document.create_element("p").unwrap();
        // val.set_text_content(Some("Hello form"));

        // div.append_child(&val).unwrap();
        
        
        // let val = JsValue::from(document().get_element_by_id("log_area"));  
            
        //     console_log!("{:?}",val);
        // sent file name to server first
        match cloned_ws.send_with_str("name:abc,file:dfa.txt"){
            Ok(_) => log!("File name successfully sent"),
            Err(err) => log!("error sending message: {:?}",err),
        }
        
        
 
        
        
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let file_name_clone = file_name.clone();
    let onclose_callback = Closure::<dyn FnMut()>::new(move || {
        
        log!("Websocket close sueecess...{}", &file_name_clone);
    });
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
    
    
    Ok(())
}