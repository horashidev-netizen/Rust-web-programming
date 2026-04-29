#[derive(Debug)]
pub struct ContractOne {
    input_data: String,
    output_data: Option<Result<String, String>>
}

#[derive(Debug)]
pub struct ContractTwo {
    input_data: String,
    output_data: Option<Result<String, String>>,
}

pub fn handle_contract_one(mut contract: ContractOne) -> ContractOne {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output data".to_string()));
    contract
}

pub fn handle_contract_two(mut contract: ContractTwo) -> ContractTwo {
    println!("{}", contract.input_data);
    contract.output_data = Some(Ok("Output data".to_string()));
    contract
}

#[ macro_export ]
macro_rules! register_contract_routes {
    (
        $handler_enum:ident, // ContractHandler
        $fn_name:ident, //handle_contract
        $( $contracts:ident => $handler_fn:path ),* //
    ) => {
        pub fn $fn_name(received_msg: $handler_enum) -> $handler_enum {
                match received_msg {
                    $(
                        $handler_enum::$contract(inner) => {
                            let executed_contract = $handler_fn(inner);
                            return $handler_enum::$contract(
                                executed_contract
                            )
                        }
                    )*
                }
            }
    };
}

fn main() {
    println!("Hello, world!");
}
