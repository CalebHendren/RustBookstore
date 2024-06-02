use std::io::{self, Write};
use std::error::Error;

// Date struct
#[derive(Debug, Clone, Copy)]
struct Date {
    day: i32,
    month: i32,
    year: i32,
}

impl Date {
    fn new(day: i32, month: i32, year: i32) -> Self {
        Date { day, month, year }
    }
}

// Book struct
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    genre: String,
    isbn: String,
    publication_date: Date,
    price: i32,
    quantity: i32,
}

impl Book {
    fn new(
        title: String,
        author: String,
        genre: String,
        isbn: String,
        publication_date: Date,
        price: i32,
        quantity: i32,
    ) -> Self {
        Book {
            title,
            author,
            genre,
            isbn,
            publication_date,
            price,
            quantity,
        }
    }
}

// Catalog struct
#[derive(Debug)]
struct Catalog {
    books: Vec<Book>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn remove_book(&mut self, isbn: &str) {
        self.books.retain(|book| book.isbn != isbn);
    }

    fn search_book(&mut self, isbn: &str) -> Result<&mut Book, Box<dyn Error>> {
        self.books
            .iter_mut()
            .find(|book| book.isbn == isbn)
            .ok_or_else(|| "Book not found".into())
    }
}

// Transaction trait
trait Transaction {
    fn process(&mut self);
}

// Order struct
#[derive(Debug)]
struct Order {
    transaction_id: String,
    transaction_date: Date,
    customer_id: String,
    ordered_book: Book,
    quantity: i32,
}

impl Order {
    fn new(
        transaction_id: String,
        transaction_date: Date,
        customer_id: String,
        ordered_book: Book,
        quantity: i32,
    ) -> Self {
        Order {
            transaction_id,
            transaction_date,
            customer_id,
            ordered_book,
            quantity,
        }
    }
}

impl Transaction for Order {
    fn process(&mut self) {
        if self.ordered_book.quantity >= self.quantity {
            self.ordered_book.quantity -= self.quantity;
            println!("Order processed successfully.");
        } else {
            eprintln!("Insufficient stock for the ordered book.");
        }
    }
}

// Sale struct
#[derive(Debug)]
struct Sale {
    transaction_id: String,
    transaction_date: Date,
    customer_id: String,
    sold_book: Book,
    quantity: i32,
}

impl Sale {
    fn new(
        transaction_id: String,
        transaction_date: Date,
        customer_id: String,
        sold_book: Book,
        quantity: i32,
    ) -> Self {
        Sale {
            transaction_id,
            transaction_date,
            customer_id,
            sold_book,
            quantity,
        }
    }
}

impl Transaction for Sale {
    fn process(&mut self) {
        if self.sold_book.quantity >= self.quantity {
            self.sold_book.quantity -= self.quantity;
            println!("Sale processed successfully.");
        } else {
            eprintln!("Insufficient stock for the sold book.");
        }
    }
}

// Bookstore struct
struct Bookstore {
    catalog: Catalog,
    transactions: Vec<Box<dyn TransactionTrait>>,
}

impl Bookstore {
    fn new() -> Self {
        Bookstore {
            catalog: Catalog::new(),
            transactions: Vec::new(),
        }
    }

    fn manage_catalog(&self) {
        for book in &self.catalog.books {
            println!("Title: {}", book.title);
            println!("Author: {}", book.author);
            println!("Genre: {}", book.genre);
            println!("ISBN: {}", book.isbn);
            println!(
                "Publication Date: {}/{}/{}",
                book.publication_date.day, book.publication_date.month, book.publication_date.year
            );
            println!("Price: {}", book.price);
            println!("Quantity: {}", book.quantity);
            println!();
        }
    }

    fn process_transaction(&mut self, transaction: Box<dyn TransactionTrait>) {
        self.transactions.push(transaction);
    }
}

fn display_menu() {
    println!("Bookstore Management System");
    println!("---------------------------");
    println!("1. Add book");
    println!("2. Display books");
    println!("3. Record Order transaction");
    println!("4. Record Sale transaction");
    println!("5. Display transactions");
    println!("6. Exit");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut bookstore = Bookstore::new();
    let mut choice;

    loop {
        display_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        choice = input.trim().parse().unwrap();

        match choice {
            1 => {
                let mut title = String::new();
                let mut author = String::new();
                let mut genre = String::new();
                let mut isbn = String::new();
                let mut day = 0;
                let mut month = 0;
                let mut year = 0;
                let mut quantity = 0;
                let mut price = 0;

                println!("Enter book title: ");
                io::stdin().read_line(&mut title).unwrap();

                println!("Enter author: ");
                io::stdin().read_line(&mut author).unwrap();

                println!("Enter genre: ");
                io::stdin().read_line(&mut genre).unwrap();

                println!("Enter ISBN: ");
                io::stdin().read_line(&mut isbn).unwrap();

                println!("Enter published date (dd mm yyyy): ");
                let mut date_input = String::new();
                io::stdin().read_line(&mut date_input).unwrap();
                let date_parts: Vec<&str> = date_input.trim().split_whitespace().collect();
                day = date_parts[0].parse().unwrap();
                month = date_parts[1].parse().unwrap();
                year = date_parts[2].parse().unwrap();

                println!("Enter quantity: ");
                let mut quantity_input = String::new();
                io::stdin().read_line(&mut quantity_input).unwrap();
                quantity = quantity_input.trim().parse().unwrap();

                println!("Enter price: ");
                let mut price_input = String::new();
                io::stdin().read_line(&mut price_input).unwrap();
                price = price_input.trim().parse().unwrap();

                let date = Date::new(day, month, year);
                let book = Book::new(
                    title.trim().to_string(),
                    author.trim().to_string(),
                    genre.trim().to_string(),
                    isbn.trim().to_string(),
                    date,
                    price,
                    quantity,
                );
                bookstore.catalog.add_book(book);
                println!("Book added successfully!");
            }
            2 => {
                bookstore.manage_catalog();
            }
            3 => {
                let mut transaction_id = String::new();
                let mut customer_id = String::new();
                let mut isbn = String::new();
                let mut day = 0;
                let mut month = 0;
                let mut year = 0;
                let mut quantity = 0;

                println!("Enter transaction ID: ");
                io::stdin().read_line(&mut transaction_id).unwrap();

                println!("Enter customer ID: ");
                io::stdin().read_line(&mut customer_id).unwrap();

                println!("Enter ISBN: ");
                io::stdin().read_line(&mut isbn).unwrap();

                println!("Enter transaction date (dd mm yyyy): ");
                let mut date_input = String::new();
                io::stdin().read_line(&mut date_input).unwrap();
                let date_parts: Vec<&str> = date_input.trim().split_whitespace().collect();
                day = date_parts[0].parse().unwrap();
                month = date_parts[1].parse().unwrap();
                year = date_parts[2].parse().unwrap();

                println!("Enter quantity: ");
                let mut quantity_input = String::new();
                io::stdin().read_line(&mut quantity_input).unwrap();
                quantity = quantity_input.trim().parse().unwrap();

                let transaction_date = Date::new(day, month, year);
                match bookstore.catalog.search_book(&isbn.trim()) {
                    Ok(ordered_book) => {
                        let order = Order::new(
                            transaction_id.trim().to_string(),
                            transaction_date,
                            customer_id.trim().to_string(),
                            ordered_book.clone(),
                            quantity,
                        );
                        bookstore.process_transaction(Box::new(order));
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            4 => {
                let mut transaction_id = String::new();
                let mut customer_id = String::new();
                let mut isbn = String::new();
                let mut day = 0;
                let mut month = 0;
                let mut year = 0;
                let mut quantity = 0;

                println!("Enter transaction ID: ");
                io::stdin().read_line(&mut transaction_id).unwrap();

                println!("Enter customer ID: ");
                io::stdin().read_line(&mut customer_id).unwrap();

                println!("Enter ISBN: ");
                io::stdin().read_line(&mut isbn).unwrap();

                println!("Enter transaction date (dd mm yyyy): ");
                let mut date_input = String::new();
                io::stdin().read_line(&mut date_input).unwrap();
                let date_parts: Vec<&str> = date_input.trim().split_whitespace().collect();
                day = date_parts[0].parse().unwrap();
                month = date_parts[1].parse().unwrap();
                year = date_parts[2].parse().unwrap();

                println!("Enter quantity: ");
                let mut quantity_input = String::new();
                io::stdin().read_line(&mut quantity_input).unwrap();
                quantity = quantity_input.trim().parse().unwrap();

                let transaction_date = Date::new(day, month, year);
                match bookstore.catalog.search_book(&isbn.trim()) {
                    Ok(sold_book) => {
                        let sale = Sale::new(
                            transaction_id.trim().to_string(),
                            transaction_date,
                            customer_id.trim().to_string(),
                            sold_book.clone(),
                            quantity,
                        );
                        bookstore.process_transaction(Box::new(sale));
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            5 => {
                for transaction in &bookstore.transactions {
                    println!("Transaction ID: {}", transaction.get_transaction_id());
                    println!(
                        "Transaction Date: {}/{}/{}",
                        transaction.get_transaction_date().day,
                        transaction.get_transaction_date().month,
                        transaction.get_transaction_date().year
                    );
                    println!();
                }
            }
            6 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

trait TransactionTrait {
    fn get_transaction_id(&self) -> &str;
    fn get_transaction_date(&self) -> Date;
}

impl TransactionTrait for Order {
    fn get_transaction_id(&self) -> &str {
        &self.transaction_id
    }

    fn get_transaction_date(&self) -> Date {
        self.transaction_date
    }
}

impl TransactionTrait for Sale {
    fn get_transaction_id(&self) -> &str {
        &self.transaction_id
    }

    fn get_transaction_date(&self) -> Date {
        self.transaction_date
    }
}