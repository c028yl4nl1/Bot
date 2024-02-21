
/**
 * Bem vindo função 
 */
pub fn command_front_end(first_name: &str , id: i64) -> String {


    let msg_welcome = format!("
Hello {} Welcome 
id:{}
        
        Commands:
/myaccount _> view your account 
    ver sua conta
/search target view password and user 
    procura por senhas
/donate Convert your bot money into a gift 
    Converta o dinheiro do seu bot em um presente

exemples: 
    /search login.exemple.com
    /donate 10 


🇷🇺
В запросе должен быть правильный URL-адрес страницы входа, поскольку бот не будет искать поддомены!
Только цель, не указывайте каталог!
За каждый успешный запрос с вашего баланса будет снято 10 баллов.
🇷🇺

🇺🇸
The query must be the correct URL of the login page as the bot will not search for subdomains !
Just the target, don't put the directory !
Each successful query will deduct 10 points from your balance.
🇺🇸

🇧🇷
A consulta deve ser a URL correta da página de login, pois o bot não procurará subdomínios ! 
Apenas o alvo, não coloque o diretório !
Cada consulta bem-sucedida deduzirá 10 pontos do seu saldo.
🇧🇷

Administrator: @Kaiouiue

    " , first_name, id);

    msg_welcome.to_owned()
}