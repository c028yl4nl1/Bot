pub mod methods {

    pub mod sqlite3 {
        use rand::*;
        use sqlite::*;
        use std::iter;
        use teloxide::dptree::di;

        #[derive(Debug)]
        pub struct info_user {
            pub id: String,
            pub premiun: String,
            pub saldo: String,
            pub first_name: String,
        }

        fn generate(len: usize) -> String {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            let mut rng = rand::thread_rng();
            let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
            iter::repeat_with(one_char).take(len).collect()
        }

        pub fn create_banco_sql(banco: &str) -> bool {
            use sqlite::*;
            let conection_sql = Connection::open(banco);
            if let Ok(mut Connection) = conection_sql {
                let query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY, 
                premiun INTEGER,
                saldo INT,
                first_name TEXT
            );
            "#;
                if let Ok(_) = Connection.execute(query) {
                    return true;
                }
            }
            false
        }

        ///* banco: nome do banco de dados
        ///* id: id so usuario para ser adcionado no banco de dados
        ///* first_name: o nome do usuario a ser adcionado
        pub fn adcionar_usuario_banco_de_dados(id: &str, first_name: &str, banco: &str) -> bool {
            use sqlite::*;
            let conection_sql = Connection::open(banco);
            if let Ok(mut Connection) = conection_sql {
                let query = format!("INSERT INTO users(id,premiun,saldo, first_name) VALUES ('{}', '{}', '{}', '{}')", id, 0 , 0, first_name);

                if let Ok(_) = Connection.execute(query.as_str()) {
                    return true;
                }
            }
            false
        }

        pub fn view_user(banco: &str, id: &str) -> Option<info_user> {
            use sqlite::*;
            let conection_sql = Connection::open(banco);
            if let Ok(mut Connection) = conection_sql {
                let query = format!("SELECT * FROM users WHERE id = '{}'", id);
                let mut inter: Vec<String> = Vec::new();
                let result = Connection.iterate(query, |pairs| {
                    for &(name, value) in pairs.iter() {
                        inter.push(value.unwrap().to_string());
                    }
                    true
                });
                if inter.len() == 4 {
                    return Some(info_user {
                        id: inter[0].clone(),
                        premiun: inter[1].clone(),
                        saldo: inter[2].clone(),
                        first_name: inter[3].clone(),
                    });
                }
            }
            None
        }

        pub fn updater_saldo(banco: &str, id: &str, novo_valor: i32, prefix: &str ) -> bool {
            use sqlite::*;
            let conection_sql = Connection::open(banco);
            if let Ok(mut Connection) = conection_sql {
                let query = format!(
                    "UPDATE users SET saldo = saldo {} '{}' , premiun = '1' WHERE id = '{}'",
                    prefix,novo_valor, id
                );
                let mut inter: Vec<String> = Vec::new();
                let result = Connection.execute(query);
                if result.is_ok() {
                    return true;
                }
            }
            false
        }

        pub fn create_table_and_gift(banco: &str, valor: i32) -> Option<String> {
            let view_rand = generate(15);
            let conection_sql = Connection::open(banco);

            if let Ok(mut Connection) = conection_sql {
                let query = format!(
                    " CREATE TABLE IF NOT EXISTS gift(gift TEXT PRIMARY KEY , dinheiro TEXT); INSERT INTO gift(gift,dinheiro) VALUES ('{}', '{}');",&view_rand, valor );

                let result = Connection.execute(query);
                if result.is_ok() {
                    return Some(view_rand);
                }
            }
            None
        }

        pub fn consult_gift_and_adduser(banco: &str, gift: &str, id: &str) -> Option<i32> {
            let conection_sql = Connection::open(banco);

            if let Ok(mut Connection) = conection_sql {
                let query = format!("SELECT dinheiro FROM gift WHERE gift = '{}' LIMIT 1", gift);
                let mut dinheiro_: i32 = 0;
                let result = Connection.iterate(query, |pairs| {
                    for &(name, value) in pairs.iter() {
                        let valor: i32 = value.unwrap().parse().unwrap();
                        dinheiro_ = valor;
                    }
                    true
                });

                if dinheiro_ != 0 {
                    println!("{}", dinheiro_);
                    let query = updater_saldo(banco, id, dinheiro_, "+");
                    if query {
                        Connection.execute(format!("DELETE FROM gift WHERE gift ='{}'", gift));
                        return Some(dinheiro_);
                    } else {
                        return None;
                    }
                }
            }
            None
        }
    }

    pub mod mysql_conector{
        use mysql::*;
        use mysql::prelude::Queryable;
        #[derive(Debug)]
        pub struct user{
            pub id: String,
            pub url: String,
            pub path: String,
            pub username: String,
            pub password: String
        }
        pub fn consult(value:&str) -> Option<Vec<user>>{
            let url = "mysql://user_mysql_aqui:senha_Aqui@host_aqui:3306/db_aqui";

            let connection_ = Pool::new(url).unwrap();

            let mut connect = connection_.get_conn().unwrap();
           // SELECT * FROM Logs WHERE url LIKE 'sua_string_aqui' LIMIT 100;
           //SELECT * FROM Logs WHERE url = 'i-find.org' OR url LIKE 'i-find.org%';
           //SELECT * FROM Logs WHERE url = 'i-find.org' OR url LIKE 'i-find.org%' LIMIT 100;

            // SELECT * FROM Logs FORCE INDEX (idx_url) WHERE url REGEXP '.*sua_regex_pattern';


           let query = format!("SELECT * FROM Logs USE INDEX (idx_url) WHERE url REGEXP '.*{}' LIMIT 400;
           ",value);


            let selected_payments = connect
            .query_map(
                query,
                |(id, url,path ,username , password)| {
                    user { id,url,path, username, password}
                },
            );

            
            if let Ok(list) = selected_payments{
                if list.len() >= 1{
                    return Some(list);
                }
                else {
                    return None;
                }
            }
        None
        }


        pub fn consult2(value:&str) -> Option<Vec<user>>{
            let url = "mysql://user_mysql_aqui:senha_Aqui@host_aqui:3306/db_aqui";

            let connection_ = Pool::new(url).unwrap();

            let mut connect = connection_.get_conn().unwrap();
           // SELECT * FROM Logs WHERE url LIKE 'sua_string_aqui' LIMIT 100;
           //SELECT * FROM Logs WHERE url = 'i-find.org' OR url LIKE 'i-find.org%';
           //SELECT * FROM Logs WHERE url = 'i-find.org' OR url LIKE 'i-find.org%' LIMIT 100;

            // SELECT * FROM Logs FORCE INDEX (idx_url) WHERE url REGEXP '.*sua_regex_pattern';


           let query = format!("SELECT * FROM Logs USE INDEX (idx_url) WHERE url = '{}' LIMIT 10000;
           ",value);


            let selected_payments = connect
            .query_map(
                query,
                |(id, url,path ,username , password)| {
                    user { id,url,path, username, password}
                },
            );

            
            if let Ok(list) = selected_payments{
                if list.len() >= 1{
                    return Some(list);
                }
                else {
                    return None;
                }
            }
        None
        }

    }
}
