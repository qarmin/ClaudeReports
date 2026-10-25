use color_quant::NeuQuant;

fn main() {
    let mut neu_quant = NeuQuant::new(1i32, 65536usize, &vec![]);
    neu_quant.init(&vec![]);
}
