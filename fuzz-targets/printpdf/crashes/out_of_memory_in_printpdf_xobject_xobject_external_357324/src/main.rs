use printpdf::svg::Svg;
use printpdf::xobject::XObject;

fn main() {
    let _ = XObject::External(Svg::parse("<svg xmlns=\"http://www.w3.org/2000/svg\" h=\"1\"><defs><filter id=\"f\"><feGaussianBlur sion=\"2\"/></filter></defs><g mask=\"urply\" filter=\"url(#f)\"><text > 😀</text><polyline points=\"181,108 31,1e9\" s=\"0)\"/>\">></g></svg>", &mut vec![]).unwrap());
}
