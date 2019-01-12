use model::room::map::Pos;
use room::model::parser::get_tile_height;
use room::model::parser::ModelParser;


#[test]
pub fn parse_model_tile_height() {
    assert_eq!(0, get_tile_height(b'0'));
    assert_eq!(5, get_tile_height(b'5'));
    assert_eq!(9, get_tile_height(b'9'));
    assert_eq!(10, get_tile_height(b'a'));
    assert_eq!(35, get_tile_height(b'z'));
}

#[test]
pub fn parse_model_data_basic() {
    let model = String::from("xxxxxxxxxxxx\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxxxxxxxxxx\nxxxxxxxxxxxx")
        .into_tile_map(Pos { x: 0, y: 0, z: 0f64 });

    assert_eq!(model.size_x, 12);
    assert_eq!(model.size_y, 16);
}