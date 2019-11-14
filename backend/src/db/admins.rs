use crate::app::POOL;

/* pub fn admin_sign_up(id: &str, nickname: &str, password: &str)->bool{
    TODO: for test
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let mut command = "INSERT INTO admin_account (account_id, username, password) VALUES (".to_string();
    command = command + "'" + id + "',";
    command = command + "'" + name + "',";
    command = command + "'" + password + "');";
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }
    */
//}
#[inline]
fn format_string(src: &String) -> String {
    src[1..src.len() - 1].to_string()
}

fn admin_sign_up(id: String, name: String, raw_password: String)->String{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("INSERT INTO admin_account (account_id, admin_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id=id, name=name, password=password);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return "success".to_string(),
        Result::Err(_err)=> return _err.to_string(),
    }
}

fn admin_log_in(id :String, raw_password: String)->i8{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("SELECT password FROM admin_account WHERE account_id='{id}';", id=id);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Err(_err) => return -1,
        _ => {}
    }
    for row in req.unwrap(){
        let pwd = format_string(&row.unwrap().unwrap()[0].as_sql(true));
        if password == pwd{
            return 1;
        } else {
            return 0;
        }
    }
    return -1;
}//返回值：-1：账号不存在，0：密码错误， 1：登录成功