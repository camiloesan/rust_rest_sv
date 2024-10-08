use mysql::{Pool, PooledConn};

pub fn get_connection() -> PooledConn {
    let url = "mysql://root:storm6@localhost:3306/study_vault";
    let pool = Pool::new(url).expect("wrong");
    let conn = pool.get_conn().expect("wrong");
    conn
}
