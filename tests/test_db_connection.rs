use backend::db;

#[test]
fn test_db_connection() {
    let conn = db::establish_connection();
    assert!(conn.is_ok());
}
