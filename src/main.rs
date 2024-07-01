use std::{env, fs::{self, File}, io::{self, Write}, path::Path};

static CONFIG_FILE:&str = "askme.conf";

fn main() {

    std::panic::set_hook(Box::new(|_                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | {
        //https://stackoverflow.com/questions/77826272/disable-note-run-with-rust-backtrace-1-environment-variable-to-display-a-bac/
        //全局抑制Panic
    }));

    let args :Vec<String> = env::args().collect();

    let mut vec_raw :Vec<String> = Vec::new();

    if args.len() == 2{
        let sigle_hash = args.get(args.len()-1).unwrap();
        vec_raw.push(sigle_hash.to_string());
    }else{
        println!("[*] 输入要解密的哈希,以行分割,Ctrl+D 结束.");
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
        do_action(hash);
    }

}

fn parse_config()->(String,String){
    if !Path::new(CONFIG_FILE).exists(){
        create_config_file(&CONFIG_FILE);
    }

    let data = fs::read_to_string(CONFIG_FILE).expect("[-] Failed to parse config");
    let credentials :Vec<&str>= data.split("=").collect();

    if credentials.len() != 3{
        println!("[-] Parse config failed.");
        std::process::exit(0);
    }

    let email = credentials[1].replace("\nkey","");
    let key = credentials[credentials.len()-1].to_string();
    // println!("Email:{}\nKey:{}\n",email.trim(),key.trim());
    // println!("{:?},Length:{}",credentials,credentials.len());

    return (email,key);
}

#[tokio::main]
async fn do_action(hash_list:Vec<String>){
    let hash_str :String = hash_list.join(",");

    let res_text:Vec<String> = send_request(&hash_str).await;
    //println!("PlanText: {:?}",res_text);
    println!("------------------------------\n");
    if res_text.get(0).unwrap() == "CMD5-ERROR:-1"{
        println!("[-] Email或key无效,请修改askme.conf!");
        std::process::exit(0);
    }

    for(i,element) in hash_list.iter().enumerate(){
        let result = res_text.get(i).expect("[-] 查询出错!");
        let plantext = match result.as_ref() {
            "CMD5-ERROR:0" => "解密失败",
            "CMD5-ERROR:-2" => "余额不足",
            "CMD5-ERROR:-3" => "解密服务器故障",
            "CMD5-ERROR:-4" => "不识别的密文",
            "CMD5-ERROR:-7" => "不支持的类型",
            "CMD5-ERROR:-8" => "API权限被禁止",
            "CMD5-ERROR:-9" => "条数超过100条",
            "CMD5-ERROR:-999" => "其他错误",
            _ => result
        };
        println!("{} {}",element,plantext);
    }
}

fn create_config_file(config_file:&str){
    let mut file = File::create(&config_file).expect("[-] Failed to create .askme.conf");
    file.write_all("email=EXAMPLE@example.com\nkey=21a7176c0277f2b25c85d9a6a2e6c421".as_bytes()).expect("[-] Failed to write.");
    println!("[+] 配置文件askme.conf未找到，已自动创建，请手动填写API。");
    std::process::exit(0);
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

    let (cmd5_email,cmd5_key) = parse_config();
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
        return result_text;
        // return response.text().await.unwrap();
        //println!("{}",response.text().await.unwrap());
        //result:admin	123456
    }else{
        println!("{}","[-] 请求失败");
        std::process::exit(0);
    }
}