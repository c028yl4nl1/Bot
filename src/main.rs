use std::{collections::HashMap, fmt::format, path::PathBuf, str::FromStr, vec};
pub mod config;
pub mod lib;
use teloxide::{
    dispatching::dialogue::GetChatId, dptree::di::DependencySupplier, payloads::{SendDocument, SendMessage}, prelude::*, types::{InputFile, MessageId}
};

use tokio::task;
use tokio::io::split;
use teloxide::dispatching::dialogue::Dialogue; 




#[tokio::main]
async fn main() {

    //Dialogue::new(, chat_id)


   
    if lib::methods::sqlite3::create_banco_sql(config::sqlitebanco) {
        println!("Banco de dados User foi criado ");
    } else {
        println!("Banco de dados não foi criado ouver um error ");
    }
    println!("Starting bot...");
    let bot = Bot::new(config::bot_token);







    teloxide::repl(bot, |bot: Bot, msg: Message| async move {

        let bot2 = bot.clone();
        let msg2 = msg.clone();
        
    println!("{}","nem mensagem");

     task::spawn( async move {
        println!("Entrei na thread");

           // println!("ola");
            let bot = bot2;
            let msg = msg2;
           // bot.send_message(msg.chat_id(), "Foda estou rodando em outra threaD").await;

       

        if config::logs_ {
            logs(&bot, &msg);
        }
       
        _ = lib::methods::sqlite3::adcionar_usuario_banco_de_dados(
            msg.chat_id().to_string().as_str(),
            msg.chat.first_name().unwrap_or("Group"),
            config::sqlitebanco,
        );

        if let Some((key,mut value)) = commands(&bot, &msg).await {
            let mut sub: String = String::new();


            println!("{}", value);
            if value.contains("/"){
                let valus: &str = value.split("/").collect::<Vec<&str>>()[0];
               
                println!("{valus}");

                sub.push_str(format!("{}",valus.to_owned()).as_str());

            }
            else{
                let value = sub.push_str(value.to_owned().to_lowercase().as_str()); 
            }
            let value = sub.to_lowercase();
            println!("{}", value);
            match key.replace(" ", "").as_str() {
                "full" => {
                    if config::id_adm != msg.from().unwrap().id.to_string().as_str() {
                        bot.send_message(msg.chat_id(), "You not is adm").reply_to_message_id(msg.id).await;
                       
                    } else {
                        if value.len() == 0 || value.parse::<i32>().is_err() {
                            bot.send_message(msg.chat_id(), "/full value number ").reply_to_message_id(msg.id).await;
                        } else {
                            let gift = lib::methods::sqlite3::create_table_and_gift(
                                config::sqlitebanco,
                                value.parse::<i32>().unwrap(),
                            );
                            if let Some(gift) = gift {
                                bot.send_message(msg.chat_id(), format!("/gift {}", gift)).reply_to_message_id(msg.id).await;
                            } else {
                                bot.send_message(msg.chat_id(), "Não conseguir criar O gift").reply_to_message_id(msg.id).await;
                            }
                        }
                    }
                }

                "gift" => {
                    if value.len() < 1 {
                        bot.send_message(msg.chat_id(), "/gift value").reply_to_message_id(msg.id).await;
                    } else {
                        if value.contains("'") || value.to_ascii_uppercase().contains("or") {
                            bot.send_message(msg.chat_id(), "Sqlijection hahahahaha")
                            .reply_to_message_id(msg.id).await;
                        } else {
                            let cont = lib::methods::sqlite3::consult_gift_and_adduser(
                                config::sqlitebanco,
                                value.as_str().to_uppercase().as_str(),
                                msg.chat_id().to_string().as_str(),
                            );
                            if let Some(number) = cont {
                                bot.send_message(msg.chat_id(), format!("Add: {}", number))
                                .reply_to_message_id(msg.id).await;
                            } else {
                                bot.send_message(msg.chat_id(), "gift invalido").reply_to_message_id(msg.id).await;
                            }
                        }
                    }
                }

                "myaccount" => {
                    let user = lib::methods::sqlite3::view_user(
                        config::sqlitebanco,
                        msg.chat_id().to_string().as_str(),
                    );

                    if let Some(user) = user {
                        let user = format!(
                            "
                        User: {}
                        Money: {}
                        ID: {}                    
                        ",
                            user.first_name, user.saldo, user.id
                        );
                        bot.send_message(msg.chat_id(), user.replace(" ", "")).await;
                    } else {
                        bot.send_message(msg.chat_id(), "Your count not found").reply_to_message_id(msg.id).await;
                    }
                },

                "searchlanby" => {
                    if value.len() < 2 || value.contains("'"){
                        bot.send_message(msg.chat_id(), "/search target").reply_to_message_id(msg.id).await;
                    }
                    else if lib::methods::sqlite3::view_user(config::sqlitebanco, msg.chat_id().to_string().as_str()).unwrap().saldo.parse::<i32>().unwrap() < config::valor_consult{
                        bot.send_message(msg.chat_id(), "your not has money").reply_to_message_id(msg.id).await;
                    }
                    else {
                       
                        let file_name  = value.clone();
                        let mut text = String::new();
                    
                        if let Some(vector_value) = lib::methods::mysql_conector::consult(value.as_str()){
                           
                            
                            for user in vector_value{
                                let user_ = format!("
                                \rURL: {}{}
                                \rUSERNAME: {}
                                \rPASSWORD: {}
                                ", user.url , user.path, user.username, user.password);
                                text.push_str(user_.to_owned().as_str());

                            }

                            if text.len() > 4888{
                                let a = std::fs::write(format!("{}.txt", file_name.clone()), text);
                                
                               
                                let a = bot.send_document(msg.chat_id(),InputFile::file(format!("{}.txt", file_name.clone()).as_str())).send().await;
                            }
                            else {
                                bot.send_message(msg.chat_id(),text.trim().replace(" ", "")).reply_to_message_id(msg.id).await;
                            }
                            lib::methods::sqlite3::updater_saldo(config::sqlitebanco, msg.chat_id().to_string().as_str(),config::valor_consult, "-");




                          
                        }
                        else {
                            
                            bot.send_message(msg.chat_id(), "Not found").reply_to_message_id(msg.id).await;
                        }
                       
                    
                    }
                    



                },


                "search2" => {
                    if value.len() < 2 || value.contains("'"){
                        bot.send_message(msg.chat_id(), "/search target").reply_to_message_id(msg.id).await;
                    }
                    else if lib::methods::sqlite3::view_user(config::sqlitebanco, msg.chat_id().to_string().as_str()).unwrap().saldo.parse::<i32>().unwrap() < config::valor_consult{
                        bot.send_message(msg.chat_id(), "your not has money").reply_to_message_id(msg.id).await;
                    }
                    else {
                       
                        let file_name  = value.clone();
                        let mut text = String::new();
                    
                        if let Some(vector_value) = lib::methods::mysql_conector::consult2(value.as_str()){
                           
                            
                            for user in vector_value{
                                let user_ = format!("
                                \rURL: {}{}
                                \rUSERNAME: {}
                                \rPASSWORD: {}
                                ", user.url , user.path, user.username, user.password);
                                text.push_str(user_.to_owned().as_str());

                            }

                            if text.len() > 4888{
                                let a = std::fs::write(format!("{}.txt", file_name.clone()), text);
                                
                               
                                let a = bot.send_document(msg.chat_id(),InputFile::file(format!("{}.txt", file_name.clone()).as_str())).send().await;
                            }
                            else {
                                bot.send_message(msg.chat_id(),text.trim().replace(" ", "")).reply_to_message_id(msg.id).await;
                            }
                            lib::methods::sqlite3::updater_saldo(config::sqlitebanco, msg.chat_id().to_string().as_str(),config::valor_consult, "-");




                          
                        }
                        else {
                            
                            bot.send_message(msg.chat_id(), "Not found").reply_to_message_id(msg.id).await;
                        }
                       
                    
                    }
                    



                },

                _ => {
                    bot.send_message(msg.chat_id(), "Command Not found").reply_to_message_id(msg.id).await;
                }
            }
        }
    }).await;
        Ok(())
    }).await;
}

fn logs(bot: &Bot, msg: &Message) {
    //let msg_id: MessageId = msg.id;
    let chat_id: ChatId = msg.chat_id();
    let username = msg.chat.first_name().unwrap_or("Hi");
    let date = msg.date.date();
    println!(
        "
    'user':'{}'
    'date':'{}',
    'id':'{}' ",
        username, date, chat_id
    );
}

async fn commands(bot: &Bot, msg: &Message) -> Option<(String, String)> {

    let mut text_comand = msg.text().unwrap_or("notfound").to_ascii_lowercase().replace("https://", " ").replace("http://", "");
    let mut key: String = String::new();
    let mut value = String::new();
    if text_comand.contains("/"){
        let key_ : Vec<&str> = text_comand.split("/").collect();
        key = key_[1].split(" ").collect::<Vec<&str>>()[0].to_string();    
        
        if key_.len() > 2{
            value = key_[1].split(" ").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>()[0].to_string();
        }
        else {
            let g = key_[1].split(" ").collect::<Vec<&str>>();
            if g.len() > 1 {
              value =  g[1].to_string();                
            }
            
        }
        
    }


    

    if let Some(front_text) = config::command_front_end().get(key.as_str()) {
            
            bot.send_message(msg.chat_id(), &front_text.to_string())
            .reply_to_message_id(msg.id).await;
            return None;
        } else {
            return Some((key.to_string(), value.trim().to_string()));
        }
    

    None
}
