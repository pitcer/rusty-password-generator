/*
 * MIT License
 *
 * Copyright (c) 2019 Piotr Dobiech
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::{env, process};
use rand::prelude::ThreadRng;
use rand::Rng;

fn main() {
	let arguments = get_arguments();
	check_argument(&arguments);
	let password_length = get_password_length(&arguments);
	let password_characters = get_password_characters(&arguments);
	let password = generate_password(password_length, &password_characters);
	println!("Password: {}", &password);
}

fn get_arguments() -> Vec<String> {
	let args = env::args();
	let mut arguments: Vec<_> = args.collect();
	&arguments.remove(0);
	return arguments;
}

fn check_argument(arguments: &Vec<String>) {
	let length = arguments.len();
	if length < 2 {
		println!("Usage: ./rusty-password-generator <password length> <characters>");
		process::exit(1)
	}
}

fn get_password_length(arguments: &Vec<String>) -> u32 {
	let password_length_string = &arguments[0];
	let parsed = password_length_string.parse::<u32>();
	if parsed.is_err() {
		println!("Password length must be an integer.");
		process::exit(1)
	}
	return parsed.unwrap();
}

fn get_password_characters(arguments: &Vec<String>) -> Vec<char> {
	let password_characters_string = &arguments[1];
	let chars = password_characters_string.chars();
	let characters: Vec<_> = chars.collect();
	return characters;
}

fn generate_password(length: u32, characters: &Vec<char>) -> String {
	let mut random = rand::thread_rng();
	let password_characters_length = characters.len() as u32;
	let mut password = String::new();
	for _index in 0..length {
		let character_index = get_random_number(&mut random, 0, password_characters_length);
		let character = characters[character_index];
		password.push(character);
	}
	return password;
}

fn get_random_number(random: &mut ThreadRng, first: u32, second: u32) -> usize {
	return random.gen_range(first, second) as usize;
}
