use crate::Diff;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use urlencoding::encode;

#[derive(Clone, PartialEq)]
pub struct Patch {
	pub diffs: Vec<Diff>,
	pub start1: i32,
	pub start2: i32,
	pub length1: i32,
	pub length2: i32,
}

impl Patch {
	/// A new diff patch object created.
	pub fn new(diffs: Vec<Diff>, start1: i32, start2: i32, length1: i32, length2: i32) -> Patch {
		Patch {
			diffs,
			start1,
			start2,
			length1,
			length2,
		}
	}
}

impl Debug for Patch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{{diffs:\n {:?},\n start1: {},\n start2: {},\n length1: {},\n length2: {} }}",
			self.diffs, self.start1, self.start2, self.length1, self.length2
		)
	}
}

impl Display for Patch {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(&self.build_string())
	}
}

impl Patch {
	fn build_string(&self) -> String {
		// Convert patch to string.
		let mut text = "@@ -".to_string();
		let mut start1: u32 = (self.start1 + 1) as u32;
		if self.length1 == 0 && start1 == 1 {
			start1 -= 1;
		}
		text += start1.to_string().as_str();
		if self.length1 > 0 || start1 == 0 {
			text += ",";
			let length1: u32 = self.length1 as u32;
			text += length1.to_string().as_str();
		}
		text += " +";
		let mut start2: u32 = (self.start2 + 1) as u32;
		if self.length2 == 0 && start2 == 1 {
			start2 -= 1;
		}
		text += start2.to_string().as_str();
		if self.length2 > 0 || start2 == 0 {
			text += ",";
			let length2: u32 = self.length2 as u32;
			text += length2.to_string().as_str();
		}
		text += " @@\n";
		for i in 0..self.diffs.len() {
			let ch: char;
			if self.diffs[i].operation == 0 {
				ch = ' ';
			} else if self.diffs[i].operation == -1 {
				ch = '-';
			} else {
				ch = '+';
			}
			text.push(ch);
			let text_vec: Vec<char> = self.diffs[i].text.chars().collect();
			let temp5: Vec<char> = vec![
				'!', '~', '*', '(', ')', ';', '/', '?', ':', '@', '&', '=', '+', '$', ',', '#',
				' ', '\'',
			];
			for text_vec_item in &text_vec {
				let mut is: bool = false;
				for temp5_item in &temp5 {
					if *text_vec_item == *temp5_item {
						is = true;
					}
				}
				if is {
					text.push(*text_vec_item);
					continue;
				} else if *text_vec_item == '%' {
					text += "%25";
					continue;
				}
				let mut temp6: String = "".to_string();
				temp6.push(*text_vec_item);
				temp6 = encode(temp6.as_str()).into_owned();
				text += temp6.as_str();
			}
			text += "\n";
		}
		text
	}
}
