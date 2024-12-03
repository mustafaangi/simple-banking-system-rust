// Account trait definition
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// BankAccount struct
struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

// Constructor implementation
impl BankAccount {
    fn new(account_number: String, holder_name: String) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: 0.0,
        }
    }
}

// Account trait implementation
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${:.2} to account {}", amount, self.account_number);
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrawn ${:.2} from account {}", amount, self.account_number);
        } else {
            println!("Insufficient funds in account {}", self.account_number);
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Create two bank accounts
    let mut account1 = BankAccount::new("1001".to_string(), "Alice Smith".to_string());
    let mut account2 = BankAccount::new("1002".to_string(), "Bob Jones".to_string());

    // Test deposit
    account1.deposit(1000.0);
    
    // Test withdraw
    account2.deposit(500.0);
    account2.withdraw(200.0);

    // Print final balances
    println!("Account {} balance: ${:.2}", account1.account_number, account1.balance());
    println!("Account {} balance: ${:.2}", account2.account_number, account2.balance());
}