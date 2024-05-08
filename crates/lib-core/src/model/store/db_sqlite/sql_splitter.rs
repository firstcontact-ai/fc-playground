use std::io::{BufRead, BufReader, Lines};

pub fn split_sql(whole_sql: &str) -> Sqls<BufReader<&[u8]>> {
	// new line reader
	let buf = BufReader::new(whole_sql.as_bytes());
	let lines = buf.lines();
	Sqls {
		lines,
		in_end_block_sql: false,
	}
}

pub struct Sqls<B: BufRead> {
	lines: Lines<B>,
	/// If the current line is in a SQL block (starts with BEGIN and end with END;)
	in_end_block_sql: bool,
}

impl<B: BufRead> Iterator for Sqls<B> {
	type Item = String;

	fn next(&mut self) -> Option<String> {
		let mut sql_block: Vec<String> = Vec::new();

		// TODO: Probably needs to implement when Error occurs.
		//       While we never return Some(Err..) as this point, we might later,
		//       hence, the iterator siganture.
		for line in self.lines.by_ref() {
			// return None if there is an error (no need to cascade this for now)
			let Ok(line) = line else {
				return None;
			};

			let trimmed_line = line.trim();

			// We skip the comment lines (if only comment line in a block, will throw error)
			if trimmed_line.starts_with("--") {
				continue;
			}

			// TOOD: Needs to handle the case if 'begin' is withing a column name
			if trimmed_line.contains("BEGIN") || trimmed_line.contains("begin") {
				self.in_end_block_sql = true;
			}

			let end_block = if self.in_end_block_sql {
				let end = trimmed_line.starts_with("END;") || trimmed_line.starts_with("end;");
				if end {
					self.in_end_block_sql = false;
				}
				end
			} else {
				trimmed_line.ends_with(';')
			};

			sql_block.push(line);

			// break if we are in end block
			if end_block {
				let sql_block = sql_block.join("\n");
				return Some(sql_block);
			}
		}

		None
	}
}
