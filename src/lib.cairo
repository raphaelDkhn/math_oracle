mod oracle;

use oracle::{MathRequest, MathOracle, Operation};

fn main() -> bool {
    let num = 1764;

    let request = MathRequest { operation: Operation::Sqrt, operands: array![1764] };
    let result = MathOracle::calculate(request);

    result.result * result.result == num
}
