const NAME_AREA: i32 = 18; // Name area end
const VALUE_AREA: i32 = NAME_AREA + 2; // Value area start
const MESSAGE_ROWS: i32 = 2;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Incorrect amount of parameters! Give only json filename!");
        return;
    }

    let mut parsed_json = json::parse(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();

    let mut row = MESSAGE_ROWS;

    let mut ez_curses = easycurses::EasyCurses::initialize_system().unwrap();

    ez_curses.set_echo(false);

    ez_curses.move_rc(0, 0);
    ez_curses.print("Json editor, editing ");
    ez_curses.print(&args[1]);
    ez_curses.print(". Press F1 to quit.");

    // Only accepting JSON arrays for now. Change this if you want to broaden the use to your own use case.
    // NOTE: This doesn't check whether the array itself has the correct formatting!
    if !parsed_json.is_array() {
        ez_curses.move_rc(5, 5);
        ez_curses.print("CURRENTLY ONLY SUPPORTING A SIMPLE ARRAY JSON!");
        ez_curses.refresh();
        ez_curses.get_input();
        return;
    }

    let mut value_vec: Vec<String> = Vec::new();

    // |----------------------------------------------------------------------------------------------------------
    // | MAJOR_PORTION: Reading JSON values to vector and writing names along with starting values to the screen |
    // |----------------------------------------------------------------------------------------------------------
    for json_array_index in 0..parsed_json.len() {
        ez_curses.move_rc(row, 0);
        ez_curses.print(parsed_json[json_array_index]["name"].as_str().unwrap());
        ez_curses.move_rc(row, NAME_AREA);
        ez_curses.print(":");
        ez_curses.move_rc(row, VALUE_AREA);

        if parsed_json[json_array_index]["value"].is_string() {
            ez_curses.print(parsed_json[json_array_index]["value"].as_str().unwrap());
            value_vec.push(
                parsed_json[json_array_index]["value"]
                    .as_str()
                    .unwrap()
                    .to_owned(),
            );
        } else if parsed_json[json_array_index]["value"].is_number() {
            ez_curses.print(
                parsed_json[json_array_index]["value"]
                    .as_number()
                    .unwrap()
                    .to_string(),
            );
            value_vec.push(
                parsed_json[json_array_index]["value"]
                    .as_number()
                    .unwrap()
                    .to_string(),
            );
        } else {
            ez_curses.print("UNSUPPORTED VALUE TYPE!");
            value_vec.push("".to_owned());
        }
        row += 1;
    }
    // JSON reading major portion end

    row = MESSAGE_ROWS;
    let mut col: i32 = VALUE_AREA + value_vec[0].len() as i32;
    ez_curses.move_rc(row, col);

    //|--------------------------------------------------------------------------------------------
    //| MAJOR_PORTION: Reading user inputs into the vector                                        |
    //|--------------------------------------------------------------------------------------------
    while let Some(input) = ez_curses.get_input() {
        let mut running = true;

        // println!("{:?}", input); // <--DEBUG

        // NOTE: For some reason all the inputs are read as characters, not as KeyF1, KeyBackspace, KeyDown etc.
        if let easycurses::Input::Character(char) = input {
            match char {
                '\u{7f}' => {
                    // Backspace
                    if col > VALUE_AREA {
                        col -= 1;
                        ez_curses.move_rc(row, col);
                        ez_curses.print_char(' ');
                        value_vec[row as usize - MESSAGE_ROWS as usize].pop();
                    }
                }
                '\u{1b}' => {
                    // Arrow keys, F1
                    // NOTE: Needs refactoring!
                    // NOTE: If break is used in match, it breaks from the while loop eventually
                    // NOTE: Aka. it cannot be used in here
                    if let easycurses::Input::Character(char) = ez_curses.get_input().unwrap() {
                        // Arrow keys: '[' 'A-D'
                        if char == '[' {
                            if let easycurses::Input::Character(char) =
                                ez_curses.get_input().unwrap()
                            {
                                match char {
                                    'A' => {
                                        if row > MESSAGE_ROWS {
                                            row -= 1;
                                            col = VALUE_AREA
                                                + value_vec[row as usize - MESSAGE_ROWS as usize]
                                                    .len()
                                                    as i32;
                                        }
                                    }
                                    'B' => {
                                        if row < value_vec.len() as i32 + 1 {
                                            row += 1;
                                            col = VALUE_AREA
                                                + value_vec[row as usize - MESSAGE_ROWS as usize]
                                                    .len()
                                                    as i32;
                                        }
                                    }
                                    // Not using left/right arrows
                                    // 'C' => col += 1,
                                    // 'D' => col -= 1,
                                    _ => {}
                                }
                            }
                        }
                        // F1 key: 'O' 'P'
                        else if char == 'O' {
                            if let easycurses::Input::Character(char) =
                                ez_curses.get_input().unwrap()
                            {
                                if char == 'P' {
                                    running = false;
                                }
                            }
                        }
                    }
                }
                '\n' => {
                    // NOTE: Same action as arrow down, could be refactored?
                    if row < value_vec.len() as i32 + 1 {
                        row += 1;
                        col = VALUE_AREA
                            + value_vec[row as usize - MESSAGE_ROWS as usize].len() as i32;
                    }
                }
                _ => {
                    ez_curses.print_char(char);
                    value_vec[row as usize - MESSAGE_ROWS as usize].push(char);
                    col += 1;
                }
            }
        }
        if !running {
            break;
        }
        ez_curses.move_rc(row, col);
    }
    // User input major portion end

    //|--------------------------------------------------------------------------------------------
    //| MAJOR_PORTION: Parsing new values and writing new JSON                                    |
    //|--------------------------------------------------------------------------------------------
    for json_array_index in 0..parsed_json.len() {
        if parsed_json[json_array_index]["value"].is_string() {
            parsed_json[json_array_index]["value"] =
                json::JsonValue::from(value_vec[json_array_index].clone());
        } else if parsed_json[json_array_index]["value"].is_number() {
            // TODO: Handle parse error
            parsed_json[json_array_index]["value"] =
                json::JsonValue::from(value_vec[json_array_index].parse::<i32>().unwrap());
        }
    }
    // Parsing values for JSON major portion end

    // TODO: Warn the user if write failed
    std::fs::write(&args[1], parsed_json.pretty(2));
}
