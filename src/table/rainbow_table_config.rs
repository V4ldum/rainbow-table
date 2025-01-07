pub struct RainbowTableConfig {
    pub charset: String,
    pub chain_length: u32,
    pub chain_number: usize,
    pub password_length: u32,
    #[cfg(debug_assertions)]
    pub debug: bool,
}

impl RainbowTableConfig {
    pub(crate) fn sort_charset(&mut self) {
        let mut charset = self.charset.split("").collect::<Vec<_>>();
        charset.sort();
        self.charset = charset.join("");
    }
}
