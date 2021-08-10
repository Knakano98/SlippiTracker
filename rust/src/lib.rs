extern crate cpython;
extern crate serde_json;
use std::collections::HashMap;
use cpython::{PyResult, Python,PyString,PyList,PyDict, py_module_initializer, py_fn};
use serde_json::{Result, Value};
//Rust extension in python based on template provided by :https://codeburst.io/how-to-use-rust-to-extend-python-360174ee5819

py_module_initializer!(mylib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "get_result", py_fn!(py, get_result(val: &str)))?;
    m.add(py, "dict_process", py_fn!(py, dict_process(val: &str)))?;
    Ok(())
});

fn get_result(_py: Python, val: &str) -> PyResult<String> {
    Ok("Rust says: ".to_owned() + val)
}


fn dict_process(_py: Python ,val: &str) -> PyResult<HashMap<String, f32>> {
    let v: Value = serde_json::from_str(val).unwrap();
    let mut returnMap= HashMap::new(); 

    let mut total=0;
    let mut wins=0; 
    let mut winRate=0.0; 

    let arr=v.as_array().unwrap();

    //println!("rust test{:?}",arr);

    for i in arr {
        //println!("{}",i["victor"]);
        if i["victor"]==i["netplayCode"]{
            wins=wins+1; 
        }
        total=total+1;
    }

    winRate=wins as f32/total as f32; 

    returnMap.insert("winRate".to_string(), winRate); 
    returnMap.insert("total".to_string(), total as f32); 
    
    Ok(returnMap)
 }



