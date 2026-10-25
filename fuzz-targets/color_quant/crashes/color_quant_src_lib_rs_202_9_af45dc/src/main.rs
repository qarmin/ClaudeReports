use color_quant::NeuQuant;

fn main() {
    let neu_quant = NeuQuant::new(1i32, 1usize, &vec![]);
    let _ = neu_quant.index_of(&vec![]);
}
