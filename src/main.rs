// src/main.rs

// --- HOW TO START THIS PROJECT ---
//
// Step 1: Install Rust
// If you don't have Rust installed, open your terminal or command prompt and run:
// curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
// Follow the on-screen instructions. This will install rustup, rustc (the Rust compiler),
// and cargo (Rust's package manager and build tool).
//
// Step 2: Create a New Rust Project
// Open your terminal or command prompt and navigate to the directory where you want to create your project.
// Then run the following command:
// cargo new student_report_app
// This command creates a new directory named student_report_app with a basic Rust project structure inside it.
//
// Step 3: Navigate into the Project Directory
// Change your current directory to the newly created project:
// cd student_report_app
//
// Step 4: Open the src/main.rs file
// Inside the student_report_app directory, you'll find a src folder, and inside that, a main.rs file.
// Open this main.rs file using your favorite text editor (like VS Code, Sublime Text, or Notepad++).
//
// Step 5: Replace the Content of src/main.rs
// Delete all the existing content in src/main.rs and paste the entire code provided below into it.
//
// Step 6: Run the Application
// Save the main.rs file. Then, back in your terminal (still inside the student_report_app directory), run:
// cargo run
// Cargo will compile your code and then execute it. The application will prompt you to enter student details.
//
// Step 7: (Optional) Build for Release
// If you want to create an optimized executable for distribution, you can run:
// cargo build --release
// This will create an executable in target/release/student_report_app (or student_report_app.exe on Windows).
//
// --- END OF STARTING INSTRUCTIONS ---


use std::io::{self, Write}; // Import necessary modules for input/output

/// ---
/// ### 1. Define Student Struct
/// This struct will hold the data for each student.
/// It encapsulates the student's name, their total marks across all subjects,
/// and the total number of subjects they took.
/// ---
struct Student {
    name: String,
    total_marks: u32,
    num_subjects: u32,
}

/// ---
/// ### 2. Implement Grade Enum
/// An enum (enumeration) is a good way to represent a fixed set of possible grades.
/// This makes our code more readable and prevents invalid grade assignments.
/// Invalid is included for edge cases, like a student having zero subjects.
/// ---
enum Grade {
    A, // 90+
    B, // 75-89
    C, // 60-74
    D, // Below 60
    Invalid, // Grade cannot be determined (e.g., num_subjects = 0)
}

impl Grade {
    /// A simple method to convert the Grade enum variant into a user-friendly string.
    /// This is useful when displaying the grade in the report card.
    fn as_str(&self) -> &str {
        match self {
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
            Grade::Invalid => "N/A", // Not Applicable
        }
    }
}

/// ---
/// ### 3. Implement Student Methods
/// The impl block associates functions (called "methods" when associated with a struct)
/// with our Student struct. These methods operate on an instance of a Student.
/// ---
impl Student {
    /// Constructor-like function to create a new Student instance.
    /// Self is a type alias for the struct itself (Student in this case).
    fn new(name: String, total_marks: u32, num_subjects: u32) -> Self {
        Self {
            name,
            total_marks,
            num_subjects,
        }
    }

    /// Calculates the average marks for the student.
    /// It performs floating-point division to ensure accuracy.
    /// Handles the case of num_subjects being 0 to prevent division by zero errors.
    fn calculate_average(&self) -> f64 {
        if self.num_subjects == 0 {
            0.0 // Return 0 if there are no subjects to avoid division by zero
        } else {
            // Cast u32 to f64 for floating-point division
            self.total_marks as f64 / self.num_subjects as f64
        }
    }

    /// Assigns a grade based on the calculated average marks.
    /// Follows the specified grading criteria:
    /// A: 90+
    /// B: 75-89
    /// C: 60-74
    /// D: Below 60
    /// Returns Grade::Invalid if the number of subjects is zero.
    fn assign_grade(&self) -> Grade {
        let average = self.calculate_average(); // Get the average marks
        if self.num_subjects == 0 {
            Grade::Invalid // Cannot assign a grade if there are no subjects
        } else if average >= 90.0 {
            Grade::A
        } else if average >= 75.0 {
            Grade::B
        } else if average >= 60.0 {
            Grade::C
        } else {
            Grade::D
        }
    }

    /// Prints a neatly formatted report card for the student to the console.
    /// Uses println! macros with formatting specifiers for alignment ({:<15})
    /// and decimal precision ({:.2}).
    fn print_report_card(&self) {
        let average = self.calculate_average(); // Get average
        let grade = self.assign_grade();         // Get assigned grade

        println!("\n--- Student Report Card ---");
        println!("{:<15}: {}", "Name", self.name);          // Left-align name, 15 chars wide
        println!("{:<15}: {}", "Total Marks", self.total_marks);
        println!("{:<15}: {}", "No. Subjects", self.num_subjects);
        println!("{:<15}: {:.2}", "Average Marks", average); // .2 for 2 decimal places
        println!("{:<15}: {}", "Grade", grade.as_str());    // Display grade string
        println!("---------------------------\n");
    }
}

/// ---
/// ### 4. Input Functions
/// These are helper functions to safely and reliably get input from the user
/// via the command line. They include basic error handling for invalid input types.
/// ---

/// Reads a single line of text input from the standard input (keyboard).
/// Returns a Result to indicate success (Ok) or failure (Err) in reading the line.
fn read_line() -> io::Result<String> {
    let mut input = String::new(); // Create an empty, mutable String to store input
    io::stdin().read_line(&mut input)?; // Read line into the string. The ? operator propagates errors.
    Ok(input.trim().to_string()) // Trim whitespace (like newline characters) and convert to owned String
}

/// Prompts the user for a string input and ensures it's not empty.
/// Loops until valid (non-empty) input is provided.
fn get_string_input(prompt: &str) -> String {
    loop { // Infinite loop until valid input is received
        print!("{}", prompt); // Display the prompt to the user
        io::stdout().flush().expect("Failed to flush stdout"); // Crucial to ensure the prompt is displayed before waiting for input

        match read_line() { // Attempt to read a line
            Ok(input) if !input.is_empty() => return input, // If successful and not empty, return the input
            _ => println!("Input cannot be empty. Please try again."), // Otherwise, print error and loop again
        }
    }
}

/// Prompts the user for an unsigned 32-bit integer (u32) input.
/// Loops until a valid number is entered. Includes error handling for non-numeric input.
fn get_u32_input(prompt: &str) -> u32 {
    loop { // Infinite loop until valid input is received
        print!("{}", prompt); // Display the prompt
        io::stdout().flush().expect("Failed to flush stdout"); // Flush stdout

        let input = read_line().expect("Failed to read line"); // Read input, panic if unable to read

        match input.parse::<u32>() { // Attempt to parse the string input into a u32
            Ok(num) => return num, // If parsing is successful, return the number
            Err(_) => println!("Invalid input. Please enter a valid number."), // If parsing fails (e.g., not a number), print error and loop
        }
    }
}

/// ---
/// ### 5. Main Application Logic
/// This is the entry point of the program. Execution begins here.
/// It orchestrates the flow: welcoming the user, getting student details,
/// creating a Student object, and then printing the report card.
/// ---
fn main() {
    println!("Welcome to the Student Report Card Generator!");

    // Get student details using our helper functions
    let name = get_string_input("Enter student's name: ");
    let total_marks = get_u32_input("Enter total marks: ");
    let num_subjects = get_u32_input("Enter number of subjects: ");

    // Create a new Student instance with the collected data
    let student = Student::new(name, total_marks, num_subjects);

    // Print the report card for the created student
    student.print_report_card();

    println!("Thank you for using the Student Report Card Generator!");
}
