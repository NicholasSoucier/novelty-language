use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

//

fn main() {
    //Grab file from command args
    let args: Vec<String> = env::args().collect();
    let mut filename = String::new();
    let mut step = "";
    if args.len() == 1{
        print!("Please enter a file to read from: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut filename).unwrap();
    }
    if args.len() == 2 {
        filename = String::from(&args[1]);
    }
    if args.len() == 3 {
        filename = String::from(&args[1]);
        step = &args[2];
    }

    const INCREMENT: u32 = 0;               //+
    const DECREMENT: u32 = 1;               //-
    const SHIFTLEFT: u32 = 2;               //<
    const SHIFTRIGHT: u32 = 3;              //>
    const SHIFTNUM: u32 = 4;                //^
    const RESET: u32 = 5;                   //_
    const STACKPUSH: u32 = 6;               //#
    const STACKPOP: u32 = 7;                //$
    const INPUTNUM: u32 = 8;                //?0
    const INPUTALPHA: u32 = 9;              //?a
    const OUTPUTNUM: u32 = 10;              //&0
    const OUTPUTALPHA: u32 = 11;            //&a
    const CONDITIONALJUMP: u32 = 12;        //{
    const CONDITIONALMARKER: u32 = 13;      //}
    const NONCONDITIONALJUMP: u32 = 14;     //:
    const NONCONDITIONALMARKER: u32 = 15;   //=
    const ENDOFINPUT: u32 = 16;             //EOI

    //Grab contents of file
    let file_buffer = fs::read_to_string(filename).expect("Unable to read file");
    let mut input_string = file_buffer.clone();
    let mut line_number = 0;

    let mut token_list: Vec<u32> = Vec::new();
    let mut token_list_pointer: usize = 0;

    let mut exec_array: Vec<u32> = vec![0; 256];
    let mut exec_stack: Vec<u32> = vec![0; 64];
    let mut index_pointer: usize = 0;

    //Read in operations as tokens from the input buffer
    //---------------------------------------------------
    //                  P H A S E   1
    //---------------------------------------------------
    loop {
        ignore_whitespace(&mut input_string, &mut line_number);
        if input_string.is_empty() {
            token_list.push(ENDOFINPUT);
            break;
        }
        let next_char = get_next_char(&input_string);
        match next_char {
            '+'=> {
                token_list.push(INCREMENT);
                consume_char(&mut input_string);
            }
            '-'=> {
                token_list.push(DECREMENT);
                consume_char(&mut input_string);
            }
            '<'=> {
                token_list.push(SHIFTLEFT);
                consume_char(&mut input_string);
            }
            '>'=> {
                token_list.push(SHIFTRIGHT);
                consume_char(&mut input_string);
            }
            '^'=> {
                token_list.push(SHIFTNUM);
                consume_char(&mut input_string);
            }
            '_'=> {
                token_list.push(RESET);
                consume_char(&mut input_string);
            }
            '#'=> {
                token_list.push(STACKPUSH);
                consume_char(&mut input_string);
            }
            '$'=> {
                token_list.push(STACKPOP);
                consume_char(&mut input_string);
            }
            '?'=> {
                consume_char(&mut input_string);
                process_input_token(&mut input_string, &mut token_list);
            }
            '&'=> {
                consume_char(&mut input_string);
                process_output_token(&mut input_string, &mut token_list);
            }
            '{'=> {
                token_list.push(CONDITIONALJUMP);
                consume_char(&mut input_string);
            }
            '}'=> {
                token_list.push(CONDITIONALMARKER);
                consume_char(&mut input_string);
            }
            ':'=> {
                token_list.push(NONCONDITIONALJUMP);
                consume_char(&mut input_string);
            }
            '='=> {
                token_list.push(NONCONDITIONALMARKER);
                consume_char(&mut input_string);
            }
            _ => { print_error("Unexpected character."); }
        }
    }

    fn process_input_token(input: &mut String, tokens: &mut Vec<u32>){
        if get_next_char(input) == '0' {
            consume_char(input);
            tokens.push(INPUTNUM);
        }else if get_next_char(input) == 'a' {
            consume_char(input);
            tokens.push(INPUTALPHA);
        } else {
            print_error("Unexcexted character after ?, expected '0' or 'a'.");
        }
    }

    fn process_output_token(input: &mut String, tokens: &mut Vec<u32>){
        if get_next_char(input) == '0' {
            consume_char(input);
            tokens.push(OUTPUTNUM);
        }else if get_next_char(input) == 'a' {
            consume_char(input);
            tokens.push(OUTPUTALPHA);
        } else {
            print_error("Unexcexted character after &, expected '0' or 'a'.");
        }
    }

    //Begin execution 
    //---------------------------------------------------
    //                  P H A S E   2
    //---------------------------------------------------
    loop{
        if step == &String::from("step"){
            print!("ARRAY: [ ");
            for val in &exec_array {
                print!("{} ", val);
            }
            println!("]");
            println!("> {}", &index_pointer);
            print!("STACK: [ ");
            for val in &exec_stack {
                print!("{} ", val);
            }
            println!("]");
            println!("Press enter to continue...");
            let mut dummy = String::new();
            io::stdin().read_line(&mut dummy).unwrap();
        }
        if token_list[token_list_pointer] == ENDOFINPUT {
            break;
        }
        let current_token = token_list[token_list_pointer];
        match current_token {
           INCREMENT => {
            increment(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           DECREMENT => {
            decrement(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           SHIFTLEFT => {
            shiftleft(&mut index_pointer, &mut token_list_pointer);
           }
           SHIFTRIGHT => {
            shiftright(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           SHIFTNUM => {
            shiftnum(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           RESET => {
            reset(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           STACKPUSH => {
            stackpush(&mut exec_array, &mut exec_stack, &mut index_pointer, &mut token_list_pointer);
           }
           STACKPOP => {
            stackpop(&mut exec_array, &mut exec_stack, &mut index_pointer, &mut token_list_pointer);
           }
           INPUTNUM => {
            inputnum(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           INPUTALPHA => {
            inputalpha(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           OUTPUTNUM => {
            outputnum(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           OUTPUTALPHA => {
            outputalpha(&mut exec_array, &mut index_pointer, &mut token_list_pointer);
           }
           CONDITIONALJUMP => {
            conditionaljump(&mut exec_array, &mut index_pointer, &mut token_list_pointer, &mut token_list);
           }
           CONDITIONALMARKER => {
            token_list_pointer += 1;
           }
           NONCONDITIONALJUMP => {
            nonconditionaljump(&mut token_list, &mut token_list_pointer);
           }
           NONCONDITIONALMARKER => {
            token_list_pointer += 1;
           }
           _ => {
            print_error("We have no idea how, but the token list is corrupted. Very sorry");
           }
        }
    }
}

fn increment(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    if exec_array[*index_pointer] < 4294967295 {
        exec_array[*index_pointer] += 1;
    }else{
        print_error("Attempted to increment above value 2^32 at INCREMENT '+'");
    }
    *token_list_pointer += 1;
}

fn decrement(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    if exec_array[*index_pointer] > 0 {
        exec_array[*index_pointer] -= 1;
    }else{
        print_error("Attempted to decrement below value 0 at DECREMENT '-'");
    }
    *token_list_pointer += 1;
}

fn shiftleft(index_pointer: &mut usize, token_list_pointer: &mut usize){
    if *index_pointer > 0 {
        *index_pointer -= 1;
    }else{
        print_error("Attempted to index below 0 at SHIFTLEFT '<'");
    }
    *token_list_pointer += 1;
}
fn shiftright(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    if *index_pointer < exec_array.len()  {
        *index_pointer += 1;
    }else{
        print_error("Attempted to index above array size at SHIFTRIGHT '>'");
    }
    *token_list_pointer += 1;
}
fn shiftnum(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    if (exec_array[*index_pointer] as usize) < exec_array.len() {
        *index_pointer = exec_array[*index_pointer] as usize;
    }else{
        print_error("Attempted to index above array size at SHIFTNUM '_'");
    }
    *token_list_pointer += 1;
}
fn reset(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    exec_array[*index_pointer] = 0;
    *token_list_pointer += 1;
}
fn stackpush(exec_array: &mut Vec<u32>, exec_stack: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    exec_stack.push(exec_array[*index_pointer]);
    *token_list_pointer += 1;
}
fn stackpop(exec_array: &mut Vec<u32>, exec_stack: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    if exec_stack.is_empty() {
        exec_array[*index_pointer] = 0;
    }else{
        let pop_value = exec_stack.pop().unwrap();
        exec_array[*index_pointer] = pop_value;
    }
    *token_list_pointer += 1;
}
fn outputnum(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    print!("{}", exec_array[*index_pointer]);
    io::stdout().flush().unwrap();
    *token_list_pointer += 1;
}
fn outputalpha(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    print!("{}", char::from_u32(exec_array[*index_pointer]).unwrap());
    io::stdout().flush().unwrap();
    *token_list_pointer += 1;
}
fn inputnum(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).unwrap();
    input_buffer = String::from(input_buffer.trim());
    exec_array[*index_pointer] = input_buffer.parse::<u32>().unwrap();
    *token_list_pointer += 1;
}
fn inputalpha(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize){
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).unwrap();
    input_buffer = String::from(input_buffer.trim());
    let index_store = *index_pointer;
    for letter in input_buffer.chars(){
        exec_array[*index_pointer] = letter as u32;
        *index_pointer += 1;
    }
    *index_pointer = index_store;
    *token_list_pointer += 1;
}
fn conditionaljump(exec_array: &mut Vec<u32>, index_pointer: &mut usize, token_list_pointer: &mut usize, token_list: &mut Vec<u32>){
    let mut conditional_match = 0;
    //If the value of the array[index] = 0, perform a conditional and matched jump.
    if exec_array[*index_pointer] == 0 {
        *token_list_pointer += 1;
        loop {
            if *token_list_pointer == token_list.len() {
                print_error("Unable to find matching '}' for conditional jump '{'");
            }
            else if token_list[*token_list_pointer] == 12 {     //Found '{' before matching marker
                conditional_match += 1;
                *token_list_pointer += 1;
            }
            else if token_list[*token_list_pointer] == 13 {     //Found '}'
                if conditional_match == 0{                      //Found matching '}'
                    *token_list_pointer += 1;
                    break;
                } else {                                        //Found nonmatching '}'
                    conditional_match -= 1;
                    *token_list_pointer += 1;
                }
            } else {
                *token_list_pointer += 1;
            }
        }
    } else {
        *token_list_pointer += 1;
    }
}
fn nonconditionaljump(token_list: &mut Vec<u32>, token_list_pointer: &mut usize){
    let mut noncon_match = 0;
    *token_list_pointer -= 1;
    loop {
        if *token_list_pointer == 0 {
            print_error("Unable to find matching '=' for non conditional jump ':'");
        } else if token_list[*token_list_pointer] == 14 {   //Found ':' before matching marker
            noncon_match += 1;
            *token_list_pointer -= 1;
        } else if token_list[*token_list_pointer] == 15 {   //Found '='
            if noncon_match == 0 {                          //Found matching '='
                *token_list_pointer += 1;
                break;
            } else {                                        //Found nonmatching '='
                noncon_match -= 1;
                *token_list_pointer -= 1;
            }
        } else {
            *token_list_pointer -= 1;
        }
    }
}


fn get_next_char(s: &String) -> char {
    return s.chars().next().unwrap();
}

fn consume_char(s: &mut String) {
    s.remove(0);
}

fn compare_chars(c1: char, c2: char) -> bool {
    if c1 == c2 {
        return true;
    }else{
        return false;
    }
}

fn ignore_whitespace(s: &mut String, linenum: &mut u32) {
    loop {
        if s.is_empty() {
            break;
        }
        if compare_chars(get_next_char(&s), ' ') {
            consume_char(s);
        } 
        //Test for tab whitespace
        else if compare_chars(get_next_char(&s), '\t') {
            consume_char(s);
        } 
        //Test for newline whitespace
        else if get_next_char(&s).is_whitespace() {
            consume_char(s);
            *linenum += 1;
        } 
        //Test for comment-based whitespace
        //Comments start with '//' and continue until the next line
        else if compare_chars(get_next_char(&s), '/') {
            if compare_chars(s.chars().nth(1).unwrap(), '/') {
                //Treat rest of line as comment
                loop {
                    if s.chars().nth(0).unwrap() != '\n'{
                        consume_char(s);
                    }else{
                        consume_char(s);
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
}

fn print_error(msg: &str) {
    println!("[ERROR]: {}", msg);
    process::exit(0);
}