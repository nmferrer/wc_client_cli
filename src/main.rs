use std::io;
use std::net::TcpStream;

fn establish_connection() {
    //Confirm that server is live.
    //For debug: check local development server.
    //For production: check host address.

    //Connect to remote host. Close on application exit or user input: "[q]uit"
    let ip = String::from("127.0.0.1:8080"); //TODO: set constants in config file
    let mut stream = TcpStream::connect(ip);
    
    //Connection successful: Continue to main app.
    //Connection fails: Notify user. Terminate.
}

fn parse_buffer(buf: &String) {
    //String Parsing //Expected format: {command|location0...n|-optional_flags}
    let fields: Vec<&str> = buf.split(' ').collect();
    //TODO: note that splitting by white space mishandles compound words
    //Handle locations differently? In square braces? Separate by commas?

    //Possible feature: optional database interactions, log user profile on server
    //Command:  [weather|add|remove|compare]
    //Location: [city | city,state | city,country] REGULAR EXPRESSION?
        //Handle ambiguity, offer suggestion when multiple instances exist OR no options exist
    //Flag:     [-explicit (detailed list)]

    //Command: 1 command only, always first index. Consider piped commands?
    //Location(s): 1 or more, no leading character
    //Flag: optional, denoted by '-'
    
    let command: &str = fields[0];
    //Error check: Match input to valid commands.
    
    //Should I use fixed array here? Size will never exceed size of buffer.
    let mut locations: Vec<&str> = Vec::new();
    let mut flags: Vec<&str> = Vec::new();
    for s in &fields[1..] {
        if s.as_bytes()[0] == b'-' {
            flags.push(s.trim());
        } else {
            locations.push(s.trim());
        }
    }
    //DEBUG
    println!("DEBUG_COMMAND: {}", command);
    println!("DEBUG_LOCATIONS: {:?}", locations);
    println!("DEBUG_FLAGS: {:?}", flags);

    //OK: PARSING
    //TODO: VERIFY VALID INPUT
    //TODO: SEND REQUEST TO SERVER, server will confirm via API + DB
    
    //Command
    match command { //OK. Extend to other commands.
        "weather" => println!("Requesting weather report."),
        _ => println!("Invalid Command."),
    }
    //Location (not client's job to verify existence)
    
    //Flags TODO: Check that each flag is valid, raise error if one fails
    //for f in flags {}

    //Valid: Build JSON.
    //Error: Raise error.
}
fn generate_api_request() {
    //TODO: read and write to TcpStream 
}
fn receive_response() {
    //await response from server
}

fn main() {
    //While localhost:8080 is live, read terminal until newline char.
    //Parse string, create valid REST object adhering to API, send to server.
    //Output response at terminal.
    
    //TODO: Check if server live.
    establish_connection();
    println!("Server is live. Input request:");

    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(n) => {
            println!("DEBUG: {} bytes read.", n);
            parse_buffer(&buf);
            generate_api_request();
            receive_response();
        }
        Err(error) => println!("error: {error}"),
    }
}
