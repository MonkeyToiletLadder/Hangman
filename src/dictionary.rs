pub mod prelude {
	use std::io::prelude::*;
	use std::fs::File;
	use rand::prelude::*;

	pub struct Dictionary {
		pub data: Vec<String>,
	}
	impl Dictionary {
		pub fn load(&mut self, filename: &str) -> std::io::Result<()> {
			if self.data.len() > 0 {
				self.data.clear()
			}
			let mut file = File::open(filename)?;
			let mut string = String::new();
			file.read_to_string(&mut string)?;
			let split = string.split("\n");
			let vector: Vec<&str> = split.collect();
			//Only add if the first line is not empty string
			if vector[0] != "" {
				for item in vector {
					self.data.push(item.to_string());
				}
			}
			Ok(())
		}
		pub fn get_random_word(&self) -> String {
			let mut rng = rand::thread_rng();
			let index = rng.gen_range(0,self.data.len());
			self.data.get(index).unwrap().to_string()
		}
		pub fn new() -> Dictionary {
			Dictionary { data: vec![] }
		}
		pub fn is_empty(&self) -> bool {
			self.data.is_empty()
		}
	}
}