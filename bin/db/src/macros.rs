macro_rules! hash_get_and_respond {
    ($key:expr, $field:expr) => {
        let res = db.hget($key, $field).await;
        match res {
            Ok(data) => {
                // TODO: implement
            }
            Ok(Ok(None)) => {
                // TODO: implement
            }
            Ok(Err(e)) => {
                // TODO: implement
            }
            Err(_) => {
                // TODO: implement
            }
        }
    }
}