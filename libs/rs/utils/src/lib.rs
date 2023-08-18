pub fn rs_utils() -> String {
    "rs_utils".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(rs_utils(), "rs_utils".to_string());
    }
}
