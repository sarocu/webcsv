# webcsv
webcsv is a csv loader built in Rust and compiled to WebAssembly. The goal of the project is to provide a really fast CSV package in JavaScript that will load the CSV file, return basic info about the file (headers & number of rows), and provide a pagination method for getting row data. 

## To-Do
- Implement indexing via (csv-index)[https://docs.rs/csv-index/0.1.6/csv_index/]
