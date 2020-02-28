use std::fs;

pub struct TextAsset {
    pub data: String
}

pub fn load(text_name: &str) -> TextAsset {
    match std::env::current_exe() {
        Ok(mut absolute_path) => {
            absolute_path.pop();

            absolute_path.push("assets/text/");
            absolute_path.push(text_name);

            match fs::read_to_string(absolute_path) {
                Ok(text) => {                    

                    return TextAsset {
                        data: text
                    };
                },
                Err(e) => panic!("Could not load text {}: {}", text_name, e)
            }
        },
        Err(e) => panic!("current exe path error: {}", e)
    }
}
