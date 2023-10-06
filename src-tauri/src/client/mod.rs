pub struct FinTSClient {
    iban: String
}

impl FinTSClient {
    pub fn new(iban: String) -> FinTSClient {
        FinTSClient {
            iban
        }
    }

    pub fn get_statements(&self) -> &String {
      &self.iban  
    }
}
