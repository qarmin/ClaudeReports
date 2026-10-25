use rawler::imgop::xyz::Illuminant;

fn main() {
    let _ = rawler::imgop::chromatic_adaption::bradford_adaption_matrix(&Illuminant::A, &Illuminant::CloudyWeather);
}
