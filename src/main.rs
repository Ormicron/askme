use std::{env, io};


fn main() {

    let args :Vec<String> = env::args().collect();

    let mut vec_raw :Vec<String> = Vec::new();

    if args.len() == 2{
        let sigle_hash = args.get(args.len()-1).unwrap();
        vec_raw.push(sigle_hash.to_string());
    }else{
        println!("[*] 输入要解密的哈希,Ctrl+D 结束.");
        let lines = io::stdin().lines();
        for line in lines {
            vec_raw.push(line.unwrap());
        }
    }

    let hash = remove_dumplicate(&vec_raw);
    //let hash_collection :String= hash_list.join(",");
    //println!("{:?}",hash_list);
    // println!("{}",hash_collection);

    if hash.len() !=0{
        do_action(hash)
    }

}

#[tokio::main]
async fn do_action(hash_list:Vec<String>){
    let hash_str :String = hash_list.join(",");

    let plan_text:Vec<String> = send_request(&hash_str).await;
    //println!("PlanText: {:?}",plan_text);
    println!("------------------------------\n");
    for(i,element) in hash_list.iter().enumerate(){
        println!("{} {}",element,plan_text.get(i).unwrap());
    }
    // for i in 0..hash_list.len(){

    // }
}

fn remove_dumplicate(vec_raw :&Vec<String>)->Vec<String>{
    let mut unique_set = std::collections::HashSet::new();
    vec_raw.iter().for_each(|s|{
        let trimmed_s = s.trim();
        if !trimmed_s.is_empty(){
            unique_set.insert(s.to_string());
        }
    });
    return unique_set.into_iter().collect();
}

async fn send_request(hash:&String)->Vec<String>{

    let cmd5_email = "1508695576@qq.com";
    let cmd5_key = "6147176c0177a3a35c85d9a6a2f6d245";
    let cmd5_api = format!("http://www.cmd5.com/api.ashx?email={}&key={}&hash={}",cmd5_email,cmd5_key,hash);
    // let cmd5_api = "http://www.cmd5.com/api.ashx?email=&key=&hash=";

    let http_client = reqwest::Client::builder()
        .http1_title_case_headers()
        .build().unwrap();

    let response = http_client.get(cmd5_api)
        .send()
        .await
        .unwrap();

    if response.status().is_success(){
        let result_text :Vec<String> = response.text().await.unwrap().split("	")
            .map(|s|s.trim().to_string())
            .collect();
        return result_text
        // return response.text().await.unwrap();
        //println!("{}",response.text().await.unwrap());
        //result:admin	123456
    }else{
        println!("{}","[-] 请求失败");
        std::process::exit(0);
        // return Vec::new();
    }
}