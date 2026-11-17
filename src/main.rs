use std::io::{Read, Write}; // Read gives us the ability to read bytes from a connection. Write gives us the ability to send bytes.
use std::net::TcpListener; // TcpListener lets us accept incoming TCP connections. TCP is a byte stream. A single read() is not guaranteed to receive an entire HTTP request. It might receive only part of it.

fn main() {
    let host: &str = "127.0.0.1"; // 127.0.0.1 is the loopback address. It means the server is listening for connections from the same machine.
    let port: u16 = 8080;

    let address = format!("{}:{}", host, port);

    // Create the TCP Listening socket
    let listener = TcpListener::bind(&address).unwrap(); //bind() attempts to reserve the requested address and port for our listening socket.unwrap() means: if this operation fails, panic and stop the program.

    println!("Rust HTTP Server started!");
    println!("Listening on http://{}", address);

    // Wait for incoming client connection
    for connection in listener.incoming() {
        // connection is the loop variable. Each time through the loop, listener.incoming() waits for and accepts another TCP connection, yielding a Result<TcpStream, std::io::Error>. So connection.unwrap() gets the TcpStream when accepting succeeds, or panics if it fails.
        let mut stream = connection.unwrap();

        println!("Client connected!");

        let mut buffer = [0u8; 4096]; // This creates an array of 4096 bytes, initialized to zero. mut -Because the socket read operation needs permission to modify the array by filling it with received bytes.

        let bytes_read: usize = stream.read(&mut buffer).unwrap(); // The read() method receives a mutable reference to our buffer. The operating system receives network data, and the Rust standard library makes the available bytes accessible to our program. The return value tells us how many bytes were read.

        println!("Received {} bytes", bytes_read);

        let request = String::from_utf8_lossy(&buffer[..bytes_read]); // converts bytes to text and if not valid utf-8 then replaces with � rather than returning an error.

        println!("Request: {}", request);

        let response = "HTTP/1.1 200 OK\r\n\
             Content-Type: text/plain\r\n\
             Content-Length: 17\r\n\
             Connection: close\r\n\
             \r\n\
             Hello from Rust!\n";

        stream.write_all(response.as_bytes()).unwrap(); // as_bytes() gives us a byte representation of the response. write_all() attempts to write the entire byte sequence, handling partial writes internally. It returns an error if it cannot complete.

        println!("Response sent!");
    }
}

// fn main() {
//     // let port: u16 = 8080;
//     // let connections: u32 = 50000;
//     // let request_size: u64 = 1048576;

//     // println!("port size: {}", std::mem::size_of_val(&port));
//     // println!("connections size: {}", std::mem::size_of_val(&connections));
//     // println!("request size: {}", std::mem::size_of_val(&request_size));

//     // println!("port address: {:p}", &port);
//     // println!("connections address: {:p}", &connections);
//     // println!("request address: {:p}", &request_size);

//     let name = String::from("Rust HTTP Server");

//     println!("String value: {}", name);

//     println!("Size of String: {} bytes", std::mem::size_of_val(&name));

//     println!("Address of String: {:p}", &name);

//     println!("Address of string data: {:p}", name.as_ptr());

//     println!("Length: {}", name.len());

//     println!("Capacity: {}", name.capacity());

//     println!("\nIndividual bytes:");

//     for (index, byte) in name.as_bytes().iter().enumerate() {
//         println!(
//             "Index: {:2} | Decimal: {:3} | Hex: 0x{:02X} | Char: {}",
//             index, byte, byte, *byte as char
//         );
//     }

//     // let mut data = String::with_capacity(100);

//     // println!("Initial length: {}", data.len());
//     // println!("Initial capacity: {}", data.capacity());

//     // data.push_str("Hello");

//     // println!("After insertion:");
//     // println!("Length: {}", data.len());
//     // println!("Capacity: {}", data.capacity());

//     // data.push_str(" World");

//     // println!("After second insertion:");
//     // println!("Length: {}", data.len());
//     // println!("Capacity: {}", data.capacity());

//     let mut data = String::with_capacity(8);

//     println!("Initial:");
//     println!("Address: {:p}", data.as_ptr());
//     println!("Length: {}", data.len());
//     println!("Capacity: {}", data.capacity());

//     data.push_str("Hello");

//     println!("\nAfter Hello:");
//     println!("Address: {:p}", data.as_ptr());
//     println!("Length: {}", data.len());
//     println!("Capacity: {}", data.capacity());

//     data.push_str(" World!");

//     println!("\nAfter World:");
//     println!("Address: {:p}", data.as_ptr());
//     println!("Length: {}", data.len());
//     println!("Capacity: {}", data.capacity());

//     let mut buffer = [0u8; 4096];

//     let bytes_read = 100;
//     let received_data = &buffer[..bytes_read]; //take elements starting at index 0 up to, but not including, index 100”—indices 0, 1, 2, 3, 4 ... 99

//     println!("Received Data: {:?}", received_data);

//     let request: String = String::from("GET /hello HTTP/1.1");

//     let another_request = request;

//     // println!("Another request: {}", request);
//     println!("Another request: {}", another_request);

//     // Reference
//     let number: u64 = 8080;

//     let owned_string = String::from("HTTP");

//     let borrowed_string: &str = &owned_string;

//     println!("u64 size: {} bytes", std::mem::size_of_val(&number)); // &number - This creates a reference to number.

//     println!(
//         "String size: {} bytes",
//         std::mem::size_of_val(&owned_string)
//     );

//     println!(
//         "&str size: {} bytes",
//         std::mem::size_of_val(&borrowed_string)
//     );

//     let request_example = String::from("GET /hello HTTP/1.1");

//     let request_line = &request_example[..18]; // This selects a portion of the string. It produces a string slice (str), which is a view into the original string data. It does not allocate another string or copy those 18 bytes.

//     // The & operator creates a reference to that selected portion.

// }

// fn main() {
//     let mut number: u64 = 100;

//     let reference1 = &number;
//     let reference2 = reference1;

//     println!("Reference 1: {}", reference1);
//     println!("Reference 2: {}", reference2);

//     let reference3 = &mut number;

//     *reference3 = 500;

//     println!("Number: {}", number);
// }

// fn main() {
//     let mut value = 100;

//     let reader = &value;

//     println!("Reader: {}", reader);
//     let writer = &mut value;

//     *writer = 200;
// }
