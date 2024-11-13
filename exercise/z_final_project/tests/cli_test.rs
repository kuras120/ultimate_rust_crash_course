use mirage::split_command_vector;

#[test]
fn split_commands_vector_simple_test() {
    // given
    let commands = vec!["blur"];

    // when
    split_command_vector(commands);
}