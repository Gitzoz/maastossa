use crate::manager::LayerManager;
use std::collections::HashMap;
use crate::layer::{LayerType, Layer, LayerAccess};
use std::fmt;

pub struct Terrain {
    pub width: usize,
    pub height: usize,
    layer_manager: LayerManager,
    pub composition: Vec<Vec<Nature>>
}

#[derive(Debug)]
pub struct Nature {
    x: usize,
    y: usize,
    possibilities: HashMap<LayerType, f64>
}

impl Terrain {
    pub fn new(width: usize, height: usize) -> Self {
        let layer_manager = LayerManager::new();
        let composition = Terrain::init_nature(width, height, &layer_manager);
        Terrain {
            width,
            height,
            layer_manager,
            composition
        }
    }

    fn init_nature(width: usize, height: usize, layer_manager: &LayerManager) -> Vec<Vec<Nature>> {
        fn map_to_possibility(x: usize, y: usize, layer_type: &LayerType, layer: &Layer) -> (LayerType, f64) {
            (layer_type.to_owned(), layer.value_at(x as f64, y as f64))
        }

        let mut field = Vec::with_capacity(width);
        for x in 0..width {
            let mut y_fields = Vec::with_capacity(height);
            for y in 0..height {
                let possibilities: HashMap<LayerType, f64> = layer_manager.generators
                    .iter()
                    .map(|(layer_type, layer)| map_to_possibility(x ,y,layer_type,layer))
                    .collect();

                y_fields.push( Nature {
                    x, y, possibilities
                })
            }
            field.push(y_fields);
        }

        field
    }
}