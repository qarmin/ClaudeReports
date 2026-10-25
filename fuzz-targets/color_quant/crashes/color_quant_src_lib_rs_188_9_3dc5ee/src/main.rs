use color_quant::NeuQuant;

fn main() {
    let neu_quant = NeuQuant::new(1i32, 1usize, &vec![]);
    neu_quant.map_pixel(&mut vec![]);
}
