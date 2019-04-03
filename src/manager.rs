use crate::layer::{Layer, LayerType};
use std::collections::HashMap;


pub struct LayerManager {
    generators: HashMap<LayerType, Layer>
}

impl LayerManager {
    pub fn new() -> Self {
        let generators: HashMap<LayerType, Layer> = LayerType::all()
            .into_iter()
            .map(|layer_type| (layer_type, Layer::new()))
            .rev()
            .collect();


        LayerManager{
            generators,
        }
    }
}