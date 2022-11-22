use crate::{
	helpers::{create_linear_numbers_array, next_token_with_index},
	Entity, Node,
};
use core::ops::Range;
use lexer::tokens::{TokenDeclaration, TokenType};

use self::variables::InterfaceVariable;
use self::variables::parse_variable;

use super::enumerate::parse_enum;

mod variables; 

#[derive(Debug)]
pub struct Interface {
	pub name: String,
	pub variables: Vec<InterfaceVariable>,
}

// 
// Interface declaration
// 
// Example:
// interface Test {
//     variables;
//	   enums;
//	   interfaces; 
// }
//
// Structure:
// 1: InterfaceDeclaration Text RightCurlyBrace
// 2:		| InterfaceDeclaration => parse_interface
// 3:		| EnumDeclaration      => parse_enum
// 4:		| OptionalModifier	   => parse_variable
// 5:		| RequiredModifier     => parse_variable
// 6: LeftCurlyBrace
pub fn parse_interface(tokens: &Vec<TokenDeclaration>, start_index: usize) -> Node {
	// Interface information
	let mut name: Option<String> = Option::None;
	let mut nodes = Vec::<Node>::new();
	let mut variables = Vec::<InterfaceVariable>::new();
	
	// Parsing info
	let mut current_index = start_index;
	let mut parsed_indicies = Vec::<usize>::new();
	let mut end_index: Option<usize> = Option::None;

	// 
	// Parsing First Line of structure
	// > InterfaceDeclaration
	{
		let token = tokens.get(current_index).unwrap();

		if token.token_type != TokenType::InterfaceDeclaration {
			panic!("Interface declaration expected, got {:?}", tokens);
		};
	};

	// 
	// Interface name
	// > Text
	{
		let (index, token) = next_token_with_index(tokens, current_index, Option::None);

		if token.token_type == TokenType::Text {
			// Updating interface name
			name = token.value;

			// Updating current index
			current_index = index;
		} else {
			panic!("Interface name expected, got {:?}", token);
		};
	};

	// 
	// Right Curly Braces
	{
		let (index, token) = next_token_with_index(tokens, current_index, Option::None);

		if token.token_type != TokenType::RightCurlyBraces {
			panic!("Right curly braces expected, got {:?}", token);
		};

		current_index = index;
	};

	println!("start_index: {}, current_index: {}", start_index, current_index);

	// 
	// Parsing interface's body
	for (index, token) in tokens.iter().enumerate() {
		// Starting parsing from interface's body
		if index <= current_index {
			continue;
		};

		// Skipping parsed content
		if parsed_indicies.contains(&index) {
			continue;
		};

		match token.token_type.clone() {
			TokenType::OptionalModifier | TokenType::RequiredModifier => {
				let (variable, range) = parse_variable(tokens, index);

				// Adding this variable to interface's variable list
				variables.push(variable);

				// Adding this range to parsed_indicies
				for parsed_index in create_linear_numbers_array(range.start, range.end) {
					parsed_indicies.push(parsed_index);
				};
			}
			TokenType::EnumerateDeclaration => {
				// Parsing sub-enumerate
				let sub_enumerate = parse_enum(tokens, index);

				// Adding this range to parsed_indicies
				for parsed_index in create_linear_numbers_array(
					sub_enumerate.range.start,
					sub_enumerate.range.end,
				) {
					parsed_indicies.push(parsed_index);
				};

				// Adding this sub_enumerate to our nodes variable
				nodes.push(sub_enumerate);
			}
			TokenType::InterfaceDeclaration => {
				let sub_interface = parse_interface(tokens, index);

				// Adding this range to parsed_indicies
				for parsed_index in create_linear_numbers_array(
					sub_interface.range.start,
					sub_interface.range.end,
				) {
					parsed_indicies.push(parsed_index);
				}

				// Adding this sub_interface to our nodes variable
				nodes.push(sub_interface);
			}

			// 
			// Left Curly Braces
			TokenType::LeftCurlyBraces => {
				// Interface is parsed. Checking if we have a semicolon after
				// this brace
				{
					let (index, token) = next_token_with_index(tokens, index, Option::None);

					if token.token_type != TokenType::Semicolon {
						panic!("Semicolon expected, got {:?}", token);
					};

					// Updating end_index and breaking loop
					end_index = Option::Some(index);
                    break;
				};
			}
			TokenType::Whitespace => { /* Ignoring */ }
			_ => {
				panic!(
					"Variable or interface declaration expected, got: {:?}",
					token
				);
			}
		};
	}

	if end_index == Option::None {
		panic!("No end index");
	};

	Node {
		range: Range {
			start: start_index,
			end: end_index.unwrap(),
		},
		nodes,
		entity: Entity::Interface(Interface {
			name: name.unwrap(),
			variables,
		}),
	}
}
