use std::io;

/// Populates an empty String object with user input through standard in and returns it.
///
/// # Arguments
///
/// * `user_input` - An empty String
///
/// # Examples
///
/// ```
/// let mut user_input = String::new();
/// user_input = get_user_input(user_input);
/// ```
pub fn get_user_input(mut user_input: String) -> String {
    io::stdin()
        .read_line(&mut user_input)
        .expect("ERROR: Failed to read command.");

    return user_input;
}