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
    //There probably exists a better approach for calculating the winrates for stages/characters that involves a lot less code but I am short on time, and have other deadlines approaching, so forgive me. 

    let v: Value = serde_json::from_str(val).unwrap();
    let mut returnMap= HashMap::new(); 

    let mut total=0;
    let mut wins=0; 
    let mut winRate=0.0; 

    let arr=v.as_array().unwrap();

    //println!("rust test{:?}",arr);

    //Need to get total games played, and total wins for each stage and character 
    //Stage WR 
    //Have 2 hashmap with key as map name, and value as wins/total 

    
    let mut stageWins=Vec::new();
    let mut stageTotals=Vec::new(); 
    let mut stageWinRates=Vec::new(); 

    let mut charWins=Vec::new();
    let mut charTotals=Vec::new(); 
    let mut charWinRates=Vec::new(); 

    for i in 0..26{
        charWins.push(0);
        charTotals.push(0);
        charWinRates.push(0.0);
    }

    for i in 0..6{
        stageWins.push(0);
        stageTotals.push(0);
        stageWinRates.push(0.0);
    }

    //"Stage.FOUNTAIN_OF_DREAMS", "Stage.POKEMON_STADIUM","Stage.YOSHIS_STORY","Stage.DREAM_LAND_N64","Stage.BATTLEFIELD","Stage.FINAL_DESTINATION"


    for i in arr {
        //println!("{}",i["victor"]);
        let mut stage= i["stage"].to_string();
        let mut character=i["character"].to_string(); 

        println!("{:?}", character );

        match stage {
            _ if stage == String::from("\"Stage.FOUNTAIN_OF_DREAMS\"")=> stageTotals[0]= stageTotals[0]+1,
            _ if stage == String::from("\"Stage.POKEMON_STADIUM\"")=> stageTotals[1]= stageTotals[1]+1,
            _ if stage == String::from("\"Stage.YOSHIS_STORY\"")=> stageTotals[2]= stageTotals[2]+1,
            _ if stage == String::from("\"Stage.BATTLEFIELD\"")=> stageTotals[3]= stageTotals[3]+1,
            _ if stage == String::from("\"Stage.DREAM_LAND_N64\"")=> stageTotals[4]= stageTotals[4]+1,
            _ if stage == String::from("\"Stage.FINAL_DESTINATION\"")=> stageTotals[5]= stageTotals[5]+1,
            _ => println!("nothing"),
        }


        match character{
            _ if character == String::from("\"CSSCharacter.CAPTAIN_FALCON\"")=> charTotals[0]=charTotals[0]+1,
            _ if character == String::from("\"CSSCharacter.DONKEY_KONG\"")=> charTotals[1]=charTotals[1]+1,
            _ if character == String::from("\"CSSCharacter.FOX\"")=> charTotals[2]=charTotals[2]+1,
            _ if character == String::from("\"CSSCharacter.MR_GAME_&_WATCH\"")=> charTotals[3]=charTotals[3]+1,
            _ if character == String::from("\"CSSCharacter.KIRBY\"")=> charTotals[4]=charTotals[4]+1,
            _ if character == String::from("\"CSSCharacter.BOWSER\"")=> charTotals[5]=charTotals[5]+1,
            _ if character == String::from("\"CSSCharacter.LINK\"")=> charTotals[6]=charTotals[6]+1,
            _ if character == String::from("\"CSSCharacter.LUIGI\"")=> charTotals[7]=charTotals[7]+1,
            _ if character == String::from("\"CSSCharacter.MARIO\"")=> charTotals[8]=charTotals[8]+1,
            _ if character == String::from("\"CSSCharacter.MARTH\"")=> charTotals[9]=charTotals[9]+1,
            _ if character == String::from("\"CSSCharacter.MEWTWO\"")=> charTotals[10]=charTotals[10]+1,
            _ if character == String::from("\"CSSCharacter.NESS\"")=> charTotals[11]=charTotals[11]+1,
            _ if character == String::from("\"CSSCharacter.PEACH\"")=> charTotals[12]=charTotals[12]+1,
            _ if character == String::from("\"CSSCharacter.PIKACHU\"")=> charTotals[13]=charTotals[13]+1,
            _ if character == String::from("\"CSSCharacter.ICE_CLIMBERS\"")=> charTotals[14]=charTotals[14]+1,
            _ if character == String::from("\"CSSCharacter.JIGGLYPUFF\"")=> charTotals[15]=charTotals[15]+1,
            _ if character == String::from("\"CSSCharacter.SAMUS\"")=> charTotals[16]=charTotals[16]+1,
            _ if character == String::from("\"CSSCharacter.YOSHI\"")=> charTotals[17]=charTotals[17]+1,
            _ if character == String::from("\"CSSCharacter.ZELDA\"")=> charTotals[18]=charTotals[18]+1,
            _ if character == String::from("\"CSSCharacter.SHEIK\"")=> charTotals[19]=charTotals[19]+1,
            _ if character == String::from("\"CSSCharacter.FALCO\"")=> charTotals[20]=charTotals[20]+1,
            _ if character == String::from("\"CSSCharacter.YOUNG_LINK\"")=> charTotals[21]=charTotals[21]+1,
            _ if character == String::from("\"CSSCharacter.DR_MARIO\"")=> charTotals[22]=charTotals[22]+1,
            _ if character == String::from("\"CSSCharacter.ROY\"")=> charTotals[23]=charTotals[23]+1,
            _ if character == String::from("\"CSSCharacter.PICHU\"")=> charTotals[24]=charTotals[24]+1,
            _ if character == String::from("\"CSSCharacter.GANONDORF\"")=> charTotals[25]=charTotals[25]+1,
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

            match character{
                _ if character == String::from("\"CSSCharacter.CAPTAIN_FALCON\"")=> charWins[0]=charWins[0]+1,
                _ if character == String::from("\"CSSCharacter.DONKEY_KONG\"")=> charWins[1]=charWins[1]+1,
                _ if character == String::from("\"CSSCharacter.FOX\"")=> charWins[2]=charWins[2]+1,
                _ if character == String::from("\"CSSCharacter.MR_GAME_&_WATCH\"")=> charWins[3]=charWins[3]+1,
                _ if character == String::from("\"CSSCharacter.KIRBY\"")=> charWins[4]=charWins[4]+1,
                _ if character == String::from("\"CSSCharacter.BOWSER\"")=> charWins[5]=charWins[5]+1,
                _ if character == String::from("\"CSSCharacter.LINK\"")=> charWins[6]=charWins[6]+1,
                _ if character == String::from("\"CSSCharacter.LUIGI\"")=> charWins[7]=charWins[7]+1,
                _ if character == String::from("\"CSSCharacter.MARIO\"")=> charWins[8]=charWins[8]+1,
                _ if character == String::from("\"CSSCharacter.MARTH\"")=> charWins[9]=charWins[9]+1,
                _ if character == String::from("\"CSSCharacter.MEWTWO\"")=> charWins[10]=charWins[10]+1,
                _ if character == String::from("\"CSSCharacter.NESS\"")=> charWins[11]=charWins[11]+1,
                _ if character == String::from("\"CSSCharacter.PEACH\"")=> charWins[12]=charWins[12]+1,
                _ if character == String::from("\"CSSCharacter.PIKACHU\"")=> charWins[13]=charWins[13]+1,
                _ if character == String::from("\"CSSCharacter.ICE_CLIMBERS\"")=> charWins[14]=charWins[14]+1,
                _ if character == String::from("\"CSSCharacter.JIGGLYPUFF\"")=> charWins[15]=charWins[15]+1,
                _ if character == String::from("\"CSSCharacter.SAMUS\"")=> charWins[16]=charWins[16]+1,
                _ if character == String::from("\"CSSCharacter.YOSHI\"")=> charWins[17]=charWins[17]+1,
                _ if character == String::from("\"CSSCharacter.ZELDA\"")=> charWins[18]=charWins[18]+1,
                _ if character == String::from("\"CSSCharacter.SHEIK\"")=> charWins[19]=charWins[19]+1,
                _ if character == String::from("\"CSSCharacter.FALCO\"")=> charWins[20]=charWins[20]+1,
                _ if character == String::from("\"CSSCharacter.YOUNG_LINK\"")=> charWins[21]=charWins[21]+1,
                _ if character == String::from("\"CSSCharacter.DR_MARIO\"")=> charWins[22]=charWins[22]+1,
                _ if character == String::from("\"CSSCharacter.ROY\"")=> charWins[23]=charWins[23]+1,
                _ if character == String::from("\"CSSCharacter.PICHU\"")=> charWins[24]=charWins[24]+1,
                _ if character == String::from("\"CSSCharacter.GANONDORF\"")=> charWins[25]=charWins[25]+1,
                _ => println!("nothing"),
            }
    
        }
        total=total+1;
    }

    if total!=0 {
        winRate=wins as f32/total as f32; 
    }
    


    //println!("stage totals {:?}",stageTotals);
    //println!("stage wins {:?}",stageWins);

 
    for i in 0..6{
        if stageTotals[i]!=0{
            stageWinRates[i]= stageWins[i] as f32/  stageTotals[i] as f32; 
        }
    }

    for i in 0..26{
        //charWinRates[i]=i as f32;
        if charTotals[i]!=0{
            charWinRates[i]= charWins[i] as f32/  charTotals[i] as f32; 
        }
    }

    println!("{:?}",  charWins);
    println!("{:?}",  charTotals);
    println!("{:?}",  charWinRates);

    returnMap.insert("winRate".to_string(), winRate); 
    returnMap.insert("total".to_string(), total as f32); 
    returnMap.insert("FODwinRate".to_string(), stageWinRates[0]); 
    returnMap.insert("PSwinRate".to_string(), stageWinRates[1]); 
    returnMap.insert("YSwinRate".to_string(), stageWinRates[2]); 
    returnMap.insert("DLwinRate".to_string(), stageWinRates[3]); 
    returnMap.insert("BFwinRate".to_string(), stageWinRates[4]); 
    returnMap.insert("FDwinRate".to_string(), stageWinRates[5]); 

    returnMap.insert("CaptainFalconWinRate".to_string(), charWinRates[0]); 
    returnMap.insert("DonkeyKongWinRate".to_string(), charWinRates[1]); 
    returnMap.insert("FoxWinRate".to_string(), charWinRates[2]); 
    returnMap.insert("MrGameAndWatchWinRate".to_string(), charWinRates[3]); 
    returnMap.insert("KirbyWinRate".to_string(), charWinRates[4]); 
    returnMap.insert("BowserWinRate".to_string(), charWinRates[5]); 
    returnMap.insert("LinkWinRate".to_string(), charWinRates[6]); 
    returnMap.insert("LuigiWinRate".to_string(), charWinRates[7]); 
    returnMap.insert("MarioWinRate".to_string(), charWinRates[8]); 
    returnMap.insert("MarthWinRate".to_string(), charWinRates[9]); 
    returnMap.insert("MewtwoWinRate".to_string(), charWinRates[10]); 
    returnMap.insert("NessWinRate".to_string(), charWinRates[11]); 
    returnMap.insert("PeachWinRate".to_string(), charWinRates[12]); 
    returnMap.insert("PikachuWinRate".to_string(), charWinRates[13]); 
    returnMap.insert("IceClimbersWinRate".to_string(), charWinRates[14]); 
    returnMap.insert("JigglypuffWinRate".to_string(), charWinRates[15]); 
    returnMap.insert("SamusWinRate".to_string(), charWinRates[16]); 
    returnMap.insert("YoshiWinRate".to_string(), charWinRates[17]); 
    returnMap.insert("ZeldaWinRate".to_string(), charWinRates[18]); 
    returnMap.insert("SheikWinRate".to_string(), charWinRates[19]); 
    returnMap.insert("FalcoWinRate".to_string(), charWinRates[20]); 
    returnMap.insert("YoungLinkWinRate".to_string(), charWinRates[21]); 
    returnMap.insert("DrMarioWinRate".to_string(), charWinRates[22]); 
    returnMap.insert("RoyWinRate".to_string(), charWinRates[23]); 
    returnMap.insert("PichuWinRate".to_string(), charWinRates[24]); 
    returnMap.insert("GanondorfWinRate".to_string(), charWinRates[25]); 
  
    
    Ok(returnMap)
 }



