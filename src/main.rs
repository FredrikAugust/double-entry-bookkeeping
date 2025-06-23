use std::fs::read_to_string;

use time::format_description::well_known::Iso8601;

struct JournalEntry {
    pub datetime: time::PrimitiveDateTime,
    pub account_name: String,
    pub amount: f64,
    pub transaction_name: String,
}

#[derive(Clone)]
struct Account {
    pub name: String,
    pub balance_sheet: bool,
    pub balance: f64,
}

fn main() {
    let mut accounts = [
        Account {
            name: "Cash in Bank".to_owned(),
            balance_sheet: true,
            balance: 0.0f64,
        },
        Account {
            name: "Fixed Assets".to_owned(),
            balance_sheet: true,
            balance: 0.0f64,
        },
        Account {
            name: "Sales".to_owned(),
            balance_sheet: false,
            balance: 0.0f64,
        },
    ];

    let transactions = read_to_string("transactions.csv").unwrap();

    let transactions = transactions.lines();

    let mut journal: Vec<JournalEntry> = Vec::with_capacity(transactions.clone().count());

    for transaction in transactions {
        let mut parts = transaction.split(',');

        let datetime =
            time::PrimitiveDateTime::parse(parts.next().unwrap(), &Iso8601::DEFAULT).unwrap();

        let from_account_name = parts.next().unwrap().to_owned();
        let to_account_name = parts.next().unwrap().to_owned();

        let value: f64 = parts.next().unwrap().parse().unwrap();

        let transaction_name = parts.next().unwrap().to_owned();

        journal.push(JournalEntry {
            datetime,
            account_name: from_account_name,
            amount: -value,
            transaction_name: transaction_name.clone(),
        });

        journal.push(JournalEntry {
            datetime,
            account_name: to_account_name,
            amount: value,
            transaction_name,
        });
    }

    journal.sort_by_key(|x| x.datetime);

    let days = journal
        .chunk_by(|a, b| a.datetime.date() == b.datetime.date())
        .collect::<Vec<_>>();

    for day in days {
        for transaction in day {
            let account = accounts
                .iter_mut()
                .find(|account| account.name == transaction.account_name)
                .unwrap();

            account.balance += transaction.amount;
        }
    }

    println!("-- Balance sheets -------------------------------");
    for account in accounts.iter().filter(|a| a.balance_sheet) {
        println!("| {:<30} | € {:>10.2} |", account.name, account.balance)
    }
    println!("-------------------------------------------------");

    println!();

    println!("-- Profit/loss ----------------------------------");
    for account in accounts.iter().filter(|a| !a.balance_sheet) {
        println!("| {:<30} | € {:>10.2} |", account.name, account.balance);
        for transaction in journal.iter().filter(|t| t.account_name == account.name) {
            println!(
                "| \\ {:<28} | € {:>10.2} |",
                transaction.transaction_name, transaction.amount
            );
        }
    }
    println!("-------------------------------------------------");
}
