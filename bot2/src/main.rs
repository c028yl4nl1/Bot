

use reqwest::Response;
use tbot::{contexts::fields::{Forward, Message}, errors::MethodCall, prelude::*, types::{message::text, parameters::ChatId, User}};
use std::{env::var, f32::consts::E, fs, future::IntoFuture, io::Write, os::unix::raw::gid_t, time::Duration, vec}; 
use tbot::*;
use bot2::{bancodedados::{self, methods::{mysql_conector::user, sqlite3::{info_user, list_users}}}, config, help};







type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error>  {
    logs_start();
 


    let mut  bot = Bot::new(config::token.to_string()).event_loop();
    let commands = vec!["start", "help", "ajuda", "menu"];
    for command in commands {
        bot.command(command, |context| async move {
            let id = context.chat.id.0;
            let first_name = context.from.clone().unwrap().first_name.clone();
            let date = context.date;
            let msg = help::command_front_end(first_name.as_str(), id);

            context.send_message_in_reply(&msg).call().await;

            bancodedados::methods::sqlite3::adcionar_usuario_banco_de_dados(format!("{}",id).as_str(), first_name.as_str(), config::name_db_sqlite3);

        });
    }

    bot.command("myaccount", |context| async move {
        let id = context.chat.id.0;
        
        let user = bancodedados::methods::sqlite3::view_user(config::name_db_sqlite3,format!("{}",id).as_str());

        if let Some(info) = user {
            let msg = format!("Username: {}\nId: {}\nMoney: {}", info.first_name, info.id, info.saldo);
            context.send_message_in_reply(&msg).call().await;
        }
        else {
            if 
            bancodedados::methods::sqlite3::adcionar_usuario_banco_de_dados(format!("{}",id).as_str(), context.from.clone().unwrap().first_name.clone().as_str(), config::name_db_sqlite3){
                context.send_message_in_reply("Create").call().await;
            }   
        }
    });
    

    bot.command("donate", |context| async move {
        let id = context.chat.id.0;
        let msg = &context.text.value;
        // verificar se está no grupo , pois se tiver não pode tirar os pontos 

        let v =   context.get_chat().call().await;

        if v.is_ok(){
            let valor = v.unwrap();
            if valor.kind.is_group(){
                context.send_message_in_reply("Not authorized, only in private / Não autorizado, apenas em privado").call().await;

                return ();
            }
        }
        else {
            context.send_message_in_reply("try new").call().await;
            return ();
        }
  
        if msg.len() != 0{
            let valor =  msg.parse::<u64>().unwrap_or(0);

            if valor == 0{
                context.send_message_in_reply("Wrong value").call().await;
                return ();
            }

            if let Some(gift) = bancodedados::methods::sqlite3::vender_pontos(id, valor  as  i32 , config::name_db_sqlite3){

                context.send_message_in_reply(format!("Here is your gift , Value {}  ! Use /gift {}",valor ,  gift ).as_str()).call().await;

            }
            else  {
                context.send_message_in_reply("You have no points to gift  / Você não tem pontos para presentear").call().await;
            }

        }
        else  {
            context.send_message_in_reply("not value").call().await;
        }

    
    });

    bot.command("viewusers", | bot | async move {

        if bot.chat.id.0 == config::id_dono{
            if let Some(user) = bancodedados::methods::sqlite3::view_user_list(config::name_db_sqlite3){
                let mut string_view = String::new();
                for users in user{
                    let user = format!("Username: {}|  id: {} |saldo: {}\n" , users.first_name , users.id , users.saldo).as_str().to_owned();
                    string_view.push_str(&user);
                }
                let send =  bot.send_message_in_reply(string_view.as_str()).call().await;
                if send.is_err(){
                    bot.send_document_in_reply(tbot::types::input_file::Document::bytes(format!("users.txt").as_str() , string_view.as_bytes())).call().await;
                }
                
            }
            else {
                bot.send_message_in_reply("Sem usuarios no banco de dados").call().await;
            }
        }
        else {
            bot.send_message_in_reply("You not is adm").call().await;
            return ();
        }
    });

    bot.command("search", |context| async move {
        let msg = &context.text.value;
        let id = context.chat.id.0;
        let valor =  context.send_message_in_reply("Search ...").call().await.unwrap().id;
        if msg.len() < 3 || msg.contains(":") || msg.contains("/") || !msg.contains(".") {
            context.edit_message_text( valor , "/search target").call().await;
            return ();
        }

        if let Some(id) = bancodedados::methods::sqlite3::view_user(config::name_db_sqlite3, format!("{}",id).as_str()){

            let saldo:i32 = id.saldo.parse::<i32>().unwrap();

            if saldo < config::saldo_retirar_user {
                context.edit_message_text(valor,"You lack money / Você não tem dinheiro ").call().await;
            }
            else {
                // faça a consulta e subtraia 
                


                let consulta = bancodedados::methods::mysql_conector::consult2(context.text.value.to_owned().as_str());

                match consulta {
                    Some(vecs) => {
                        let mut contador = 0;
                        let mut string = String::new();
                        for linha in vecs{
                            let user = format!("Host: {}{}\nUsername: {}\nPassword: {}\n\n", linha.url, linha.path, linha.username, linha.password);
                            string.push_str(user.as_str());
                            contador +=1;
                        }
                        
                        if contador > 1000 {
                            context.delete_message(valor).call().await;
                      
                            context.send_document_in_reply(tbot::types::input_file::Document::bytes(format!("{}.txt", msg).as_str(), string.as_bytes())).call().await;

                        }
                        else {
                        
                            let err = context.edit_message_text(valor, &string).call().await;
                            if let Err(a ) = err{
                                context.delete_message(valor).call().await;
                            
                                context.send_document_in_reply(tbot::types::input_file::Document::bytes(format!("{}.txt", msg).as_str(), string.as_bytes())).call().await;
    
                            }
                        }

                        bancodedados::methods::sqlite3::updater_saldo(config::name_db_sqlite3, format!("{}",context.chat.id).as_str(), config::saldo_retirar_user, "-");
                        return ();
                    },
                    _ => {
                        context.edit_message_text(valor, "not found").call().await;
                        return (

                        );
                    }
                }
            }   
        }
       
    });
    
    bot.command("viewuser", | bot | async move {
        let msg = &bot.text.value;

        if msg.len() <  2{
            bot.send_message_in_reply("/viewuser id").call().await;
            return ();
        } 
        if bot.chat.id.0 == config::id_dono{
            if let Some(user_info) = bancodedados::methods::sqlite3::view_user(config::name_db_sqlite3, &msg.as_str()){
                let id =  user_info.id;
                let saldo = &user_info.saldo;
               
                if let Ok(infos) =  bot.bot.get_chat(ChatId::Id(format!("{}", id).as_str().parse::<i64>().unwrap().into())).call().await{
                    let format = format!("Id: {:?}\nUsername: {}\nSaldo: {}\nUm grupo ? : {:?}", id, user_info.first_name,saldo , infos.kind.is_group());
                    bot.send_message_in_reply(format.as_str()).call().await;
                    return ();
                }
               
            }
            bot.send_message_in_reply("Id notfound or blocked").call().await;
                    return ();
        }   
        else {
            bot.send_message_in_reply("you not is adm").call().await;
        }


    });

    bot.command("full", |context| async move {
        let id = context.chat.id.0;
        
        if id == config::id_dono {
            let gift_valor = context.text.value.parse::<i32>();

            if gift_valor.is_ok(){
                let valor = bancodedados::methods::sqlite3::create_table_and_gift(config::name_db_sqlite3, gift_valor.unwrap());

                context.send_message_in_reply(format!("/gift {}", valor.unwrap_or("erro adm".to_string())).as_str()).call().await;

            }
            else {
            
                context.send_message_in_reply("/full number (`-+_+-`)").call().await;
                return ();
            }
            
        }
        else {
            context.send_message_in_reply("You not is adm").call().await;
            
        }
    });

    bot.command("gift", |context| async move {
        let msg = &context.text.value;

        if msg.len() < 2 {
            context.send_message_in_reply("/gift value valid").call().await;
        }
        else {
            if let Some(money) = bancodedados::methods::sqlite3::consult_gift_and_adduser(config::name_db_sqlite3, msg.as_str(), format!("{}", context.chat.id.0).as_str()){
                context.send_message_in_reply(format!("Congratulations, {} has been added to your /myaccount to view your current balance\n\n\nParabéns, {} foi adicionado à sua /myaccount para visualizar seu saldo atual", money, money).as_str()).call().await;
            }
            else {
                context.send_message_in_reply("/gift invalid ").call().await;
            }
        }
    });

    bot.command("admin", | bot | async move {
        let users_list = bancodedados::methods::sqlite3::list_users(config::name_db_sqlite3);
        let msg = &bot.text.value;
        if bot.from.clone().unwrap().id.0 == config::id_dono{

            if msg.len() < 3{
                bot.send_message_in_reply("Envie apenas uma mensagem maior que 3 caracter").call().await;
                return ();
            }
            let mut numero_de_menbros:u64 = 0;
            match users_list {
                Some(list_user) => {
                    bot.send_message_in_reply("Enviado a msg para todos os usuarios do bot ").call().await;
                    for id in list_user{
                        let id: ChatId =  id.as_str().into();
                       // bot.send_message(msg).call().await;]
                       let valor = bot.bot.send_message(id, msg).call().await;
                       if valor.is_ok(){
                            numero_de_menbros +=1;
                       }
                       else {
                        let a = bot.bot.send_message(id, msg).call().await;
                        if a.is_ok(){
                            numero_de_menbros +=1;
                        }
                       }

                       std::thread::sleep(std::time::Duration::from_nanos(100));
                       
                    }
                    
                    bot.send_message_in_reply(format!("Enviei a mensagem para: {} menbros",numero_de_menbros).as_str()).call().await;
                    return ();
                },
                None => {
                    bot.send_message_in_reply("Nenhum user cadastrado").call().await;
                    return ();
                }
            }

        }
        
        bot.send_message_in_reply("Not is adm").call().await;
    });

    
    if let Err(err) = bot.polling().start().await {
        eprintln!("Erro ao iniciar o polling: {:?}", err);
    }
    Ok(())
}


fn logs_start(){
    {
        // Startar bot 

        if bancodedados::methods::sqlite3::create_banco_sql(config::name_db_sqlite3){
        println!("Iniciando o banco de dados sql");
        }
    }
}