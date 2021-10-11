struct Transaction {
  emetter: String,
  recipient: String,
  amount: i32
}

impl fmt::Display for Transaction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}->{}:{}",self.emetter, self.recipient, self.amount)
    }
}

impl Transaction {
  pub fn new(emetter: String, recipient: String, amount: i32) -> Transaction {
    Transaction {
      emetter,
      recipient,
      amount,
    }
  }
}