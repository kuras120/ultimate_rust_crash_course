use std::panic::catch_unwind;
use mirage::{split_command_vector, ChainCommands, Rotation};

#[test]
fn split_commands_vector_happy_path_test() {
    // given
    let commands: Vec<String> = "blur / rotate 90".split_whitespace().map(String::from).collect();

    // when
    let result = split_command_vector(&commands);

    // then
    let expected_result = vec![
        ChainCommands::Blur {},
        ChainCommands::Rotate { rotation: Rotation::Ninety }
    ];
    assert_eq!(result, expected_result);
}

#[test]
fn split_commands_vector_wrong_number_of_arguments_test() {
    // given
    let commands: Vec<String> = "blur / rotate".split_whitespace().map(String::from).collect();

    // when
    let result = catch_unwind(|| split_command_vector(&commands));

    // then
    assert!(result.is_err());
    let binding = result.unwrap_err();
    let err = binding.downcast_ref::<String>();
    assert_eq!(err.unwrap(), "Rotate <rotation (90, 180, 270)>");
}
