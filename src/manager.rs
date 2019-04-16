use std::collections::HashMap;

use crate::layer::{Layer, LayerType};

pub struct LayerManager {
    pub generators: HashMap<LayerType, Layer>
}

impl LayerManager {
    pub fn new() -> Self {
        let generators: HashMap<LayerType, Layer> = LayerType::all()
            .into_iter()
            .map(|layer_type| (layer_type, Layer::new()))
            .collect();


        LayerManager {
            generators,
        }
    }
}