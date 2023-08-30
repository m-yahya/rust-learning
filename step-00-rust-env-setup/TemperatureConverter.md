# Temperature Converter

## Table of Contents

- [Introduction](#introduction)
  - [Objectives](#objectives)
  - [Concepts](#concepts)
- [Features](#features)
- [Workflow](#workflow)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [How to Use](#how-to-use)
- [Examples](#examples)

## Introduction

Welcome to the Temperature Converter project! This is a simple Rust application that allows you to convert temperatures between Celsius and Fahrenheit scales. By building this application, you'll practice fundamental Rust concepts while creating a practical tool for temperature conversion.

### Objectives

- Build a basic Rust application to convert temperatures.
- Gain hands-on experience with user input, mathematical operations, and conditional statements in Rust.
- Solidify your understanding of the Rust development workflow.

### Concepts

- **User Input:** Receive temperature values from the user.
- **Mathematical Operations:** Perform calculations to convert temperatures.
- **Conditional Statements:** Implement logic to handle different conversion directions.

## Features

- Convert temperatures from Celsius to Fahrenheit and vice versa.
- User-friendly interface for input and output.

## Workflow

1. Objectives

   - By completing this project, you’ll gain experience with Rust’s syntax, data types, control flow structures, and standard input/output.
   - You’ll also learn how to write and call functions in Rust.

2. Concepts

   - **Data types:** Rust has several built-in data types, including integers, floating-point numbers, booleans, characters, and strings. In this project, we’ll use floating-point numbers to represent temperatures.

   ```rust
   let celsius: f32 = 25.0;
   let fahrenheit: f32 = 77.0;
   ```

   - **Functions:** Functions are used to organize code into reusable blocks. In this project, we’ll write two functions: one to convert Celsius to Fahrenheit and another to convert Fahrenheit to Celsius.

   ```bash
   fn celsius_to_fahrenheit(celsius: f32) -> f32 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }
   ```

   - **Control flow:** Rust has several control flow structures, including `if/else` statements and `match` expressions. In this project, we’ll use an `if/else` statement to determine which conversion function to call based on user input.

   ```rust
   if conversion_type == "c"{
        let celceius = fahrenheit_to_celsius(temperature);
        println!(
            "{} degrees Fahrenheit is equal to {} degrees Celsius",
            temperature, celsius
        );
    }else{
        let fahrenheit=celsius_to_fahrenheit(temperature);
        println!(
            "{} degrees Celsius is equal to {} degrees Fahrenheit",
            temperature, fahrenheit
        );
    }
   ```

   - **Standard input/output:** Rust has built-in support for reading from standard input and writing to standard output. In this project, we’ll use the `std::io` module to read user input and the `println!` macro to print output.

   ```rust
   use std::io;
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");

   println!("You entered: {}", input);
   ```

3. Features

    - The Temperature Converter will allow users to convert temperatures between Celsius and Fahrenheit.
    - Users will be able to enter a temperature and specify the scale they want to convert it to.

4. Workflow

    - Import the necessary modules and declare any variables you’ll need.
    - Prompt the user for the scale they want to convert their temperature to (Celsius or Fahrenheit).
    - Read the user’s response from standard input and store it in a variable.
    - Prompt the user for the temperature they want to convert.
    - Read the user’s response from standard input and store it in a variable.
    - Use an if/else statement or a match expression to determine which conversion function to call based on the user’s desired scale.
    - Call the appropriate conversion function with the user’s temperature as an argument.
    - Print the result of the conversion.


## Getting Started

To use the Temperature Converter, ensure you have Rust and Cargo installed.

### Prerequisites

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)

### Installation

- Build the project:

```bash
$ cargo build
```

### Usage

Run the Temperature Converter:

```bash
$ cargo run
```

### How to Use

- Enter the temperature value.
- Select the conversion direction (Celsius to Fahrenheit or vice versa).
- View the converted temperature.

### Examples

Here are a few example conversions:

- Celsius to Fahrenheit:

Input: 25 °C

Output: 77 °F

- Fahrenheit to Celsius:

Input: 98.6 °F

Output: 37 °C

Explore and experiment with different temperature values to see the converter in action!
