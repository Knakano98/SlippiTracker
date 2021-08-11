extern crate cpython;
extern crate serde_json;
use std::collections::HashMap;
use std::any::type_name;
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn dict_process(_py: Python ,val: &str) -> PyResult<HashMap<String, f32>> {
    let v: Value = serde_json::from_str(val).unwrap();
    let mut returnMap= HashMap::new(); 

    let mut total=0;
    let mut wins=0; 
    let mut winRate=0.0; 

    let arr=v.as_array().unwrap();

    println!("rust test{:?}",arr);

    //Need to get total games played, and total wins for each stage and character 
    //Stage WR 
    //Have 2 hashmap with key as map name, and value as wins/total 

    
    let mut stageWins=Vec::new();
    let mut stageTotals=Vec::new(); 
    let mut stageWinRates=Vec::new(); 


    for i in 0..6{
        stageWins.push(0);
        stageTotals.push(0);
        stageWinRates.push(0.0);
    }

    //"Stage.FOUNTAIN_OF_DREAMS", "Stage.POKEMON_STADIUM","Stage.YOSHIS_STORY","Stage.DREAM_LAND_N64","Stage.BATTLEFIELD","Stage.FINAL_DESTINATION"


    for i in arr {
        //println!("{}",i["victor"]);
        let mut stage= i["stage"].to_string();
  


        match stage {
            _ if stage == String::from("\"Stage.FOUNTAIN_OF_DREAMS\"")=> stageTotals[0]= stageTotals[0]+1,
            _ if stage == String::from("\"Stage.POKEMON_STADIUM\"")=> stageTotals[1]= stageTotals[1]+1,
            _ if stage == String::from("\"Stage.YOSHIS_STORY\"")=> stageTotals[2]= stageTotals[2]+1,
            _ if stage == String::from("\"Stage.BATTLEFIELD\"")=> stageTotals[3]= stageTotals[3]+1,
            _ if stage == String::from("\"Stage.DREAM_LAND_N64\"")=> stageTotals[4]= stageTotals[4]+1,
            _ if stage == String::from("\"Stage.FINAL_DESTINATION\"")=> stageTotals[5]= stageTotals[5]+1,
            _ => println!("nothing"),
        }

        if i["victor"]==i["netplayCode"]{
            //stageWins.insert(&stage, stageTotals.get(&stage).unwrap()+1); 
            wins=wins+1; 

            match stage {
                _ if stage == String::from("\"Stage.FOUNTAIN_OF_DREAMS\"")=> stageWins[0]= stageWins[0]+1,
                _ if stage == String::from("\"Stage.POKEMON_STADIUM\"")=> stageWins[1]= stageWins[1]+1,
                _ if stage == String::from("\"Stage.YOSHIS_STORY\"")=> stageWins[2]= stageWins[2]+1,
                _ if stage == String::from("\"Stage.BATTLEFIELD\"")=> stageWins[3]= stageWins[3]+1,
                _ if stage == String::from("\"Stage.DREAM_LAND_N64\"")=> stageWins[4]= stageWins[4]+1,
                _ if stage == String::from("\"Stage.FINAL_DESTINATION\"")=> stageWins[5]= stageWins[5]+1,
                _ => println!("nothing"),
            }
        }
        total=total+1;
    }

    winRate=wins as f32/total as f32; 

    println!("stage totals {:?}",stageTotals);
    println!("stage wins {:?}",stageWins);


    for i in 0..6{
        stageWinRates[i]= stageWins[i] as f32/  stageTotals[i] as f32; 
    }


    returnMap.insert("winRate".to_string(), winRate); 
    returnMap.insert("total".to_string(), total as f32); 
    returnMap.insert("FODwinRate".to_string(), stageWinRates[0]); 
    returnMap.insert("PSwinRate".to_string(), stageWinRates[1]); 
    returnMap.insert("YSwinRate".to_string(), stageWinRates[2]); 
    returnMap.insert("DLwinRate".to_string(), stageWinRates[3]); 
    returnMap.insert("BFwinRate".to_string(), stageWinRates[4]); 
    returnMap.insert("FDwinRate".to_string(), stageWinRates[5]); 
    
    Ok(returnMap)
 }



