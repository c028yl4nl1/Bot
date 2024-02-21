use std::collections::HashMap;

pub const bot_token: &str = "token_bot";
pub const logs_: bool = true;
pub const sqlitebanco: &str = "banco.db";
pub const id_adm: &str = "6889045315";
pub const valor_consult: i32 = 10; // Cada consulta vai tirar esse valor do saldo 
pub fn command_front_end() -> HashMap<&'static str, &'static str> {
    let mut commands: HashMap<&str, &str> = HashMap::new();

    commands.insert(
        "start",
        r#"
Welcome 

Commands:

/myaccount _> view your account 
/search target view password and user

exemple: 
    /search gov.cn 
    "#,
    );
    commands
}
