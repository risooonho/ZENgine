extern crate zengine;

fn main() {
    println!("Hello, world!");

    /*let contents = fs::read_to_string("resources/settings.json")
        .expect("Something went wrong reading settings file");

    println!("With text:\n{}", contents);

    let p: Test = serde_json::from_str(contents.as_str()).unwrap();*/

    let option = zengine::engine::EngineOption {
        title: std::string::String::from("ZENgine - PONG"),
        fullscreen: false,
        virtual_width: 800,
        virtual_height: 600,
        screen_width: 800,
        screen_height: 600
    };

    zengine::engine::start(option);
}
