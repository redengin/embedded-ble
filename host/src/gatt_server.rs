use crate::gatt;

struct GattServer<'a> {
    services: &'a [gatt::GattService<'a>],
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gatt_server() {
    }
}