use mirage::{split_command_vector, ChainCommands, Rotation};

#[test]
fn split_commands_vector_simple_test() {
    // given
    let commands = vec!["blur".to_string(), "rotate 90".to_string()];

    // when
    let result = split_command_vector(&commands);

    // then
    let expected_result = vec![
        ChainCommands::Blur {},
        ChainCommands::Rotate { rotation: Rotation::Ninety }
    ];
    assert_eq!(result, expected_result);
}