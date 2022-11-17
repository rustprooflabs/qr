use pgx::prelude::*;
use qrcode_generator::QrCodeEcc;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_qr() -> &'static str {
    "Hello, qr"
}


#[pg_extern]
fn generate_qr(base_url: String, table: String, id: String) -> String {
    let input = format!("{}/{}/{}", base_url, table, id);
    qrcode_generator::to_svg_to_string(input,
                                       QrCodeEcc::Low,
                                       1024,
                                       None::<&str>).unwrap()
}


#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_qr() {
        assert_eq!("Hello, qr", crate::hello_qr());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
