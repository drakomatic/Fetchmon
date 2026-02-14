//private functions

//public functions
pub fn get_message_string(message_string_id : String) -> String {
    let full_string_name: String = "msg_".to_string() + &message_string_id;

    return full_string_name;
}

