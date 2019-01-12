use model::room::map::Pos;
use room::model::parser::ModelParser;

#[test]
pub fn parse_model_data_basic() {
    let model = String::from("xxxxxxxxxxxx\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxx00000000\nxxxxxxxxxxxx\nxxxxxxxxxxxx")
        .into_tile_map(Pos { x: 0, y: 0, z: 0f64 });

    assert_eq!(model.size_y, 16);
    assert_eq!(model.size_x, 12);
}