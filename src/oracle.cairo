use starknet::testing::cheatcode;
#[derive(Drop, Serde)]
struct MathRequest {
    operation: super::oracle::Operation,
    operands: Array<i64>,
}
#[derive(Drop, Serde)]
struct MathResponse {
    result: i64,
}
#[derive(Drop, Serde, PartialEq)]
enum Operation {
    Unknown,
    Sqrt,
    Exp,
}
#[generate_trait]
impl MathOracle of MathOracleTrait {
    fn calculate(arg: super::oracle::MathRequest) -> super::oracle::MathResponse {
        let mut serialized = ArrayTrait::new();
        arg.serialize(ref serialized);
        let mut result = cheatcode::<'calculate'>(serialized.span());
        Serde::deserialize(ref result).unwrap()
    }
}
