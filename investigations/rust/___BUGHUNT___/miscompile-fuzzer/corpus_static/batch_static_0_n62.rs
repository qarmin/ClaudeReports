// Batched corpus file: multiple independent sub-cases, each isolated in its
// own `mod case_N`, called in sequence from main() so a single rustc
// invocation amortizes process-startup overhead across all of them.
// See README.md "Batching for throughput". Do not hand-edit - regenerate
// via generator.py/static_generator.py --cases-per-file instead.

mod case_0 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(4063u64).rotate_left(29) ^ 5143u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(9290u64).rotate_left(8) ^ 844u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(9205u64).rotate_left(30) ^ 9980u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(6141u64).rotate_left(16) ^ 4882u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(44257u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(169798u64);
    acc = acc ^ 576542u64;
    acc = acc ^ 643563u64;
    acc = acc ^ 334876u64;
    acc = acc.wrapping_mul(6317u64);
    acc = acc.wrapping_mul(22795u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(681057u64);
    acc = acc.wrapping_mul(56605u64);
    acc = acc.wrapping_add(538389u64);
    acc = acc.wrapping_mul(27593u64);
    acc = acc ^ 38840u64;
    acc = acc.wrapping_mul(161u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(529355u64);
    acc = acc.wrapping_add(317894u64);
    acc = acc.wrapping_add(89521u64);
    acc = acc ^ 523711u64;
    acc = acc ^ 651326u64;
    acc = acc ^ 727176u64;
    acc = acc.wrapping_add(409125u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(23521u64);
    acc = acc ^ 661416u64;
    acc = acc.wrapping_mul(16095u64);
    acc = acc.wrapping_add(137908u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(918138u64);
    acc = acc ^ 295821u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 9870u64;
    acc = acc.wrapping_add(495489u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(11063u64);
    acc = acc.wrapping_add(386556u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 279398u64;
    acc = acc ^ 103951u64;
    acc = acc.wrapping_add(460515u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(200046u64);
    acc = acc.wrapping_mul(57837u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(737712u64);
    acc = acc.wrapping_add(25542u64);
    acc = acc ^ 865488u64;
    acc = acc ^ 192649u64;
    acc = acc.wrapping_add(480506u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(44255u64);
    acc = acc.wrapping_mul(8771u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(20993u64);
    acc = acc.wrapping_add(720143u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(118241u64);
    acc = acc.wrapping_add(1379u64);
    acc = acc.wrapping_mul(21249u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(6739u64);
    acc = acc.wrapping_mul(40805u64);
    acc = acc.wrapping_mul(6169u64);
    acc = acc ^ 459193u64;
    acc = acc.wrapping_add(731122u64);
    acc = acc.wrapping_add(825559u64);
    acc = acc.wrapping_mul(34783u64);
    acc = acc ^ 831024u64;
    acc = acc ^ 981490u64;
    acc = acc ^ 516053u64;
    acc = acc.wrapping_add(326494u64);
    acc = acc.wrapping_mul(29169u64);
    acc = acc ^ 634263u64;
    acc = acc.wrapping_add(724682u64);
    acc = acc.wrapping_add(953595u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 679038u64;
    acc = acc.wrapping_add(528046u64);
    acc = acc ^ 661044u64;
    acc = acc.wrapping_add(591115u64);
    acc = acc.wrapping_mul(58563u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 554732u64;
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc ^ 764743u64;
    acc = acc.wrapping_mul(1465u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(468038u64);
    acc = acc ^ 748957u64;
    acc = acc ^ 91233u64;
    acc = acc ^ 313373u64;
    acc = acc ^ 618811u64;
    acc = acc.wrapping_add(691887u64);
    acc = acc.wrapping_mul(61123u64);
    acc = acc.wrapping_mul(7747u64);
    acc = acc.wrapping_add(620901u64);
    acc = acc ^ 454947u64;
    acc = acc.wrapping_mul(43023u64);
    acc = acc.wrapping_mul(23517u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 271664u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(40501u64);
    acc = acc ^ 568031u64;
    acc = acc.wrapping_mul(25357u64);
    acc = acc.wrapping_add(678534u64);
    acc = acc.wrapping_mul(39625u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(699994u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(16111u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(770062u64);
    acc = acc.wrapping_add(550445u64);
    acc = acc.wrapping_add(991909u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 565020u64;
    acc = acc.wrapping_mul(46077u64);
    acc = acc.wrapping_mul(4623u64);
    acc = acc ^ 275939u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc ^ 865297u64;
    acc = acc.wrapping_add(101163u64);
    acc = acc.wrapping_add(124662u64);
    acc = acc.wrapping_add(503192u64);
    acc = acc.wrapping_add(376992u64);
    acc = acc.wrapping_mul(54953u64);
    acc = acc ^ 625038u64;
    acc = acc.wrapping_mul(32917u64);
    acc = acc.wrapping_mul(35215u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(56409u64);
    acc = acc.wrapping_add(15798u64);
    acc = acc.wrapping_mul(13097u64);
    acc = acc ^ 103922u64;
    acc = acc.wrapping_mul(28491u64);
    acc = acc.wrapping_mul(5947u64);
    acc = acc.wrapping_add(560107u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(43557u64);
    acc = acc ^ 127019u64;
    acc = acc.wrapping_mul(29409u64);
    acc = acc.wrapping_add(687256u64);
    acc = acc.wrapping_mul(8643u64);
    acc = acc ^ 827273u64;
    acc = acc.wrapping_add(978570u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 801116u64;
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(784u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 465u64, b: 510i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V3(779u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_0 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_0: SPacked_0 = SPacked_0 { f0: 232, f1: 1037, f2: 1074 };
const SCONST_0_ALT: SPacked_0 = SPacked_0 { f0: 208, f1: 2053, f2: 2106 };

    pub fn run() {
        assert!(!(1532833476u64) == (1532833476u64) ^ u64::MAX);
        debug_assert!((3545591107u32).rotate_left(17).rotate_right(17) == 3545591107u32);
        debug_assert!((1602692232u32) | (3808151070u32) | (1602692232u32) == (1602692232u32) | (3808151070u32));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(746831623u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(4111191799u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(271978859u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_0 } else { SCONST_0_ALT };
    if std::hint::black_box(true) {
        s.f0 = s.f0.wrapping_add(13);
    } else {
        s.f1 = s.f1.wrapping_add(29);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_0:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_0.f0;
    print!("case_0:"); println!("{}", c);

        }
        debug_assert!((3658156647u64).wrapping_mul(1) == 3658156647u64);
        assert!((4209027091u64) ^ (4209027091u64) == 0u64);
        assert!((509348854u64).wrapping_add(509348854u64).wrapping_sub(509348854u64) == 509348854u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1836768212u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(45885338u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2881710653u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_1 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(4780u64).rotate_left(17) ^ 419u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(5305u64).rotate_left(2) ^ 2636u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(803u64).rotate_left(21) ^ 206u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(5396u64).rotate_left(12) ^ 8722u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc ^ 245266u64;
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(33209u64);
    acc = acc.wrapping_add(843325u64);
    acc = acc.wrapping_mul(35161u64);
    acc = acc ^ 376380u64;
    acc = acc.wrapping_mul(33303u64);
    acc = acc.wrapping_add(683434u64);
    acc = acc ^ 770342u64;
    acc = acc.wrapping_add(473361u64);
    acc = acc ^ 909264u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(388018u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(774364u64);
    acc = acc.wrapping_add(863446u64);
    acc = acc.wrapping_mul(1939u64);
    acc = acc.wrapping_add(573448u64);
    acc = acc ^ 85821u64;
    acc = acc.wrapping_add(124194u64);
    acc = acc.wrapping_add(903393u64);
    acc = acc ^ 720084u64;
    acc = acc.wrapping_add(989402u64);
    acc = acc ^ 289000u64;
    acc = acc ^ 396180u64;
    acc = acc ^ 480765u64;
    acc = acc ^ 127110u64;
    acc = acc ^ 643400u64;
    acc = acc ^ 459579u64;
    acc = acc.wrapping_mul(51123u64);
    acc = acc ^ 199176u64;
    acc = acc ^ 761120u64;
    acc = acc.wrapping_add(145649u64);
    acc = acc ^ 607600u64;
    acc = acc.wrapping_add(285321u64);
    acc = acc.wrapping_add(896259u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 32538u64;
    acc = acc.wrapping_mul(53059u64);
    acc = acc ^ 568658u64;
    acc = acc.rotate_left(19);
    acc = acc ^ 49312u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(32397u64);
    acc = acc.wrapping_add(657809u64);
    acc = acc.wrapping_add(220165u64);
    acc = acc.wrapping_mul(39725u64);
    acc = acc.wrapping_mul(38579u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(62593u64);
    acc = acc ^ 319281u64;
    acc = acc.wrapping_add(153980u64);
    acc = acc.wrapping_mul(35143u64);
    acc = acc.wrapping_mul(37815u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(592084u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(941367u64);
    acc = acc ^ 932615u64;
    acc = acc.wrapping_mul(53193u64);
    acc = acc ^ 112471u64;
    acc = acc.wrapping_mul(27175u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(59869u64);
    acc = acc.wrapping_mul(21651u64);
    acc = acc.wrapping_add(584272u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(398538u64);
    acc = acc ^ 711195u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(818272u64);
    acc = acc.wrapping_add(311579u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(52171u64);
    acc = acc.wrapping_add(277840u64);
    acc = acc.wrapping_mul(43161u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 684288u64;
    acc = acc.rotate_left(3);
    acc = acc ^ 49543u64;
    acc = acc.wrapping_mul(53667u64);
    acc = acc.wrapping_add(111664u64);
    acc = acc.wrapping_add(818706u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 471348u64;
    acc = acc.wrapping_mul(61411u64);
    acc = acc ^ 290364u64;
    acc = acc.wrapping_add(865041u64);
    acc = acc.wrapping_mul(21929u64);
    acc = acc ^ 554069u64;
    acc = acc ^ 351042u64;
    acc = acc.wrapping_mul(36727u64);
    acc = acc.wrapping_add(965530u64);
    acc = acc ^ 188342u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(904600u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(22235u64);
    acc = acc ^ 440829u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(125367u64);
    acc = acc.wrapping_mul(18257u64);
    acc = acc.wrapping_add(131335u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 60696u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(617346u64);
    acc = acc.wrapping_add(231390u64);
    acc = acc.wrapping_add(332173u64);
    acc = acc.wrapping_add(665820u64);
    acc = acc.wrapping_add(429880u64);
    acc = acc.wrapping_add(661038u64);
    acc = acc.wrapping_mul(17569u64);
    acc = acc ^ 628126u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(220740u64);
    acc = acc ^ 667919u64;
    acc = acc.wrapping_mul(25463u64);
    acc = acc ^ 238234u64;
    acc = acc.wrapping_mul(5559u64);
    acc = acc.wrapping_mul(30437u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(45671u64);
    acc = acc.wrapping_mul(46987u64);
    acc = acc ^ 763832u64;
    acc = acc.wrapping_add(351777u64);
    acc = acc ^ 210111u64;
    acc = acc.wrapping_add(23380u64);
    acc = acc.wrapping_add(546124u64);
    acc = acc.wrapping_add(328585u64);
    acc = acc.wrapping_add(561043u64);
    acc = acc.wrapping_mul(46919u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(7687u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(873812u64);
    acc = acc.wrapping_mul(57343u64);
    acc = acc.wrapping_mul(14205u64);
    acc = acc.wrapping_add(407090u64);
    acc = acc.wrapping_mul(35879u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(169208u64);
    acc = acc.wrapping_add(874653u64);
    acc = acc.wrapping_add(749845u64);
    acc = acc.wrapping_add(104636u64);
    acc = acc.wrapping_add(84576u64);
    acc = acc.wrapping_add(719563u64);
    acc = acc.wrapping_mul(12879u64);
    acc = acc.wrapping_mul(3805u64);
    acc = acc.wrapping_add(894520u64);
    acc = acc ^ 874438u64;
    acc = acc.wrapping_add(61328u64);
    acc = acc.wrapping_add(384360u64);
    acc = acc.wrapping_add(831067u64);
    acc = acc ^ 509401u64;
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc ^ 300464u64;
    acc = acc ^ 888671u64;
    acc = acc.wrapping_add(114418u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(12255u64);
    acc = acc ^ 446562u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(457846u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 412984u64;
    acc = acc ^ 81074u64;
    acc = acc.rotate_left(17);
    acc = acc ^ 95561u64;
    acc = acc.wrapping_mul(30683u64);
    acc = acc.wrapping_add(813496u64);
    acc = acc.wrapping_mul(63147u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(55225u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(399741u64);
    acc = acc.wrapping_add(255348u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(948u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(789u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 927u64, b: -710i32, c: true } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_1 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_1: SPacked_1 = SPacked_1 { f0: 239, f1: 1044, f2: 1081 };
const SCONST_1_ALT: SPacked_1 = SPacked_1 { f0: 219, f1: 2064, f2: 2117 };

    pub fn run() {
        assert!((2483833906u64).wrapping_add(2483833906u64).wrapping_sub(2483833906u64) == 2483833906u64);
        assert!((4125643114u64).wrapping_add(4125643114u64).wrapping_sub(4125643114u64) == 4125643114u64);
        assert!((3531745472u64).wrapping_add(3531745472u64).wrapping_sub(3531745472u64) == 3531745472u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1297348318u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1385948401u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2405696823u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_1 } else { SCONST_1_ALT };
    if std::hint::black_box(true) {
        s.f1 = s.f1.wrapping_add(14);
    } else {
        s.f2 = s.f2.wrapping_add(30);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_1:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_1.f1;
    print!("case_1:"); println!("{}", c);

        }
        assert!(!(814272878u64) == (814272878u64) ^ u64::MAX);
        debug_assert!((1327502805u32).rotate_left(13).rotate_right(13) == 1327502805u32);
        assert!(!(2175245144u64) == (2175245144u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1963441511u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(528720808u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1519894580u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_2 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(4359u64).rotate_left(13) ^ 9188u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(2724u64).rotate_left(22) ^ 610u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(9624u64).rotate_left(23) ^ 22u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(2262u64).rotate_left(22) ^ 8608u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(726622u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(15299u64);
    acc = acc.wrapping_mul(13809u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(893925u64);
    acc = acc.wrapping_mul(26049u64);
    acc = acc.wrapping_mul(43865u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 59393u64;
    acc = acc.wrapping_add(560270u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(20221u64);
    acc = acc.wrapping_add(201416u64);
    acc = acc ^ 451998u64;
    acc = acc ^ 657402u64;
    acc = acc ^ 412597u64;
    acc = acc.wrapping_mul(57467u64);
    acc = acc.wrapping_mul(19115u64);
    acc = acc ^ 718732u64;
    acc = acc.wrapping_add(557737u64);
    acc = acc ^ 786612u64;
    acc = acc.wrapping_add(694346u64);
    acc = acc.wrapping_add(339449u64);
    acc = acc.wrapping_mul(30315u64);
    acc = acc.wrapping_mul(8419u64);
    acc = acc ^ 71785u64;
    acc = acc ^ 832850u64;
    acc = acc.wrapping_mul(4239u64);
    acc = acc.wrapping_add(74514u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 970427u64;
    acc = acc.wrapping_mul(13619u64);
    acc = acc.wrapping_add(360509u64);
    acc = acc ^ 975438u64;
    acc = acc ^ 36454u64;
    acc = acc.wrapping_add(670176u64);
    acc = acc ^ 576894u64;
    acc = acc ^ 391326u64;
    acc = acc ^ 318481u64;
    acc = acc.wrapping_mul(38077u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 914825u64;
    acc = acc ^ 327924u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 791187u64;
    acc = acc.wrapping_add(442864u64);
    acc = acc.wrapping_add(80055u64);
    acc = acc ^ 728513u64;
    acc = acc.wrapping_mul(53487u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(63881u64);
    acc = acc ^ 832765u64;
    acc = acc ^ 670524u64;
    acc = acc.wrapping_add(648802u64);
    acc = acc.wrapping_add(980638u64);
    acc = acc.wrapping_add(349501u64);
    acc = acc.wrapping_add(182595u64);
    acc = acc ^ 61948u64;
    acc = acc ^ 532451u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 970122u64;
    acc = acc ^ 749053u64;
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 291953u64;
    acc = acc ^ 81557u64;
    acc = acc.wrapping_add(59528u64);
    acc = acc.wrapping_add(60593u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(1573u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 804597u64;
    acc = acc.wrapping_mul(60221u64);
    acc = acc.wrapping_mul(13181u64);
    acc = acc.wrapping_mul(10819u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 381618u64;
    acc = acc ^ 148833u64;
    acc = acc ^ 778304u64;
    acc = acc.wrapping_add(272128u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc ^ 384216u64;
    acc = acc.wrapping_mul(37341u64);
    acc = acc.wrapping_mul(20551u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(662707u64);
    acc = acc.wrapping_mul(52213u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(962162u64);
    acc = acc ^ 893517u64;
    acc = acc.wrapping_mul(41259u64);
    acc = acc.wrapping_mul(48423u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(67616u64);
    acc = acc.wrapping_add(701496u64);
    acc = acc.wrapping_mul(38071u64);
    acc = acc.wrapping_mul(60053u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(41457u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 460707u64;
    acc = acc.wrapping_add(324548u64);
    acc = acc.wrapping_add(141826u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 558491u64;
    acc = acc.wrapping_add(180890u64);
    acc = acc.wrapping_add(125437u64);
    acc = acc.wrapping_add(924514u64);
    acc = acc ^ 514792u64;
    acc = acc.wrapping_add(44918u64);
    acc = acc ^ 322644u64;
    acc = acc.wrapping_add(503030u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(656426u64);
    acc = acc ^ 736126u64;
    acc = acc.wrapping_add(222678u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(32409u64);
    acc = acc.wrapping_mul(9321u64);
    acc = acc.wrapping_mul(3977u64);
    acc = acc ^ 787545u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(12813u64);
    acc = acc ^ 255947u64;
    acc = acc.wrapping_add(354814u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(60649u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(17591u64);
    acc = acc ^ 795185u64;
    acc = acc.wrapping_mul(32263u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 460889u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(949412u64);
    acc = acc.wrapping_mul(20423u64);
    acc = acc.wrapping_mul(31725u64);
    acc = acc.wrapping_mul(29013u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(47723u64);
    acc = acc ^ 693600u64;
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 254145u64;
    acc = acc.wrapping_mul(22833u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(880877u64);
    acc = acc.wrapping_add(685673u64);
    acc = acc.wrapping_mul(35797u64);
    acc = acc.wrapping_mul(33595u64);
    acc = acc.wrapping_add(324756u64);
    acc = acc ^ 695953u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(61801u64);
    acc = acc ^ 615275u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(244216u64);
    acc = acc.wrapping_add(475568u64);
    acc = acc ^ 154278u64;
    acc = acc ^ 552428u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(877883u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(41483u64);
    acc = acc.wrapping_add(380964u64);
    acc = acc.wrapping_mul(49077u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 589912u64;
    acc = acc.wrapping_mul(42867u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(11161u64);
    acc = acc.wrapping_add(268005u64);
    acc = acc.wrapping_mul(23615u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(26883u64);
    acc = acc.wrapping_mul(61933u64);
    acc = acc.wrapping_mul(23785u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(24401u64);
    acc = acc ^ 993677u64;
    acc = acc.wrapping_mul(32063u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 675030u64;
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(4285u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(957597u64);
    acc = acc.wrapping_mul(14467u64);
    acc = acc.wrapping_mul(9465u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 331676u64;
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(460996u64);
    acc = acc ^ 914602u64;
    acc = acc ^ 7030u64;
    acc = acc ^ 507045u64;
    acc = acc.wrapping_mul(27423u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(21u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 224u64, b: 179i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 823u64, b: 871i32, c: true } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_2 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_2: SPacked_2 = SPacked_2 { f0: 246, f1: 1051, f2: 1088 };
const SCONST_2_ALT: SPacked_2 = SPacked_2 { f0: 230, f1: 2075, f2: 2128 };

    pub fn run() {
        debug_assert!((2756389290u32).rotate_left(19).rotate_right(19) == 2756389290u32);
        debug_assert!((81507851u32) | (2372855292u32) | (81507851u32) == (81507851u32) | (2372855292u32));
        assert!(!(2268686338u64) == (2268686338u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2731905645u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(81465786u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1769682724u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_2 } else { SCONST_2_ALT };
    if std::hint::black_box(false) {
        s.f0 = s.f0.wrapping_add(15);
    } else {
        s.f1 = s.f1.wrapping_add(31);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_2:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_2.f2;
    print!("case_2:"); println!("{}", c);

        }
        debug_assert!((993255964u32).rotate_left(31).rotate_right(31) == 993255964u32);
        assert!((4169879073u64).wrapping_add(4169879073u64).wrapping_sub(4169879073u64) == 4169879073u64);
        assert!((672846345u64) ^ (672846345u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(186830288u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3191535010u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3142334832u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_3 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(5048u64).rotate_left(27) ^ 460u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(6784u64).rotate_left(22) ^ 2191u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(2950u64).rotate_left(16) ^ 5967u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(6970u64).rotate_left(1) ^ 6262u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(4349u64);
    acc = acc.wrapping_add(587051u64);
    acc = acc.wrapping_add(792351u64);
    acc = acc.wrapping_mul(1115u64);
    acc = acc.wrapping_add(264016u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 227272u64;
    acc = acc.wrapping_add(111019u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 24792u64;
    acc = acc.rotate_left(9);
    acc = acc ^ 166621u64;
    acc = acc ^ 528484u64;
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 780992u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(181032u64);
    acc = acc.wrapping_add(342316u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 27073u64;
    acc = acc ^ 320578u64;
    acc = acc.wrapping_mul(28187u64);
    acc = acc ^ 706328u64;
    acc = acc.wrapping_mul(55851u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(493522u64);
    acc = acc ^ 337614u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(35223u64);
    acc = acc.wrapping_add(446454u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 896205u64;
    acc = acc.wrapping_add(26078u64);
    acc = acc.wrapping_add(424646u64);
    acc = acc.wrapping_add(100601u64);
    acc = acc.wrapping_add(872439u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(467576u64);
    acc = acc.wrapping_add(180470u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(36493u64);
    acc = acc.wrapping_mul(53039u64);
    acc = acc ^ 12520u64;
    acc = acc.wrapping_add(808825u64);
    acc = acc.wrapping_add(119032u64);
    acc = acc.wrapping_mul(37239u64);
    acc = acc.wrapping_add(714870u64);
    acc = acc ^ 201746u64;
    acc = acc.wrapping_add(681229u64);
    acc = acc.wrapping_add(16063u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(786945u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(853929u64);
    acc = acc.wrapping_add(654403u64);
    acc = acc ^ 808303u64;
    acc = acc.wrapping_add(377805u64);
    acc = acc ^ 104697u64;
    acc = acc.wrapping_mul(11195u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(38465u64);
    acc = acc.wrapping_mul(64453u64);
    acc = acc ^ 683348u64;
    acc = acc ^ 948718u64;
    acc = acc.wrapping_mul(21575u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(723461u64);
    acc = acc ^ 956709u64;
    acc = acc.wrapping_add(101308u64);
    acc = acc.wrapping_mul(56201u64);
    acc = acc.wrapping_add(304891u64);
    acc = acc.wrapping_mul(7801u64);
    acc = acc.wrapping_add(150218u64);
    acc = acc.wrapping_mul(6465u64);
    acc = acc.wrapping_mul(63675u64);
    acc = acc.wrapping_mul(11723u64);
    acc = acc.wrapping_add(283260u64);
    acc = acc.wrapping_mul(6535u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(22619u64);
    acc = acc.wrapping_add(825009u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(13237u64);
    acc = acc ^ 319272u64;
    acc = acc.wrapping_add(566994u64);
    acc = acc.wrapping_add(721770u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(6129u64);
    acc = acc ^ 804361u64;
    acc = acc.wrapping_mul(62949u64);
    acc = acc.wrapping_add(73987u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(3717u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(24305u64);
    acc = acc.wrapping_mul(36369u64);
    acc = acc ^ 551852u64;
    acc = acc.wrapping_mul(14105u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(39491u64);
    acc = acc.wrapping_add(572697u64);
    acc = acc ^ 393124u64;
    acc = acc.wrapping_mul(62117u64);
    acc = acc ^ 541491u64;
    acc = acc.wrapping_add(153952u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 257812u64;
    acc = acc ^ 708825u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(13771u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(822462u64);
    acc = acc ^ 161291u64;
    acc = acc.wrapping_mul(16735u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(23587u64);
    acc = acc.wrapping_mul(357u64);
    acc = acc.wrapping_add(362628u64);
    acc = acc ^ 332361u64;
    acc = acc ^ 156828u64;
    acc = acc.wrapping_add(411882u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(6627u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(34547u64);
    acc = acc ^ 914400u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(321389u64);
    acc = acc ^ 579698u64;
    acc = acc.wrapping_mul(9891u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 870068u64;
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(10591u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(259792u64);
    acc = acc ^ 31399u64;
    acc = acc.wrapping_mul(21467u64);
    acc = acc.wrapping_mul(32147u64);
    acc = acc ^ 374643u64;
    acc = acc.wrapping_mul(33703u64);
    acc = acc ^ 111675u64;
    acc = acc.wrapping_mul(9763u64);
    acc = acc.wrapping_add(797697u64);
    acc = acc.wrapping_add(434980u64);
    acc = acc ^ 479313u64;
    acc = acc.wrapping_add(28211u64);
    acc = acc.wrapping_add(753256u64);
    acc = acc.wrapping_mul(49981u64);
    acc = acc.wrapping_add(528467u64);
    acc = acc.wrapping_mul(45929u64);
    acc = acc.wrapping_add(140795u64);
    acc = acc.wrapping_add(42175u64);
    acc = acc.wrapping_mul(2587u64);
    acc = acc.wrapping_mul(47859u64);
    acc = acc.wrapping_mul(38585u64);
    acc = acc ^ 885551u64;
    acc = acc.wrapping_add(627823u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 51878u64;
    acc = acc.wrapping_add(539702u64);
    acc = acc.wrapping_mul(25059u64);
    acc = acc.wrapping_mul(63973u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(337178u64);
    acc = acc ^ 951830u64;
    acc = acc ^ 857186u64;
    acc = acc ^ 196009u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(632651u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(28835u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 649327u64;
    acc = acc.wrapping_mul(12207u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(4793u64);
    acc = acc ^ 861393u64;
    acc = acc.wrapping_add(935092u64);
    acc = acc ^ 392737u64;
    acc = acc.wrapping_add(258501u64);
    acc = acc.wrapping_add(100992u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(619253u64);
    acc = acc.wrapping_add(616827u64);
    acc = acc.wrapping_mul(13913u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 752801u64;
    acc = acc ^ 323106u64;
    acc = acc.wrapping_mul(52375u64);
    acc = acc ^ 872580u64;
    acc = acc.wrapping_mul(48295u64);
    acc = acc.wrapping_add(586650u64);
    acc = acc.wrapping_mul(22317u64);
    acc = acc.wrapping_add(278671u64);
    acc = acc.wrapping_mul(37169u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc ^ 457959u64;
    acc = acc ^ 115957u64;
    acc = acc.wrapping_add(634329u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(170871u64);
    acc = acc.wrapping_add(172893u64);
    acc = acc ^ 614460u64;
    acc = acc.wrapping_mul(9767u64);
    acc = acc.wrapping_mul(17895u64);
    acc = acc.wrapping_mul(44229u64);
    acc = acc.wrapping_mul(41591u64);
    acc = acc.wrapping_add(608781u64);
    acc = acc.wrapping_add(61758u64);
    acc = acc ^ 985222u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(126523u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 800u64, b: -961i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 668u64, b: -357i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(0u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_3 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_3: SPacked_3 = SPacked_3 { f0: 253, f1: 1058, f2: 1095 };
const SCONST_3_ALT: SPacked_3 = SPacked_3 { f0: 241, f1: 2086, f2: 2139 };

    pub fn run() {
        assert!((2520398422u64) ^ (2520398422u64) == 0u64);
        assert!(!(1703140113u64) == (1703140113u64) ^ u64::MAX);
        debug_assert!((3574454121u32).rotate_left(7).rotate_right(7) == 3574454121u32);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3933864885u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(386011457u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3563949516u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_3 } else { SCONST_3_ALT };
    if std::hint::black_box(false) {
        s.f1 = s.f1.wrapping_add(16);
    } else {
        s.f2 = s.f2.wrapping_add(32);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_3:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_3.f0;
    print!("case_3:"); println!("{}", c);

        }
        debug_assert!((455593729u32).rotate_left(10).rotate_right(10) == 455593729u32);
        debug_assert!((3752726691u32).rotate_left(1).rotate_right(1) == 3752726691u32);
        debug_assert!((121263659u32).rotate_left(31).rotate_right(31) == 121263659u32);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1033757127u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(112220501u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2599571325u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_4 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(4559u64).rotate_left(21) ^ 7615u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(4918u64).rotate_left(19) ^ 948u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(8370u64).rotate_left(27) ^ 6846u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(4757u64).rotate_left(11) ^ 8535u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(54113u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 642481u64;
    acc = acc.wrapping_add(974834u64);
    acc = acc.wrapping_mul(65393u64);
    acc = acc.wrapping_add(550976u64);
    acc = acc.wrapping_add(967980u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 362948u64;
    acc = acc.rotate_left(19);
    acc = acc ^ 442608u64;
    acc = acc ^ 424919u64;
    acc = acc.wrapping_add(615881u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(52691u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(559085u64);
    acc = acc.wrapping_add(965716u64);
    acc = acc.wrapping_mul(58877u64);
    acc = acc ^ 446179u64;
    acc = acc ^ 822357u64;
    acc = acc ^ 661488u64;
    acc = acc.wrapping_mul(6787u64);
    acc = acc ^ 213570u64;
    acc = acc ^ 474577u64;
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(4009u64);
    acc = acc.wrapping_add(900257u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 669347u64;
    acc = acc.wrapping_add(922588u64);
    acc = acc ^ 236726u64;
    acc = acc.wrapping_mul(9627u64);
    acc = acc.wrapping_add(505966u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(352638u64);
    acc = acc.wrapping_add(772777u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(61993u64);
    acc = acc ^ 188789u64;
    acc = acc ^ 506947u64;
    acc = acc ^ 96026u64;
    acc = acc.wrapping_add(157464u64);
    acc = acc ^ 417055u64;
    acc = acc.wrapping_add(687580u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(325326u64);
    acc = acc ^ 848987u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(13171u64);
    acc = acc ^ 153418u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(326626u64);
    acc = acc ^ 169284u64;
    acc = acc.rotate_left(28);
    acc = acc ^ 341522u64;
    acc = acc.rotate_left(11);
    acc = acc ^ 716844u64;
    acc = acc.wrapping_add(812418u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(50501u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(664840u64);
    acc = acc.wrapping_add(140561u64);
    acc = acc ^ 456205u64;
    acc = acc.wrapping_add(36437u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(25365u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(500870u64);
    acc = acc ^ 599940u64;
    acc = acc.wrapping_add(615451u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(872902u64);
    acc = acc ^ 901467u64;
    acc = acc.wrapping_add(490543u64);
    acc = acc.wrapping_add(448314u64);
    acc = acc.wrapping_add(25666u64);
    acc = acc.wrapping_mul(53159u64);
    acc = acc ^ 535895u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 219771u64;
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 178670u64;
    acc = acc ^ 224215u64;
    acc = acc.wrapping_mul(59513u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(848140u64);
    acc = acc.wrapping_add(416082u64);
    acc = acc ^ 306536u64;
    acc = acc.wrapping_mul(64549u64);
    acc = acc ^ 946040u64;
    acc = acc.wrapping_add(121390u64);
    acc = acc ^ 630636u64;
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(29603u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 992833u64;
    acc = acc ^ 158761u64;
    acc = acc.wrapping_mul(41501u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(40803u64);
    acc = acc ^ 268388u64;
    acc = acc.wrapping_mul(13759u64);
    acc = acc.wrapping_add(197226u64);
    acc = acc.wrapping_add(529347u64);
    acc = acc ^ 411123u64;
    acc = acc ^ 313899u64;
    acc = acc.wrapping_add(549664u64);
    acc = acc.wrapping_add(486495u64);
    acc = acc ^ 229302u64;
    acc = acc.wrapping_mul(54289u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(162790u64);
    acc = acc.wrapping_add(409438u64);
    acc = acc ^ 665623u64;
    acc = acc ^ 685699u64;
    acc = acc.wrapping_add(528114u64);
    acc = acc.wrapping_mul(46077u64);
    acc = acc.wrapping_add(16400u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(989097u64);
    acc = acc ^ 613858u64;
    acc = acc.wrapping_add(266468u64);
    acc = acc ^ 682519u64;
    acc = acc.wrapping_add(528125u64);
    acc = acc ^ 27919u64;
    acc = acc.wrapping_mul(35533u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(51725u64);
    acc = acc.wrapping_add(379056u64);
    acc = acc.wrapping_mul(53281u64);
    acc = acc.wrapping_mul(1149u64);
    acc = acc.rotate_left(26);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 833u64, b: -235i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 547u64, b: 918i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(55u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_4 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_4: SPacked_4 = SPacked_4 { f0: 4, f1: 1065, f2: 1102 };
const SCONST_4_ALT: SPacked_4 = SPacked_4 { f0: 252, f1: 2097, f2: 2150 };

    pub fn run() {
        assert!(!(1412397597u64) == (1412397597u64) ^ u64::MAX);
        debug_assert!((2017657414u64).wrapping_mul(1) == 2017657414u64);
        assert!(!(2068040825u64) == (2068040825u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(97180263u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3220110990u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(97661473u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_4 } else { SCONST_4_ALT };
    if std::hint::black_box(true) {
        s.f0 = s.f0.wrapping_add(17);
    } else {
        s.f1 = s.f1.wrapping_add(33);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_4:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_4.f1;
    print!("case_4:"); println!("{}", c);

        }
        debug_assert!((1054413777u64).wrapping_mul(1) == 1054413777u64);
        debug_assert!((2294143627u32) | (2255369580u32) | (2294143627u32) == (2294143627u32) | (2255369580u32));
        assert!((2467559943u64).wrapping_add(2467559943u64).wrapping_sub(2467559943u64) == 2467559943u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1328237383u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1236139696u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2825032779u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_5 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(9024u64).rotate_left(10) ^ 3419u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(4667u64).rotate_left(14) ^ 6988u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(5659u64).rotate_left(2) ^ 6759u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(5673u64).rotate_left(6) ^ 8003u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(2855u64);
    acc = acc.wrapping_mul(9195u64);
    acc = acc.wrapping_add(393752u64);
    acc = acc.wrapping_mul(38825u64);
    acc = acc.wrapping_add(128277u64);
    acc = acc ^ 823544u64;
    acc = acc.wrapping_add(30559u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(177175u64);
    acc = acc ^ 800587u64;
    acc = acc.wrapping_mul(9395u64);
    acc = acc.wrapping_mul(44449u64);
    acc = acc.wrapping_mul(13939u64);
    acc = acc.wrapping_add(420649u64);
    acc = acc.wrapping_mul(23019u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(47354u64);
    acc = acc ^ 721844u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 67327u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(773386u64);
    acc = acc.wrapping_add(8169u64);
    acc = acc.wrapping_mul(64781u64);
    acc = acc.wrapping_add(667215u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(15643u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(13845u64);
    acc = acc.wrapping_add(971652u64);
    acc = acc.wrapping_add(676908u64);
    acc = acc.wrapping_add(897580u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(875305u64);
    acc = acc ^ 308240u64;
    acc = acc ^ 606584u64;
    acc = acc.wrapping_add(163617u64);
    acc = acc.wrapping_add(289752u64);
    acc = acc.wrapping_mul(31335u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(30571u64);
    acc = acc.wrapping_add(463463u64);
    acc = acc.wrapping_add(889241u64);
    acc = acc ^ 206965u64;
    acc = acc.wrapping_mul(43079u64);
    acc = acc ^ 896738u64;
    acc = acc ^ 775735u64;
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(42423u64);
    acc = acc ^ 369643u64;
    acc = acc.rotate_left(20);
    acc = acc ^ 904313u64;
    acc = acc.wrapping_mul(54049u64);
    acc = acc.wrapping_mul(9763u64);
    acc = acc ^ 150746u64;
    acc = acc.wrapping_add(299928u64);
    acc = acc.wrapping_add(965631u64);
    acc = acc.wrapping_mul(64957u64);
    acc = acc ^ 272121u64;
    acc = acc ^ 943851u64;
    acc = acc ^ 145124u64;
    acc = acc.wrapping_mul(57633u64);
    acc = acc.wrapping_add(878057u64);
    acc = acc.wrapping_mul(62747u64);
    acc = acc ^ 927295u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 268662u64;
    acc = acc.wrapping_mul(39509u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(63443u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 701990u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(50249u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(148106u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 904789u64;
    acc = acc ^ 494141u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 366445u64;
    acc = acc.wrapping_mul(27983u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(935698u64);
    acc = acc ^ 951449u64;
    acc = acc ^ 403274u64;
    acc = acc.wrapping_add(47008u64);
    acc = acc.wrapping_mul(43431u64);
    acc = acc.wrapping_add(682919u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 795741u64;
    acc = acc ^ 759196u64;
    acc = acc ^ 257426u64;
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(64001u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(772807u64);
    acc = acc.wrapping_mul(63761u64);
    acc = acc.rotate_left(22);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(47243u64);
    acc = acc.wrapping_mul(45703u64);
    acc = acc ^ 29144u64;
    acc = acc.wrapping_add(680245u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(968433u64);
    acc = acc.wrapping_mul(62225u64);
    acc = acc ^ 448945u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(655695u64);
    acc = acc ^ 17343u64;
    acc = acc.wrapping_mul(48169u64);
    acc = acc.wrapping_mul(21913u64);
    acc = acc.wrapping_mul(56339u64);
    acc = acc.wrapping_mul(64163u64);
    acc = acc ^ 382969u64;
    acc = acc.rotate_left(23);
    acc = acc ^ 69979u64;
    acc = acc.wrapping_mul(55885u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 536666u64;
    acc = acc.wrapping_mul(55621u64);
    acc = acc.wrapping_mul(4639u64);
    acc = acc.wrapping_add(223978u64);
    acc = acc.wrapping_add(252728u64);
    acc = acc ^ 117356u64;
    acc = acc.wrapping_mul(2023u64);
    acc = acc.wrapping_mul(64257u64);
    acc = acc.wrapping_add(10496u64);
    acc = acc.wrapping_mul(58885u64);
    acc = acc.wrapping_mul(3345u64);
    acc = acc ^ 189883u64;
    acc = acc.wrapping_add(265462u64);
    acc = acc ^ 618022u64;
    acc = acc ^ 254059u64;
    acc = acc.wrapping_add(758862u64);
    acc = acc.wrapping_mul(18509u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc ^ 798838u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 948466u64;
    acc = acc.rotate_left(19);
    acc = acc ^ 759304u64;
    acc = acc.wrapping_mul(18259u64);
    acc = acc.wrapping_add(521540u64);
    acc = acc.wrapping_add(19566u64);
    acc = acc.wrapping_mul(63811u64);
    acc = acc.wrapping_add(527233u64);
    acc = acc.wrapping_add(521098u64);
    acc = acc.wrapping_add(597017u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(665976u64);
    acc = acc.wrapping_add(857312u64);
    acc = acc ^ 781225u64;
    acc = acc.wrapping_mul(46851u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 988547u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(969619u64);
    acc = acc.wrapping_add(890198u64);
    acc = acc ^ 171309u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 864699u64;
    acc = acc.wrapping_add(554279u64);
    acc = acc ^ 544907u64;
    acc = acc.wrapping_add(498570u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(177u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(157u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 227u64, b: 418i32, c: false } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_5 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_5: SPacked_5 = SPacked_5 { f0: 11, f1: 1072, f2: 1109 };
const SCONST_5_ALT: SPacked_5 = SPacked_5 { f0: 7, f1: 2108, f2: 2161 };

    pub fn run() {
        assert!((576966899u64) ^ (576966899u64) == 0u64);
        debug_assert!((2976449212u64).wrapping_mul(1) == 2976449212u64);
        debug_assert!((1096173624u32).rotate_left(25).rotate_right(25) == 1096173624u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1534300226u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(177695051u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2028164474u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_5 } else { SCONST_5_ALT };
    if std::hint::black_box(true) {
        s.f1 = s.f1.wrapping_add(18);
    } else {
        s.f2 = s.f2.wrapping_add(34);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_5:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_5.f2;
    print!("case_5:"); println!("{}", c);

        }
        assert!((38145128u64) ^ (38145128u64) == 0u64);
        debug_assert!((3418539678u64).wrapping_mul(1) == 3418539678u64);
        assert!(!(885334855u64) == (885334855u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(87168203u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(481072397u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2981621239u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_6 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(9128u64).rotate_left(29) ^ 276u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(1670u64).rotate_left(26) ^ 5512u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(6264u64).rotate_left(7) ^ 1773u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(3609u64).rotate_left(24) ^ 3371u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(60665u64);
    acc = acc ^ 900739u64;
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc ^ 112163u64;
    acc = acc.wrapping_mul(48009u64);
    acc = acc.wrapping_mul(31679u64);
    acc = acc ^ 246842u64;
    acc = acc.wrapping_add(466941u64);
    acc = acc.wrapping_mul(12871u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(13271u64);
    acc = acc.wrapping_add(395583u64);
    acc = acc ^ 957634u64;
    acc = acc ^ 68000u64;
    acc = acc.wrapping_add(271998u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 473284u64;
    acc = acc.wrapping_add(11757u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 922130u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(43081u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 897453u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(891894u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(384626u64);
    acc = acc.wrapping_add(553812u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 238107u64;
    acc = acc.wrapping_mul(8955u64);
    acc = acc.wrapping_mul(56735u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(31771u64);
    acc = acc.wrapping_add(940556u64);
    acc = acc.wrapping_mul(14491u64);
    acc = acc.wrapping_mul(44859u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(330943u64);
    acc = acc.wrapping_mul(57943u64);
    acc = acc ^ 442933u64;
    acc = acc ^ 84512u64;
    acc = acc ^ 849809u64;
    acc = acc.wrapping_mul(19371u64);
    acc = acc ^ 490558u64;
    acc = acc.wrapping_add(255479u64);
    acc = acc ^ 635002u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(30779u64);
    acc = acc ^ 788570u64;
    acc = acc.wrapping_add(905397u64);
    acc = acc ^ 622901u64;
    acc = acc ^ 279509u64;
    acc = acc ^ 911369u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(641256u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 298095u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(502251u64);
    acc = acc.wrapping_mul(14985u64);
    acc = acc.wrapping_mul(53389u64);
    acc = acc.wrapping_add(216366u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(40963u64);
    acc = acc ^ 281644u64;
    acc = acc.wrapping_add(802535u64);
    acc = acc.wrapping_add(822581u64);
    acc = acc.wrapping_mul(19785u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc ^ 491818u64;
    acc = acc.wrapping_add(111891u64);
    acc = acc.wrapping_mul(54669u64);
    acc = acc.wrapping_mul(59483u64);
    acc = acc ^ 58758u64;
    acc = acc.wrapping_add(277368u64);
    acc = acc.wrapping_mul(771u64);
    acc = acc.wrapping_mul(56891u64);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc ^ 510183u64;
    acc = acc.wrapping_add(708624u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(65137u64);
    acc = acc.wrapping_add(157819u64);
    acc = acc.wrapping_add(400331u64);
    acc = acc ^ 184012u64;
    acc = acc.wrapping_mul(16305u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(17789u64);
    acc = acc.wrapping_mul(24135u64);
    acc = acc ^ 710147u64;
    acc = acc.wrapping_mul(32385u64);
    acc = acc.wrapping_mul(26179u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(467102u64);
    acc = acc ^ 42990u64;
    acc = acc.wrapping_mul(18865u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(51197u64);
    acc = acc.wrapping_mul(37917u64);
    acc = acc.wrapping_add(87114u64);
    acc = acc ^ 55743u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(58653u64);
    acc = acc.wrapping_add(379028u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 361541u64;
    acc = acc.wrapping_add(893732u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(44299u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(600241u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(504773u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(8761u64);
    acc = acc ^ 253982u64;
    acc = acc.wrapping_add(331931u64);
    acc = acc.wrapping_mul(11439u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 433211u64;
    acc = acc ^ 586053u64;
    acc = acc.wrapping_add(716807u64);
    acc = acc.wrapping_add(313275u64);
    acc = acc.wrapping_add(44925u64);
    acc = acc.wrapping_add(100844u64);
    acc = acc ^ 454929u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 504026u64;
    acc = acc.wrapping_add(610794u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(8993u64);
    acc = acc.wrapping_add(267307u64);
    acc = acc.wrapping_add(53904u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(53599u64);
    acc = acc ^ 489004u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(42847u64);
    acc = acc.wrapping_mul(58839u64);
    acc = acc.wrapping_mul(21421u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(61623u64);
    acc = acc ^ 986324u64;
    acc = acc ^ 445831u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(236345u64);
    acc = acc ^ 800281u64;
    acc = acc ^ 809619u64;
    acc = acc.rotate_left(10);
    acc = acc ^ 479864u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(35083u64);
    acc = acc ^ 171482u64;
    acc = acc.rotate_left(14);
    acc = acc ^ 3063u64;
    acc = acc.wrapping_add(261988u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(32559u64);
    acc = acc.wrapping_add(981878u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(34941u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(465388u64);
    acc = acc.wrapping_add(447819u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 957967u64;
    acc = acc.wrapping_mul(63665u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(47977u64);
    acc = acc.wrapping_add(128488u64);
    acc = acc.wrapping_add(707842u64);
    acc = acc ^ 542732u64;
    acc = acc.wrapping_mul(19263u64);
    acc = acc.wrapping_add(267293u64);
    acc = acc.wrapping_add(721522u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(118931u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(9569u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 114132u64;
    acc = acc ^ 798052u64;
    acc = acc.wrapping_mul(3495u64);
    acc = acc.wrapping_mul(65053u64);
    acc = acc ^ 700416u64;
    acc = acc.wrapping_mul(34529u64);
    acc = acc ^ 366554u64;
    acc = acc.wrapping_add(220669u64);
    acc = acc ^ 229622u64;
    acc = acc ^ 430915u64;
    acc = acc.wrapping_mul(49837u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 460587u64;
    acc = acc.wrapping_add(953868u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(487u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(136u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V2(472u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_6 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_6: SPacked_6 = SPacked_6 { f0: 18, f1: 1079, f2: 1116 };
const SCONST_6_ALT: SPacked_6 = SPacked_6 { f0: 18, f1: 2119, f2: 2172 };

    pub fn run() {
        debug_assert!((84849626u64).wrapping_mul(1) == 84849626u64);
        debug_assert!((2627643942u32) | (3664232124u32) | (2627643942u32) == (2627643942u32) | (3664232124u32));
        assert!((2866848609u64) ^ (2866848609u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2266931581u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3033980927u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1841654u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_6 } else { SCONST_6_ALT };
    if std::hint::black_box(false) {
        s.f0 = s.f0.wrapping_add(19);
    } else {
        s.f1 = s.f1.wrapping_add(35);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_6:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_6.f0;
    print!("case_6:"); println!("{}", c);

        }
        assert!((4223066794u64) ^ (4223066794u64) == 0u64);
        assert!((4263787202u64).wrapping_add(4263787202u64).wrapping_sub(4263787202u64) == 4263787202u64);
        debug_assert!((2576982897u32) | (2937927794u32) | (2576982897u32) == (2576982897u32) | (2937927794u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3335431866u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(110309442u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1634456937u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_7 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(6658u64).rotate_left(13) ^ 8537u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(4469u64).rotate_left(19) ^ 4668u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(7733u64).rotate_left(16) ^ 3890u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(6254u64).rotate_left(18) ^ 6496u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 744802u64;
    acc = acc.wrapping_add(824995u64);
    acc = acc.wrapping_add(679327u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(8807u64);
    acc = acc.wrapping_mul(54855u64);
    acc = acc ^ 300224u64;
    acc = acc.wrapping_mul(57215u64);
    acc = acc.wrapping_add(956522u64);
    acc = acc.wrapping_add(619656u64);
    acc = acc.wrapping_mul(13863u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(673507u64);
    acc = acc ^ 732231u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 602627u64;
    acc = acc.wrapping_mul(13333u64);
    acc = acc ^ 745952u64;
    acc = acc.wrapping_add(112625u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 921319u64;
    acc = acc.wrapping_add(481743u64);
    acc = acc.wrapping_add(843033u64);
    acc = acc ^ 862640u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(34435u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(26745u64);
    acc = acc.wrapping_mul(15591u64);
    acc = acc.wrapping_add(709919u64);
    acc = acc.wrapping_add(127889u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 391232u64;
    acc = acc.wrapping_add(177495u64);
    acc = acc.wrapping_add(677997u64);
    acc = acc ^ 542060u64;
    acc = acc.wrapping_mul(14113u64);
    acc = acc.wrapping_add(816659u64);
    acc = acc ^ 852616u64;
    acc = acc ^ 842587u64;
    acc = acc.wrapping_add(872160u64);
    acc = acc.wrapping_add(26242u64);
    acc = acc.wrapping_add(371691u64);
    acc = acc.wrapping_add(255651u64);
    acc = acc.wrapping_add(18776u64);
    acc = acc.wrapping_add(633246u64);
    acc = acc ^ 333762u64;
    acc = acc ^ 40008u64;
    acc = acc.wrapping_mul(34857u64);
    acc = acc.wrapping_mul(9279u64);
    acc = acc ^ 861165u64;
    acc = acc.wrapping_mul(47989u64);
    acc = acc.wrapping_mul(46639u64);
    acc = acc.wrapping_add(190928u64);
    acc = acc.wrapping_mul(52135u64);
    acc = acc.wrapping_add(148497u64);
    acc = acc.wrapping_add(252167u64);
    acc = acc ^ 444326u64;
    acc = acc.wrapping_mul(58515u64);
    acc = acc.wrapping_mul(45257u64);
    acc = acc.wrapping_add(876076u64);
    acc = acc ^ 592195u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 640792u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 814743u64;
    acc = acc.wrapping_mul(3941u64);
    acc = acc.wrapping_mul(10415u64);
    acc = acc ^ 437883u64;
    acc = acc.wrapping_mul(12155u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(36961u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(60843u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 703043u64;
    acc = acc.wrapping_mul(65167u64);
    acc = acc ^ 809143u64;
    acc = acc.rotate_left(13);
    acc = acc ^ 62004u64;
    acc = acc.wrapping_add(483636u64);
    acc = acc.wrapping_mul(10619u64);
    acc = acc ^ 222006u64;
    acc = acc.wrapping_add(915913u64);
    acc = acc.wrapping_mul(51851u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(23051u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(500800u64);
    acc = acc.wrapping_add(113771u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(44641u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(31323u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(962677u64);
    acc = acc.wrapping_add(989116u64);
    acc = acc.wrapping_mul(60517u64);
    acc = acc ^ 492446u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 202557u64;
    acc = acc.rotate_left(19);
    acc = acc ^ 938762u64;
    acc = acc.wrapping_mul(1189u64);
    acc = acc.wrapping_mul(13419u64);
    acc = acc ^ 814406u64;
    acc = acc.wrapping_add(916477u64);
    acc = acc.wrapping_add(119875u64);
    acc = acc.wrapping_add(816571u64);
    acc = acc.wrapping_add(584026u64);
    acc = acc.wrapping_add(278627u64);
    acc = acc.wrapping_mul(31025u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(55107u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(65353u64);
    acc = acc ^ 917246u64;
    acc = acc.wrapping_add(172177u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(52139u64);
    acc = acc.wrapping_add(367039u64);
    acc = acc ^ 134630u64;
    acc = acc.wrapping_mul(26495u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 275624u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(54097u64);
    acc = acc.wrapping_add(175398u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 696258u64;
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(651153u64);
    acc = acc.wrapping_add(276405u64);
    acc = acc.wrapping_mul(32415u64);
    acc = acc.wrapping_add(563203u64);
    acc = acc.wrapping_mul(53u64);
    acc = acc.wrapping_add(135529u64);
    acc = acc ^ 586238u64;
    acc = acc.wrapping_mul(21785u64);
    acc = acc ^ 327629u64;
    acc = acc.wrapping_mul(35727u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 418617u64;
    acc = acc.wrapping_add(11332u64);
    acc = acc.wrapping_mul(63451u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 738758u64;
    acc = acc ^ 108859u64;
    acc = acc.wrapping_mul(44309u64);
    acc = acc ^ 749181u64;
    acc = acc.wrapping_mul(41547u64);
    acc = acc ^ 959759u64;
    acc = acc.wrapping_mul(41335u64);
    acc = acc ^ 767518u64;
    acc = acc.rotate_left(8);
    acc = acc ^ 498153u64;
    acc = acc ^ 565744u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(447616u64);
    acc = acc.wrapping_mul(33671u64);
    acc = acc.wrapping_mul(65453u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(289451u64);
    acc = acc.wrapping_add(357215u64);
    acc = acc.wrapping_add(318481u64);
    acc = acc.wrapping_add(416383u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(859636u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(580842u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 762859u64;
    acc = acc.wrapping_add(129155u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(21511u64);
    acc = acc.wrapping_mul(56191u64);
    acc = acc.wrapping_add(996505u64);
    acc = acc.wrapping_add(65607u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(934u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 838u64, b: 701i32, c: false } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 768u64, b: -104i32, c: false } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_7 {
    f0: u8,
    f1: u16,
    f2: u32,
}

const SCONST_7: SPacked_7 = SPacked_7 { f0: 25, f1: 1086, f2: 1123 };
const SCONST_7_ALT: SPacked_7 = SPacked_7 { f0: 29, f1: 2130, f2: 2183 };

    pub fn run() {
        assert!((3360654026u64).wrapping_add(3360654026u64).wrapping_sub(3360654026u64) == 3360654026u64);
        debug_assert!((1439492800u32).rotate_left(18).rotate_right(18) == 1439492800u32);
        assert!((2162065495u64) ^ (2162065495u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3447957015u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3226392497u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2385427440u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_7 } else { SCONST_7_ALT };
    if std::hint::black_box(false) {
        s.f1 = s.f1.wrapping_add(20);
    } else {
        s.f2 = s.f2.wrapping_add(36);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_7:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_7.f1;
    print!("case_7:"); println!("{}", c);

        }
        debug_assert!((3156392994u32).rotate_left(6).rotate_right(6) == 3156392994u32);
        debug_assert!((1090895994u32).rotate_left(29).rotate_right(29) == 1090895994u32);
        assert!((3417316516u64).wrapping_add(3417316516u64).wrapping_sub(3417316516u64) == 3417316516u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2853235586u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2766852059u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1938160147u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_8 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(1316u64).rotate_left(1) ^ 8556u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(7918u64).rotate_left(1) ^ 2542u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(5579u64).rotate_left(4) ^ 7319u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(3153u64).rotate_left(20) ^ 9815u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(44742u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(52405u64);
    acc = acc.wrapping_add(693327u64);
    acc = acc ^ 164970u64;
    acc = acc.wrapping_add(84180u64);
    acc = acc ^ 347085u64;
    acc = acc.wrapping_mul(21103u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(244894u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(6429u64);
    acc = acc.wrapping_mul(27405u64);
    acc = acc ^ 949394u64;
    acc = acc.wrapping_add(776955u64);
    acc = acc.wrapping_mul(1563u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 930999u64;
    acc = acc.wrapping_mul(21813u64);
    acc = acc ^ 5006u64;
    acc = acc.wrapping_mul(58665u64);
    acc = acc.wrapping_mul(38943u64);
    acc = acc.wrapping_add(22804u64);
    acc = acc.wrapping_mul(19143u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(54407u64);
    acc = acc.wrapping_mul(9341u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 116099u64;
    acc = acc.wrapping_add(742516u64);
    acc = acc.wrapping_mul(33429u64);
    acc = acc.wrapping_add(816853u64);
    acc = acc.wrapping_add(411060u64);
    acc = acc.wrapping_mul(48791u64);
    acc = acc ^ 616298u64;
    acc = acc.wrapping_mul(49337u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(818949u64);
    acc = acc.wrapping_add(463660u64);
    acc = acc.wrapping_add(407478u64);
    acc = acc.wrapping_add(502565u64);
    acc = acc ^ 741172u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(64083u64);
    acc = acc.wrapping_add(586173u64);
    acc = acc ^ 418809u64;
    acc = acc ^ 592675u64;
    acc = acc.wrapping_add(920372u64);
    acc = acc ^ 143076u64;
    acc = acc.wrapping_mul(22249u64);
    acc = acc.wrapping_add(792693u64);
    acc = acc.wrapping_add(149156u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 686685u64;
    acc = acc.wrapping_mul(37077u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(613396u64);
    acc = acc ^ 484995u64;
    acc = acc.wrapping_mul(55055u64);
    acc = acc ^ 108393u64;
    acc = acc.wrapping_add(658883u64);
    acc = acc.wrapping_add(226887u64);
    acc = acc ^ 739501u64;
    acc = acc.wrapping_mul(39915u64);
    acc = acc ^ 475713u64;
    acc = acc ^ 842827u64;
    acc = acc ^ 250476u64;
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(74968u64);
    acc = acc.wrapping_add(514107u64);
    acc = acc ^ 124729u64;
    acc = acc.wrapping_add(688100u64);
    acc = acc.wrapping_mul(5423u64);
    acc = acc.wrapping_mul(13823u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(13145u64);
    acc = acc.wrapping_mul(37171u64);
    acc = acc.wrapping_add(94338u64);
    acc = acc.wrapping_mul(39163u64);
    acc = acc ^ 671852u64;
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(45919u64);
    acc = acc.wrapping_mul(59949u64);
    acc = acc.wrapping_mul(33125u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(48593u64);
    acc = acc ^ 952638u64;
    acc = acc ^ 486959u64;
    acc = acc.wrapping_mul(24075u64);
    acc = acc.wrapping_mul(12101u64);
    acc = acc.wrapping_add(558801u64);
    acc = acc ^ 832529u64;
    acc = acc ^ 638979u64;
    acc = acc ^ 205714u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(29115u64);
    acc = acc.wrapping_add(194824u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 997827u64;
    acc = acc ^ 516146u64;
    acc = acc ^ 504606u64;
    acc = acc.wrapping_mul(49295u64);
    acc = acc ^ 822070u64;
    acc = acc.wrapping_add(881208u64);
    acc = acc ^ 660304u64;
    acc = acc ^ 210336u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc ^ 439857u64;
    acc = acc.wrapping_add(335072u64);
    acc = acc.wrapping_add(428u64);
    acc = acc.wrapping_add(710265u64);
    acc = acc ^ 605038u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(740974u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(54851u64);
    acc = acc.wrapping_add(931075u64);
    acc = acc ^ 560946u64;
    acc = acc ^ 550536u64;
    acc = acc.wrapping_mul(6629u64);
    acc = acc.wrapping_mul(21897u64);
    acc = acc.wrapping_add(782246u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 618688u64;
    acc = acc.wrapping_add(798416u64);
    acc = acc.wrapping_mul(59401u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(56665u64);
    acc = acc.wrapping_add(710041u64);
    acc = acc.wrapping_add(334535u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(228650u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(408138u64);
    acc = acc.wrapping_add(406630u64);
    acc = acc.wrapping_mul(23665u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(109450u64);
    acc = acc.wrapping_add(688308u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 988095u64;
    acc = acc.wrapping_mul(3861u64);
    acc = acc ^ 426642u64;
    acc = acc ^ 519707u64;
    acc = acc ^ 531590u64;
    acc = acc.wrapping_add(628542u64);
    acc = acc.wrapping_add(77404u64);
    acc = acc.wrapping_mul(52815u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(515963u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(5423u64);
    acc = acc ^ 205547u64;
    acc = acc ^ 786354u64;
    acc = acc.wrapping_add(726500u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(58043u64);
    acc = acc ^ 943718u64;
    acc = acc.wrapping_mul(60717u64);
    acc = acc ^ 629376u64;
    acc = acc ^ 903972u64;
    acc = acc.wrapping_mul(929u64);
    acc = acc.wrapping_mul(29885u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 920u64, b: 749i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 796u64, b: -74i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V3(965u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_8 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_8: SPacked_8 = SPacked_8 { f0: 32, f1: 1093, f2: 1130 };
const SCONST_8_ALT: SPacked_8 = SPacked_8 { f0: 40, f1: 2141, f2: 2194 };

    pub fn run() {
        debug_assert!((1277349746u32) | (69216039u32) | (1277349746u32) == (1277349746u32) | (69216039u32));
        debug_assert!((724584861u32) | (2629100596u32) | (724584861u32) == (724584861u32) | (2629100596u32));
        debug_assert!((3692527186u32) | (1178883094u32) | (3692527186u32) == (3692527186u32) | (1178883094u32));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2078289136u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1102671518u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2818711231u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_8 } else { SCONST_8_ALT };
    if std::hint::black_box(true) {
        s.f0 = s.f0.wrapping_add(21);
    } else {
        s.f1 = s.f1.wrapping_add(37);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_8:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_8.f2;
    print!("case_8:"); println!("{}", c);

        }
        debug_assert!((2338495389u32).rotate_left(10).rotate_right(10) == 2338495389u32);
        assert!(!(802134697u64) == (802134697u64) ^ u64::MAX);
        debug_assert!((655712212u32).rotate_left(28).rotate_right(28) == 655712212u32);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(140201055u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(4184364245u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3414847472u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_9 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(6920u64).rotate_left(7) ^ 7136u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(1128u64).rotate_left(3) ^ 2978u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(4949u64).rotate_left(31) ^ 3812u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(686u64).rotate_left(29) ^ 8150u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 411965u64;
    acc = acc.wrapping_add(986697u64);
    acc = acc.wrapping_add(979218u64);
    acc = acc ^ 145268u64;
    acc = acc.wrapping_mul(24815u64);
    acc = acc.wrapping_add(736646u64);
    acc = acc ^ 497351u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(135644u64);
    acc = acc ^ 328018u64;
    acc = acc.wrapping_mul(8459u64);
    acc = acc.wrapping_mul(45121u64);
    acc = acc.wrapping_mul(5875u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(474832u64);
    acc = acc ^ 689029u64;
    acc = acc.wrapping_mul(19107u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(15269u64);
    acc = acc.wrapping_add(248927u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 526814u64;
    acc = acc.wrapping_add(665832u64);
    acc = acc.wrapping_mul(32017u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(6505u64);
    acc = acc ^ 529814u64;
    acc = acc ^ 880038u64;
    acc = acc.wrapping_add(366859u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(941575u64);
    acc = acc ^ 775202u64;
    acc = acc.wrapping_mul(34793u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(805767u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(486255u64);
    acc = acc.wrapping_add(985473u64);
    acc = acc.wrapping_add(655428u64);
    acc = acc ^ 171446u64;
    acc = acc ^ 808321u64;
    acc = acc.wrapping_add(757500u64);
    acc = acc.wrapping_add(197460u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(7269u64);
    acc = acc.wrapping_mul(9141u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(260525u64);
    acc = acc ^ 302906u64;
    acc = acc ^ 653065u64;
    acc = acc.wrapping_mul(17607u64);
    acc = acc.wrapping_mul(12773u64);
    acc = acc.wrapping_mul(35139u64);
    acc = acc.wrapping_add(913536u64);
    acc = acc.wrapping_add(146608u64);
    acc = acc.wrapping_add(244529u64);
    acc = acc.wrapping_add(966747u64);
    acc = acc.wrapping_add(171678u64);
    acc = acc ^ 283024u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(752226u64);
    acc = acc.wrapping_mul(24361u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(30895u64);
    acc = acc.wrapping_add(67456u64);
    acc = acc ^ 259071u64;
    acc = acc.wrapping_add(7523u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(764451u64);
    acc = acc ^ 354475u64;
    acc = acc.wrapping_mul(8485u64);
    acc = acc.wrapping_mul(42473u64);
    acc = acc.wrapping_mul(23629u64);
    acc = acc.wrapping_mul(2197u64);
    acc = acc.wrapping_add(922284u64);
    acc = acc ^ 506365u64;
    acc = acc ^ 80046u64;
    acc = acc.wrapping_mul(56541u64);
    acc = acc.wrapping_add(242495u64);
    acc = acc.wrapping_mul(14173u64);
    acc = acc ^ 561200u64;
    acc = acc.wrapping_mul(1295u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(50143u64);
    acc = acc.wrapping_add(181573u64);
    acc = acc.wrapping_add(466012u64);
    acc = acc.wrapping_mul(44131u64);
    acc = acc.wrapping_mul(2171u64);
    acc = acc.wrapping_add(239569u64);
    acc = acc.wrapping_mul(29157u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 928149u64;
    acc = acc.wrapping_add(481416u64);
    acc = acc.wrapping_add(591394u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(319u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(104044u64);
    acc = acc.wrapping_mul(16935u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc ^ 309116u64;
    acc = acc ^ 148568u64;
    acc = acc.wrapping_add(938697u64);
    acc = acc.wrapping_add(766053u64);
    acc = acc ^ 900273u64;
    acc = acc ^ 99871u64;
    acc = acc.wrapping_mul(52621u64);
    acc = acc.wrapping_add(443009u64);
    acc = acc.wrapping_mul(56537u64);
    acc = acc.wrapping_mul(44431u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(727728u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(11175u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 391960u64;
    acc = acc.wrapping_add(374977u64);
    acc = acc ^ 146899u64;
    acc = acc.wrapping_mul(20435u64);
    acc = acc.wrapping_mul(8981u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 662747u64;
    acc = acc ^ 802628u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(666599u64);
    acc = acc ^ 314897u64;
    acc = acc.wrapping_mul(46817u64);
    acc = acc ^ 679403u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(671134u64);
    acc = acc ^ 968718u64;
    acc = acc.wrapping_add(546533u64);
    acc = acc.wrapping_add(715468u64);
    acc = acc.wrapping_add(727653u64);
    acc = acc ^ 496026u64;
    acc = acc ^ 315822u64;
    acc = acc.wrapping_add(200642u64);
    acc = acc ^ 8438u64;
    acc = acc ^ 813210u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(99384u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(1489u64);
    acc = acc ^ 596228u64;
    acc = acc.wrapping_mul(43629u64);
    acc = acc.wrapping_mul(64757u64);
    acc = acc.wrapping_mul(38993u64);
    acc = acc ^ 851848u64;
    acc = acc.rotate_left(27);
    acc = acc ^ 25574u64;
    acc = acc.wrapping_mul(40625u64);
    acc = acc.wrapping_add(144597u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 245502u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(26855u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 780187u64;
    acc = acc.wrapping_add(251104u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(382202u64);
    acc = acc ^ 892098u64;
    acc = acc.wrapping_mul(16763u64);
    acc = acc.wrapping_add(101601u64);
    acc = acc ^ 26983u64;
    acc = acc ^ 435624u64;
    acc = acc.wrapping_add(563990u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(210845u64);
    acc = acc.wrapping_mul(24643u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(2257u64);
    acc = acc ^ 771995u64;
    acc = acc.wrapping_mul(47221u64);
    acc = acc ^ 934903u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(6835u64);
    acc = acc.wrapping_add(914846u64);
    acc = acc.wrapping_mul(26449u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 302110u64;
    acc = acc ^ 65124u64;
    acc = acc.wrapping_add(575652u64);
    acc = acc.wrapping_mul(36811u64);
    acc = acc ^ 878466u64;
    acc = acc ^ 810220u64;
    acc = acc.wrapping_add(357610u64);
    acc = acc.wrapping_mul(44277u64);
    acc = acc ^ 700648u64;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 412957u64;
    acc = acc ^ 983609u64;
    acc = acc ^ 468349u64;
    acc = acc.wrapping_add(322428u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(298212u64);
    acc = acc.wrapping_add(240952u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 9u64, b: -568i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 354u64, b: 648i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 488u64, b: -366i32, c: false } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_9 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_9: SPacked_9 = SPacked_9 { f0: 39, f1: 1100, f2: 1137 };
const SCONST_9_ALT: SPacked_9 = SPacked_9 { f0: 51, f1: 2152, f2: 2205 };

    pub fn run() {
        assert!(!(1620408186u64) == (1620408186u64) ^ u64::MAX);
        debug_assert!((1129464106u64).wrapping_mul(1) == 1129464106u64);
        debug_assert!((3502606872u32).rotate_left(29).rotate_right(29) == 3502606872u32);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2299696753u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2699298279u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3515259488u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_9 } else { SCONST_9_ALT };
    if std::hint::black_box(true) {
        s.f1 = s.f1.wrapping_add(22);
    } else {
        s.f2 = s.f2.wrapping_add(38);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_9:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_9.f0;
    print!("case_9:"); println!("{}", c);

        }
        debug_assert!((101818427u32) | (3819141289u32) | (101818427u32) == (101818427u32) | (3819141289u32));
        debug_assert!((2552185798u32) | (2800674485u32) | (2552185798u32) == (2552185798u32) | (2800674485u32));
        debug_assert!((2640485754u32) | (2117290472u32) | (2640485754u32) == (2640485754u32) | (2117290472u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(427941383u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1564003692u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3311827600u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_10 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(6525u64).rotate_left(14) ^ 2958u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(9199u64).rotate_left(9) ^ 1186u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(5828u64).rotate_left(20) ^ 9567u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(6409u64).rotate_left(17) ^ 5968u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 828668u64;
    acc = acc.rotate_left(11);
    acc = acc ^ 491230u64;
    acc = acc ^ 825792u64;
    acc = acc.wrapping_add(31754u64);
    acc = acc.wrapping_add(109387u64);
    acc = acc.wrapping_mul(21753u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 25112u64;
    acc = acc.wrapping_add(427447u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(51425u64);
    acc = acc.wrapping_add(40171u64);
    acc = acc ^ 978151u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 994923u64;
    acc = acc ^ 275397u64;
    acc = acc ^ 792669u64;
    acc = acc.wrapping_add(309138u64);
    acc = acc.wrapping_add(981948u64);
    acc = acc.wrapping_mul(56769u64);
    acc = acc.wrapping_mul(22909u64);
    acc = acc.wrapping_add(249115u64);
    acc = acc ^ 277720u64;
    acc = acc.wrapping_add(404118u64);
    acc = acc.wrapping_mul(57281u64);
    acc = acc.wrapping_add(874431u64);
    acc = acc ^ 246714u64;
    acc = acc.wrapping_add(759199u64);
    acc = acc.wrapping_mul(6649u64);
    acc = acc.wrapping_mul(2771u64);
    acc = acc ^ 701670u64;
    acc = acc.wrapping_mul(3015u64);
    acc = acc.wrapping_mul(15093u64);
    acc = acc.wrapping_mul(34377u64);
    acc = acc.wrapping_add(16639u64);
    acc = acc.wrapping_add(814400u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(25467u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(20429u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(45987u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(10679u64);
    acc = acc.wrapping_mul(38561u64);
    acc = acc ^ 318209u64;
    acc = acc ^ 85770u64;
    acc = acc.wrapping_add(870203u64);
    acc = acc.wrapping_add(952629u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 598303u64;
    acc = acc.wrapping_mul(14477u64);
    acc = acc ^ 140701u64;
    acc = acc.wrapping_add(892910u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(543148u64);
    acc = acc ^ 309866u64;
    acc = acc ^ 304773u64;
    acc = acc ^ 984357u64;
    acc = acc.wrapping_add(80920u64);
    acc = acc ^ 272964u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(7211u64);
    acc = acc ^ 397096u64;
    acc = acc ^ 657686u64;
    acc = acc.wrapping_mul(5621u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(525033u64);
    acc = acc ^ 661732u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(374801u64);
    acc = acc ^ 719951u64;
    acc = acc ^ 40292u64;
    acc = acc.rotate_left(2);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 115076u64;
    acc = acc.wrapping_mul(15663u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(466609u64);
    acc = acc.wrapping_add(611895u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(33719u64);
    acc = acc ^ 687730u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(803083u64);
    acc = acc.rotate_left(19);
    acc = acc ^ 475101u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(959555u64);
    acc = acc ^ 430708u64;
    acc = acc ^ 544302u64;
    acc = acc ^ 461609u64;
    acc = acc.wrapping_mul(50519u64);
    acc = acc.wrapping_mul(15419u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 147895u64;
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(915787u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(1509u64);
    acc = acc ^ 622960u64;
    acc = acc ^ 375615u64;
    acc = acc.wrapping_add(199054u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 834250u64;
    acc = acc ^ 398549u64;
    acc = acc ^ 716905u64;
    acc = acc.wrapping_add(891678u64);
    acc = acc.wrapping_add(683077u64);
    acc = acc ^ 976479u64;
    acc = acc.wrapping_mul(48983u64);
    acc = acc ^ 298362u64;
    acc = acc.wrapping_add(780205u64);
    acc = acc ^ 786646u64;
    acc = acc.wrapping_mul(41757u64);
    acc = acc ^ 538108u64;
    acc = acc ^ 400395u64;
    acc = acc.wrapping_add(912795u64);
    acc = acc ^ 559843u64;
    acc = acc.wrapping_add(353458u64);
    acc = acc ^ 792497u64;
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 495868u64;
    acc = acc ^ 373932u64;
    acc = acc.wrapping_mul(42589u64);
    acc = acc.wrapping_mul(50057u64);
    acc = acc.wrapping_add(101853u64);
    acc = acc ^ 576199u64;
    acc = acc ^ 267838u64;
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc ^ 328079u64;
    acc = acc.wrapping_add(545901u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(28211u64);
    acc = acc.wrapping_mul(43915u64);
    acc = acc.wrapping_add(418445u64);
    acc = acc.wrapping_add(477392u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(669424u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(48225u64);
    acc = acc ^ 846760u64;
    acc = acc.wrapping_mul(21691u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(396520u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(878111u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(171034u64);
    acc = acc.wrapping_add(156274u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 442755u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(7331u64);
    acc = acc.wrapping_mul(19867u64);
    acc = acc.wrapping_add(715013u64);
    acc = acc.wrapping_mul(13645u64);
    acc = acc.wrapping_mul(65083u64);
    acc = acc.wrapping_add(467115u64);
    acc = acc.wrapping_mul(4295u64);
    acc = acc.wrapping_add(11599u64);
    acc = acc.wrapping_add(360266u64);
    acc = acc ^ 419520u64;
    acc = acc.wrapping_mul(4631u64);
    acc = acc.wrapping_mul(63105u64);
    acc = acc ^ 62673u64;
    acc = acc ^ 355024u64;
    acc = acc.wrapping_mul(12975u64);
    acc = acc ^ 539141u64;
    acc = acc.wrapping_mul(62431u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(1527u64);
    acc = acc.wrapping_mul(9833u64);
    acc = acc.wrapping_add(599849u64);
    acc = acc ^ 366641u64;
    acc = acc ^ 867057u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(458978u64);
    acc = acc.wrapping_add(908586u64);
    acc = acc.wrapping_add(620201u64);
    acc = acc ^ 718303u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(996794u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 239u64, b: 529i32, c: true } } }

enum NoiseType1 { V0(u64), V1(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(542u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 44u64, b: 795i32, c: true } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_10 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_10: SPacked_10 = SPacked_10 { f0: 46, f1: 1107, f2: 1144 };
const SCONST_10_ALT: SPacked_10 = SPacked_10 { f0: 62, f1: 2163, f2: 2216 };

    pub fn run() {
        assert!((3688308802u64) ^ (3688308802u64) == 0u64);
        assert!((2477900469u64) ^ (2477900469u64) == 0u64);
        debug_assert!((464318321u64).wrapping_mul(1) == 464318321u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(748172536u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3243508479u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3400165665u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_10 } else { SCONST_10_ALT };
    if std::hint::black_box(false) {
        s.f0 = s.f0.wrapping_add(23);
    } else {
        s.f1 = s.f1.wrapping_add(39);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_10:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_10.f1;
    print!("case_10:"); println!("{}", c);

        }
        debug_assert!((204774711u32).rotate_left(24).rotate_right(24) == 204774711u32);
        assert!((2420000035u64).wrapping_add(2420000035u64).wrapping_sub(2420000035u64) == 2420000035u64);
        debug_assert!((4242589566u32) | (4234930665u32) | (4242589566u32) == (4242589566u32) | (4234930665u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3647124138u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3128643722u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(555924414u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_11 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(791u64).rotate_left(8) ^ 689u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(2490u64).rotate_left(20) ^ 9784u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(4825u64).rotate_left(5) ^ 9583u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(9913u64).rotate_left(1) ^ 5859u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(100561u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(521u64);
    acc = acc.wrapping_add(935903u64);
    acc = acc.wrapping_add(280518u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(206196u64);
    acc = acc.wrapping_add(180122u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 635671u64;
    acc = acc.wrapping_add(629450u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(20325u64);
    acc = acc ^ 51556u64;
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(574151u64);
    acc = acc ^ 603332u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 856363u64;
    acc = acc ^ 339956u64;
    acc = acc ^ 387849u64;
    acc = acc ^ 165362u64;
    acc = acc.rotate_left(10);
    acc = acc ^ 815442u64;
    acc = acc.rotate_left(24);
    acc = acc ^ 149740u64;
    acc = acc.wrapping_mul(50249u64);
    acc = acc ^ 484975u64;
    acc = acc.wrapping_add(244510u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(358579u64);
    acc = acc.wrapping_add(968133u64);
    acc = acc.wrapping_mul(47647u64);
    acc = acc.wrapping_add(713215u64);
    acc = acc ^ 166156u64;
    acc = acc ^ 39127u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(974224u64);
    acc = acc.wrapping_add(921271u64);
    acc = acc.wrapping_mul(39989u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 795359u64;
    acc = acc.wrapping_mul(56981u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 331673u64;
    acc = acc.wrapping_add(781408u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 600163u64;
    acc = acc.wrapping_add(807623u64);
    acc = acc.wrapping_add(813665u64);
    acc = acc ^ 842184u64;
    acc = acc.wrapping_mul(50053u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(59161u64);
    acc = acc ^ 358684u64;
    acc = acc ^ 238904u64;
    acc = acc.wrapping_add(5402u64);
    acc = acc.wrapping_add(26248u64);
    acc = acc.wrapping_mul(32811u64);
    acc = acc.wrapping_mul(47661u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(30941u64);
    acc = acc.wrapping_mul(36047u64);
    acc = acc ^ 864808u64;
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(40887u64);
    acc = acc ^ 139771u64;
    acc = acc.wrapping_mul(15349u64);
    acc = acc.wrapping_mul(47591u64);
    acc = acc.wrapping_add(145633u64);
    acc = acc.wrapping_add(121490u64);
    acc = acc.wrapping_add(703944u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 803355u64;
    acc = acc ^ 104818u64;
    acc = acc.wrapping_add(944018u64);
    acc = acc ^ 240318u64;
    acc = acc.wrapping_add(281433u64);
    acc = acc ^ 300737u64;
    acc = acc.wrapping_add(159508u64);
    acc = acc ^ 891469u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 695725u64;
    acc = acc.wrapping_add(573913u64);
    acc = acc.wrapping_add(62822u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(40363u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(25299u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(60639u64);
    acc = acc ^ 188623u64;
    acc = acc.wrapping_add(2299u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(942276u64);
    acc = acc.wrapping_mul(599u64);
    acc = acc ^ 763986u64;
    acc = acc.wrapping_add(54072u64);
    acc = acc ^ 348318u64;
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(20281u64);
    acc = acc.wrapping_mul(33379u64);
    acc = acc.wrapping_mul(63483u64);
    acc = acc.wrapping_add(384103u64);
    acc = acc.wrapping_mul(8849u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 181929u64;
    acc = acc ^ 429034u64;
    acc = acc.wrapping_add(479624u64);
    acc = acc ^ 190237u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(170698u64);
    acc = acc.wrapping_mul(5933u64);
    acc = acc.wrapping_add(966109u64);
    acc = acc.wrapping_add(375109u64);
    acc = acc ^ 467103u64;
    acc = acc ^ 157278u64;
    acc = acc.wrapping_mul(24729u64);
    acc = acc.wrapping_mul(31191u64);
    acc = acc ^ 698158u64;
    acc = acc.wrapping_mul(7171u64);
    acc = acc.wrapping_mul(3767u64);
    acc = acc.wrapping_mul(16433u64);
    acc = acc.wrapping_add(295307u64);
    acc = acc.wrapping_mul(35225u64);
    acc = acc.wrapping_add(228905u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(400020u64);
    acc = acc ^ 705783u64;
    acc = acc.wrapping_mul(50761u64);
    acc = acc.wrapping_mul(10081u64);
    acc = acc.wrapping_add(54355u64);
    acc = acc.wrapping_add(82700u64);
    acc = acc ^ 205601u64;
    acc = acc ^ 893290u64;
    acc = acc.wrapping_mul(23265u64);
    acc = acc.wrapping_add(998896u64);
    acc = acc.wrapping_add(98346u64);
    acc = acc.wrapping_mul(38501u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(49733u64);
    acc = acc.wrapping_add(245142u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(40627u64);
    acc = acc ^ 97606u64;
    acc = acc.wrapping_add(302199u64);
    acc = acc.wrapping_add(891586u64);
    acc = acc ^ 780089u64;
    acc = acc.wrapping_mul(8499u64);
    acc = acc.wrapping_mul(7525u64);
    acc = acc.wrapping_mul(22145u64);
    acc = acc.wrapping_add(533484u64);
    acc = acc.wrapping_add(684604u64);
    acc = acc ^ 813037u64;
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(9973u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 111711u64;
    acc = acc ^ 694753u64;
    acc = acc.wrapping_add(370701u64);
    acc = acc.wrapping_add(672772u64);
    acc = acc.wrapping_add(666974u64);
    acc = acc.wrapping_mul(59151u64);
    acc = acc.wrapping_mul(57911u64);
    acc = acc ^ 971309u64;
    acc = acc.wrapping_mul(32519u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(75608u64);
    acc = acc ^ 978100u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(167268u64);
    acc = acc.wrapping_add(237798u64);
    acc = acc.wrapping_mul(41993u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(39273u64);
    acc = acc.wrapping_add(803162u64);
    acc = acc.wrapping_mul(45841u64);
    acc = acc.wrapping_mul(56609u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(30333u64);
    acc = acc.wrapping_mul(39785u64);
    acc = acc.wrapping_add(223585u64);
    acc = acc.wrapping_mul(7819u64);
    acc = acc.wrapping_add(709248u64);
    acc = acc.wrapping_add(401152u64);
    acc = acc.wrapping_mul(38745u64);
    acc = acc.wrapping_add(605631u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(76u64) } }

enum NoiseType1 { V0(u64), V1(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(805u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(653u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_11 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_11: SPacked_11 = SPacked_11 { f0: 53, f1: 1114, f2: 1151 };
const SCONST_11_ALT: SPacked_11 = SPacked_11 { f0: 73, f1: 2174, f2: 2227 };

    pub fn run() {
        debug_assert!((1538305975u32).rotate_left(17).rotate_right(17) == 1538305975u32);
        debug_assert!((2670840679u32) | (215548100u32) | (2670840679u32) == (2670840679u32) | (215548100u32));
        assert!(!(1914190539u64) == (1914190539u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(325859357u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(86941244u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1402924050u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_11 } else { SCONST_11_ALT };
    if std::hint::black_box(false) {
        s.f1 = s.f1.wrapping_add(24);
    } else {
        s.f2 = s.f2.wrapping_add(40);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_11:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_11.f2;
    print!("case_11:"); println!("{}", c);

        }
        debug_assert!((2294698606u64).wrapping_mul(1) == 2294698606u64);
        debug_assert!((4259472628u64).wrapping_mul(1) == 4259472628u64);
        debug_assert!((3820959858u32) | (1685144664u32) | (3820959858u32) == (3820959858u32) | (1685144664u32));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2335320614u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1766063340u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1026956983u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_12 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(9949u64).rotate_left(19) ^ 6900u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(3361u64).rotate_left(21) ^ 6623u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(7228u64).rotate_left(22) ^ 8050u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(4286u64).rotate_left(25) ^ 5728u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 327797u64;
    acc = acc.wrapping_add(533884u64);
    acc = acc.wrapping_mul(62707u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 386327u64;
    acc = acc.rotate_left(15);
    acc = acc ^ 405196u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(62937u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(939906u64);
    acc = acc.wrapping_mul(8127u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 619141u64;
    acc = acc ^ 944542u64;
    acc = acc.wrapping_mul(40823u64);
    acc = acc.wrapping_mul(44695u64);
    acc = acc.wrapping_add(81307u64);
    acc = acc.wrapping_add(538497u64);
    acc = acc.wrapping_add(934445u64);
    acc = acc.wrapping_mul(1761u64);
    acc = acc.wrapping_add(381300u64);
    acc = acc ^ 697527u64;
    acc = acc ^ 19681u64;
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc ^ 384988u64;
    acc = acc.wrapping_add(774579u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(193352u64);
    acc = acc.wrapping_add(373692u64);
    acc = acc.wrapping_mul(16157u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 552583u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(756145u64);
    acc = acc.wrapping_add(807716u64);
    acc = acc.wrapping_add(205110u64);
    acc = acc.wrapping_mul(19997u64);
    acc = acc ^ 365568u64;
    acc = acc.wrapping_add(414946u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(19675u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(41851u64);
    acc = acc.wrapping_add(594493u64);
    acc = acc.wrapping_mul(38063u64);
    acc = acc.wrapping_mul(63229u64);
    acc = acc.wrapping_mul(16543u64);
    acc = acc.wrapping_mul(539u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 191544u64;
    acc = acc ^ 620876u64;
    acc = acc.wrapping_mul(21613u64);
    acc = acc.wrapping_mul(9149u64);
    acc = acc ^ 121852u64;
    acc = acc.wrapping_add(982890u64);
    acc = acc.wrapping_mul(38655u64);
    acc = acc.wrapping_mul(59463u64);
    acc = acc.wrapping_add(194479u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(11497u64);
    acc = acc.wrapping_add(12777u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc ^ 78314u64;
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(4171u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(308021u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(43881u64);
    acc = acc.wrapping_mul(14059u64);
    acc = acc ^ 271643u64;
    acc = acc.wrapping_add(970667u64);
    acc = acc ^ 923811u64;
    acc = acc.wrapping_add(761430u64);
    acc = acc ^ 172344u64;
    acc = acc.wrapping_add(67450u64);
    acc = acc.wrapping_add(991265u64);
    acc = acc.wrapping_add(288664u64);
    acc = acc.wrapping_add(840791u64);
    acc = acc ^ 782969u64;
    acc = acc ^ 574967u64;
    acc = acc.rotate_left(28);
    acc = acc ^ 551764u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(13323u64);
    acc = acc.wrapping_mul(42585u64);
    acc = acc.wrapping_mul(14905u64);
    acc = acc ^ 259736u64;
    acc = acc.wrapping_mul(5045u64);
    acc = acc ^ 56219u64;
    acc = acc.wrapping_mul(4937u64);
    acc = acc.wrapping_mul(38753u64);
    acc = acc ^ 892101u64;
    acc = acc.wrapping_add(407914u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(38639u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(63311u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(675u64);
    acc = acc.wrapping_mul(55025u64);
    acc = acc.wrapping_add(341630u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(659555u64);
    acc = acc ^ 68528u64;
    acc = acc ^ 779725u64;
    acc = acc ^ 865282u64;
    acc = acc ^ 72264u64;
    acc = acc.wrapping_mul(39317u64);
    acc = acc.wrapping_mul(45645u64);
    acc = acc.wrapping_mul(63265u64);
    acc = acc.wrapping_mul(57395u64);
    acc = acc.wrapping_add(993462u64);
    acc = acc ^ 11498u64;
    acc = acc.wrapping_mul(10355u64);
    acc = acc.wrapping_add(585041u64);
    acc = acc.wrapping_add(114858u64);
    acc = acc.wrapping_add(359442u64);
    acc = acc.wrapping_mul(46955u64);
    acc = acc.wrapping_add(650085u64);
    acc = acc.wrapping_add(810571u64);
    acc = acc.wrapping_mul(233u64);
    acc = acc.wrapping_add(747107u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 635161u64;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(729110u64);
    acc = acc.wrapping_add(463265u64);
    acc = acc ^ 630398u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(513099u64);
    acc = acc.wrapping_mul(7197u64);
    acc = acc.wrapping_mul(8389u64);
    acc = acc.wrapping_add(873255u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc ^ 911617u64;
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 985148u64;
    acc = acc.wrapping_mul(29795u64);
    acc = acc.wrapping_add(441312u64);
    acc = acc.wrapping_mul(52105u64);
    acc = acc.wrapping_add(841420u64);
    acc = acc ^ 818579u64;
    acc = acc.wrapping_add(166142u64);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(125808u64);
    acc = acc.wrapping_mul(56179u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(15687u64);
    acc = acc.wrapping_mul(44793u64);
    acc = acc ^ 371629u64;
    acc = acc.wrapping_mul(13685u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 458u64, b: -604i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 470u64, b: 245i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 120u64, b: 827i32, c: false } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_12 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_12: SPacked_12 = SPacked_12 { f0: 60, f1: 1121, f2: 1158 };
const SCONST_12_ALT: SPacked_12 = SPacked_12 { f0: 84, f1: 2185, f2: 2238 };

    pub fn run() {
        debug_assert!((3937592317u32).rotate_left(20).rotate_right(20) == 3937592317u32);
        assert!((3017753593u64) ^ (3017753593u64) == 0u64);
        assert!((498371407u64) ^ (498371407u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2379937432u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(154075484u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2530081646u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_12 } else { SCONST_12_ALT };
    if std::hint::black_box(true) {
        s.f0 = s.f0.wrapping_add(25);
    } else {
        s.f1 = s.f1.wrapping_add(41);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_12:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_12.f0;
    print!("case_12:"); println!("{}", c);

        }
        debug_assert!((4100587196u64).wrapping_mul(1) == 4100587196u64);
        debug_assert!((3543362111u32).rotate_left(12).rotate_right(12) == 3543362111u32);
        debug_assert!((1969864670u32) | (3306069146u32) | (1969864670u32) == (1969864670u32) | (3306069146u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1471634435u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1516960961u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3551539212u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_13 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(848u64).rotate_left(2) ^ 5860u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(4389u64).rotate_left(14) ^ 2369u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(6505u64).rotate_left(5) ^ 7382u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(1820u64).rotate_left(4) ^ 2514u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 618071u64;
    acc = acc.wrapping_add(419317u64);
    acc = acc.wrapping_mul(23497u64);
    acc = acc.wrapping_add(842317u64);
    acc = acc.wrapping_mul(47765u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(5450u64);
    acc = acc.wrapping_add(491584u64);
    acc = acc ^ 333011u64;
    acc = acc ^ 888536u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(29023u64);
    acc = acc.wrapping_mul(31771u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(49543u64);
    acc = acc ^ 173213u64;
    acc = acc.wrapping_add(843528u64);
    acc = acc ^ 549358u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(57970u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 704449u64;
    acc = acc ^ 249848u64;
    acc = acc.wrapping_add(787651u64);
    acc = acc.wrapping_mul(56951u64);
    acc = acc.wrapping_add(999577u64);
    acc = acc ^ 360522u64;
    acc = acc.wrapping_mul(59617u64);
    acc = acc ^ 421561u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(548697u64);
    acc = acc.wrapping_mul(12147u64);
    acc = acc ^ 160256u64;
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(891122u64);
    acc = acc.wrapping_add(483063u64);
    acc = acc.wrapping_add(951255u64);
    acc = acc.wrapping_mul(33973u64);
    acc = acc ^ 200843u64;
    acc = acc.wrapping_mul(49229u64);
    acc = acc ^ 92513u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(588012u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(63805u64);
    acc = acc ^ 234169u64;
    acc = acc ^ 794777u64;
    acc = acc.wrapping_add(291489u64);
    acc = acc.wrapping_add(49461u64);
    acc = acc ^ 68743u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(167855u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(14207u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(49755u64);
    acc = acc ^ 777325u64;
    acc = acc.wrapping_add(743045u64);
    acc = acc ^ 4456u64;
    acc = acc ^ 752320u64;
    acc = acc.wrapping_mul(36121u64);
    acc = acc.wrapping_mul(53307u64);
    acc = acc ^ 416465u64;
    acc = acc.wrapping_mul(4147u64);
    acc = acc.wrapping_add(933445u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 73890u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(15111u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 447410u64;
    acc = acc.wrapping_mul(32959u64);
    acc = acc.wrapping_mul(35101u64);
    acc = acc ^ 95903u64;
    acc = acc.rotate_left(2);
    acc = acc ^ 776652u64;
    acc = acc.wrapping_add(19881u64);
    acc = acc ^ 515362u64;
    acc = acc ^ 408849u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(16623u64);
    acc = acc.wrapping_add(348460u64);
    acc = acc.wrapping_add(836437u64);
    acc = acc.wrapping_mul(41319u64);
    acc = acc.wrapping_add(85622u64);
    acc = acc ^ 177625u64;
    acc = acc ^ 778048u64;
    acc = acc.wrapping_add(490925u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 2832u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(1889u64);
    acc = acc.rotate_left(25);
    acc = acc ^ 719263u64;
    acc = acc ^ 668867u64;
    acc = acc.wrapping_add(749654u64);
    acc = acc.wrapping_mul(54407u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 695025u64;
    acc = acc.wrapping_mul(57217u64);
    acc = acc.wrapping_mul(16965u64);
    acc = acc ^ 483799u64;
    acc = acc.wrapping_add(866350u64);
    acc = acc.wrapping_mul(9807u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(402207u64);
    acc = acc.wrapping_mul(20953u64);
    acc = acc ^ 212385u64;
    acc = acc.wrapping_mul(20097u64);
    acc = acc.wrapping_add(975398u64);
    acc = acc.wrapping_add(940368u64);
    acc = acc ^ 919875u64;
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(20793u64);
    acc = acc.wrapping_add(541846u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(96209u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 111913u64;
    acc = acc.wrapping_mul(13147u64);
    acc = acc ^ 97489u64;
    acc = acc ^ 933887u64;
    acc = acc.wrapping_mul(27371u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(419254u64);
    acc = acc.wrapping_mul(23443u64);
    acc = acc.wrapping_add(815013u64);
    acc = acc.wrapping_add(80756u64);
    acc = acc.wrapping_mul(58573u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(403524u64);
    acc = acc.wrapping_add(498385u64);
    acc = acc.wrapping_add(338298u64);
    acc = acc.wrapping_mul(7763u64);
    acc = acc ^ 603080u64;
    acc = acc.wrapping_mul(1107u64);
    acc = acc.wrapping_mul(21207u64);
    acc = acc.wrapping_add(253929u64);
    acc = acc.wrapping_add(190006u64);
    acc = acc.wrapping_mul(20713u64);
    acc = acc.wrapping_mul(32859u64);
    acc = acc.wrapping_mul(62625u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(95310u64);
    acc = acc.wrapping_mul(18191u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(28509u64);
    acc = acc.wrapping_mul(19967u64);
    acc = acc.wrapping_mul(3435u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(29841u64);
    acc = acc.wrapping_add(871556u64);
    acc = acc.wrapping_add(594575u64);
    acc = acc.wrapping_add(812874u64);
    acc = acc.wrapping_mul(19059u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(256921u64);
    acc = acc.wrapping_add(278233u64);
    acc = acc.wrapping_mul(3235u64);
    acc = acc.wrapping_add(607132u64);
    acc = acc.wrapping_mul(14433u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(912u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(941u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(282u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_13 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_13: SPacked_13 = SPacked_13 { f0: 67, f1: 1128, f2: 1165 };
const SCONST_13_ALT: SPacked_13 = SPacked_13 { f0: 95, f1: 2196, f2: 2249 };

    pub fn run() {
        debug_assert!((2367635145u64).wrapping_mul(1) == 2367635145u64);
        debug_assert!((1433627350u32).rotate_left(23).rotate_right(23) == 1433627350u32);
        assert!((168044320u64) ^ (168044320u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(38643114u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2146211565u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3886466600u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_13 } else { SCONST_13_ALT };
    if std::hint::black_box(true) {
        s.f1 = s.f1.wrapping_add(26);
    } else {
        s.f2 = s.f2.wrapping_add(42);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_13:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_13.f1;
    print!("case_13:"); println!("{}", c);

        }
        assert!(!(4194937416u64) == (4194937416u64) ^ u64::MAX);
        debug_assert!((1554690250u32) | (2623527804u32) | (1554690250u32) == (1554690250u32) | (2623527804u32));
        assert!(!(2170029387u64) == (2170029387u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3324698794u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(294272374u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1077088913u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_14 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(3508u64).rotate_left(5) ^ 9025u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(5320u64).rotate_left(19) ^ 8140u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(7342u64).rotate_left(13) ^ 2616u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(3837u64).rotate_left(27) ^ 5079u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(418993u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc ^ 329427u64;
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(888182u64);
    acc = acc.wrapping_mul(15141u64);
    acc = acc.wrapping_mul(10397u64);
    acc = acc.wrapping_mul(37203u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 553611u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(49355u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(64771u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(916474u64);
    acc = acc ^ 176284u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(40139u64);
    acc = acc ^ 849242u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(904354u64);
    acc = acc.wrapping_mul(60331u64);
    acc = acc.wrapping_mul(26141u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(716137u64);
    acc = acc ^ 260153u64;
    acc = acc.wrapping_mul(11583u64);
    acc = acc ^ 575037u64;
    acc = acc.wrapping_add(390748u64);
    acc = acc.wrapping_mul(26053u64);
    acc = acc ^ 457473u64;
    acc = acc.wrapping_mul(39603u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc ^ 933656u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(231020u64);
    acc = acc.wrapping_mul(60303u64);
    acc = acc.wrapping_mul(27159u64);
    acc = acc ^ 124717u64;
    acc = acc ^ 783778u64;
    acc = acc ^ 741153u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(225015u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc ^ 146856u64;
    acc = acc.wrapping_mul(57257u64);
    acc = acc ^ 175282u64;
    acc = acc ^ 565260u64;
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(533097u64);
    acc = acc.wrapping_add(424360u64);
    acc = acc.wrapping_mul(18793u64);
    acc = acc.wrapping_add(787934u64);
    acc = acc ^ 354926u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 847754u64;
    acc = acc.wrapping_mul(28127u64);
    acc = acc.wrapping_mul(24769u64);
    acc = acc.wrapping_mul(3867u64);
    acc = acc.wrapping_mul(7255u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(56843u64);
    acc = acc.wrapping_add(113614u64);
    acc = acc.wrapping_add(71680u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 623393u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(27177u64);
    acc = acc ^ 254958u64;
    acc = acc ^ 795482u64;
    acc = acc.wrapping_add(981763u64);
    acc = acc.wrapping_add(123522u64);
    acc = acc.wrapping_mul(39667u64);
    acc = acc ^ 688174u64;
    acc = acc ^ 678661u64;
    acc = acc.rotate_left(20);
    acc = acc ^ 842356u64;
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(607153u64);
    acc = acc ^ 139275u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(660472u64);
    acc = acc.wrapping_mul(60011u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(53303u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(46351u64);
    acc = acc.wrapping_add(475773u64);
    acc = acc.wrapping_mul(17069u64);
    acc = acc ^ 154650u64;
    acc = acc.wrapping_add(668074u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc ^ 983366u64;
    acc = acc.rotate_left(12);
    acc = acc ^ 465017u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(463449u64);
    acc = acc.wrapping_add(355343u64);
    acc = acc.wrapping_mul(45651u64);
    acc = acc ^ 703438u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(31263u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(44781u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(13863u64);
    acc = acc.wrapping_add(954326u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(35537u64);
    acc = acc ^ 726570u64;
    acc = acc.wrapping_mul(27549u64);
    acc = acc.wrapping_mul(14631u64);
    acc = acc.wrapping_add(488871u64);
    acc = acc.wrapping_add(904016u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 745769u64;
    acc = acc.wrapping_mul(43597u64);
    acc = acc.wrapping_add(819461u64);
    acc = acc.wrapping_mul(59315u64);
    acc = acc.wrapping_add(188550u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(857082u64);
    acc = acc.wrapping_mul(29323u64);
    acc = acc ^ 760559u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(901038u64);
    acc = acc ^ 432866u64;
    acc = acc.wrapping_add(752928u64);
    acc = acc.wrapping_add(806859u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 805084u64;
    acc = acc.wrapping_add(30937u64);
    acc = acc.wrapping_add(73905u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 750645u64;
    acc = acc.wrapping_add(310242u64);
    acc = acc.wrapping_add(847904u64);
    acc = acc.wrapping_mul(2431u64);
    acc = acc.wrapping_add(7388u64);
    acc = acc.wrapping_mul(15997u64);
    acc = acc.wrapping_mul(23433u64);
    acc = acc ^ 940520u64;
    acc = acc ^ 695681u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(65267u64);
    acc = acc.wrapping_mul(5697u64);
    acc = acc ^ 982737u64;
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 925u64, b: 41i32, c: true } } }

enum NoiseType1 { V0(u64), V1(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(419u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 328u64, b: 242i32, c: true } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_14 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_14: SPacked_14 = SPacked_14 { f0: 74, f1: 1135, f2: 1172 };
const SCONST_14_ALT: SPacked_14 = SPacked_14 { f0: 106, f1: 2207, f2: 2260 };

    pub fn run() {
        debug_assert!((2834180412u64).wrapping_mul(1) == 2834180412u64);
        assert!((4192509035u64) ^ (4192509035u64) == 0u64);
        debug_assert!((3531718864u32) | (1909555149u32) | (3531718864u32) == (3531718864u32) | (1909555149u32));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3586055316u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1063328196u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1084959010u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_14 } else { SCONST_14_ALT };
    if std::hint::black_box(false) {
        s.f0 = s.f0.wrapping_add(27);
    } else {
        s.f1 = s.f1.wrapping_add(43);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_14:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_14.f2;
    print!("case_14:"); println!("{}", c);

        }
        debug_assert!((25602576u32) | (1021081935u32) | (25602576u32) == (25602576u32) | (1021081935u32));
        debug_assert!((2726003315u32).rotate_left(10).rotate_right(10) == 2726003315u32);
        assert!((3482086587u64) ^ (3482086587u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2521803801u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3639904234u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1429886182u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_15 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(1023u64).rotate_left(16) ^ 7331u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(6832u64).rotate_left(16) ^ 2898u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(5627u64).rotate_left(12) ^ 9528u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(3250u64).rotate_left(12) ^ 11u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(34989u64);
    acc = acc ^ 723588u64;
    acc = acc.wrapping_mul(19543u64);
    acc = acc.wrapping_add(614618u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(629124u64);
    acc = acc.wrapping_add(947987u64);
    acc = acc ^ 196273u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(26985u64);
    acc = acc.wrapping_mul(18023u64);
    acc = acc.wrapping_mul(2397u64);
    acc = acc.wrapping_add(472582u64);
    acc = acc.wrapping_add(862433u64);
    acc = acc.wrapping_mul(3601u64);
    acc = acc.wrapping_mul(5701u64);
    acc = acc ^ 795963u64;
    acc = acc.wrapping_mul(48301u64);
    acc = acc ^ 844204u64;
    acc = acc.wrapping_add(185616u64);
    acc = acc.wrapping_mul(37393u64);
    acc = acc.wrapping_add(417993u64);
    acc = acc.wrapping_add(949159u64);
    acc = acc.wrapping_mul(3999u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(17467u64);
    acc = acc.wrapping_add(218700u64);
    acc = acc ^ 642302u64;
    acc = acc.wrapping_mul(1587u64);
    acc = acc.wrapping_add(238196u64);
    acc = acc.wrapping_mul(23969u64);
    acc = acc.wrapping_add(141037u64);
    acc = acc.wrapping_add(909180u64);
    acc = acc.wrapping_mul(13205u64);
    acc = acc ^ 923394u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(9245u64);
    acc = acc.wrapping_add(477806u64);
    acc = acc.wrapping_mul(42465u64);
    acc = acc ^ 962504u64;
    acc = acc.wrapping_mul(37725u64);
    acc = acc.wrapping_mul(35035u64);
    acc = acc.wrapping_add(350382u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 987133u64;
    acc = acc.wrapping_mul(46787u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 655939u64;
    acc = acc.wrapping_mul(34719u64);
    acc = acc.wrapping_mul(45873u64);
    acc = acc.wrapping_mul(61031u64);
    acc = acc ^ 518094u64;
    acc = acc.wrapping_mul(32967u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(891163u64);
    acc = acc.wrapping_add(447663u64);
    acc = acc ^ 549503u64;
    acc = acc.wrapping_mul(42641u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(61157u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(46243u64);
    acc = acc.wrapping_mul(40975u64);
    acc = acc.wrapping_add(595686u64);
    acc = acc ^ 617736u64;
    acc = acc.wrapping_add(453370u64);
    acc = acc.wrapping_mul(25353u64);
    acc = acc.wrapping_mul(23889u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(21531u64);
    acc = acc.wrapping_mul(37525u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 389808u64;
    acc = acc.wrapping_add(332961u64);
    acc = acc.wrapping_add(937468u64);
    acc = acc ^ 730057u64;
    acc = acc.wrapping_mul(4301u64);
    acc = acc ^ 314922u64;
    acc = acc ^ 696835u64;
    acc = acc.wrapping_mul(55961u64);
    acc = acc.wrapping_add(428175u64);
    acc = acc ^ 705895u64;
    acc = acc.wrapping_mul(9299u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(248742u64);
    acc = acc.wrapping_mul(21449u64);
    acc = acc.wrapping_mul(61723u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 348343u64;
    acc = acc.wrapping_add(223904u64);
    acc = acc ^ 233674u64;
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(734031u64);
    acc = acc ^ 476988u64;
    acc = acc ^ 344169u64;
    acc = acc.wrapping_mul(5197u64);
    acc = acc.wrapping_mul(967u64);
    acc = acc.wrapping_mul(16449u64);
    acc = acc.wrapping_mul(54889u64);
    acc = acc ^ 120784u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(321548u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 121710u64;
    acc = acc.wrapping_mul(22843u64);
    acc = acc.wrapping_add(108754u64);
    acc = acc.wrapping_mul(59029u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 655121u64;
    acc = acc.wrapping_mul(20165u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 931045u64;
    acc = acc.wrapping_add(341363u64);
    acc = acc ^ 881085u64;
    acc = acc ^ 200442u64;
    acc = acc.wrapping_mul(15877u64);
    acc = acc.wrapping_add(35976u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 308176u64;
    acc = acc.wrapping_add(663920u64);
    acc = acc.wrapping_mul(61149u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(30313u64);
    acc = acc.wrapping_add(560388u64);
    acc = acc.wrapping_add(942496u64);
    acc = acc.wrapping_add(934357u64);
    acc = acc.wrapping_add(751068u64);
    acc = acc.wrapping_add(695568u64);
    acc = acc.wrapping_add(837188u64);
    acc = acc ^ 309017u64;
    acc = acc.wrapping_mul(23317u64);
    acc = acc.wrapping_mul(64247u64);
    acc = acc ^ 574629u64;
    acc = acc ^ 175957u64;
    acc = acc ^ 795450u64;
    acc = acc ^ 502901u64;
    acc = acc ^ 585661u64;
    acc = acc.wrapping_add(642879u64);
    acc = acc ^ 192608u64;
    acc = acc.wrapping_mul(14757u64);
    acc = acc.wrapping_add(86100u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 290571u64;
    acc = acc.wrapping_mul(45861u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 506u64, b: -35i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 729u64, b: 163i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(439u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_15 {
    f0: i8,
    f1: i64,
    f2: usize,
}

const SCONST_15: SPacked_15 = SPacked_15 { f0: 81, f1: 1142, f2: 1179 };
const SCONST_15_ALT: SPacked_15 = SPacked_15 { f0: 117, f1: 2218, f2: 2271 };

    pub fn run() {
        assert!((3141952384u64) ^ (3141952384u64) == 0u64);
        assert!(!(2396361135u64) == (2396361135u64) ^ u64::MAX);
        assert!((1524720997u64).wrapping_add(1524720997u64).wrapping_sub(1524720997u64) == 1524720997u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2497677220u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3479839434u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3903622114u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_15 } else { SCONST_15_ALT };
    if std::hint::black_box(false) {
        s.f1 = s.f1.wrapping_add(28);
    } else {
        s.f2 = s.f2.wrapping_add(44);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    print!("case_15:"); println!("{} {} {}", v0, v1, v2);

    let c = SCONST_15.f0;
    print!("case_15:"); println!("{}", c);

        }
        assert!(!(1048119545u64) == (1048119545u64) ^ u64::MAX);
        debug_assert!((27246372u32).rotate_left(1).rotate_right(1) == 27246372u32);
        assert!((2451542028u64).wrapping_add(2451542028u64).wrapping_sub(2451542028u64) == 2451542028u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2755540303u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2143277745u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3749719910u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_16 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(5067u64).rotate_left(25) ^ 9541u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(7887u64).rotate_left(18) ^ 4078u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(8947u64).rotate_left(10) ^ 6984u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(5957u64).rotate_left(5) ^ 7630u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(982374u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 623287u64;
    acc = acc.wrapping_add(33624u64);
    acc = acc.wrapping_add(940988u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 624880u64;
    acc = acc.wrapping_add(14293u64);
    acc = acc ^ 498364u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 965455u64;
    acc = acc ^ 344393u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 234706u64;
    acc = acc.wrapping_mul(19981u64);
    acc = acc.wrapping_add(976268u64);
    acc = acc.wrapping_add(146139u64);
    acc = acc.wrapping_mul(15931u64);
    acc = acc ^ 682268u64;
    acc = acc ^ 475888u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(935799u64);
    acc = acc.wrapping_add(118568u64);
    acc = acc ^ 997374u64;
    acc = acc.wrapping_mul(25249u64);
    acc = acc.wrapping_mul(15627u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(774382u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc ^ 274762u64;
    acc = acc.wrapping_add(322620u64);
    acc = acc ^ 559190u64;
    acc = acc.wrapping_add(356062u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(59117u64);
    acc = acc ^ 693124u64;
    acc = acc.wrapping_add(17045u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(63363u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(760213u64);
    acc = acc.wrapping_add(903149u64);
    acc = acc.wrapping_add(146165u64);
    acc = acc ^ 742143u64;
    acc = acc.wrapping_mul(34921u64);
    acc = acc.wrapping_mul(61307u64);
    acc = acc.wrapping_add(652892u64);
    acc = acc.wrapping_add(985466u64);
    acc = acc.wrapping_mul(10745u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(109982u64);
    acc = acc.wrapping_mul(16147u64);
    acc = acc.wrapping_mul(9683u64);
    acc = acc.wrapping_add(665270u64);
    acc = acc.wrapping_add(32669u64);
    acc = acc ^ 966334u64;
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(13399u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 566620u64;
    acc = acc.wrapping_add(989239u64);
    acc = acc.wrapping_mul(25991u64);
    acc = acc.wrapping_add(716559u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(51873u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 422910u64;
    acc = acc ^ 884992u64;
    acc = acc.wrapping_add(402740u64);
    acc = acc ^ 40757u64;
    acc = acc ^ 637832u64;
    acc = acc ^ 960508u64;
    acc = acc.wrapping_add(489641u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(20745u64);
    acc = acc ^ 390808u64;
    acc = acc.wrapping_mul(38265u64);
    acc = acc.wrapping_add(872384u64);
    acc = acc.wrapping_mul(17953u64);
    acc = acc ^ 781345u64;
    acc = acc ^ 310344u64;
    acc = acc.wrapping_mul(59253u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 970150u64;
    acc = acc.wrapping_add(583440u64);
    acc = acc.wrapping_add(352305u64);
    acc = acc ^ 662225u64;
    acc = acc.wrapping_add(945013u64);
    acc = acc.wrapping_mul(35437u64);
    acc = acc.wrapping_mul(15919u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(522862u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 176874u64;
    acc = acc.wrapping_mul(40165u64);
    acc = acc.wrapping_mul(64329u64);
    acc = acc.wrapping_mul(42931u64);
    acc = acc.wrapping_add(781643u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(940969u64);
    acc = acc.wrapping_mul(14483u64);
    acc = acc.wrapping_mul(63u64);
    acc = acc ^ 158065u64;
    acc = acc.wrapping_add(953500u64);
    acc = acc.wrapping_add(434959u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 504910u64;
    acc = acc ^ 822522u64;
    acc = acc.wrapping_add(809910u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(525224u64);
    acc = acc.wrapping_add(629320u64);
    acc = acc.wrapping_mul(22805u64);
    acc = acc.wrapping_mul(57785u64);
    acc = acc.wrapping_mul(22563u64);
    acc = acc.wrapping_add(826580u64);
    acc = acc.wrapping_add(213561u64);
    acc = acc.wrapping_mul(65275u64);
    acc = acc ^ 66963u64;
    acc = acc.wrapping_mul(107u64);
    acc = acc.wrapping_add(189181u64);
    acc = acc.wrapping_mul(22269u64);
    acc = acc.wrapping_mul(40607u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(52973u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(478488u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(36135u64);
    acc = acc.wrapping_mul(34539u64);
    acc = acc ^ 653904u64;
    acc = acc.wrapping_mul(7881u64);
    acc = acc.wrapping_mul(63421u64);
    acc = acc.wrapping_mul(41189u64);
    acc = acc ^ 619456u64;
    acc = acc.wrapping_mul(2241u64);
    acc = acc.wrapping_add(218967u64);
    acc = acc.wrapping_add(536150u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 970135u64;
    acc = acc.wrapping_mul(36477u64);
    acc = acc.wrapping_add(62380u64);
    acc = acc.wrapping_add(480751u64);
    acc = acc.wrapping_mul(53973u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 497u64, b: -453i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 464u64, b: 610i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 655u64, b: 331i32, c: false } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_16 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_16: SPacked_16 = SPacked_16 { f0: 88, f1: 125, f2: 162, f3: 199 };
const SCONST_16_ALT: SPacked_16 = SPacked_16 { f0: 128, f1: 181, f2: 234, f3: 31 };

    pub fn run() {
        debug_assert!((3859357303u32) | (1169104986u32) | (3859357303u32) == (3859357303u32) | (1169104986u32));
        assert!((3623483097u64) ^ (3623483097u64) == 0u64);
        debug_assert!((3740962943u64).wrapping_mul(1) == 3740962943u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(587528462u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(420075247u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(803229447u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_16 } else { SCONST_16_ALT };
    if std::hint::black_box(true) {
        s.f0 = s.f0.wrapping_add(29);
    } else {
        s.f1 = s.f1.wrapping_add(45);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_16:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_16.f0;
    print!("case_16:"); println!("{}", c);

        }
        assert!((2343303983u64).wrapping_add(2343303983u64).wrapping_sub(2343303983u64) == 2343303983u64);
        debug_assert!((3627708676u32).rotate_left(23).rotate_right(23) == 3627708676u32);
        debug_assert!((3523386732u32).rotate_left(29).rotate_right(29) == 3523386732u32);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(288634433u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1146990269u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1801725567u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_17 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(807u64).rotate_left(25) ^ 4178u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(2476u64).rotate_left(4) ^ 2235u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(1340u64).rotate_left(6) ^ 3229u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(4258u64).rotate_left(1) ^ 4619u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(23721u64);
    acc = acc.wrapping_mul(12899u64);
    acc = acc.wrapping_mul(14545u64);
    acc = acc.wrapping_add(476517u64);
    acc = acc.wrapping_add(544218u64);
    acc = acc.wrapping_add(586614u64);
    acc = acc ^ 103550u64;
    acc = acc.wrapping_add(910381u64);
    acc = acc ^ 507706u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(7991u64);
    acc = acc.wrapping_mul(39929u64);
    acc = acc.wrapping_add(714833u64);
    acc = acc.wrapping_add(383702u64);
    acc = acc.wrapping_mul(27779u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(699661u64);
    acc = acc.wrapping_add(766778u64);
    acc = acc.wrapping_add(722417u64);
    acc = acc.wrapping_add(53461u64);
    acc = acc.wrapping_add(451988u64);
    acc = acc ^ 167158u64;
    acc = acc ^ 939814u64;
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(22679u64);
    acc = acc.wrapping_mul(12651u64);
    acc = acc.wrapping_mul(17257u64);
    acc = acc.wrapping_mul(27723u64);
    acc = acc.wrapping_mul(43911u64);
    acc = acc.wrapping_add(89141u64);
    acc = acc.wrapping_mul(3003u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 976803u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(2995u64);
    acc = acc.wrapping_add(554667u64);
    acc = acc.wrapping_mul(54083u64);
    acc = acc ^ 282584u64;
    acc = acc.wrapping_mul(38045u64);
    acc = acc.wrapping_add(422448u64);
    acc = acc ^ 12876u64;
    acc = acc ^ 965071u64;
    acc = acc.wrapping_mul(5939u64);
    acc = acc.wrapping_mul(2541u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(957529u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc ^ 868007u64;
    acc = acc ^ 243317u64;
    acc = acc ^ 742164u64;
    acc = acc.rotate_left(17);
    acc = acc ^ 278310u64;
    acc = acc.wrapping_add(923876u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(9217u64);
    acc = acc.wrapping_mul(24483u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(29131u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 731214u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(330371u64);
    acc = acc.wrapping_add(245719u64);
    acc = acc ^ 561372u64;
    acc = acc.wrapping_add(495129u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(502092u64);
    acc = acc ^ 544150u64;
    acc = acc ^ 670336u64;
    acc = acc.rotate_left(9);
    acc = acc ^ 436088u64;
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(469816u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(117039u64);
    acc = acc ^ 641318u64;
    acc = acc.wrapping_mul(7247u64);
    acc = acc.wrapping_add(996021u64);
    acc = acc ^ 343416u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(60877u64);
    acc = acc ^ 937631u64;
    acc = acc ^ 504845u64;
    acc = acc.wrapping_add(229657u64);
    acc = acc.wrapping_add(902605u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 621277u64;
    acc = acc.wrapping_add(324869u64);
    acc = acc.wrapping_add(569481u64);
    acc = acc.wrapping_add(411948u64);
    acc = acc.wrapping_add(768536u64);
    acc = acc.wrapping_mul(45879u64);
    acc = acc ^ 790853u64;
    acc = acc ^ 462414u64;
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc ^ 560158u64;
    acc = acc.wrapping_add(250814u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc ^ 817959u64;
    acc = acc ^ 810533u64;
    acc = acc.wrapping_mul(54289u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(991669u64);
    acc = acc.rotate_left(31);
    acc = acc ^ 627396u64;
    acc = acc.wrapping_add(991819u64);
    acc = acc.wrapping_mul(57415u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(62577u64);
    acc = acc ^ 582106u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(160647u64);
    acc = acc.wrapping_add(985405u64);
    acc = acc ^ 666236u64;
    acc = acc.wrapping_mul(36191u64);
    acc = acc.wrapping_add(453287u64);
    acc = acc.wrapping_add(328554u64);
    acc = acc.wrapping_mul(58777u64);
    acc = acc.wrapping_add(152293u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 220714u64;
    acc = acc.wrapping_add(92156u64);
    acc = acc.wrapping_mul(18781u64);
    acc = acc.wrapping_add(911078u64);
    acc = acc.wrapping_mul(52395u64);
    acc = acc.wrapping_mul(53951u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(343371u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(789480u64);
    acc = acc ^ 150262u64;
    acc = acc.wrapping_add(816980u64);
    acc = acc.wrapping_mul(56533u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(24537u64);
    acc = acc.wrapping_add(920746u64);
    acc = acc ^ 967845u64;
    acc = acc.wrapping_add(471664u64);
    acc = acc ^ 903937u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(61031u64);
    acc = acc ^ 237722u64;
    acc = acc.wrapping_add(665538u64);
    acc = acc.wrapping_mul(28181u64);
    acc = acc.wrapping_mul(37373u64);
    acc = acc.wrapping_add(356823u64);
    acc = acc ^ 491333u64;
    acc = acc.wrapping_add(818131u64);
    acc = acc ^ 894179u64;
    acc = acc.wrapping_add(919768u64);
    acc = acc ^ 145483u64;
    acc = acc.wrapping_add(4238u64);
    acc = acc.wrapping_add(664856u64);
    acc = acc ^ 969555u64;
    acc = acc ^ 902323u64;
    acc = acc.wrapping_add(29262u64);
    acc = acc.wrapping_mul(16833u64);
    acc = acc ^ 932944u64;
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(7751u64);
    acc = acc.wrapping_mul(32219u64);
    acc = acc ^ 178105u64;
    acc = acc.wrapping_add(956294u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(43355u64);
    acc = acc ^ 611685u64;
    acc = acc.wrapping_add(22414u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(843221u64);
    acc = acc ^ 359853u64;
    acc = acc.wrapping_add(124016u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V3(802u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(20u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 196u64, b: -536i32, c: true } } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_17 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_17: SPacked_17 = SPacked_17 { f0: 95, f1: 132, f2: 169, f3: 206 };
const SCONST_17_ALT: SPacked_17 = SPacked_17 { f0: 139, f1: 192, f2: 245, f3: 42 };

    pub fn run() {
        assert!((4027714996u64) ^ (4027714996u64) == 0u64);
        debug_assert!((4070431183u32) | (3049424343u32) | (4070431183u32) == (4070431183u32) | (3049424343u32));
        assert!((1682261868u64).wrapping_add(1682261868u64).wrapping_sub(1682261868u64) == 1682261868u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2324172456u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1955080975u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3810856822u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_17 } else { SCONST_17_ALT };
    if std::hint::black_box(true) {
        s.f1 = s.f1.wrapping_add(30);
    } else {
        s.f2 = s.f2.wrapping_add(46);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_17:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_17.f1;
    print!("case_17:"); println!("{}", c);

        }
        assert!((2866520845u64).wrapping_add(2866520845u64).wrapping_sub(2866520845u64) == 2866520845u64);
        debug_assert!((975982069u32).rotate_left(1).rotate_right(1) == 975982069u32);
        assert!((1600044846u64) ^ (1600044846u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2548826861u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1569238515u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2417829789u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_18 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(5394u64).rotate_left(2) ^ 3690u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(6502u64).rotate_left(24) ^ 3371u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(4145u64).rotate_left(30) ^ 5454u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(3885u64).rotate_left(18) ^ 6434u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(327053u64);
    acc = acc.wrapping_add(774404u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(21883u64);
    acc = acc.wrapping_mul(64323u64);
    acc = acc.wrapping_mul(54947u64);
    acc = acc ^ 727062u64;
    acc = acc ^ 306481u64;
    acc = acc.wrapping_mul(17747u64);
    acc = acc.wrapping_mul(39665u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(229764u64);
    acc = acc ^ 916783u64;
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(54125u64);
    acc = acc.wrapping_add(49844u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(56527u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(221820u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(387168u64);
    acc = acc.wrapping_mul(3355u64);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(23654u64);
    acc = acc.wrapping_add(414501u64);
    acc = acc.wrapping_mul(26245u64);
    acc = acc ^ 603280u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 771342u64;
    acc = acc.wrapping_add(286962u64);
    acc = acc.wrapping_add(945577u64);
    acc = acc.wrapping_add(635645u64);
    acc = acc.wrapping_add(869231u64);
    acc = acc.wrapping_mul(32285u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(58921u64);
    acc = acc.wrapping_mul(56815u64);
    acc = acc.rotate_left(19);
    acc = acc ^ 375024u64;
    acc = acc.wrapping_mul(26057u64);
    acc = acc.wrapping_mul(7489u64);
    acc = acc.wrapping_add(583671u64);
    acc = acc ^ 286792u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(200186u64);
    acc = acc.wrapping_mul(11667u64);
    acc = acc ^ 404576u64;
    acc = acc.rotate_left(27);
    acc = acc ^ 476322u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(59843u64);
    acc = acc.wrapping_mul(9955u64);
    acc = acc.wrapping_mul(40043u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc ^ 757930u64;
    acc = acc.wrapping_mul(55127u64);
    acc = acc ^ 637197u64;
    acc = acc.wrapping_add(823814u64);
    acc = acc.wrapping_add(928633u64);
    acc = acc ^ 150468u64;
    acc = acc.wrapping_add(774112u64);
    acc = acc.wrapping_mul(40853u64);
    acc = acc ^ 498223u64;
    acc = acc.wrapping_add(37941u64);
    acc = acc.wrapping_mul(35545u64);
    acc = acc ^ 753133u64;
    acc = acc.wrapping_mul(59363u64);
    acc = acc ^ 203001u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 943471u64;
    acc = acc ^ 813240u64;
    acc = acc.wrapping_add(209729u64);
    acc = acc.wrapping_add(218091u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 975753u64;
    acc = acc.wrapping_mul(9447u64);
    acc = acc.wrapping_mul(44173u64);
    acc = acc.wrapping_add(173901u64);
    acc = acc.wrapping_mul(60805u64);
    acc = acc.wrapping_add(190480u64);
    acc = acc ^ 459486u64;
    acc = acc.wrapping_mul(223u64);
    acc = acc ^ 119647u64;
    acc = acc ^ 823353u64;
    acc = acc.wrapping_add(586262u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 989288u64;
    acc = acc.wrapping_mul(28731u64);
    acc = acc.wrapping_mul(46467u64);
    acc = acc.wrapping_mul(28635u64);
    acc = acc ^ 343489u64;
    acc = acc ^ 782703u64;
    acc = acc ^ 836446u64;
    acc = acc.wrapping_mul(58363u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(569388u64);
    acc = acc ^ 339647u64;
    acc = acc.wrapping_add(814348u64);
    acc = acc.rotate_left(19);
    acc = acc ^ 861073u64;
    acc = acc ^ 519708u64;
    acc = acc.wrapping_add(257983u64);
    acc = acc.wrapping_mul(37759u64);
    acc = acc ^ 232609u64;
    acc = acc.wrapping_mul(62869u64);
    acc = acc ^ 78011u64;
    acc = acc.rotate_left(13);
    acc = acc ^ 334186u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(5909u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(375000u64);
    acc = acc.wrapping_add(210857u64);
    acc = acc.wrapping_mul(50041u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(58329u64);
    acc = acc ^ 256570u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 139893u64;
    acc = acc.wrapping_mul(49427u64);
    acc = acc.wrapping_mul(14701u64);
    acc = acc ^ 821193u64;
    acc = acc.wrapping_mul(42553u64);
    acc = acc.wrapping_add(22006u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc ^ 440860u64;
    acc = acc.wrapping_add(319710u64);
    acc = acc ^ 388506u64;
    acc = acc ^ 486502u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(13353u64);
    acc = acc.wrapping_mul(2579u64);
    acc = acc.wrapping_add(234441u64);
    acc = acc.wrapping_mul(69u64);
    acc = acc ^ 309172u64;
    acc = acc.wrapping_add(285806u64);
    acc = acc ^ 322879u64;
    acc = acc ^ 289918u64;
    acc = acc.wrapping_mul(16037u64);
    acc = acc ^ 545358u64;
    acc = acc ^ 520928u64;
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(418251u64);
    acc = acc.rotate_left(28);
    acc = acc ^ 687281u64;
    acc = acc ^ 795537u64;
    acc = acc.wrapping_mul(16689u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(23769u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(14669u64);
    acc = acc.wrapping_add(299419u64);
    acc = acc.wrapping_mul(22855u64);
    acc = acc.wrapping_add(243512u64);
    acc = acc.wrapping_add(12250u64);
    acc = acc.wrapping_mul(3571u64);
    acc = acc.wrapping_mul(15979u64);
    acc = acc.wrapping_mul(46825u64);
    acc = acc.wrapping_add(37811u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(417368u64);
    acc = acc.wrapping_mul(48527u64);
    acc = acc.wrapping_mul(57251u64);
    acc = acc ^ 315193u64;
    acc = acc.wrapping_mul(12191u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(52649u64);
    acc = acc.wrapping_add(334405u64);
    acc = acc.wrapping_mul(56623u64);
    acc = acc.wrapping_add(729871u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(309056u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(466457u64);
    acc = acc.wrapping_mul(56363u64);
    acc = acc ^ 780838u64;
    acc = acc.wrapping_mul(34319u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(23003u64);
    acc = acc ^ 917142u64;
    acc = acc.wrapping_add(371398u64);
    acc = acc.wrapping_mul(54441u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(26107u64);
    acc = acc.wrapping_add(675014u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(14825u64);
    acc = acc.wrapping_add(769207u64);
    acc = acc ^ 641821u64;
    acc = acc.wrapping_mul(41663u64);
    acc = acc ^ 877590u64;
    acc = acc.wrapping_add(349073u64);
    acc = acc ^ 765335u64;
    acc = acc.wrapping_add(552338u64);
    acc = acc ^ 182538u64;
    acc = acc.wrapping_add(110180u64);
    acc = acc ^ 758216u64;
    acc = acc.wrapping_mul(2419u64);
    acc = acc ^ 841580u64;
    acc = acc.wrapping_mul(4655u64);
    acc = acc.wrapping_mul(19597u64);
    acc = acc ^ 69771u64;
    acc = acc.wrapping_add(724728u64);
    acc = acc.wrapping_add(556454u64);
    acc = acc ^ 860286u64;
    acc = acc.wrapping_add(370932u64);
    acc = acc ^ 746768u64;
    acc = acc.rotate_left(25);
    acc = acc ^ 767843u64;
    acc = acc.wrapping_mul(51011u64);
    acc = acc.rotate_left(15);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(331u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 748u64, b: 792i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(272u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_18 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_18: SPacked_18 = SPacked_18 { f0: 102, f1: 139, f2: 176, f3: 213 };
const SCONST_18_ALT: SPacked_18 = SPacked_18 { f0: 150, f1: 203, f2: 0, f3: 53 };

    pub fn run() {
        assert!(!(617964293u64) == (617964293u64) ^ u64::MAX);
        debug_assert!((4079415103u32).rotate_left(10).rotate_right(10) == 4079415103u32);
        debug_assert!((1494471425u64).wrapping_mul(1) == 1494471425u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(407607195u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1883664812u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(908280789u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_18 } else { SCONST_18_ALT };
    if std::hint::black_box(false) {
        s.f0 = s.f0.wrapping_add(31);
    } else {
        s.f1 = s.f1.wrapping_add(47);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_18:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_18.f2;
    print!("case_18:"); println!("{}", c);

        }
        assert!((1967205352u64) ^ (1967205352u64) == 0u64);
        debug_assert!((2443592762u32).rotate_left(27).rotate_right(27) == 2443592762u32);
        assert!(!(2015531551u64) == (2015531551u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1265226919u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1424611893u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2972035854u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_19 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(8400u64).rotate_left(2) ^ 466u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(9423u64).rotate_left(4) ^ 6327u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(8953u64).rotate_left(19) ^ 4389u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(6330u64).rotate_left(3) ^ 4373u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(7893u64);
    acc = acc.wrapping_mul(61205u64);
    acc = acc ^ 285335u64;
    acc = acc.wrapping_mul(23691u64);
    acc = acc.wrapping_add(301667u64);
    acc = acc ^ 59752u64;
    acc = acc ^ 98742u64;
    acc = acc ^ 813403u64;
    acc = acc.wrapping_add(439044u64);
    acc = acc.wrapping_add(347682u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(58693u64);
    acc = acc.wrapping_mul(65517u64);
    acc = acc ^ 245881u64;
    acc = acc ^ 482742u64;
    acc = acc ^ 108718u64;
    acc = acc.wrapping_mul(30117u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(594601u64);
    acc = acc.wrapping_add(688968u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc ^ 725704u64;
    acc = acc.wrapping_add(872773u64);
    acc = acc.wrapping_add(211082u64);
    acc = acc ^ 149223u64;
    acc = acc.wrapping_add(832999u64);
    acc = acc ^ 279590u64;
    acc = acc ^ 753645u64;
    acc = acc ^ 767851u64;
    acc = acc.wrapping_add(410397u64);
    acc = acc.wrapping_add(354017u64);
    acc = acc ^ 693913u64;
    acc = acc.wrapping_add(517189u64);
    acc = acc.wrapping_mul(4131u64);
    acc = acc.wrapping_mul(61671u64);
    acc = acc ^ 131730u64;
    acc = acc.wrapping_mul(32021u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 606494u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(353102u64);
    acc = acc.wrapping_add(963265u64);
    acc = acc ^ 427406u64;
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(17339u64);
    acc = acc ^ 667259u64;
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc ^ 295478u64;
    acc = acc ^ 139161u64;
    acc = acc.wrapping_add(483625u64);
    acc = acc ^ 466041u64;
    acc = acc.wrapping_add(735715u64);
    acc = acc ^ 382178u64;
    acc = acc.wrapping_mul(57479u64);
    acc = acc ^ 318395u64;
    acc = acc.wrapping_mul(52383u64);
    acc = acc.wrapping_mul(16147u64);
    acc = acc ^ 886008u64;
    acc = acc.rotate_left(11);
    acc = acc ^ 143635u64;
    acc = acc.rotate_left(25);
    acc = acc ^ 395820u64;
    acc = acc.wrapping_mul(41233u64);
    acc = acc ^ 689676u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(394875u64);
    acc = acc ^ 662982u64;
    acc = acc.wrapping_mul(7923u64);
    acc = acc ^ 791487u64;
    acc = acc.wrapping_mul(449u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(14971u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(761089u64);
    acc = acc ^ 17444u64;
    acc = acc ^ 818493u64;
    acc = acc.rotate_left(20);
    acc = acc ^ 294190u64;
    acc = acc ^ 857140u64;
    acc = acc.wrapping_mul(26249u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(839522u64);
    acc = acc.wrapping_add(453347u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(44871u64);
    acc = acc.wrapping_mul(8131u64);
    acc = acc ^ 941719u64;
    acc = acc ^ 817132u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 77021u64;
    acc = acc.wrapping_add(943094u64);
    acc = acc ^ 253392u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(957591u64);
    acc = acc.wrapping_mul(34903u64);
    acc = acc.wrapping_add(227617u64);
    acc = acc ^ 735527u64;
    acc = acc.wrapping_mul(2501u64);
    acc = acc.wrapping_add(117678u64);
    acc = acc ^ 480171u64;
    acc = acc.wrapping_mul(41311u64);
    acc = acc ^ 361218u64;
    acc = acc.wrapping_mul(25357u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(42227u64);
    acc = acc.wrapping_add(449583u64);
    acc = acc.wrapping_mul(9935u64);
    acc = acc.wrapping_add(613720u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(946491u64);
    acc = acc.wrapping_add(447187u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(397943u64);
    acc = acc.wrapping_add(823717u64);
    acc = acc ^ 938367u64;
    acc = acc ^ 276551u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(4355u64);
    acc = acc.wrapping_add(682408u64);
    acc = acc.wrapping_add(849150u64);
    acc = acc ^ 147924u64;
    acc = acc.wrapping_add(245510u64);
    acc = acc ^ 626757u64;
    acc = acc ^ 898553u64;
    acc = acc.wrapping_add(240597u64);
    acc = acc.wrapping_add(948420u64);
    acc = acc ^ 696189u64;
    acc = acc.wrapping_mul(47535u64);
    acc = acc.wrapping_add(994572u64);
    acc = acc.wrapping_add(276124u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(13715u64);
    acc = acc ^ 161979u64;
    acc = acc.wrapping_mul(27677u64);
    acc = acc.wrapping_mul(59807u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 986820u64;
    acc = acc ^ 160367u64;
    acc = acc.wrapping_mul(3239u64);
    acc = acc.wrapping_add(817853u64);
    acc = acc.wrapping_mul(53313u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(29727u64);
    acc = acc.wrapping_mul(29415u64);
    acc = acc.wrapping_add(464989u64);
    acc = acc ^ 793164u64;
    acc = acc.wrapping_add(776987u64);
    acc = acc.wrapping_add(645768u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(61685u64);
    acc = acc.wrapping_mul(6285u64);
    acc = acc.wrapping_mul(15859u64);
    acc = acc.wrapping_mul(28045u64);
    acc = acc.wrapping_add(576158u64);
    acc = acc.wrapping_add(526728u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(42487u64);
    acc = acc.wrapping_add(82698u64);
    acc = acc ^ 478473u64;
    acc = acc ^ 338694u64;
    acc = acc.wrapping_mul(29601u64);
    acc = acc.wrapping_mul(3783u64);
    acc = acc.wrapping_mul(25971u64);
    acc = acc.wrapping_add(408820u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(6263u64);
    acc = acc.wrapping_add(62971u64);
    acc = acc.wrapping_add(361681u64);
    acc = acc.wrapping_mul(13849u64);
    acc = acc.wrapping_add(151521u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(361u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(2u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(746u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_19 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_19: SPacked_19 = SPacked_19 { f0: 109, f1: 146, f2: 183, f3: 220 };
const SCONST_19_ALT: SPacked_19 = SPacked_19 { f0: 161, f1: 214, f2: 11, f3: 64 };

    pub fn run() {
        assert!(!(2285796511u64) == (2285796511u64) ^ u64::MAX);
        assert!((690150184u64) ^ (690150184u64) == 0u64);
        debug_assert!((1109437331u32) | (2520294607u32) | (1109437331u32) == (1109437331u32) | (2520294607u32));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2772854925u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1227524598u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1097543422u64)));
        {

    let mut s = if std::hint::black_box(true) { SCONST_19 } else { SCONST_19_ALT };
    if std::hint::black_box(false) {
        s.f1 = s.f1.wrapping_add(32);
    } else {
        s.f2 = s.f2.wrapping_add(48);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_19:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_19.f3;
    print!("case_19:"); println!("{}", c);

        }
        debug_assert!((125659433u64).wrapping_mul(1) == 125659433u64);
        assert!((1975298597u64).wrapping_add(1975298597u64).wrapping_sub(1975298597u64) == 1975298597u64);
        assert!(!(2073751550u64) == (2073751550u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1448043434u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2965063245u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3053608869u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_20 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(3273u64).rotate_left(2) ^ 125u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(288u64).rotate_left(10) ^ 3574u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(5161u64).rotate_left(15) ^ 696u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(2545u64).rotate_left(27) ^ 6257u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(407541u64);
    acc = acc.wrapping_add(965860u64);
    acc = acc.wrapping_add(24415u64);
    acc = acc ^ 587155u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(52685u64);
    acc = acc.wrapping_mul(21377u64);
    acc = acc ^ 932841u64;
    acc = acc.wrapping_mul(54351u64);
    acc = acc ^ 701368u64;
    acc = acc.wrapping_mul(26441u64);
    acc = acc ^ 664251u64;
    acc = acc ^ 41037u64;
    acc = acc ^ 228704u64;
    acc = acc.wrapping_add(115433u64);
    acc = acc ^ 520385u64;
    acc = acc.wrapping_mul(18939u64);
    acc = acc ^ 961113u64;
    acc = acc.rotate_left(13);
    acc = acc ^ 226909u64;
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 819994u64;
    acc = acc.wrapping_add(334966u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc ^ 960495u64;
    acc = acc.wrapping_add(655101u64);
    acc = acc.wrapping_mul(14991u64);
    acc = acc.wrapping_mul(57283u64);
    acc = acc ^ 513814u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(61179u64);
    acc = acc.wrapping_mul(61047u64);
    acc = acc.wrapping_mul(54549u64);
    acc = acc ^ 247999u64;
    acc = acc ^ 341371u64;
    acc = acc.wrapping_mul(4627u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(40357u64);
    acc = acc.wrapping_add(320746u64);
    acc = acc.wrapping_add(486500u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(368778u64);
    acc = acc.wrapping_add(894674u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(738008u64);
    acc = acc.wrapping_add(418707u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(11087u64);
    acc = acc ^ 865741u64;
    acc = acc.wrapping_mul(21329u64);
    acc = acc.wrapping_mul(51563u64);
    acc = acc.wrapping_mul(15157u64);
    acc = acc ^ 494079u64;
    acc = acc.wrapping_mul(21017u64);
    acc = acc.wrapping_add(94841u64);
    acc = acc ^ 168658u64;
    acc = acc.wrapping_add(965541u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 773072u64;
    acc = acc.wrapping_add(463632u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(550357u64);
    acc = acc ^ 841353u64;
    acc = acc.wrapping_mul(3721u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 757345u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(6869u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(13499u64);
    acc = acc ^ 783929u64;
    acc = acc ^ 314511u64;
    acc = acc.wrapping_add(425302u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(1349u64);
    acc = acc.wrapping_add(36589u64);
    acc = acc.wrapping_add(185748u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(18457u64);
    acc = acc ^ 586760u64;
    acc = acc.wrapping_mul(39631u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(13063u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 586019u64;
    acc = acc ^ 362124u64;
    acc = acc.rotate_left(29);
    acc = acc ^ 98381u64;
    acc = acc ^ 486850u64;
    acc = acc.wrapping_mul(39251u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(63091u64);
    acc = acc.wrapping_mul(48609u64);
    acc = acc.wrapping_add(928783u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 2131u64;
    acc = acc.wrapping_mul(44303u64);
    acc = acc ^ 129880u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(44253u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(40133u64);
    acc = acc.wrapping_mul(16959u64);
    acc = acc ^ 26514u64;
    acc = acc.rotate_left(31);
    acc = acc ^ 186069u64;
    acc = acc.wrapping_add(358144u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(27141u64);
    acc = acc ^ 819874u64;
    acc = acc ^ 966501u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 787864u64;
    acc = acc.wrapping_add(481643u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(876902u64);
    acc = acc.wrapping_mul(57581u64);
    acc = acc.wrapping_mul(49347u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(35901u64);
    acc = acc.wrapping_mul(2273u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(264407u64);
    acc = acc.wrapping_add(20522u64);
    acc = acc ^ 324518u64;
    acc = acc.wrapping_add(300314u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 812293u64;
    acc = acc.wrapping_add(466971u64);
    acc = acc.wrapping_mul(11977u64);
    acc = acc ^ 763734u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(24528u64);
    acc = acc ^ 461140u64;
    acc = acc.wrapping_add(616018u64);
    acc = acc ^ 517729u64;
    acc = acc.wrapping_add(173724u64);
    acc = acc ^ 531330u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(740065u64);
    acc = acc.wrapping_mul(35401u64);
    acc = acc ^ 691993u64;
    acc = acc.wrapping_mul(30265u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(509679u64);
    acc = acc ^ 939905u64;
    acc = acc ^ 971451u64;
    acc = acc.wrapping_mul(38277u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(35573u64);
    acc = acc.wrapping_mul(12411u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(18075u64);
    acc = acc.wrapping_add(641189u64);
    acc = acc.wrapping_add(427168u64);
    acc = acc.wrapping_mul(60383u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(924236u64);
    acc = acc ^ 749343u64;
    acc = acc.wrapping_add(862775u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 777u64, b: -148i32, c: true } } }

enum NoiseType1 { V0(u64), V1(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(424u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(756u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_20 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_20: SPacked_20 = SPacked_20 { f0: 116, f1: 153, f2: 190, f3: 227 };
const SCONST_20_ALT: SPacked_20 = SPacked_20 { f0: 172, f1: 225, f2: 22, f3: 75 };

    pub fn run() {
        assert!(!(3936323549u64) == (3936323549u64) ^ u64::MAX);
        assert!(!(16973997u64) == (16973997u64) ^ u64::MAX);
        assert!(!(1905989663u64) == (1905989663u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(4017879331u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1415471783u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(448863334u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_20 } else { SCONST_20_ALT };
    if std::hint::black_box(true) {
        s.f0 = s.f0.wrapping_add(33);
    } else {
        s.f1 = s.f1.wrapping_add(49);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_20:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_20.f0;
    print!("case_20:"); println!("{}", c);

        }
        assert!(!(978841579u64) == (978841579u64) ^ u64::MAX);
        debug_assert!((2875955464u32).rotate_left(28).rotate_right(28) == 2875955464u32);
        debug_assert!((4088225550u32) | (3488777873u32) | (4088225550u32) == (4088225550u32) | (3488777873u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1099249000u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2759962576u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2660692558u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_21 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(8181u64).rotate_left(14) ^ 7732u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(2574u64).rotate_left(31) ^ 7842u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(9939u64).rotate_left(19) ^ 2560u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(1980u64).rotate_left(23) ^ 22u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(1997u64);
    acc = acc.wrapping_mul(3913u64);
    acc = acc.wrapping_add(178192u64);
    acc = acc.wrapping_mul(31627u64);
    acc = acc.wrapping_mul(2413u64);
    acc = acc.wrapping_mul(25299u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(34417u64);
    acc = acc.wrapping_add(730849u64);
    acc = acc ^ 2746u64;
    acc = acc.rotate_left(12);
    acc = acc ^ 148968u64;
    acc = acc.wrapping_mul(25051u64);
    acc = acc.wrapping_add(86099u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(46717u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(29907u64);
    acc = acc.wrapping_mul(41457u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(56331u64);
    acc = acc.wrapping_add(93018u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 614289u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(30779u64);
    acc = acc.wrapping_add(269276u64);
    acc = acc.wrapping_add(563114u64);
    acc = acc ^ 26662u64;
    acc = acc ^ 318388u64;
    acc = acc ^ 113807u64;
    acc = acc.wrapping_add(539165u64);
    acc = acc ^ 941126u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(82784u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(8133u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(58573u64);
    acc = acc.wrapping_add(174788u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(35101u64);
    acc = acc.wrapping_mul(27461u64);
    acc = acc ^ 765800u64;
    acc = acc.wrapping_mul(3523u64);
    acc = acc.wrapping_add(58723u64);
    acc = acc.wrapping_add(794341u64);
    acc = acc.wrapping_add(174598u64);
    acc = acc.wrapping_mul(6233u64);
    acc = acc.wrapping_mul(23629u64);
    acc = acc.wrapping_add(650941u64);
    acc = acc.wrapping_add(469415u64);
    acc = acc ^ 268256u64;
    acc = acc.wrapping_mul(45267u64);
    acc = acc.wrapping_add(702394u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 129791u64;
    acc = acc.wrapping_add(810146u64);
    acc = acc.wrapping_add(206286u64);
    acc = acc.wrapping_mul(57797u64);
    acc = acc.rotate_left(28);
    acc = acc ^ 732160u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(58571u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(401067u64);
    acc = acc.wrapping_add(974267u64);
    acc = acc.wrapping_mul(5083u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(803263u64);
    acc = acc ^ 583850u64;
    acc = acc ^ 593198u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(105862u64);
    acc = acc.wrapping_add(321611u64);
    acc = acc ^ 726111u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(676886u64);
    acc = acc.wrapping_add(575531u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 443330u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(299277u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(4713u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(26249u64);
    acc = acc.wrapping_mul(21129u64);
    acc = acc.wrapping_add(670918u64);
    acc = acc ^ 916613u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(8231u64);
    acc = acc.wrapping_mul(2575u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(696520u64);
    acc = acc ^ 523964u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(690627u64);
    acc = acc.wrapping_mul(57991u64);
    acc = acc.wrapping_mul(23523u64);
    acc = acc.wrapping_mul(1291u64);
    acc = acc.wrapping_mul(38169u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(515604u64);
    acc = acc.wrapping_mul(31253u64);
    acc = acc ^ 325924u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(395660u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(602838u64);
    acc = acc ^ 118314u64;
    acc = acc ^ 444596u64;
    acc = acc.wrapping_mul(33809u64);
    acc = acc ^ 432831u64;
    acc = acc.wrapping_add(304504u64);
    acc = acc ^ 971464u64;
    acc = acc ^ 746758u64;
    acc = acc ^ 421120u64;
    acc = acc ^ 280373u64;
    acc = acc.wrapping_mul(32691u64);
    acc = acc ^ 333370u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(63845u64);
    acc = acc.wrapping_mul(54133u64);
    acc = acc.wrapping_mul(35061u64);
    acc = acc.wrapping_add(820993u64);
    acc = acc.wrapping_add(212286u64);
    acc = acc.wrapping_add(793315u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(37363u64);
    acc = acc ^ 3574u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(53771u64);
    acc = acc ^ 993078u64;
    acc = acc.wrapping_add(230017u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 480221u64;
    acc = acc.wrapping_mul(13855u64);
    acc = acc.wrapping_mul(60975u64);
    acc = acc.wrapping_add(365593u64);
    acc = acc.wrapping_mul(54649u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(141239u64);
    acc = acc.wrapping_mul(36961u64);
    acc = acc ^ 222020u64;
    acc = acc.wrapping_add(386097u64);
    acc = acc ^ 985807u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(36695u64);
    acc = acc.wrapping_mul(5003u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(27601u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(965851u64);
    acc = acc.wrapping_add(760664u64);
    acc = acc ^ 428913u64;
    acc = acc ^ 43930u64;
    acc = acc ^ 806580u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 341015u64;
    acc = acc.wrapping_mul(59883u64);
    acc = acc.wrapping_mul(54589u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(906708u64);
    acc = acc.wrapping_add(708730u64);
    acc = acc.wrapping_mul(65233u64);
    acc = acc.wrapping_mul(53317u64);
    acc = acc.wrapping_mul(54665u64);
    acc = acc ^ 212741u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(78268u64);
    acc = acc ^ 108673u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(22779u64);
    acc = acc ^ 26315u64;
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(656u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 573u64, b: 3i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V2(518u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_21 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_21: SPacked_21 = SPacked_21 { f0: 123, f1: 160, f2: 197, f3: 234 };
const SCONST_21_ALT: SPacked_21 = SPacked_21 { f0: 183, f1: 236, f2: 33, f3: 86 };

    pub fn run() {
        assert!(!(518136245u64) == (518136245u64) ^ u64::MAX);
        assert!((498205998u64).wrapping_add(498205998u64).wrapping_sub(498205998u64) == 498205998u64);
        assert!((2668920176u64).wrapping_add(2668920176u64).wrapping_sub(2668920176u64) == 2668920176u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1302989942u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1490871511u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1857433411u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_21 } else { SCONST_21_ALT };
    if std::hint::black_box(true) {
        s.f1 = s.f1.wrapping_add(34);
    } else {
        s.f2 = s.f2.wrapping_add(50);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_21:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_21.f1;
    print!("case_21:"); println!("{}", c);

        }
        debug_assert!((1559688927u64).wrapping_mul(1) == 1559688927u64);
        assert!((1029369422u64) ^ (1029369422u64) == 0u64);
        debug_assert!((3499681179u64).wrapping_mul(1) == 3499681179u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1550490141u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3368084679u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2301877529u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_22 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(3895u64).rotate_left(12) ^ 7172u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(7687u64).rotate_left(13) ^ 3360u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(8637u64).rotate_left(21) ^ 3709u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(1484u64).rotate_left(6) ^ 3136u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 837613u64;
    acc = acc ^ 44414u64;
    acc = acc.wrapping_mul(30581u64);
    acc = acc ^ 410648u64;
    acc = acc ^ 657884u64;
    acc = acc ^ 58706u64;
    acc = acc ^ 620178u64;
    acc = acc ^ 462930u64;
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(166232u64);
    acc = acc.wrapping_add(565787u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(613959u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc ^ 757526u64;
    acc = acc.wrapping_mul(63839u64);
    acc = acc.wrapping_add(272785u64);
    acc = acc ^ 385628u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(698203u64);
    acc = acc.wrapping_mul(57717u64);
    acc = acc.wrapping_add(486393u64);
    acc = acc.wrapping_mul(24979u64);
    acc = acc.wrapping_mul(51207u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(725972u64);
    acc = acc ^ 685291u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(627815u64);
    acc = acc ^ 687350u64;
    acc = acc ^ 467623u64;
    acc = acc.wrapping_mul(1625u64);
    acc = acc.wrapping_add(255628u64);
    acc = acc.wrapping_add(178205u64);
    acc = acc.wrapping_mul(47323u64);
    acc = acc ^ 880642u64;
    acc = acc.wrapping_mul(33179u64);
    acc = acc ^ 6853u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(28879u64);
    acc = acc.wrapping_add(8738u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 41032u64;
    acc = acc.wrapping_mul(30709u64);
    acc = acc ^ 990259u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(16639u64);
    acc = acc.wrapping_mul(45239u64);
    acc = acc.wrapping_add(512082u64);
    acc = acc.wrapping_add(992750u64);
    acc = acc.wrapping_mul(61469u64);
    acc = acc.wrapping_mul(2453u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(82539u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(62537u64);
    acc = acc ^ 989743u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(433304u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 788970u64;
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc ^ 945294u64;
    acc = acc.wrapping_mul(7685u64);
    acc = acc.wrapping_mul(22997u64);
    acc = acc ^ 507149u64;
    acc = acc.wrapping_mul(20111u64);
    acc = acc ^ 939380u64;
    acc = acc ^ 886037u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 306376u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(13051u64);
    acc = acc.wrapping_add(595688u64);
    acc = acc.wrapping_mul(28791u64);
    acc = acc.wrapping_add(857964u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(445656u64);
    acc = acc.wrapping_mul(15771u64);
    acc = acc.wrapping_add(923439u64);
    acc = acc.wrapping_add(723934u64);
    acc = acc.wrapping_mul(54319u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc ^ 414474u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(425084u64);
    acc = acc.wrapping_mul(9365u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(569592u64);
    acc = acc.wrapping_mul(5007u64);
    acc = acc.wrapping_add(271042u64);
    acc = acc ^ 264449u64;
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc ^ 238775u64;
    acc = acc.wrapping_mul(3429u64);
    acc = acc ^ 66677u64;
    acc = acc.wrapping_mul(31913u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(537813u64);
    acc = acc.wrapping_mul(56027u64);
    acc = acc ^ 471237u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 485355u64;
    acc = acc ^ 553220u64;
    acc = acc.wrapping_add(117918u64);
    acc = acc.wrapping_mul(41027u64);
    acc = acc ^ 201182u64;
    acc = acc.rotate_left(29);
    acc = acc ^ 651076u64;
    acc = acc.wrapping_add(144020u64);
    acc = acc.wrapping_mul(50615u64);
    acc = acc ^ 519970u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(48929u64);
    acc = acc.wrapping_mul(26313u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 796360u64;
    acc = acc ^ 835436u64;
    acc = acc.wrapping_add(66013u64);
    acc = acc.wrapping_add(20263u64);
    acc = acc ^ 328245u64;
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(281727u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(37251u64);
    acc = acc.wrapping_add(716695u64);
    acc = acc.wrapping_add(192748u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(152243u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(329u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 311u64, b: 625i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(110u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_22 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_22: SPacked_22 = SPacked_22 { f0: 130, f1: 167, f2: 204, f3: 241 };
const SCONST_22_ALT: SPacked_22 = SPacked_22 { f0: 194, f1: 247, f2: 44, f3: 97 };

    pub fn run() {
        debug_assert!((3071841726u64).wrapping_mul(1) == 3071841726u64);
        debug_assert!((1594323914u32) | (2861171982u32) | (1594323914u32) == (1594323914u32) | (2861171982u32));
        assert!(!(2518118878u64) == (2518118878u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(749736251u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3372590733u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(561025893u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_22 } else { SCONST_22_ALT };
    if std::hint::black_box(false) {
        s.f0 = s.f0.wrapping_add(35);
    } else {
        s.f1 = s.f1.wrapping_add(51);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_22:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_22.f2;
    print!("case_22:"); println!("{}", c);

        }
        assert!((842364025u64) ^ (842364025u64) == 0u64);
        assert!(!(277967771u64) == (277967771u64) ^ u64::MAX);
        debug_assert!((2694075834u32).rotate_left(4).rotate_right(4) == 2694075834u32);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3119109364u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(328349719u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1030460988u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_23 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(8566u64).rotate_left(23) ^ 828u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(2925u64).rotate_left(19) ^ 6094u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(3575u64).rotate_left(29) ^ 3903u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(5020u64).rotate_left(23) ^ 22u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(447854u64);
    acc = acc.wrapping_add(631420u64);
    acc = acc.wrapping_add(531552u64);
    acc = acc.wrapping_add(896115u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(51989u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(409620u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(360065u64);
    acc = acc.wrapping_add(848493u64);
    acc = acc.wrapping_add(488258u64);
    acc = acc ^ 574675u64;
    acc = acc.wrapping_add(892592u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(730212u64);
    acc = acc ^ 480526u64;
    acc = acc ^ 936460u64;
    acc = acc.wrapping_add(602182u64);
    acc = acc.wrapping_add(949465u64);
    acc = acc.wrapping_add(614406u64);
    acc = acc.wrapping_add(781917u64);
    acc = acc ^ 925788u64;
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc ^ 111497u64;
    acc = acc ^ 659593u64;
    acc = acc.wrapping_mul(65389u64);
    acc = acc ^ 499025u64;
    acc = acc.wrapping_mul(19919u64);
    acc = acc.wrapping_mul(25245u64);
    acc = acc ^ 931868u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(30493u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(606278u64);
    acc = acc ^ 362142u64;
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(577580u64);
    acc = acc.wrapping_add(456002u64);
    acc = acc.wrapping_mul(22839u64);
    acc = acc ^ 368995u64;
    acc = acc ^ 838456u64;
    acc = acc.rotate_left(23);
    acc = acc ^ 76519u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 529879u64;
    acc = acc ^ 699955u64;
    acc = acc.wrapping_mul(28593u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(52225u64);
    acc = acc.wrapping_mul(35511u64);
    acc = acc.wrapping_mul(24915u64);
    acc = acc.wrapping_mul(42241u64);
    acc = acc.wrapping_mul(22679u64);
    acc = acc.wrapping_mul(8369u64);
    acc = acc ^ 544128u64;
    acc = acc.wrapping_mul(33001u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(41867u64);
    acc = acc.wrapping_add(9257u64);
    acc = acc.wrapping_mul(38129u64);
    acc = acc.wrapping_mul(15289u64);
    acc = acc ^ 689993u64;
    acc = acc ^ 589108u64;
    acc = acc.wrapping_mul(65305u64);
    acc = acc ^ 730503u64;
    acc = acc ^ 42000u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc ^ 256812u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(856077u64);
    acc = acc.wrapping_mul(16083u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(10913u64);
    acc = acc.wrapping_mul(50327u64);
    acc = acc ^ 642438u64;
    acc = acc.wrapping_mul(57803u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 150394u64;
    acc = acc.wrapping_mul(17933u64);
    acc = acc.wrapping_mul(50547u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(62489u64);
    acc = acc.wrapping_mul(31001u64);
    acc = acc ^ 689026u64;
    acc = acc.wrapping_add(855436u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(500867u64);
    acc = acc.wrapping_add(665531u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(7861u64);
    acc = acc ^ 561652u64;
    acc = acc.wrapping_mul(16649u64);
    acc = acc.wrapping_mul(61075u64);
    acc = acc.wrapping_add(631067u64);
    acc = acc ^ 81110u64;
    acc = acc.wrapping_add(238164u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 186607u64;
    acc = acc ^ 951716u64;
    acc = acc ^ 91884u64;
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(52771u64);
    acc = acc.wrapping_mul(34765u64);
    acc = acc.wrapping_add(410408u64);
    acc = acc.wrapping_mul(44425u64);
    acc = acc ^ 359557u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(215070u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(996163u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 312089u64;
    acc = acc.wrapping_add(625035u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(382548u64);
    acc = acc ^ 15273u64;
    acc = acc ^ 2290u64;
    acc = acc.wrapping_mul(44811u64);
    acc = acc.wrapping_mul(25625u64);
    acc = acc.wrapping_add(965649u64);
    acc = acc.wrapping_mul(27085u64);
    acc = acc.wrapping_mul(61647u64);
    acc = acc.wrapping_add(904328u64);
    acc = acc ^ 128978u64;
    acc = acc.rotate_left(27);
    acc = acc ^ 22897u64;
    acc = acc.wrapping_mul(53013u64);
    acc = acc ^ 283873u64;
    acc = acc.wrapping_mul(49037u64);
    acc = acc ^ 357649u64;
    acc = acc ^ 912222u64;
    acc = acc.wrapping_mul(26007u64);
    acc = acc.wrapping_add(484258u64);
    acc = acc ^ 765327u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(32419u64);
    acc = acc.wrapping_add(100825u64);
    acc = acc.wrapping_add(956714u64);
    acc = acc.wrapping_mul(22135u64);
    acc = acc.wrapping_mul(53269u64);
    acc = acc ^ 10380u64;
    acc = acc.wrapping_mul(5u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 701725u64;
    acc = acc.wrapping_mul(11295u64);
    acc = acc.wrapping_add(355604u64);
    acc = acc ^ 868325u64;
    acc = acc ^ 958605u64;
    acc = acc.rotate_left(9);
    acc = acc ^ 754774u64;
    acc = acc.wrapping_add(905703u64);
    acc = acc ^ 968596u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(875669u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(9u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 810u64, b: 376i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V3(477u64) } }


#[repr(packed)]
#[derive(Clone, Copy)]
struct SPacked_23 {
    f0: u8,
    f1: u8,
    f2: u8,
    f3: u8,
}

const SCONST_23: SPacked_23 = SPacked_23 { f0: 137, f1: 174, f2: 211, f3: 248 };
const SCONST_23_ALT: SPacked_23 = SPacked_23 { f0: 205, f1: 2, f2: 55, f3: 108 };

    pub fn run() {
        debug_assert!((170436764u64).wrapping_mul(1) == 170436764u64);
        debug_assert!((1409064859u64).wrapping_mul(1) == 1409064859u64);
        assert!((194107396u64).wrapping_add(194107396u64).wrapping_sub(194107396u64) == 194107396u64);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2042461210u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3250521509u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3313560673u64)));
        {

    let mut s = if std::hint::black_box(false) { SCONST_23 } else { SCONST_23_ALT };
    if std::hint::black_box(false) {
        s.f1 = s.f1.wrapping_add(36);
    } else {
        s.f2 = s.f2.wrapping_add(52);
    }
    let v0 = s.f0;
    let v1 = s.f1;
    let v2 = s.f2;
    let v3 = s.f3;
    print!("case_23:"); println!("{} {} {} {}", v0, v1, v2, v3);

    let c = SCONST_23.f3;
    print!("case_23:"); println!("{}", c);

        }
        debug_assert!((1218094242u32).rotate_left(5).rotate_right(5) == 1218094242u32);
        assert!((1734189637u64) ^ (1734189637u64) == 0u64);
        assert!(!(575437664u64) == (575437664u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3333033083u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3429469140u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1860738041u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_24 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(1247u64).rotate_left(14) ^ 7887u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(5840u64).rotate_left(10) ^ 6240u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(57u64).rotate_left(16) ^ 8261u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(7173u64).rotate_left(2) ^ 6821u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 906111u64;
    acc = acc.wrapping_add(421132u64);
    acc = acc.wrapping_mul(15389u64);
    acc = acc.wrapping_add(116239u64);
    acc = acc.wrapping_mul(26019u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(705820u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(29713u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc ^ 890376u64;
    acc = acc.wrapping_mul(33829u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(7109u64);
    acc = acc ^ 29087u64;
    acc = acc ^ 224062u64;
    acc = acc ^ 372727u64;
    acc = acc.wrapping_add(15534u64);
    acc = acc ^ 753257u64;
    acc = acc.wrapping_mul(38737u64);
    acc = acc.wrapping_mul(3371u64);
    acc = acc.wrapping_mul(3863u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(338525u64);
    acc = acc.wrapping_add(748198u64);
    acc = acc.rotate_left(31);
    acc = acc ^ 419559u64;
    acc = acc.wrapping_add(84776u64);
    acc = acc.wrapping_add(305843u64);
    acc = acc ^ 213012u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(979068u64);
    acc = acc.wrapping_mul(60791u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(39637u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(49105u64);
    acc = acc.wrapping_mul(23991u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 373869u64;
    acc = acc ^ 29272u64;
    acc = acc.wrapping_mul(7733u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(42341u64);
    acc = acc.wrapping_mul(6495u64);
    acc = acc.wrapping_mul(42923u64);
    acc = acc.wrapping_mul(47541u64);
    acc = acc.wrapping_mul(17253u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(151356u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 765060u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(47177u64);
    acc = acc.wrapping_add(360918u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(913762u64);
    acc = acc.wrapping_mul(49781u64);
    acc = acc.wrapping_mul(51373u64);
    acc = acc.wrapping_add(56909u64);
    acc = acc.wrapping_add(962970u64);
    acc = acc.wrapping_mul(20767u64);
    acc = acc.wrapping_mul(33661u64);
    acc = acc ^ 6372u64;
    acc = acc.wrapping_add(491524u64);
    acc = acc.wrapping_add(341949u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 89169u64;
    acc = acc ^ 443818u64;
    acc = acc.wrapping_mul(8165u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(33573u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(48139u64);
    acc = acc.wrapping_add(536840u64);
    acc = acc.wrapping_add(809623u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 28802u64;
    acc = acc.wrapping_add(227758u64);
    acc = acc ^ 877804u64;
    acc = acc.wrapping_mul(12063u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(16887u64);
    acc = acc.wrapping_add(915250u64);
    acc = acc.wrapping_mul(22875u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(101621u64);
    acc = acc ^ 19001u64;
    acc = acc.rotate_left(15);
    acc = acc ^ 685119u64;
    acc = acc.wrapping_add(328227u64);
    acc = acc.wrapping_mul(64905u64);
    acc = acc ^ 209733u64;
    acc = acc ^ 467904u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 315783u64;
    acc = acc.wrapping_add(232071u64);
    acc = acc.wrapping_add(668892u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(457984u64);
    acc = acc.wrapping_mul(61983u64);
    acc = acc ^ 664848u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(654964u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 343857u64;
    acc = acc.wrapping_mul(13623u64);
    acc = acc.wrapping_add(356100u64);
    acc = acc.wrapping_mul(32547u64);
    acc = acc ^ 105977u64;
    acc = acc.wrapping_mul(44113u64);
    acc = acc.wrapping_mul(23601u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(45479u64);
    acc = acc.wrapping_mul(38633u64);
    acc = acc.wrapping_mul(60605u64);
    acc = acc.wrapping_add(756106u64);
    acc = acc.wrapping_mul(36583u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc ^ 367797u64;
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 81705u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc ^ 407363u64;
    acc = acc ^ 219028u64;
    acc = acc.wrapping_add(971955u64);
    acc = acc ^ 736853u64;
    acc = acc ^ 829393u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 992961u64;
    acc = acc.rotate_left(20);
    acc = acc ^ 107619u64;
    acc = acc.rotate_left(3);
    acc = acc ^ 368044u64;
    acc = acc ^ 576598u64;
    acc = acc ^ 325520u64;
    acc = acc.wrapping_mul(43733u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc ^ 855375u64;
    acc = acc.wrapping_mul(59249u64);
    acc = acc ^ 63670u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(32149u64);
    acc = acc.rotate_left(25);
    acc = acc ^ 421528u64;
    acc = acc.wrapping_add(32276u64);
    acc = acc.wrapping_mul(2505u64);
    acc = acc.wrapping_mul(27091u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 663057u64;
    acc = acc.wrapping_mul(27827u64);
    acc = acc ^ 637504u64;
    acc = acc.wrapping_mul(39403u64);
    acc = acc ^ 495122u64;
    acc = acc.wrapping_mul(39619u64);
    acc = acc.wrapping_add(932840u64);
    acc = acc ^ 503891u64;
    acc = acc.wrapping_add(809570u64);
    acc = acc.wrapping_add(828902u64);
    acc = acc.wrapping_add(816743u64);
    acc = acc.wrapping_mul(53437u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 15816u64;
    acc = acc.wrapping_add(848663u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(27253u64);
    acc = acc.wrapping_mul(41583u64);
    acc = acc.wrapping_mul(29349u64);
    acc = acc.wrapping_mul(52993u64);
    acc = acc ^ 552417u64;
    acc = acc.wrapping_mul(28473u64);
    acc = acc.wrapping_mul(6915u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 836017u64;
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 450969u64;
    acc = acc ^ 637657u64;
    acc = acc ^ 844037u64;
    acc = acc.wrapping_add(409967u64);
    acc = acc.wrapping_add(387749u64);
    acc = acc ^ 859028u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(21563u64);
    acc = acc.wrapping_mul(42431u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(7129u64);
    acc = acc.wrapping_add(415035u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(5099u64);
    acc = acc.wrapping_add(389713u64);
    acc = acc.wrapping_mul(58455u64);
    acc = acc.wrapping_mul(41791u64);
    acc = acc.wrapping_add(188157u64);
    acc = acc ^ 465124u64;
    acc = acc.wrapping_add(719384u64);
    acc = acc.rotate_left(31);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(755u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 362u64, b: 53i32, c: false } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 293u64, b: 347i32, c: false } } }


fn shelper_0(x: u32) -> u32 { x.wrapping_add(1) }

    pub fn run() {
        debug_assert!((3587667704u64).wrapping_mul(1) == 3587667704u64);
        debug_assert!((1951817010u32) | (3328125237u32) | (1951817010u32) == (1951817010u32) | (3328125237u32));
        assert!((2821658806u64).wrapping_add(2821658806u64).wrapping_sub(2821658806u64) == 2821658806u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3486635904u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1605275901u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1506008333u64)));
        {

    let SARR_0: [u32; 5] = [100000u32, 100097u32, 100194u32, 100291u32, 100388u32];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(4usize);
    let k = std::hint::black_box(2usize);

    let p_i = &SARR_0[i] as *const u32;
    let p_j = &SARR_0[j] as *const u32;

    print!("case_24:"); println!("{}", p_i == p_i);
    print!("case_24:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<u32>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_24:"); println!("{}", v_roundtrip);

    let base = SARR_0.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_24:"); println!("{}", v_k);
    print!("case_24:"); println!("{}", p_k == &SARR_0[k] as *const u32);

    let f1: fn(u32) -> u32 = shelper_0;
    let f2: fn(u32) -> u32 = shelper_0;
    print!("case_24:"); println!("{}", f1 == f2);
    print!("case_24:"); println!("{}", f1(100000u32));

        }
        debug_assert!((2939712453u32).rotate_left(10).rotate_right(10) == 2939712453u32);
        assert!((1859334039u64) ^ (1859334039u64) == 0u64);
        debug_assert!((2799688358u32) | (1961240672u32) | (2799688358u32) == (2799688358u32) | (1961240672u32));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(4250689124u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2321233721u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3221860831u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_25 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(4868u64).rotate_left(1) ^ 2511u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(431u64).rotate_left(10) ^ 7449u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(6000u64).rotate_left(1) ^ 4092u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(3849u64).rotate_left(21) ^ 8824u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(858053u64);
    acc = acc.wrapping_add(134553u64);
    acc = acc.wrapping_mul(13831u64);
    acc = acc ^ 300898u64;
    acc = acc ^ 185708u64;
    acc = acc.wrapping_add(692690u64);
    acc = acc.wrapping_add(906206u64);
    acc = acc ^ 23822u64;
    acc = acc.wrapping_add(954907u64);
    acc = acc.wrapping_mul(42905u64);
    acc = acc.wrapping_add(185328u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(213646u64);
    acc = acc ^ 27903u64;
    acc = acc ^ 720888u64;
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(426267u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(923332u64);
    acc = acc.wrapping_add(586043u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(215872u64);
    acc = acc.wrapping_add(812656u64);
    acc = acc.wrapping_mul(24057u64);
    acc = acc.wrapping_add(550817u64);
    acc = acc ^ 692016u64;
    acc = acc ^ 542037u64;
    acc = acc ^ 53434u64;
    acc = acc.wrapping_mul(19291u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(30743u64);
    acc = acc.wrapping_mul(45321u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(14065u64);
    acc = acc.wrapping_mul(21521u64);
    acc = acc ^ 972820u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(465842u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 412278u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 891824u64;
    acc = acc.wrapping_add(536036u64);
    acc = acc.wrapping_add(41104u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 314822u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(749377u64);
    acc = acc ^ 301207u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(599655u64);
    acc = acc.wrapping_add(542680u64);
    acc = acc ^ 316336u64;
    acc = acc ^ 919083u64;
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc ^ 446971u64;
    acc = acc.wrapping_mul(38461u64);
    acc = acc.rotate_left(28);
    acc = acc ^ 895795u64;
    acc = acc.rotate_left(11);
    acc = acc ^ 789851u64;
    acc = acc.wrapping_mul(59355u64);
    acc = acc.wrapping_add(963323u64);
    acc = acc ^ 94616u64;
    acc = acc ^ 913360u64;
    acc = acc.wrapping_mul(2571u64);
    acc = acc.wrapping_mul(11019u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc ^ 214139u64;
    acc = acc.wrapping_mul(47831u64);
    acc = acc ^ 801061u64;
    acc = acc ^ 140319u64;
    acc = acc.wrapping_mul(19811u64);
    acc = acc.wrapping_add(438144u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 123271u64;
    acc = acc ^ 709612u64;
    acc = acc ^ 276566u64;
    acc = acc.wrapping_add(137782u64);
    acc = acc.wrapping_add(487476u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(13649u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(21497u64);
    acc = acc.wrapping_add(100264u64);
    acc = acc ^ 440601u64;
    acc = acc.wrapping_mul(6403u64);
    acc = acc ^ 604517u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 680333u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(8643u64);
    acc = acc ^ 110037u64;
    acc = acc.wrapping_mul(29009u64);
    acc = acc.wrapping_add(883557u64);
    acc = acc.wrapping_add(498377u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(6989u64);
    acc = acc.wrapping_add(176745u64);
    acc = acc ^ 611001u64;
    acc = acc ^ 647495u64;
    acc = acc.wrapping_add(312430u64);
    acc = acc.wrapping_add(958394u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(238551u64);
    acc = acc ^ 329192u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(16661u64);
    acc = acc.wrapping_mul(10523u64);
    acc = acc.wrapping_add(453666u64);
    acc = acc ^ 248550u64;
    acc = acc ^ 202568u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 63103u64;
    acc = acc ^ 696702u64;
    acc = acc ^ 717784u64;
    acc = acc.wrapping_mul(41451u64);
    acc = acc.wrapping_add(602225u64);
    acc = acc.wrapping_add(76901u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 506311u64;
    acc = acc.wrapping_add(924263u64);
    acc = acc.wrapping_add(975824u64);
    acc = acc.wrapping_mul(34827u64);
    acc = acc ^ 1700u64;
    acc = acc ^ 366161u64;
    acc = acc.rotate_left(23);
    acc = acc ^ 173112u64;
    acc = acc ^ 613138u64;
    acc = acc ^ 357227u64;
    acc = acc ^ 907991u64;
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 663483u64;
    acc = acc.wrapping_add(307832u64);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc ^ 368076u64;
    acc = acc.wrapping_add(960101u64);
    acc = acc.wrapping_add(788034u64);
    acc = acc.wrapping_add(214636u64);
    acc = acc.wrapping_add(160994u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(6945u64);
    acc = acc ^ 141007u64;
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(142u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V3(83u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 904u64, b: 870i32, c: false } } }


fn shelper_1(x: u32) -> u32 { x.wrapping_add(1) }

    pub fn run() {
        assert!(!(1458228298u64) == (1458228298u64) ^ u64::MAX);
        debug_assert!((2776764660u64).wrapping_mul(1) == 2776764660u64);
        debug_assert!((321058604u32).rotate_left(5).rotate_right(5) == 321058604u32);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1383382690u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(356284536u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(4156792053u64)));
        {

    let SARR_1: [u32; 5] = [100003u32, 100100u32, 100197u32, 100294u32, 100391u32];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(4usize);
    let k = std::hint::black_box(1usize);

    let p_i = &SARR_1[i] as *const u32;
    let p_j = &SARR_1[j] as *const u32;

    print!("case_25:"); println!("{}", p_i == p_i);
    print!("case_25:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<u32>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_25:"); println!("{}", v_roundtrip);

    let base = SARR_1.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_25:"); println!("{}", v_k);
    print!("case_25:"); println!("{}", p_k == &SARR_1[k] as *const u32);

    let f1: fn(u32) -> u32 = shelper_1;
    let f2: fn(u32) -> u32 = shelper_1;
    print!("case_25:"); println!("{}", f1 == f2);
    print!("case_25:"); println!("{}", f1(100003u32));

        }
        assert!(!(1213122878u64) == (1213122878u64) ^ u64::MAX);
        debug_assert!((3487796882u64).wrapping_mul(1) == 3487796882u64);
        debug_assert!((600259320u32) | (1741248090u32) | (600259320u32) == (600259320u32) | (1741248090u32));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1043275951u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1991049928u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(242241585u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_26 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(5377u64).rotate_left(20) ^ 4452u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(6443u64).rotate_left(11) ^ 1808u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(4899u64).rotate_left(27) ^ 7869u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(2650u64).rotate_left(27) ^ 6350u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(999563u64);
    acc = acc.wrapping_add(141269u64);
    acc = acc.wrapping_mul(37337u64);
    acc = acc.wrapping_mul(20251u64);
    acc = acc ^ 58040u64;
    acc = acc.wrapping_mul(41475u64);
    acc = acc.wrapping_add(232039u64);
    acc = acc.wrapping_add(803114u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc ^ 965290u64;
    acc = acc ^ 349975u64;
    acc = acc.wrapping_add(618335u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(26975u64);
    acc = acc.wrapping_mul(50271u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(969938u64);
    acc = acc.wrapping_mul(24633u64);
    acc = acc.wrapping_mul(38783u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 629262u64;
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc ^ 563145u64;
    acc = acc.wrapping_mul(3587u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(734508u64);
    acc = acc.wrapping_add(835082u64);
    acc = acc.wrapping_add(912462u64);
    acc = acc ^ 923824u64;
    acc = acc ^ 311242u64;
    acc = acc.wrapping_mul(30695u64);
    acc = acc ^ 264812u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(582967u64);
    acc = acc ^ 827325u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(66657u64);
    acc = acc ^ 867519u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(12535u64);
    acc = acc.wrapping_add(70026u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 37954u64;
    acc = acc.wrapping_mul(13529u64);
    acc = acc.wrapping_add(390851u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(53u64);
    acc = acc.wrapping_add(198548u64);
    acc = acc.wrapping_add(11156u64);
    acc = acc.wrapping_add(39067u64);
    acc = acc.wrapping_mul(46979u64);
    acc = acc ^ 312351u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(804502u64);
    acc = acc.wrapping_add(375924u64);
    acc = acc.wrapping_add(979656u64);
    acc = acc.wrapping_mul(59585u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(29627u64);
    acc = acc.wrapping_add(337364u64);
    acc = acc ^ 899135u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(45881u64);
    acc = acc ^ 235288u64;
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(781567u64);
    acc = acc ^ 437407u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(43367u64);
    acc = acc.wrapping_add(842204u64);
    acc = acc.wrapping_add(577667u64);
    acc = acc.wrapping_mul(4579u64);
    acc = acc ^ 131941u64;
    acc = acc.rotate_left(29);
    acc = acc ^ 808901u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(14135u64);
    acc = acc.wrapping_mul(54341u64);
    acc = acc ^ 62606u64;
    acc = acc.wrapping_mul(15353u64);
    acc = acc ^ 741212u64;
    acc = acc.wrapping_add(93657u64);
    acc = acc ^ 617860u64;
    acc = acc ^ 785456u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(484691u64);
    acc = acc.wrapping_mul(4109u64);
    acc = acc ^ 970278u64;
    acc = acc ^ 828449u64;
    acc = acc.wrapping_add(717677u64);
    acc = acc ^ 416745u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(427066u64);
    acc = acc.wrapping_mul(27583u64);
    acc = acc.wrapping_add(3305u64);
    acc = acc.wrapping_add(404685u64);
    acc = acc.wrapping_add(449959u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(65277u64);
    acc = acc ^ 139194u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(32617u64);
    acc = acc.wrapping_add(744591u64);
    acc = acc ^ 242034u64;
    acc = acc ^ 988683u64;
    acc = acc.wrapping_mul(10827u64);
    acc = acc.wrapping_add(374594u64);
    acc = acc ^ 194389u64;
    acc = acc.wrapping_add(145774u64);
    acc = acc.wrapping_mul(40501u64);
    acc = acc.wrapping_mul(56109u64);
    acc = acc.wrapping_mul(61113u64);
    acc = acc ^ 531545u64;
    acc = acc.wrapping_mul(29965u64);
    acc = acc ^ 177979u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(2297u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(25625u64);
    acc = acc ^ 150867u64;
    acc = acc ^ 873813u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(26605u64);
    acc = acc ^ 796436u64;
    acc = acc.wrapping_mul(49595u64);
    acc = acc.wrapping_add(940028u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(1531u64);
    acc = acc.wrapping_mul(18327u64);
    acc = acc ^ 882745u64;
    acc = acc.wrapping_add(601313u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(8265u64);
    acc = acc ^ 334890u64;
    acc = acc.wrapping_add(305660u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(2057u64);
    acc = acc.wrapping_add(215606u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(32177u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(48693u64);
    acc = acc.wrapping_mul(21163u64);
    acc = acc.wrapping_mul(32783u64);
    acc = acc ^ 97539u64;
    acc = acc.wrapping_mul(12787u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(604907u64);
    acc = acc.wrapping_add(80966u64);
    acc = acc.wrapping_mul(35745u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(461676u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(333u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 819423u64;
    acc = acc ^ 52702u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(35387u64);
    acc = acc.wrapping_add(639996u64);
    acc = acc.wrapping_mul(34197u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 802081u64;
    acc = acc.wrapping_add(208091u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(33369u64);
    acc = acc.wrapping_mul(23923u64);
    acc = acc ^ 527991u64;
    acc = acc ^ 112663u64;
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 72297u64;
    acc = acc.wrapping_add(76170u64);
    acc = acc.wrapping_mul(40459u64);
    acc = acc.wrapping_add(185029u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V3(941u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V3(135u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V3(659u64) } }


fn shelper_2(x: u32) -> u32 { x.wrapping_add(1) }

    pub fn run() {
        assert!((3019544094u64).wrapping_add(3019544094u64).wrapping_sub(3019544094u64) == 3019544094u64);
        debug_assert!((2404777298u64).wrapping_mul(1) == 2404777298u64);
        debug_assert!((4283591086u32) | (2415281434u32) | (4283591086u32) == (4283591086u32) | (2415281434u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(4066276947u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(684975086u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(619798709u64)));
        {

    let SARR_2: [u32; 10] = [100006u32, 100103u32, 100200u32, 100297u32, 100394u32, 100491u32, 100588u32, 100685u32, 100782u32, 100879u32];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(9usize);
    let k = std::hint::black_box(5usize);

    let p_i = &SARR_2[i] as *const u32;
    let p_j = &SARR_2[j] as *const u32;

    print!("case_26:"); println!("{}", p_i == p_i);
    print!("case_26:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<u32>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_26:"); println!("{}", v_roundtrip);

    let base = SARR_2.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_26:"); println!("{}", v_k);
    print!("case_26:"); println!("{}", p_k == &SARR_2[k] as *const u32);

    let f1: fn(u32) -> u32 = shelper_2;
    let f2: fn(u32) -> u32 = shelper_2;
    print!("case_26:"); println!("{}", f1 == f2);
    print!("case_26:"); println!("{}", f1(100006u32));

        }
        debug_assert!((1214692675u64).wrapping_mul(1) == 1214692675u64);
        debug_assert!((3723315699u32) | (589653261u32) | (3723315699u32) == (3723315699u32) | (589653261u32));
        assert!((1682169374u64) ^ (1682169374u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(4031031142u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(787640632u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1714450822u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_27 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(6825u64).rotate_left(7) ^ 8965u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(7246u64).rotate_left(10) ^ 6953u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(6017u64).rotate_left(28) ^ 9668u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(8652u64).rotate_left(26) ^ 707u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(746104u64);
    acc = acc.wrapping_add(250167u64);
    acc = acc ^ 784282u64;
    acc = acc ^ 871562u64;
    acc = acc ^ 138600u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(36163u64);
    acc = acc ^ 352445u64;
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc ^ 540804u64;
    acc = acc.wrapping_mul(56697u64);
    acc = acc.wrapping_mul(7405u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(851265u64);
    acc = acc ^ 311155u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(911106u64);
    acc = acc.wrapping_mul(31959u64);
    acc = acc.wrapping_mul(28597u64);
    acc = acc.wrapping_add(741342u64);
    acc = acc ^ 109412u64;
    acc = acc.wrapping_mul(55465u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(48029u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(46089u64);
    acc = acc.wrapping_add(370309u64);
    acc = acc.wrapping_mul(3203u64);
    acc = acc.wrapping_add(168751u64);
    acc = acc.wrapping_add(999000u64);
    acc = acc ^ 54731u64;
    acc = acc.wrapping_add(600968u64);
    acc = acc.wrapping_mul(9987u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(3411u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(10639u64);
    acc = acc.wrapping_add(983007u64);
    acc = acc.wrapping_add(428379u64);
    acc = acc.wrapping_mul(39307u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(32455u64);
    acc = acc.wrapping_mul(38233u64);
    acc = acc.wrapping_add(994714u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(451111u64);
    acc = acc.wrapping_add(789046u64);
    acc = acc.wrapping_add(925691u64);
    acc = acc.wrapping_mul(2207u64);
    acc = acc.wrapping_mul(18011u64);
    acc = acc.wrapping_mul(16195u64);
    acc = acc.wrapping_mul(17749u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 982996u64;
    acc = acc.wrapping_add(578417u64);
    acc = acc.wrapping_mul(35865u64);
    acc = acc.wrapping_add(264706u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(54857u64);
    acc = acc.wrapping_mul(3871u64);
    acc = acc ^ 451509u64;
    acc = acc.wrapping_mul(62965u64);
    acc = acc.wrapping_mul(221u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 146602u64;
    acc = acc ^ 827232u64;
    acc = acc ^ 899778u64;
    acc = acc.wrapping_add(799995u64);
    acc = acc.wrapping_add(52310u64);
    acc = acc.wrapping_mul(64313u64);
    acc = acc.wrapping_add(659944u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(636995u64);
    acc = acc.wrapping_add(741291u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(566515u64);
    acc = acc.wrapping_mul(52833u64);
    acc = acc ^ 747225u64;
    acc = acc.wrapping_mul(33981u64);
    acc = acc ^ 762975u64;
    acc = acc.wrapping_mul(38445u64);
    acc = acc.wrapping_mul(1637u64);
    acc = acc.wrapping_add(216630u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(42745u64);
    acc = acc.wrapping_mul(22501u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(28883u64);
    acc = acc ^ 980627u64;
    acc = acc.wrapping_add(684261u64);
    acc = acc.wrapping_add(120954u64);
    acc = acc ^ 130936u64;
    acc = acc ^ 76107u64;
    acc = acc ^ 615219u64;
    acc = acc ^ 936957u64;
    acc = acc.wrapping_add(869515u64);
    acc = acc.wrapping_mul(50889u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(485235u64);
    acc = acc.wrapping_add(687593u64);
    acc = acc ^ 141516u64;
    acc = acc.wrapping_add(971691u64);
    acc = acc.wrapping_add(239029u64);
    acc = acc ^ 614744u64;
    acc = acc.wrapping_mul(44825u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 20394u64;
    acc = acc.wrapping_mul(28893u64);
    acc = acc ^ 911320u64;
    acc = acc.wrapping_mul(46179u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(3645u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(27407u64);
    acc = acc ^ 365617u64;
    acc = acc.wrapping_mul(13173u64);
    acc = acc.wrapping_add(918569u64);
    acc = acc ^ 971916u64;
    acc = acc.wrapping_add(966924u64);
    acc = acc ^ 954222u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(728241u64);
    acc = acc.wrapping_add(60464u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(293u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(40963u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(6881u64);
    acc = acc ^ 727849u64;
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(11875u64);
    acc = acc.wrapping_mul(65361u64);
    acc = acc.wrapping_mul(51847u64);
    acc = acc.wrapping_mul(60613u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(766699u64);
    acc = acc.wrapping_mul(25595u64);
    acc = acc.wrapping_mul(33295u64);
    acc = acc.wrapping_add(121491u64);
    acc = acc.wrapping_mul(17989u64);
    acc = acc.wrapping_mul(53101u64);
    acc = acc.wrapping_add(955511u64);
    acc = acc.wrapping_mul(49583u64);
    acc = acc.wrapping_add(339455u64);
    acc = acc ^ 16877u64;
    acc = acc.rotate_left(9);
    acc = acc ^ 963057u64;
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(99902u64);
    acc = acc.wrapping_add(454937u64);
    acc = acc ^ 636587u64;
    acc = acc ^ 610654u64;
    acc = acc ^ 297680u64;
    acc = acc.wrapping_add(685934u64);
    acc = acc ^ 723372u64;
    acc = acc.wrapping_mul(44183u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(747790u64);
    acc = acc ^ 192475u64;
    acc = acc.rotate_left(14);
    acc = acc ^ 722320u64;
    acc = acc.wrapping_add(803750u64);
    acc = acc ^ 684082u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(28849u64);
    acc = acc.wrapping_mul(56779u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(8857u64);
    acc = acc ^ 874487u64;
    acc = acc.wrapping_add(856616u64);
    acc = acc.wrapping_add(865133u64);
    acc = acc ^ 748718u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(11717u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(19831u64);
    acc = acc.wrapping_add(851437u64);
    acc = acc.wrapping_mul(47313u64);
    acc = acc.wrapping_add(959269u64);
    acc = acc ^ 533661u64;
    acc = acc.wrapping_mul(33987u64);
    acc = acc ^ 82456u64;
    acc = acc.wrapping_add(972896u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(7u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(119u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(162u64) } }


fn shelper_3(x: u32) -> u32 { x.wrapping_add(1) }

    pub fn run() {
        debug_assert!((2069438397u32).rotate_left(29).rotate_right(29) == 2069438397u32);
        assert!((1144258307u64) ^ (1144258307u64) == 0u64);
        debug_assert!((258008089u32) | (557897245u32) | (258008089u32) == (258008089u32) | (557897245u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(847347617u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(270171346u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(818717891u64)));
        {

    let SARR_3: [u32; 10] = [100009u32, 100106u32, 100203u32, 100300u32, 100397u32, 100494u32, 100591u32, 100688u32, 100785u32, 100882u32];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(9usize);
    let k = std::hint::black_box(1usize);

    let p_i = &SARR_3[i] as *const u32;
    let p_j = &SARR_3[j] as *const u32;

    print!("case_27:"); println!("{}", p_i == p_i);
    print!("case_27:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<u32>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_27:"); println!("{}", v_roundtrip);

    let base = SARR_3.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_27:"); println!("{}", v_k);
    print!("case_27:"); println!("{}", p_k == &SARR_3[k] as *const u32);

    let f1: fn(u32) -> u32 = shelper_3;
    let f2: fn(u32) -> u32 = shelper_3;
    print!("case_27:"); println!("{}", f1 == f2);
    print!("case_27:"); println!("{}", f1(100009u32));

        }
        assert!(!(2681933505u64) == (2681933505u64) ^ u64::MAX);
        assert!((3887172256u64) ^ (3887172256u64) == 0u64);
        assert!((1679348758u64) ^ (1679348758u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2789414052u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3437530344u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3595537731u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_28 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(2754u64).rotate_left(3) ^ 5613u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(1229u64).rotate_left(9) ^ 7603u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(6961u64).rotate_left(7) ^ 7012u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(14u64).rotate_left(1) ^ 1798u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(36773u64);
    acc = acc.wrapping_mul(62053u64);
    acc = acc.wrapping_mul(41861u64);
    acc = acc.rotate_left(25);
    acc = acc ^ 912218u64;
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(555310u64);
    acc = acc.wrapping_mul(14713u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(344562u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(64207u64);
    acc = acc.wrapping_mul(24101u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc ^ 349505u64;
    acc = acc ^ 197313u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(102710u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(10605u64);
    acc = acc ^ 324530u64;
    acc = acc.wrapping_mul(35725u64);
    acc = acc ^ 728487u64;
    acc = acc.wrapping_mul(15699u64);
    acc = acc ^ 766684u64;
    acc = acc.wrapping_add(995295u64);
    acc = acc.wrapping_mul(37493u64);
    acc = acc.wrapping_add(410125u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(217313u64);
    acc = acc.wrapping_add(375325u64);
    acc = acc.wrapping_add(458119u64);
    acc = acc.wrapping_mul(13101u64);
    acc = acc.wrapping_mul(28407u64);
    acc = acc.wrapping_mul(55263u64);
    acc = acc.wrapping_add(573352u64);
    acc = acc.wrapping_add(240152u64);
    acc = acc ^ 60145u64;
    acc = acc ^ 99058u64;
    acc = acc.wrapping_add(864973u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(54671u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(32159u64);
    acc = acc.wrapping_add(54273u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 55146u64;
    acc = acc ^ 482391u64;
    acc = acc.wrapping_mul(7093u64);
    acc = acc.wrapping_mul(53129u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(626075u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(61525u64);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(698430u64);
    acc = acc.wrapping_add(132316u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(177203u64);
    acc = acc.wrapping_add(656008u64);
    acc = acc.wrapping_mul(14049u64);
    acc = acc ^ 251845u64;
    acc = acc ^ 924165u64;
    acc = acc.wrapping_add(679419u64);
    acc = acc.wrapping_add(118750u64);
    acc = acc.wrapping_mul(10743u64);
    acc = acc ^ 786811u64;
    acc = acc.wrapping_mul(1819u64);
    acc = acc.wrapping_add(653755u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(11131u64);
    acc = acc ^ 856818u64;
    acc = acc.wrapping_mul(7291u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(195002u64);
    acc = acc.wrapping_add(359043u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 112995u64;
    acc = acc.wrapping_add(463721u64);
    acc = acc.wrapping_add(150995u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(637355u64);
    acc = acc ^ 165489u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(40375u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(9373u64);
    acc = acc.wrapping_add(466496u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 826338u64;
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc ^ 512102u64;
    acc = acc ^ 327602u64;
    acc = acc ^ 950475u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(199320u64);
    acc = acc ^ 967557u64;
    acc = acc ^ 756236u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(59859u64);
    acc = acc.wrapping_mul(5541u64);
    acc = acc.wrapping_mul(57593u64);
    acc = acc.wrapping_mul(5311u64);
    acc = acc ^ 687440u64;
    acc = acc ^ 972245u64;
    acc = acc.wrapping_add(69401u64);
    acc = acc.wrapping_mul(52849u64);
    acc = acc.wrapping_add(889850u64);
    acc = acc ^ 791004u64;
    acc = acc.wrapping_add(835992u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(766258u64);
    acc = acc.wrapping_add(194862u64);
    acc = acc.wrapping_mul(15145u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(8057u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(35453u64);
    acc = acc ^ 166236u64;
    acc = acc ^ 320185u64;
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(472053u64);
    acc = acc.wrapping_add(633992u64);
    acc = acc ^ 422851u64;
    acc = acc.rotate_left(20);
    acc = acc ^ 73849u64;
    acc = acc ^ 969982u64;
    acc = acc ^ 711969u64;
    acc = acc ^ 734148u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(58901u64);
    acc = acc ^ 716044u64;
    acc = acc.wrapping_add(278139u64);
    acc = acc ^ 843085u64;
    acc = acc.wrapping_add(937009u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(48137u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 392949u64;
    acc = acc ^ 483933u64;
    acc = acc.wrapping_add(784150u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(50953u64);
    acc = acc.wrapping_add(771978u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(29673u64);
    acc = acc ^ 756834u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(55939u64);
    acc = acc ^ 611242u64;
    acc = acc.wrapping_mul(3641u64);
    acc = acc.rotate_left(23);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(431u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V3(28u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(230u64) } }


fn shelper_4(x: i64) -> i64 { x.wrapping_add(1) }

    pub fn run() {
        assert!((1866144398u64) ^ (1866144398u64) == 0u64);
        debug_assert!((2915615902u32).rotate_left(1).rotate_right(1) == 2915615902u32);
        debug_assert!((3422880788u32) | (3342919846u32) | (3422880788u32) == (3422880788u32) | (3342919846u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2932714242u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3538278687u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1528685092u64)));
        {

    let SARR_4: [i64; 5] = [100012i64, 100109i64, 100206i64, 100303i64, 100400i64];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(4usize);
    let k = std::hint::black_box(2usize);

    let p_i = &SARR_4[i] as *const i64;
    let p_j = &SARR_4[j] as *const i64;

    print!("case_28:"); println!("{}", p_i == p_i);
    print!("case_28:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<i64>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_28:"); println!("{}", v_roundtrip);

    let base = SARR_4.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_28:"); println!("{}", v_k);
    print!("case_28:"); println!("{}", p_k == &SARR_4[k] as *const i64);

    let f1: fn(i64) -> i64 = shelper_4;
    let f2: fn(i64) -> i64 = shelper_4;
    print!("case_28:"); println!("{}", f1 == f2);
    print!("case_28:"); println!("{}", f1(100012i64));

        }
        assert!((2315177008u64) ^ (2315177008u64) == 0u64);
        assert!(!(148994142u64) == (148994142u64) ^ u64::MAX);
        debug_assert!((294374911u32) | (3110487472u32) | (294374911u32) == (294374911u32) | (3110487472u32));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2424125223u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(547352741u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3735013207u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_29 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(6758u64).rotate_left(1) ^ 5828u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(8670u64).rotate_left(13) ^ 4135u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(5296u64).rotate_left(1) ^ 4805u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(1420u64).rotate_left(21) ^ 3120u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(478941u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 967376u64;
    acc = acc.wrapping_add(103483u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(55971u64);
    acc = acc.wrapping_add(51318u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 819506u64;
    acc = acc ^ 979455u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(51881u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(24607u64);
    acc = acc ^ 309922u64;
    acc = acc.wrapping_mul(52287u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(682543u64);
    acc = acc.wrapping_mul(37047u64);
    acc = acc.wrapping_mul(63075u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(34123u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(44093u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(9991u64);
    acc = acc.wrapping_mul(37015u64);
    acc = acc.wrapping_mul(35725u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(443245u64);
    acc = acc.wrapping_mul(10413u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 871997u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(322802u64);
    acc = acc ^ 466824u64;
    acc = acc ^ 126894u64;
    acc = acc.rotate_left(19);
    acc = acc ^ 115338u64;
    acc = acc.wrapping_add(751243u64);
    acc = acc.wrapping_add(664543u64);
    acc = acc.wrapping_mul(53265u64);
    acc = acc ^ 952571u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(559066u64);
    acc = acc.wrapping_add(844709u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(46545u64);
    acc = acc.wrapping_add(524978u64);
    acc = acc ^ 603163u64;
    acc = acc.wrapping_add(472561u64);
    acc = acc.wrapping_mul(22147u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(52779u64);
    acc = acc.wrapping_add(795522u64);
    acc = acc.wrapping_mul(13435u64);
    acc = acc.wrapping_add(403679u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(6299u64);
    acc = acc.wrapping_add(813998u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(159877u64);
    acc = acc.wrapping_add(986616u64);
    acc = acc.wrapping_add(931274u64);
    acc = acc.wrapping_add(149635u64);
    acc = acc ^ 68561u64;
    acc = acc ^ 218635u64;
    acc = acc ^ 954914u64;
    acc = acc.wrapping_add(499727u64);
    acc = acc.wrapping_add(381916u64);
    acc = acc.wrapping_mul(58887u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(607928u64);
    acc = acc.wrapping_mul(41221u64);
    acc = acc.wrapping_mul(2241u64);
    acc = acc.wrapping_add(1824u64);
    acc = acc.wrapping_mul(36013u64);
    acc = acc.wrapping_mul(25729u64);
    acc = acc ^ 101851u64;
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc ^ 248485u64;
    acc = acc ^ 179384u64;
    acc = acc.wrapping_add(804780u64);
    acc = acc.wrapping_add(682520u64);
    acc = acc.wrapping_mul(64701u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(599855u64);
    acc = acc ^ 961720u64;
    acc = acc.wrapping_mul(46445u64);
    acc = acc ^ 227455u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc ^ 141832u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(676653u64);
    acc = acc ^ 211825u64;
    acc = acc ^ 526495u64;
    acc = acc.wrapping_mul(33505u64);
    acc = acc.wrapping_mul(11915u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 98539u64;
    acc = acc ^ 590773u64;
    acc = acc ^ 479762u64;
    acc = acc.wrapping_add(152132u64);
    acc = acc ^ 224241u64;
    acc = acc ^ 709822u64;
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(263930u64);
    acc = acc.wrapping_mul(5131u64);
    acc = acc.wrapping_add(672541u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(448543u64);
    acc = acc.wrapping_mul(54437u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(140319u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(40279u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(313889u64);
    acc = acc.wrapping_add(175168u64);
    acc = acc.wrapping_add(93081u64);
    acc = acc.wrapping_add(395831u64);
    acc = acc.wrapping_mul(49999u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 693064u64;
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 488345u64;
    acc = acc.wrapping_add(678902u64);
    acc = acc ^ 822041u64;
    acc = acc.wrapping_add(696348u64);
    acc = acc ^ 82220u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(917388u64);
    acc = acc.wrapping_add(916532u64);
    acc = acc.wrapping_add(444506u64);
    acc = acc.wrapping_add(125287u64);
    acc = acc.wrapping_mul(64885u64);
    acc = acc.wrapping_add(634328u64);
    acc = acc ^ 236632u64;
    acc = acc.wrapping_add(902499u64);
    acc = acc.wrapping_add(803419u64);
    acc = acc.wrapping_add(572455u64);
    acc = acc ^ 457547u64;
    acc = acc.wrapping_add(309751u64);
    acc = acc.wrapping_add(422099u64);
    acc = acc.wrapping_add(207561u64);
    acc = acc.wrapping_mul(9075u64);
    acc = acc ^ 925125u64;
    acc = acc.wrapping_mul(51885u64);
    acc = acc.wrapping_add(979499u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(16903u64);
    acc = acc ^ 772446u64;
    acc = acc.wrapping_mul(42621u64);
    acc = acc ^ 73265u64;
    acc = acc.wrapping_mul(26875u64);
    acc = acc ^ 947954u64;
    acc = acc.wrapping_mul(62135u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(773026u64);
    acc = acc.wrapping_add(45432u64);
    acc = acc.wrapping_add(794487u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(64881u64);
    acc = acc.wrapping_add(525854u64);
    acc = acc.wrapping_add(70280u64);
    acc = acc.wrapping_add(356234u64);
    acc = acc.wrapping_mul(48447u64);
    acc = acc ^ 164363u64;
    acc = acc ^ 357169u64;
    acc = acc ^ 406457u64;
    acc = acc.wrapping_mul(42247u64);
    acc = acc.wrapping_add(415883u64);
    acc = acc.wrapping_mul(38125u64);
    acc = acc.wrapping_mul(26541u64);
    acc = acc.wrapping_mul(54033u64);
    acc = acc ^ 799344u64;
    acc = acc.wrapping_mul(6703u64);
    acc = acc.wrapping_mul(26893u64);
    acc = acc.wrapping_add(770611u64);
    acc = acc.wrapping_mul(7783u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 917746u64;
    acc = acc.wrapping_mul(10451u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 203u64, b: -794i32, c: false } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(365u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(758u64) } }


fn shelper_5(x: i64) -> i64 { x.wrapping_add(1) }

    pub fn run() {
        assert!((423857199u64) ^ (423857199u64) == 0u64);
        assert!((25016000u64).wrapping_add(25016000u64).wrapping_sub(25016000u64) == 25016000u64);
        debug_assert!((4025289630u32).rotate_left(4).rotate_right(4) == 4025289630u32);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1615018337u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(85932108u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2620063828u64)));
        {

    let SARR_5: [i64; 5] = [100015i64, 100112i64, 100209i64, 100306i64, 100403i64];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(4usize);
    let k = std::hint::black_box(1usize);

    let p_i = &SARR_5[i] as *const i64;
    let p_j = &SARR_5[j] as *const i64;

    print!("case_29:"); println!("{}", p_i == p_i);
    print!("case_29:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<i64>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_29:"); println!("{}", v_roundtrip);

    let base = SARR_5.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_29:"); println!("{}", v_k);
    print!("case_29:"); println!("{}", p_k == &SARR_5[k] as *const i64);

    let f1: fn(i64) -> i64 = shelper_5;
    let f2: fn(i64) -> i64 = shelper_5;
    print!("case_29:"); println!("{}", f1 == f2);
    print!("case_29:"); println!("{}", f1(100015i64));

        }
        debug_assert!((347568358u32) | (319322302u32) | (347568358u32) == (347568358u32) | (319322302u32));
        assert!(!(2174201023u64) == (2174201023u64) ^ u64::MAX);
        debug_assert!((819125429u64).wrapping_mul(1) == 819125429u64);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1755496108u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2940128869u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1111372909u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_30 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(8578u64).rotate_left(28) ^ 7343u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(1351u64).rotate_left(25) ^ 5511u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(867u64).rotate_left(31) ^ 3595u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(6388u64).rotate_left(20) ^ 4917u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 470288u64;
    acc = acc ^ 686193u64;
    acc = acc.wrapping_add(772974u64);
    acc = acc ^ 524829u64;
    acc = acc ^ 886068u64;
    acc = acc.wrapping_add(697410u64);
    acc = acc.wrapping_add(859686u64);
    acc = acc.wrapping_add(916978u64);
    acc = acc.wrapping_add(291120u64);
    acc = acc.wrapping_add(109716u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc ^ 578381u64;
    acc = acc.wrapping_add(963315u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(858611u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 443564u64;
    acc = acc.wrapping_mul(29451u64);
    acc = acc ^ 500099u64;
    acc = acc ^ 303055u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 715474u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(57408u64);
    acc = acc.wrapping_mul(19497u64);
    acc = acc.wrapping_mul(27553u64);
    acc = acc ^ 364277u64;
    acc = acc ^ 872101u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(41613u64);
    acc = acc ^ 394327u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(26967u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(1765u64);
    acc = acc.wrapping_mul(3453u64);
    acc = acc.wrapping_add(556302u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 57775u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(44330u64);
    acc = acc ^ 233043u64;
    acc = acc.wrapping_add(313266u64);
    acc = acc.wrapping_add(661517u64);
    acc = acc ^ 881056u64;
    acc = acc.wrapping_mul(30513u64);
    acc = acc ^ 690324u64;
    acc = acc.wrapping_add(811234u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(394481u64);
    acc = acc.wrapping_add(651656u64);
    acc = acc.wrapping_mul(32389u64);
    acc = acc.wrapping_mul(64709u64);
    acc = acc.wrapping_mul(14955u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 752576u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(2199u64);
    acc = acc.wrapping_mul(8741u64);
    acc = acc.wrapping_add(78233u64);
    acc = acc ^ 962001u64;
    acc = acc.wrapping_mul(49503u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(315223u64);
    acc = acc.wrapping_add(410570u64);
    acc = acc.wrapping_mul(44003u64);
    acc = acc.wrapping_mul(48331u64);
    acc = acc.wrapping_add(786203u64);
    acc = acc.wrapping_mul(52519u64);
    acc = acc.wrapping_mul(4771u64);
    acc = acc ^ 954943u64;
    acc = acc.wrapping_add(329247u64);
    acc = acc ^ 338252u64;
    acc = acc.wrapping_mul(63613u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(503126u64);
    acc = acc.wrapping_mul(27385u64);
    acc = acc.wrapping_add(518242u64);
    acc = acc.wrapping_add(504773u64);
    acc = acc ^ 295911u64;
    acc = acc ^ 270139u64;
    acc = acc.wrapping_add(108512u64);
    acc = acc ^ 386222u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(93485u64);
    acc = acc ^ 370593u64;
    acc = acc.wrapping_mul(38929u64);
    acc = acc.wrapping_add(220044u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 299548u64;
    acc = acc.wrapping_add(931608u64);
    acc = acc.wrapping_mul(22421u64);
    acc = acc.wrapping_add(3420u64);
    acc = acc ^ 158367u64;
    acc = acc ^ 782493u64;
    acc = acc.rotate_left(19);
    acc = acc ^ 206423u64;
    acc = acc.wrapping_mul(36605u64);
    acc = acc.wrapping_add(248297u64);
    acc = acc.wrapping_add(437612u64);
    acc = acc.wrapping_mul(59419u64);
    acc = acc ^ 962788u64;
    acc = acc.wrapping_mul(51773u64);
    acc = acc.wrapping_mul(6545u64);
    acc = acc ^ 618840u64;
    acc = acc.wrapping_add(276943u64);
    acc = acc ^ 752409u64;
    acc = acc.rotate_left(10);
    acc = acc ^ 102923u64;
    acc = acc.wrapping_mul(47159u64);
    acc = acc ^ 231674u64;
    acc = acc.wrapping_add(813081u64);
    acc = acc ^ 429761u64;
    acc = acc.wrapping_add(290606u64);
    acc = acc ^ 301729u64;
    acc = acc.wrapping_add(267776u64);
    acc = acc.wrapping_mul(62399u64);
    acc = acc.wrapping_add(560462u64);
    acc = acc ^ 646116u64;
    acc = acc.wrapping_mul(24983u64);
    acc = acc.wrapping_mul(22061u64);
    acc = acc.wrapping_mul(37787u64);
    acc = acc.wrapping_add(511734u64);
    acc = acc ^ 629791u64;
    acc = acc.wrapping_add(71799u64);
    acc = acc.wrapping_mul(36499u64);
    acc = acc.wrapping_mul(9615u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(685870u64);
    acc = acc.wrapping_add(507999u64);
    acc = acc.wrapping_mul(19119u64);
    acc = acc.wrapping_add(511966u64);
    acc = acc.wrapping_add(203695u64);
    acc = acc.wrapping_add(17679u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 560204u64;
    acc = acc ^ 643779u64;
    acc = acc.rotate_left(3);
    acc = acc ^ 193481u64;
    acc = acc ^ 342249u64;
    acc = acc.wrapping_mul(15021u64);
    acc = acc.wrapping_mul(40281u64);
    acc = acc.wrapping_add(244679u64);
    acc = acc.wrapping_mul(41787u64);
    acc = acc ^ 87265u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(625100u64);
    acc = acc.wrapping_add(337177u64);
    acc = acc.wrapping_mul(58973u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(23131u64);
    acc = acc ^ 559919u64;
    acc = acc.wrapping_mul(21933u64);
    acc = acc.wrapping_add(861543u64);
    acc = acc ^ 562862u64;
    acc = acc.wrapping_add(522122u64);
    acc = acc ^ 562894u64;
    acc = acc ^ 799096u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(757984u64);
    acc = acc.wrapping_add(708204u64);
    acc = acc.wrapping_add(219111u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(689440u64);
    acc = acc.wrapping_add(679875u64);
    acc = acc ^ 227401u64;
    acc = acc.wrapping_add(281627u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(19113u64);
    acc = acc.wrapping_mul(26411u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(92256u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 542u64, b: 940i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 614u64, b: 188i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 245u64, b: 469i32, c: false } } }


fn shelper_6(x: i64) -> i64 { x.wrapping_add(1) }

    pub fn run() {
        assert!(!(1810921651u64) == (1810921651u64) ^ u64::MAX);
        assert!((3801681592u64).wrapping_add(3801681592u64).wrapping_sub(3801681592u64) == 3801681592u64);
        debug_assert!((334301969u32) | (2637722252u32) | (334301969u32) == (334301969u32) | (2637722252u32));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3718810215u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(664873166u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3120314613u64)));
        {

    let SARR_6: [i64; 10] = [100018i64, 100115i64, 100212i64, 100309i64, 100406i64, 100503i64, 100600i64, 100697i64, 100794i64, 100891i64];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(9usize);
    let k = std::hint::black_box(5usize);

    let p_i = &SARR_6[i] as *const i64;
    let p_j = &SARR_6[j] as *const i64;

    print!("case_30:"); println!("{}", p_i == p_i);
    print!("case_30:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<i64>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_30:"); println!("{}", v_roundtrip);

    let base = SARR_6.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_30:"); println!("{}", v_k);
    print!("case_30:"); println!("{}", p_k == &SARR_6[k] as *const i64);

    let f1: fn(i64) -> i64 = shelper_6;
    let f2: fn(i64) -> i64 = shelper_6;
    print!("case_30:"); println!("{}", f1 == f2);
    print!("case_30:"); println!("{}", f1(100018i64));

        }
        debug_assert!((1008875458u32).rotate_left(22).rotate_right(22) == 1008875458u32);
        debug_assert!((2588252633u32) | (2887812284u32) | (2588252633u32) == (2588252633u32) | (2887812284u32));
        assert!(!(1195150916u64) == (1195150916u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(174112887u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2269355826u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3729116746u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_31 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(4854u64).rotate_left(27) ^ 5141u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(1610u64).rotate_left(16) ^ 6029u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(2026u64).rotate_left(25) ^ 1884u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(3356u64).rotate_left(13) ^ 8940u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 561345u64;
    acc = acc ^ 700095u64;
    acc = acc.wrapping_add(472745u64);
    acc = acc.wrapping_mul(57105u64);
    acc = acc.wrapping_mul(64891u64);
    acc = acc ^ 997275u64;
    acc = acc.wrapping_add(843515u64);
    acc = acc.wrapping_mul(1581u64);
    acc = acc.wrapping_add(320177u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc ^ 429075u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc ^ 69904u64;
    acc = acc.wrapping_add(226289u64);
    acc = acc ^ 930263u64;
    acc = acc.wrapping_add(133358u64);
    acc = acc.wrapping_add(363829u64);
    acc = acc ^ 797973u64;
    acc = acc ^ 336017u64;
    acc = acc.wrapping_add(564264u64);
    acc = acc.wrapping_mul(12853u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(990236u64);
    acc = acc.wrapping_mul(52891u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(300005u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc ^ 336465u64;
    acc = acc.wrapping_mul(5505u64);
    acc = acc ^ 394509u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(16449u64);
    acc = acc.wrapping_mul(14897u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(32823u64);
    acc = acc.wrapping_mul(5335u64);
    acc = acc.wrapping_mul(64777u64);
    acc = acc ^ 576876u64;
    acc = acc ^ 687237u64;
    acc = acc.wrapping_mul(49617u64);
    acc = acc.wrapping_mul(61195u64);
    acc = acc.wrapping_add(717051u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(23748u64);
    acc = acc ^ 675556u64;
    acc = acc ^ 282330u64;
    acc = acc ^ 915663u64;
    acc = acc.wrapping_add(892667u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 562194u64;
    acc = acc.wrapping_add(702818u64);
    acc = acc.wrapping_add(750199u64);
    acc = acc ^ 472966u64;
    acc = acc.wrapping_add(782622u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(7315u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(628682u64);
    acc = acc ^ 566264u64;
    acc = acc.wrapping_add(429588u64);
    acc = acc.wrapping_add(977891u64);
    acc = acc.wrapping_add(925057u64);
    acc = acc.wrapping_add(548493u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(33795u64);
    acc = acc.wrapping_add(923321u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(39853u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 740141u64;
    acc = acc ^ 636785u64;
    acc = acc.wrapping_add(607115u64);
    acc = acc.wrapping_mul(64213u64);
    acc = acc.wrapping_add(230627u64);
    acc = acc.wrapping_mul(17033u64);
    acc = acc ^ 4796u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(38399u64);
    acc = acc.wrapping_add(729503u64);
    acc = acc.wrapping_add(634791u64);
    acc = acc.wrapping_mul(37693u64);
    acc = acc.wrapping_add(374507u64);
    acc = acc ^ 772573u64;
    acc = acc ^ 38826u64;
    acc = acc ^ 79320u64;
    acc = acc ^ 463997u64;
    acc = acc.wrapping_add(898898u64);
    acc = acc ^ 550202u64;
    acc = acc.wrapping_mul(30727u64);
    acc = acc.wrapping_mul(41023u64);
    acc = acc.wrapping_add(405124u64);
    acc = acc ^ 903222u64;
    acc = acc.rotate_left(29);
    acc = acc ^ 554461u64;
    acc = acc.rotate_left(12);
    acc = acc ^ 54155u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(34171u64);
    acc = acc ^ 285515u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 615443u64;
    acc = acc ^ 755150u64;
    acc = acc.wrapping_mul(3721u64);
    acc = acc ^ 892756u64;
    acc = acc ^ 790852u64;
    acc = acc.wrapping_mul(9255u64);
    acc = acc.wrapping_mul(31945u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(590323u64);
    acc = acc.wrapping_mul(5563u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(882018u64);
    acc = acc.wrapping_mul(53229u64);
    acc = acc ^ 661423u64;
    acc = acc.wrapping_add(180878u64);
    acc = acc.wrapping_mul(9625u64);
    acc = acc.wrapping_mul(5763u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(38371u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(488281u64);
    acc = acc ^ 729859u64;
    acc = acc ^ 646410u64;
    acc = acc.wrapping_mul(64821u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 149926u64;
    acc = acc.wrapping_add(349086u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 605048u64;
    acc = acc ^ 93843u64;
    acc = acc.wrapping_add(370423u64);
    acc = acc.wrapping_mul(54217u64);
    acc = acc.wrapping_mul(2145u64);
    acc = acc.wrapping_mul(54469u64);
    acc = acc ^ 412204u64;
    acc = acc.wrapping_mul(44887u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(127295u64);
    acc = acc ^ 356561u64;
    acc = acc.rotate_left(25);
    acc = acc ^ 411116u64;
    acc = acc.wrapping_mul(24045u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(64555u64);
    acc = acc.wrapping_mul(16953u64);
    acc = acc.wrapping_mul(53943u64);
    acc = acc.wrapping_add(790885u64);
    acc = acc.wrapping_add(960638u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(975594u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(38281u64);
    acc = acc.wrapping_mul(39595u64);
    acc = acc.wrapping_add(841515u64);
    acc = acc.wrapping_mul(24965u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(453691u64);
    acc = acc.wrapping_mul(39319u64);
    acc = acc.wrapping_add(1328u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 908806u64;
    acc = acc.wrapping_mul(26951u64);
    acc = acc.wrapping_add(123276u64);
    acc = acc ^ 366644u64;
    acc = acc.wrapping_mul(34471u64);
    acc = acc.wrapping_mul(7415u64);
    acc = acc ^ 453109u64;
    acc = acc.wrapping_add(931541u64);
    acc = acc ^ 419126u64;
    acc = acc.wrapping_add(707524u64);
    acc = acc.wrapping_add(890608u64);
    acc = acc ^ 557665u64;
    acc = acc.rotate_left(8);
    acc = acc ^ 896089u64;
    acc = acc.wrapping_add(487013u64);
    acc = acc ^ 975730u64;
    acc = acc.wrapping_mul(24167u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(41047u64);
    acc = acc ^ 651814u64;
    acc = acc.wrapping_add(341499u64);
    acc = acc.rotate_left(9);
    acc = acc ^ 14961u64;
    acc = acc.wrapping_add(163732u64);
    acc = acc.wrapping_mul(22355u64);
    acc = acc.wrapping_add(404690u64);
    acc = acc ^ 839581u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(23309u64);
    acc = acc ^ 35054u64;
    acc = acc.wrapping_add(256586u64);
    acc = acc.wrapping_mul(9093u64);
    acc = acc.rotate_left(24);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(98u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 298u64, b: -554i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(534u64) } }


fn shelper_7(x: i64) -> i64 { x.wrapping_add(1) }

    pub fn run() {
        assert!(!(705860321u64) == (705860321u64) ^ u64::MAX);
        debug_assert!((16134113u32).rotate_left(29).rotate_right(29) == 16134113u32);
        debug_assert!((2452255300u32) | (3905314826u32) | (2452255300u32) == (2452255300u32) | (3905314826u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(392325368u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(837210843u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1888311807u64)));
        {

    let SARR_7: [i64; 10] = [100021i64, 100118i64, 100215i64, 100312i64, 100409i64, 100506i64, 100603i64, 100700i64, 100797i64, 100894i64];

    let i = std::hint::black_box(0usize);
    let j = std::hint::black_box(9usize);
    let k = std::hint::black_box(1usize);

    let p_i = &SARR_7[i] as *const i64;
    let p_j = &SARR_7[j] as *const i64;

    print!("case_31:"); println!("{}", p_i == p_i);
    print!("case_31:"); println!("{}", p_i == p_j);

    let addr = p_i.expose_provenance();
    let p_i2 = std::ptr::with_exposed_provenance::<i64>(addr);
    let v_roundtrip = unsafe { *p_i2 };
    print!("case_31:"); println!("{}", v_roundtrip);

    let base = SARR_7.as_ptr();
    let p_k = unsafe { base.add(k) };
    let v_k = unsafe { *p_k };
    print!("case_31:"); println!("{}", v_k);
    print!("case_31:"); println!("{}", p_k == &SARR_7[k] as *const i64);

    let f1: fn(i64) -> i64 = shelper_7;
    let f2: fn(i64) -> i64 = shelper_7;
    print!("case_31:"); println!("{}", f1 == f2);
    print!("case_31:"); println!("{}", f1(100021i64));

        }
        assert!(!(34452566u64) == (34452566u64) ^ u64::MAX);
        assert!(!(2481620664u64) == (2481620664u64) ^ u64::MAX);
        assert!(!(3451602422u64) == (3451602422u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(255941219u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3500443270u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(771755179u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_32 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(1198u64).rotate_left(22) ^ 5508u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(8608u64).rotate_left(28) ^ 3189u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(3370u64).rotate_left(19) ^ 3397u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(5370u64).rotate_left(5) ^ 8932u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(51979u64);
    acc = acc.wrapping_mul(27743u64);
    acc = acc.wrapping_add(677220u64);
    acc = acc.wrapping_add(279620u64);
    acc = acc ^ 478198u64;
    acc = acc.wrapping_mul(37185u64);
    acc = acc.wrapping_mul(15023u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(382311u64);
    acc = acc ^ 958749u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(39543u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(583258u64);
    acc = acc.wrapping_mul(35035u64);
    acc = acc ^ 522169u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(497091u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(49755u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 722710u64;
    acc = acc.wrapping_add(684356u64);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc ^ 831102u64;
    acc = acc.wrapping_mul(57921u64);
    acc = acc.wrapping_add(570130u64);
    acc = acc.wrapping_add(202827u64);
    acc = acc.wrapping_add(982579u64);
    acc = acc ^ 512253u64;
    acc = acc ^ 930387u64;
    acc = acc.wrapping_add(111928u64);
    acc = acc.wrapping_add(389161u64);
    acc = acc.wrapping_add(368417u64);
    acc = acc.wrapping_add(965371u64);
    acc = acc.wrapping_mul(51355u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 566577u64;
    acc = acc.wrapping_add(931637u64);
    acc = acc ^ 763250u64;
    acc = acc ^ 832093u64;
    acc = acc.wrapping_add(581548u64);
    acc = acc.wrapping_add(518965u64);
    acc = acc ^ 348183u64;
    acc = acc.wrapping_mul(7631u64);
    acc = acc.wrapping_mul(32279u64);
    acc = acc.wrapping_mul(17941u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(2573u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 138808u64;
    acc = acc.wrapping_add(686699u64);
    acc = acc ^ 802724u64;
    acc = acc.wrapping_add(5113u64);
    acc = acc ^ 737163u64;
    acc = acc.rotate_left(7);
    acc = acc ^ 732238u64;
    acc = acc.wrapping_add(35792u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(49313u64);
    acc = acc.wrapping_add(377482u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(787023u64);
    acc = acc.wrapping_add(497871u64);
    acc = acc.wrapping_add(893002u64);
    acc = acc.wrapping_add(971804u64);
    acc = acc.wrapping_mul(56139u64);
    acc = acc ^ 911037u64;
    acc = acc.wrapping_mul(52147u64);
    acc = acc.wrapping_add(906224u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(44219u64);
    acc = acc ^ 950018u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 797809u64;
    acc = acc.wrapping_add(807175u64);
    acc = acc.wrapping_mul(13977u64);
    acc = acc ^ 975549u64;
    acc = acc ^ 287304u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(5511u64);
    acc = acc.wrapping_mul(49525u64);
    acc = acc ^ 818904u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(50305u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(945991u64);
    acc = acc ^ 344318u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(15869u64);
    acc = acc ^ 514742u64;
    acc = acc.wrapping_add(710212u64);
    acc = acc.wrapping_mul(1389u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(52435u64);
    acc = acc.wrapping_add(297978u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(63341u64);
    acc = acc ^ 20369u64;
    acc = acc.wrapping_mul(63509u64);
    acc = acc ^ 310224u64;
    acc = acc.wrapping_mul(17785u64);
    acc = acc ^ 115939u64;
    acc = acc.wrapping_add(190679u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(31353u64);
    acc = acc ^ 798712u64;
    acc = acc ^ 138893u64;
    acc = acc.wrapping_add(462538u64);
    acc = acc ^ 281410u64;
    acc = acc.wrapping_mul(60881u64);
    acc = acc ^ 275431u64;
    acc = acc ^ 189348u64;
    acc = acc.rotate_left(24);
    acc = acc ^ 202696u64;
    acc = acc.wrapping_add(665741u64);
    acc = acc.wrapping_add(950663u64);
    acc = acc.wrapping_add(82204u64);
    acc = acc.wrapping_mul(14129u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(27592u64);
    acc = acc.wrapping_add(397585u64);
    acc = acc.wrapping_mul(12067u64);
    acc = acc ^ 147625u64;
    acc = acc.wrapping_mul(54197u64);
    acc = acc ^ 763529u64;
    acc = acc.wrapping_add(398246u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 193725u64;
    acc = acc.wrapping_mul(27229u64);
    acc = acc.wrapping_mul(30615u64);
    acc = acc ^ 23925u64;
    acc = acc.wrapping_add(576148u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(410439u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(200126u64);
    acc = acc ^ 713286u64;
    acc = acc ^ 350741u64;
    acc = acc.wrapping_add(840489u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(630866u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(268640u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(432339u64);
    acc = acc.wrapping_add(437873u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(48309u64);
    acc = acc.rotate_left(28);
    acc = acc ^ 19401u64;
    acc = acc.wrapping_mul(6421u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(609596u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 271380u64;
    acc = acc ^ 746006u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(217314u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 730u64, b: -94i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 340u64, b: -176i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(900u64) } }


struct SLvl_0_0 { inner: u16, tag: u8 }
struct SLvl_0_1 { inner: SLvl_0_0, tag: u8 }

fn sbump_0_0(v: &mut SLvl_0_1) { sbump_0_1(&mut v.inner); }
fn sbump_0_1(v: &mut SLvl_0_0) { v.inner = v.inner.wrapping_add(1); }

    pub fn run() {
        debug_assert!((2842753079u64).wrapping_mul(1) == 2842753079u64);
        assert!(!(590543338u64) == (590543338u64) ^ u64::MAX);
        debug_assert!((871112312u64).wrapping_mul(1) == 871112312u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1446629941u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1817464375u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2369068056u64)));
        {

    let mut val = SLvl_0_1 { inner: SLvl_0_0 { inner: 12345, tag: 0 }, tag: 7 };
    sbump_0_0(&mut val);
    let leaf = std::hint::black_box(val.inner.inner);
    print!("case_32:"); println!("{}", leaf);
    print!("case_32:"); println!("tag0={}", val.tag);
    print!("case_32:"); println!("tag1={}", val.inner.tag);

        }
        assert!((2204734244u64) ^ (2204734244u64) == 0u64);
        assert!((2350385220u64) ^ (2350385220u64) == 0u64);
        debug_assert!((3464703501u64).wrapping_mul(1) == 3464703501u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(973415690u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(414450711u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(789361675u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_33 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(2440u64).rotate_left(6) ^ 9119u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(4599u64).rotate_left(13) ^ 9002u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(6052u64).rotate_left(16) ^ 2681u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(8111u64).rotate_left(11) ^ 971u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 855326u64;
    acc = acc ^ 537041u64;
    acc = acc.wrapping_mul(50809u64);
    acc = acc.wrapping_add(237786u64);
    acc = acc ^ 749664u64;
    acc = acc ^ 63801u64;
    acc = acc.wrapping_mul(2933u64);
    acc = acc.wrapping_add(163596u64);
    acc = acc ^ 119540u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(39355u64);
    acc = acc ^ 353011u64;
    acc = acc ^ 663821u64;
    acc = acc.wrapping_mul(9811u64);
    acc = acc.wrapping_add(515094u64);
    acc = acc ^ 516819u64;
    acc = acc.wrapping_mul(13853u64);
    acc = acc ^ 519724u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 628760u64;
    acc = acc.wrapping_mul(65245u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(49001u64);
    acc = acc ^ 380590u64;
    acc = acc.wrapping_add(890187u64);
    acc = acc.wrapping_add(259879u64);
    acc = acc ^ 761083u64;
    acc = acc.wrapping_mul(41451u64);
    acc = acc.wrapping_add(65175u64);
    acc = acc ^ 405271u64;
    acc = acc.wrapping_add(502367u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(427675u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(307476u64);
    acc = acc.wrapping_add(877912u64);
    acc = acc.wrapping_add(474360u64);
    acc = acc.wrapping_mul(45305u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(33485u64);
    acc = acc.wrapping_mul(18809u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 698901u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc ^ 942009u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 762721u64;
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(798793u64);
    acc = acc ^ 446001u64;
    acc = acc.wrapping_mul(56229u64);
    acc = acc ^ 375858u64;
    acc = acc.wrapping_add(146355u64);
    acc = acc.wrapping_mul(4515u64);
    acc = acc.wrapping_mul(36845u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(56887u64);
    acc = acc ^ 527786u64;
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(529007u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(50149u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(957406u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(10987u64);
    acc = acc.wrapping_mul(42157u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(58393u64);
    acc = acc ^ 179411u64;
    acc = acc.wrapping_mul(31713u64);
    acc = acc.wrapping_mul(41543u64);
    acc = acc.wrapping_mul(34947u64);
    acc = acc.wrapping_mul(49929u64);
    acc = acc.wrapping_mul(56125u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(257468u64);
    acc = acc.wrapping_mul(28463u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 731032u64;
    acc = acc.wrapping_add(15105u64);
    acc = acc.wrapping_add(206225u64);
    acc = acc.rotate_left(9);
    acc = acc ^ 860035u64;
    acc = acc.wrapping_add(247286u64);
    acc = acc ^ 603605u64;
    acc = acc ^ 74379u64;
    acc = acc ^ 245758u64;
    acc = acc.wrapping_mul(42153u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 380200u64;
    acc = acc.wrapping_add(857664u64);
    acc = acc ^ 686898u64;
    acc = acc.wrapping_add(463480u64);
    acc = acc.wrapping_add(149915u64);
    acc = acc ^ 610445u64;
    acc = acc.wrapping_mul(33585u64);
    acc = acc.wrapping_mul(33747u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(52791u64);
    acc = acc.wrapping_mul(24517u64);
    acc = acc.wrapping_add(975849u64);
    acc = acc.rotate_left(25);
    acc = acc ^ 530395u64;
    acc = acc ^ 194246u64;
    acc = acc.wrapping_mul(46263u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(119615u64);
    acc = acc ^ 144625u64;
    acc = acc.wrapping_mul(15753u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(960324u64);
    acc = acc.wrapping_add(636668u64);
    acc = acc.wrapping_add(586456u64);
    acc = acc ^ 86370u64;
    acc = acc.wrapping_mul(55557u64);
    acc = acc.wrapping_mul(1383u64);
    acc = acc.wrapping_mul(37835u64);
    acc = acc.wrapping_add(301317u64);
    acc = acc.wrapping_add(401202u64);
    acc = acc.wrapping_mul(48073u64);
    acc = acc.wrapping_mul(45339u64);
    acc = acc.wrapping_add(542330u64);
    acc = acc ^ 690673u64;
    acc = acc ^ 29955u64;
    acc = acc.wrapping_mul(22419u64);
    acc = acc ^ 591809u64;
    acc = acc.wrapping_add(26532u64);
    acc = acc.wrapping_add(732451u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(170824u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(39661u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(758849u64);
    acc = acc.wrapping_mul(29111u64);
    acc = acc.wrapping_mul(9699u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 743851u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(912886u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(52423u64);
    acc = acc.wrapping_mul(29247u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(49801u64);
    acc = acc.wrapping_add(73491u64);
    acc = acc.wrapping_mul(1793u64);
    acc = acc ^ 773518u64;
    acc = acc ^ 825369u64;
    acc = acc ^ 254817u64;
    acc = acc.wrapping_add(854244u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 144698u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(254457u64);
    acc = acc.wrapping_add(704148u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(16995u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(504u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 326u64, b: -847i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(938u64) } }


struct SLvl_1_0 { inner: i32, tag: u8 }
struct SLvl_1_1 { inner: SLvl_1_0, tag: u8 }

fn sbump_1_0(v: &mut SLvl_1_1) { sbump_1_1(&mut v.inner); }
fn sbump_1_1(v: &mut SLvl_1_0) { v.inner = v.inner.wrapping_add(2); }

    pub fn run() {
        debug_assert!((1526378696u32).rotate_left(25).rotate_right(25) == 1526378696u32);
        assert!((400475231u64).wrapping_add(400475231u64).wrapping_sub(400475231u64) == 400475231u64);
        assert!(!(4178082932u64) == (4178082932u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2032471101u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(821848467u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2201070877u64)));
        {

    let mut val = SLvl_1_1 { inner: SLvl_1_0 { inner: 12362, tag: 31 }, tag: 38 };
    sbump_1_0(&mut val);
    let leaf = std::hint::black_box(val.inner.inner);
    print!("case_33:"); println!("{}", leaf);
    print!("case_33:"); println!("tag0={}", val.tag);
    print!("case_33:"); println!("tag1={}", val.inner.tag);

        }
        assert!(!(2823355345u64) == (2823355345u64) ^ u64::MAX);
        assert!((2943289538u64) ^ (2943289538u64) == 0u64);
        debug_assert!((17860039u32).rotate_left(24).rotate_right(24) == 17860039u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(297116973u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(22346485u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(510836670u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_34 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(5126u64).rotate_left(27) ^ 1018u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(2239u64).rotate_left(7) ^ 3075u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(2378u64).rotate_left(6) ^ 3353u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(6562u64).rotate_left(30) ^ 5144u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 392075u64;
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(60083u64);
    acc = acc.wrapping_add(112443u64);
    acc = acc.wrapping_add(29513u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(29011u64);
    acc = acc.wrapping_mul(21171u64);
    acc = acc.wrapping_add(765437u64);
    acc = acc ^ 84775u64;
    acc = acc.wrapping_mul(53815u64);
    acc = acc ^ 521649u64;
    acc = acc.wrapping_add(441124u64);
    acc = acc.wrapping_add(692496u64);
    acc = acc.wrapping_mul(7913u64);
    acc = acc.wrapping_add(906126u64);
    acc = acc.wrapping_add(911524u64);
    acc = acc.wrapping_mul(55949u64);
    acc = acc ^ 76504u64;
    acc = acc ^ 912866u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(640265u64);
    acc = acc.wrapping_mul(60749u64);
    acc = acc ^ 787402u64;
    acc = acc.wrapping_add(749240u64);
    acc = acc ^ 570554u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(25011u64);
    acc = acc.wrapping_add(681772u64);
    acc = acc.wrapping_add(82091u64);
    acc = acc.wrapping_mul(37183u64);
    acc = acc.wrapping_add(10791u64);
    acc = acc.wrapping_mul(41777u64);
    acc = acc.wrapping_add(54144u64);
    acc = acc.wrapping_mul(47779u64);
    acc = acc ^ 244982u64;
    acc = acc.wrapping_add(742507u64);
    acc = acc.wrapping_mul(57963u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 916951u64;
    acc = acc.wrapping_add(705837u64);
    acc = acc.wrapping_mul(17827u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(133469u64);
    acc = acc.wrapping_add(597051u64);
    acc = acc.wrapping_add(460495u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(701850u64);
    acc = acc.wrapping_add(187749u64);
    acc = acc ^ 489545u64;
    acc = acc.wrapping_add(455786u64);
    acc = acc.wrapping_add(137956u64);
    acc = acc.wrapping_mul(56615u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(49443u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(45469u64);
    acc = acc.rotate_left(28);
    acc = acc ^ 890023u64;
    acc = acc ^ 287303u64;
    acc = acc ^ 904987u64;
    acc = acc ^ 169424u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(51863u64);
    acc = acc.wrapping_add(495896u64);
    acc = acc.wrapping_add(6503u64);
    acc = acc.wrapping_mul(38081u64);
    acc = acc.wrapping_add(241623u64);
    acc = acc.wrapping_add(367178u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(55617u64);
    acc = acc.wrapping_add(925909u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(18107u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(40959u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(43903u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(289276u64);
    acc = acc.wrapping_add(644538u64);
    acc = acc.wrapping_mul(41755u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(29383u64);
    acc = acc.wrapping_add(607502u64);
    acc = acc.wrapping_mul(13069u64);
    acc = acc ^ 141894u64;
    acc = acc ^ 439096u64;
    acc = acc.wrapping_mul(39607u64);
    acc = acc.wrapping_add(854427u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 133173u64;
    acc = acc ^ 721025u64;
    acc = acc ^ 880083u64;
    acc = acc ^ 982477u64;
    acc = acc.wrapping_add(815859u64);
    acc = acc.wrapping_add(333884u64);
    acc = acc.wrapping_add(382115u64);
    acc = acc.wrapping_add(492619u64);
    acc = acc ^ 981016u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 913562u64;
    acc = acc ^ 249363u64;
    acc = acc.rotate_left(29);
    acc = acc ^ 995591u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(323028u64);
    acc = acc ^ 239617u64;
    acc = acc.wrapping_mul(54653u64);
    acc = acc.wrapping_add(556062u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(8605u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(951126u64);
    acc = acc ^ 794500u64;
    acc = acc.wrapping_add(936837u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(811830u64);
    acc = acc.wrapping_mul(43121u64);
    acc = acc.wrapping_add(188744u64);
    acc = acc.wrapping_add(221355u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(7851u64);
    acc = acc.wrapping_mul(47921u64);
    acc = acc ^ 972058u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 766456u64;
    acc = acc.wrapping_mul(59219u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(464602u64);
    acc = acc.wrapping_add(551379u64);
    acc = acc ^ 549794u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 740786u64;
    acc = acc.wrapping_mul(62213u64);
    acc = acc ^ 356821u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(62331u64);
    acc = acc.wrapping_mul(49851u64);
    acc = acc.wrapping_mul(37945u64);
    acc = acc.wrapping_mul(31633u64);
    acc = acc.wrapping_mul(1789u64);
    acc = acc.wrapping_add(921972u64);
    acc = acc.wrapping_add(934978u64);
    acc = acc ^ 115247u64;
    acc = acc ^ 69885u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(30209u64);
    acc = acc.wrapping_mul(59055u64);
    acc = acc.wrapping_mul(11507u64);
    acc = acc.wrapping_add(664763u64);
    acc = acc ^ 91263u64;
    acc = acc.wrapping_mul(49405u64);
    acc = acc.wrapping_mul(28267u64);
    acc = acc.wrapping_mul(819u64);
    acc = acc.wrapping_add(1843u64);
    acc = acc.wrapping_add(73023u64);
    acc = acc ^ 944462u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 179784u64;
    acc = acc.rotate_left(22);
    acc = acc ^ 493052u64;
    acc = acc ^ 738017u64;
    acc = acc.wrapping_add(795237u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(20957u64);
    acc = acc.wrapping_mul(19313u64);
    acc = acc ^ 644072u64;
    acc = acc ^ 79203u64;
    acc = acc.wrapping_mul(11799u64);
    acc = acc.wrapping_mul(11697u64);
    acc = acc ^ 704120u64;
    acc = acc.wrapping_mul(16153u64);
    acc = acc.wrapping_mul(60849u64);
    acc = acc ^ 337889u64;
    acc = acc.wrapping_add(789880u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 573u64, b: -204i32, c: false } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(618u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V2(32u64) } }


struct SLvl_2_0 { inner: u16, tag: u8 }
struct SLvl_2_1 { inner: SLvl_2_0, tag: u8 }
struct SLvl_2_2 { inner: SLvl_2_1, tag: u8 }

fn sbump_2_0(v: &mut SLvl_2_2) { sbump_2_1(&mut v.inner); }
fn sbump_2_1(v: &mut SLvl_2_1) { sbump_2_2(&mut v.inner); }
fn sbump_2_2(v: &mut SLvl_2_0) { v.inner = v.inner.wrapping_add(3); }

    pub fn run() {
        assert!(!(2866929267u64) == (2866929267u64) ^ u64::MAX);
        assert!(!(2116977009u64) == (2116977009u64) ^ u64::MAX);
        assert!((2595088286u64).wrapping_add(2595088286u64).wrapping_sub(2595088286u64) == 2595088286u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(4185558639u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(4073863640u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3477292627u64)));
        {

    let mut val = SLvl_2_2 { inner: SLvl_2_1 { inner: SLvl_2_0 { inner: 12379, tag: 62 }, tag: 69 }, tag: 76 };
    sbump_2_0(&mut val);
    let leaf = std::hint::black_box(val.inner.inner.inner);
    print!("case_34:"); println!("{}", leaf);
    print!("case_34:"); println!("tag0={}", val.tag);
    print!("case_34:"); println!("tag1={}", val.inner.tag);
    print!("case_34:"); println!("tag2={}", val.inner.inner.tag);

        }
        assert!(!(1657235300u64) == (1657235300u64) ^ u64::MAX);
        assert!(!(1335023567u64) == (1335023567u64) ^ u64::MAX);
        debug_assert!((2517736364u32).rotate_left(7).rotate_right(7) == 2517736364u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(6268401u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3651482658u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3034636269u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_35 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(8618u64).rotate_left(25) ^ 2814u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(5994u64).rotate_left(15) ^ 9159u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(66u64).rotate_left(10) ^ 4380u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(2206u64).rotate_left(21) ^ 8018u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(56421u64);
    acc = acc ^ 921085u64;
    acc = acc.wrapping_add(407011u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(804285u64);
    acc = acc.wrapping_mul(39873u64);
    acc = acc.wrapping_mul(11833u64);
    acc = acc.wrapping_add(392046u64);
    acc = acc.wrapping_add(590807u64);
    acc = acc.wrapping_mul(27473u64);
    acc = acc.wrapping_mul(29405u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(813u64);
    acc = acc.wrapping_add(63282u64);
    acc = acc.wrapping_add(307704u64);
    acc = acc.wrapping_add(504497u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(36605u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(397939u64);
    acc = acc ^ 824761u64;
    acc = acc ^ 344429u64;
    acc = acc.wrapping_add(908195u64);
    acc = acc.wrapping_mul(28123u64);
    acc = acc ^ 237871u64;
    acc = acc.wrapping_mul(57519u64);
    acc = acc.wrapping_mul(20315u64);
    acc = acc.wrapping_add(65791u64);
    acc = acc ^ 905819u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(164998u64);
    acc = acc.wrapping_mul(52683u64);
    acc = acc.wrapping_add(79670u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 432430u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(154702u64);
    acc = acc.wrapping_add(750005u64);
    acc = acc ^ 993162u64;
    acc = acc ^ 286023u64;
    acc = acc.wrapping_add(899006u64);
    acc = acc ^ 114636u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc ^ 239133u64;
    acc = acc.wrapping_add(661936u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(770775u64);
    acc = acc.wrapping_mul(19493u64);
    acc = acc.wrapping_add(192265u64);
    acc = acc ^ 807816u64;
    acc = acc.wrapping_add(490090u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(426441u64);
    acc = acc.wrapping_mul(36139u64);
    acc = acc.wrapping_mul(49237u64);
    acc = acc.wrapping_mul(29167u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(295151u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(266343u64);
    acc = acc.wrapping_add(688020u64);
    acc = acc ^ 620231u64;
    acc = acc.wrapping_mul(58607u64);
    acc = acc.wrapping_add(155803u64);
    acc = acc.wrapping_mul(62285u64);
    acc = acc ^ 178951u64;
    acc = acc.wrapping_mul(47731u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(522850u64);
    acc = acc ^ 865913u64;
    acc = acc ^ 247502u64;
    acc = acc.wrapping_add(643261u64);
    acc = acc ^ 439675u64;
    acc = acc ^ 495489u64;
    acc = acc.wrapping_mul(38101u64);
    acc = acc.wrapping_add(7507u64);
    acc = acc ^ 137175u64;
    acc = acc ^ 490773u64;
    acc = acc.wrapping_mul(61753u64);
    acc = acc.wrapping_add(408428u64);
    acc = acc ^ 163167u64;
    acc = acc.wrapping_mul(41277u64);
    acc = acc ^ 729262u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(38240u64);
    acc = acc.wrapping_add(839632u64);
    acc = acc.wrapping_mul(20263u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(25443u64);
    acc = acc ^ 538924u64;
    acc = acc ^ 708721u64;
    acc = acc.wrapping_add(593588u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 163541u64;
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(732356u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(15753u64);
    acc = acc.wrapping_add(668666u64);
    acc = acc.wrapping_add(245336u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc ^ 728537u64;
    acc = acc.rotate_left(2);
    acc = acc ^ 900275u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(832782u64);
    acc = acc.wrapping_mul(38623u64);
    acc = acc.wrapping_add(998911u64);
    acc = acc.wrapping_mul(33731u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 127111u64;
    acc = acc.wrapping_mul(4741u64);
    acc = acc ^ 218523u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 78670u64;
    acc = acc ^ 886682u64;
    acc = acc.wrapping_mul(63147u64);
    acc = acc.wrapping_mul(48031u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 990950u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(10031u64);
    acc = acc ^ 987018u64;
    acc = acc.wrapping_add(356322u64);
    acc = acc.wrapping_add(226181u64);
    acc = acc.wrapping_add(278797u64);
    acc = acc ^ 43899u64;
    acc = acc.wrapping_mul(64411u64);
    acc = acc.wrapping_add(992055u64);
    acc = acc.wrapping_add(40359u64);
    acc = acc.wrapping_mul(32225u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 561325u64;
    acc = acc.wrapping_add(914852u64);
    acc = acc ^ 274202u64;
    acc = acc ^ 241023u64;
    acc = acc ^ 803344u64;
    acc = acc.wrapping_add(175694u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(33199u64);
    acc = acc.wrapping_add(940589u64);
    acc = acc.wrapping_mul(35639u64);
    acc = acc.wrapping_add(580043u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc ^ 960020u64;
    acc = acc.wrapping_add(753827u64);
    acc = acc ^ 236511u64;
    acc = acc.wrapping_mul(58187u64);
    acc = acc ^ 619725u64;
    acc = acc ^ 641216u64;
    acc = acc.wrapping_add(430760u64);
    acc = acc.wrapping_add(247457u64);
    acc = acc.wrapping_mul(50981u64);
    acc = acc.wrapping_add(572749u64);
    acc = acc.wrapping_add(966398u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(42741u64);
    acc = acc.wrapping_add(886721u64);
    acc = acc.wrapping_mul(4677u64);
    acc = acc ^ 182827u64;
    acc = acc.wrapping_add(240459u64);
    acc = acc.wrapping_add(616157u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(641785u64);
    acc = acc ^ 765955u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(628051u64);
    acc = acc ^ 239362u64;
    acc = acc.wrapping_add(605246u64);
    acc = acc ^ 994863u64;
    acc = acc ^ 439392u64;
    acc = acc ^ 986602u64;
    acc = acc.wrapping_mul(42799u64);
    acc = acc.wrapping_mul(1203u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(447752u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(16129u64);
    acc = acc ^ 744735u64;
    acc = acc ^ 7824u64;
    acc = acc ^ 739294u64;
    acc = acc.wrapping_mul(5237u64);
    acc = acc.wrapping_add(430373u64);
    acc = acc.wrapping_mul(6791u64);
    acc = acc.wrapping_add(929501u64);
    acc = acc.wrapping_add(146804u64);
    acc = acc ^ 748290u64;
    acc = acc.wrapping_mul(33393u64);
    acc = acc.wrapping_add(757428u64);
    acc = acc.wrapping_add(17066u64);
    acc = acc.wrapping_add(654528u64);
    acc = acc.wrapping_mul(63365u64);
    acc = acc.wrapping_mul(19515u64);
    acc = acc ^ 759801u64;
    acc = acc.wrapping_mul(15803u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(33u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 512u64, b: 70i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 145u64, b: -634i32, c: true } } }


struct SLvl_3_0 { inner: i32, tag: u8 }
struct SLvl_3_1 { inner: SLvl_3_0, tag: u8 }
struct SLvl_3_2 { inner: SLvl_3_1, tag: u8 }

fn sbump_3_0(v: &mut SLvl_3_2) { sbump_3_1(&mut v.inner); }
fn sbump_3_1(v: &mut SLvl_3_1) { sbump_3_2(&mut v.inner); }
fn sbump_3_2(v: &mut SLvl_3_0) { v.inner = v.inner.wrapping_add(4); }

    pub fn run() {
        assert!(!(589410351u64) == (589410351u64) ^ u64::MAX);
        debug_assert!((1309921531u64).wrapping_mul(1) == 1309921531u64);
        debug_assert!((2543759529u32) | (2456946851u32) | (2543759529u32) == (2543759529u32) | (2456946851u32));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3265285185u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3886193249u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3675227739u64)));
        {

    let mut val = SLvl_3_2 { inner: SLvl_3_1 { inner: SLvl_3_0 { inner: 12396, tag: 93 }, tag: 100 }, tag: 107 };
    sbump_3_0(&mut val);
    let leaf = std::hint::black_box(val.inner.inner.inner);
    print!("case_35:"); println!("{}", leaf);
    print!("case_35:"); println!("tag0={}", val.tag);
    print!("case_35:"); println!("tag1={}", val.inner.tag);
    print!("case_35:"); println!("tag2={}", val.inner.inner.tag);

        }
        assert!((4047764822u64) ^ (4047764822u64) == 0u64);
        assert!(!(344664897u64) == (344664897u64) ^ u64::MAX);
        debug_assert!((1062820618u32) | (186630552u32) | (1062820618u32) == (1062820618u32) | (186630552u32));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1994216622u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2971538709u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2285207339u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_36 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(1257u64).rotate_left(26) ^ 2102u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(8485u64).rotate_left(11) ^ 1095u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(7381u64).rotate_left(19) ^ 4916u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(1119u64).rotate_left(29) ^ 8770u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(54701u64);
    acc = acc.wrapping_add(665175u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(17545u64);
    acc = acc.wrapping_mul(23265u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(651365u64);
    acc = acc.wrapping_add(769270u64);
    acc = acc.wrapping_add(633643u64);
    acc = acc.wrapping_add(21876u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 791382u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(43655u64);
    acc = acc.wrapping_mul(29081u64);
    acc = acc.wrapping_add(844651u64);
    acc = acc ^ 542795u64;
    acc = acc.wrapping_mul(15761u64);
    acc = acc ^ 239449u64;
    acc = acc ^ 394894u64;
    acc = acc.wrapping_add(421379u64);
    acc = acc.wrapping_add(562244u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(128685u64);
    acc = acc.wrapping_mul(24055u64);
    acc = acc ^ 913462u64;
    acc = acc.wrapping_add(585606u64);
    acc = acc.wrapping_add(104541u64);
    acc = acc ^ 377375u64;
    acc = acc.rotate_left(21);
    acc = acc ^ 786091u64;
    acc = acc ^ 307119u64;
    acc = acc.wrapping_mul(38293u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(31107u64);
    acc = acc.wrapping_mul(38209u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 702849u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(907714u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(19777u64);
    acc = acc.wrapping_add(865984u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(45089u64);
    acc = acc.wrapping_add(437487u64);
    acc = acc.wrapping_add(141711u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 76140u64;
    acc = acc ^ 951938u64;
    acc = acc.wrapping_mul(18109u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 213500u64;
    acc = acc.wrapping_add(333817u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(688050u64);
    acc = acc ^ 113696u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(632428u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(897431u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 172205u64;
    acc = acc ^ 248317u64;
    acc = acc.wrapping_add(779663u64);
    acc = acc.wrapping_mul(63545u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(8737u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(54443u64);
    acc = acc ^ 27744u64;
    acc = acc ^ 586793u64;
    acc = acc ^ 710441u64;
    acc = acc.wrapping_add(713680u64);
    acc = acc.rotate_left(9);
    acc = acc ^ 689417u64;
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(18795u64);
    acc = acc.wrapping_mul(30593u64);
    acc = acc.wrapping_mul(7257u64);
    acc = acc ^ 712318u64;
    acc = acc ^ 376493u64;
    acc = acc.wrapping_mul(19397u64);
    acc = acc ^ 882325u64;
    acc = acc ^ 620515u64;
    acc = acc.wrapping_add(461790u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(19510u64);
    acc = acc.wrapping_add(470120u64);
    acc = acc.wrapping_add(676031u64);
    acc = acc.wrapping_mul(52059u64);
    acc = acc ^ 750879u64;
    acc = acc.wrapping_add(888686u64);
    acc = acc.wrapping_add(425333u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(31049u64);
    acc = acc ^ 234077u64;
    acc = acc.wrapping_add(27601u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 17329u64;
    acc = acc ^ 26738u64;
    acc = acc.wrapping_mul(32229u64);
    acc = acc.wrapping_mul(55605u64);
    acc = acc ^ 445785u64;
    acc = acc ^ 901472u64;
    acc = acc.wrapping_add(881193u64);
    acc = acc.wrapping_mul(45821u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(723111u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(4761u64);
    acc = acc.wrapping_add(401212u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(831106u64);
    acc = acc.wrapping_add(13806u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 827851u64;
    acc = acc ^ 135790u64;
    acc = acc.wrapping_mul(55265u64);
    acc = acc ^ 678283u64;
    acc = acc.wrapping_mul(14069u64);
    acc = acc.wrapping_add(122737u64);
    acc = acc ^ 210381u64;
    acc = acc.wrapping_mul(28335u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(122391u64);
    acc = acc ^ 561244u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(627864u64);
    acc = acc.wrapping_mul(19547u64);
    acc = acc.wrapping_add(267278u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 413850u64;
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(13523u64);
    acc = acc.wrapping_add(4603u64);
    acc = acc ^ 202139u64;
    acc = acc.wrapping_mul(27321u64);
    acc = acc.wrapping_mul(12107u64);
    acc = acc.wrapping_mul(48081u64);
    acc = acc ^ 200334u64;
    acc = acc.wrapping_mul(51465u64);
    acc = acc.wrapping_add(763035u64);
    acc = acc.wrapping_add(262412u64);
    acc = acc ^ 704168u64;
    acc = acc.wrapping_add(478828u64);
    acc = acc.wrapping_add(873078u64);
    acc = acc.wrapping_mul(57199u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 668u64, b: 56i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 591u64, b: 802i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(839u64) } }


struct SLvl_4_0 { inner: u16, tag: u8 }
struct SLvl_4_1 { inner: SLvl_4_0, tag: u8 }
struct SLvl_4_2 { inner: SLvl_4_1, tag: u8 }
struct SLvl_4_3 { inner: SLvl_4_2, tag: u8 }
struct SLvl_4_4 { inner: SLvl_4_3, tag: u8 }

fn sbump_4_0(v: &mut SLvl_4_4) { sbump_4_1(&mut v.inner); }
fn sbump_4_1(v: &mut SLvl_4_3) { sbump_4_2(&mut v.inner); }
fn sbump_4_2(v: &mut SLvl_4_2) { sbump_4_3(&mut v.inner); }
fn sbump_4_3(v: &mut SLvl_4_1) { sbump_4_4(&mut v.inner); }
fn sbump_4_4(v: &mut SLvl_4_0) { v.inner = v.inner.wrapping_add(5); }

    pub fn run() {
        assert!(!(2861448917u64) == (2861448917u64) ^ u64::MAX);
        debug_assert!((215979165u64).wrapping_mul(1) == 215979165u64);
        assert!((599088079u64) ^ (599088079u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2835782918u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1467841004u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2992979788u64)));
        {

    let mut val = SLvl_4_4 { inner: SLvl_4_3 { inner: SLvl_4_2 { inner: SLvl_4_1 { inner: SLvl_4_0 { inner: 12413, tag: 124 }, tag: 131 }, tag: 138 }, tag: 145 }, tag: 152 };
    sbump_4_0(&mut val);
    let leaf = std::hint::black_box(val.inner.inner.inner.inner.inner);
    print!("case_36:"); println!("{}", leaf);
    print!("case_36:"); println!("tag0={}", val.tag);
    print!("case_36:"); println!("tag1={}", val.inner.tag);
    print!("case_36:"); println!("tag2={}", val.inner.inner.tag);
    print!("case_36:"); println!("tag3={}", val.inner.inner.inner.tag);
    print!("case_36:"); println!("tag4={}", val.inner.inner.inner.inner.tag);

        }
        debug_assert!((1262389081u32) | (2232287239u32) | (1262389081u32) == (1262389081u32) | (2232287239u32));
        debug_assert!((2625055727u32) | (2469795289u32) | (2625055727u32) == (2625055727u32) | (2469795289u32));
        assert!((2252930435u64).wrapping_add(2252930435u64).wrapping_sub(2252930435u64) == 2252930435u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(659826891u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3632904125u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2708664961u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_37 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(2211u64).rotate_left(6) ^ 935u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(1687u64).rotate_left(27) ^ 4242u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(8909u64).rotate_left(25) ^ 2442u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(3988u64).rotate_left(20) ^ 2840u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 395234u64;
    acc = acc ^ 803718u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(56777u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(860902u64);
    acc = acc ^ 708297u64;
    acc = acc.wrapping_mul(10913u64);
    acc = acc.wrapping_mul(8353u64);
    acc = acc ^ 239247u64;
    acc = acc.wrapping_mul(20661u64);
    acc = acc.wrapping_add(957645u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(795322u64);
    acc = acc.wrapping_add(925661u64);
    acc = acc ^ 527562u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc ^ 391308u64;
    acc = acc.wrapping_mul(17703u64);
    acc = acc.wrapping_add(216622u64);
    acc = acc.wrapping_add(289677u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(53980u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(15641u64);
    acc = acc.wrapping_add(795499u64);
    acc = acc.wrapping_add(148726u64);
    acc = acc.wrapping_mul(62597u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(311335u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc ^ 139201u64;
    acc = acc.wrapping_mul(26953u64);
    acc = acc.wrapping_add(184241u64);
    acc = acc.wrapping_add(332859u64);
    acc = acc ^ 587138u64;
    acc = acc ^ 556709u64;
    acc = acc.wrapping_mul(4911u64);
    acc = acc ^ 588981u64;
    acc = acc.wrapping_add(524799u64);
    acc = acc ^ 981907u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(59349u64);
    acc = acc.wrapping_mul(45975u64);
    acc = acc.wrapping_add(705668u64);
    acc = acc.wrapping_add(756178u64);
    acc = acc ^ 77671u64;
    acc = acc.wrapping_add(566427u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(685833u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(53931u64);
    acc = acc ^ 868309u64;
    acc = acc.wrapping_mul(56479u64);
    acc = acc ^ 249516u64;
    acc = acc.wrapping_add(566243u64);
    acc = acc ^ 990292u64;
    acc = acc ^ 847471u64;
    acc = acc ^ 74408u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(446793u64);
    acc = acc ^ 249234u64;
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(428504u64);
    acc = acc.wrapping_add(195280u64);
    acc = acc.wrapping_mul(33625u64);
    acc = acc ^ 350883u64;
    acc = acc.wrapping_mul(13767u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 900453u64;
    acc = acc.wrapping_add(799044u64);
    acc = acc ^ 829883u64;
    acc = acc ^ 314145u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(378177u64);
    acc = acc.wrapping_mul(3473u64);
    acc = acc.wrapping_mul(11607u64);
    acc = acc.wrapping_mul(52113u64);
    acc = acc ^ 134607u64;
    acc = acc.wrapping_add(677842u64);
    acc = acc ^ 939589u64;
    acc = acc.wrapping_add(150332u64);
    acc = acc.wrapping_add(345075u64);
    acc = acc.wrapping_mul(23447u64);
    acc = acc ^ 86248u64;
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(663162u64);
    acc = acc.wrapping_mul(22973u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(144257u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 988741u64;
    acc = acc.wrapping_add(717443u64);
    acc = acc.wrapping_add(806452u64);
    acc = acc.wrapping_mul(5675u64);
    acc = acc.wrapping_add(119240u64);
    acc = acc ^ 379318u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 632221u64;
    acc = acc ^ 495499u64;
    acc = acc.wrapping_add(132735u64);
    acc = acc ^ 371142u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 939551u64;
    acc = acc.wrapping_add(963220u64);
    acc = acc.wrapping_add(110528u64);
    acc = acc.wrapping_mul(30067u64);
    acc = acc ^ 908393u64;
    acc = acc.wrapping_mul(18079u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 718361u64;
    acc = acc.wrapping_mul(1377u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(177923u64);
    acc = acc ^ 56376u64;
    acc = acc ^ 420990u64;
    acc = acc.wrapping_add(469833u64);
    acc = acc.wrapping_mul(31453u64);
    acc = acc.wrapping_mul(59425u64);
    acc = acc.wrapping_add(196808u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(504255u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(243129u64);
    acc = acc.wrapping_mul(59121u64);
    acc = acc.wrapping_mul(34441u64);
    acc = acc.wrapping_mul(44775u64);
    acc = acc.wrapping_add(825520u64);
    acc = acc.wrapping_add(661114u64);
    acc = acc.wrapping_mul(38213u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(2359u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 243620u64;
    acc = acc.wrapping_add(951923u64);
    acc = acc ^ 518601u64;
    acc = acc ^ 692127u64;
    acc = acc ^ 247935u64;
    acc = acc.wrapping_mul(12245u64);
    acc = acc.wrapping_add(533767u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(494815u64);
    acc = acc.wrapping_add(716729u64);
    acc = acc ^ 425424u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(4307u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(839192u64);
    acc = acc.wrapping_mul(61451u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(261232u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(20969u64);
    acc = acc.wrapping_add(527753u64);
    acc = acc ^ 151873u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 849401u64;
    acc = acc.wrapping_mul(36341u64);
    acc = acc.wrapping_add(739846u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(62375u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 472199u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(59433u64);
    acc = acc.wrapping_mul(44089u64);
    acc = acc ^ 738357u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(369326u64);
    acc = acc.wrapping_mul(16963u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(240370u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 661965u64;
    acc = acc.wrapping_add(896138u64);
    acc = acc.wrapping_mul(3013u64);
    acc = acc ^ 971094u64;
    acc = acc.wrapping_add(378521u64);
    acc = acc.wrapping_mul(10875u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(820605u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(93185u64);
    acc = acc.wrapping_add(782799u64);
    acc = acc ^ 634417u64;
    acc = acc ^ 793374u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(40143u64);
    acc = acc.wrapping_add(873470u64);
    acc = acc.wrapping_mul(27679u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 180u64, b: -175i32, c: false } } }

enum NoiseType1 { V0(u64), V1(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(372u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(200u64) } }


struct SLvl_5_0 { inner: i32, tag: u8 }
struct SLvl_5_1 { inner: SLvl_5_0, tag: u8 }
struct SLvl_5_2 { inner: SLvl_5_1, tag: u8 }
struct SLvl_5_3 { inner: SLvl_5_2, tag: u8 }
struct SLvl_5_4 { inner: SLvl_5_3, tag: u8 }

fn sbump_5_0(v: &mut SLvl_5_4) { sbump_5_1(&mut v.inner); }
fn sbump_5_1(v: &mut SLvl_5_3) { sbump_5_2(&mut v.inner); }
fn sbump_5_2(v: &mut SLvl_5_2) { sbump_5_3(&mut v.inner); }
fn sbump_5_3(v: &mut SLvl_5_1) { sbump_5_4(&mut v.inner); }
fn sbump_5_4(v: &mut SLvl_5_0) { v.inner = v.inner.wrapping_add(6); }

    pub fn run() {
        assert!((3387411587u64) ^ (3387411587u64) == 0u64);
        debug_assert!((1823952154u32).rotate_left(16).rotate_right(16) == 1823952154u32);
        debug_assert!((3178330366u32).rotate_left(25).rotate_right(25) == 3178330366u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(4153391931u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3017970905u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2563725782u64)));
        {

    let mut val = SLvl_5_4 { inner: SLvl_5_3 { inner: SLvl_5_2 { inner: SLvl_5_1 { inner: SLvl_5_0 { inner: 12430, tag: 155 }, tag: 162 }, tag: 169 }, tag: 176 }, tag: 183 };
    sbump_5_0(&mut val);
    let leaf = std::hint::black_box(val.inner.inner.inner.inner.inner);
    print!("case_37:"); println!("{}", leaf);
    print!("case_37:"); println!("tag0={}", val.tag);
    print!("case_37:"); println!("tag1={}", val.inner.tag);
    print!("case_37:"); println!("tag2={}", val.inner.inner.tag);
    print!("case_37:"); println!("tag3={}", val.inner.inner.inner.tag);
    print!("case_37:"); println!("tag4={}", val.inner.inner.inner.inner.tag);

        }
        debug_assert!((3818907835u64).wrapping_mul(1) == 3818907835u64);
        assert!(!(223642239u64) == (223642239u64) ^ u64::MAX);
        assert!((4027391916u64).wrapping_add(4027391916u64).wrapping_sub(4027391916u64) == 4027391916u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2276860990u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(908228878u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2848796005u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_38 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(8662u64).rotate_left(26) ^ 8736u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(7594u64).rotate_left(27) ^ 4769u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(2790u64).rotate_left(28) ^ 6847u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(1718u64).rotate_left(8) ^ 8098u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(876295u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 814125u64;
    acc = acc.wrapping_mul(1755u64);
    acc = acc.wrapping_add(956190u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(895088u64);
    acc = acc.wrapping_add(340247u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc ^ 289500u64;
    acc = acc.wrapping_mul(3283u64);
    acc = acc.wrapping_add(895689u64);
    acc = acc ^ 30924u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 829205u64;
    acc = acc.wrapping_mul(63215u64);
    acc = acc.wrapping_mul(46743u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 892344u64;
    acc = acc.wrapping_add(802988u64);
    acc = acc.wrapping_add(961060u64);
    acc = acc.wrapping_mul(63089u64);
    acc = acc ^ 158581u64;
    acc = acc.wrapping_mul(17269u64);
    acc = acc.wrapping_add(757714u64);
    acc = acc.wrapping_mul(15509u64);
    acc = acc ^ 380427u64;
    acc = acc ^ 817382u64;
    acc = acc ^ 366312u64;
    acc = acc.wrapping_add(608863u64);
    acc = acc.wrapping_mul(51657u64);
    acc = acc.wrapping_mul(28915u64);
    acc = acc.wrapping_mul(33737u64);
    acc = acc.wrapping_mul(5239u64);
    acc = acc ^ 764480u64;
    acc = acc.wrapping_mul(26761u64);
    acc = acc.wrapping_mul(455u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 609532u64;
    acc = acc ^ 817197u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 664109u64;
    acc = acc ^ 697815u64;
    acc = acc.wrapping_add(390639u64);
    acc = acc.wrapping_mul(47333u64);
    acc = acc ^ 771719u64;
    acc = acc.wrapping_add(436921u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 559254u64;
    acc = acc.wrapping_add(928523u64);
    acc = acc.wrapping_mul(1157u64);
    acc = acc.wrapping_add(179028u64);
    acc = acc.wrapping_mul(36365u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(561404u64);
    acc = acc.wrapping_mul(62595u64);
    acc = acc.wrapping_mul(27661u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 926269u64;
    acc = acc ^ 79147u64;
    acc = acc.wrapping_mul(20845u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(392057u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 878816u64;
    acc = acc ^ 350346u64;
    acc = acc.wrapping_mul(26009u64);
    acc = acc.wrapping_mul(10489u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 58125u64;
    acc = acc.wrapping_mul(32767u64);
    acc = acc.wrapping_mul(46817u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(50297u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(87817u64);
    acc = acc ^ 795260u64;
    acc = acc.wrapping_add(375155u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 207336u64;
    acc = acc.wrapping_mul(41115u64);
    acc = acc.wrapping_mul(16963u64);
    acc = acc.wrapping_add(228536u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(613566u64);
    acc = acc.wrapping_add(199940u64);
    acc = acc ^ 506760u64;
    acc = acc.wrapping_add(831478u64);
    acc = acc.wrapping_add(21462u64);
    acc = acc.wrapping_mul(23423u64);
    acc = acc.wrapping_add(482377u64);
    acc = acc ^ 452369u64;
    acc = acc.wrapping_add(99303u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 785457u64;
    acc = acc ^ 216517u64;
    acc = acc.wrapping_add(354154u64);
    acc = acc.wrapping_mul(34373u64);
    acc = acc.wrapping_mul(22681u64);
    acc = acc.wrapping_add(863075u64);
    acc = acc ^ 516872u64;
    acc = acc.wrapping_mul(41095u64);
    acc = acc.wrapping_mul(59907u64);
    acc = acc ^ 941317u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 883727u64;
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(13295u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(581209u64);
    acc = acc.wrapping_mul(51401u64);
    acc = acc.wrapping_mul(8779u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 816666u64;
    acc = acc ^ 97230u64;
    acc = acc.wrapping_mul(63837u64);
    acc = acc.wrapping_add(992972u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(390094u64);
    acc = acc ^ 424368u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(47455u64);
    acc = acc.wrapping_add(902566u64);
    acc = acc ^ 548463u64;
    acc = acc.wrapping_add(916684u64);
    acc = acc.wrapping_mul(51339u64);
    acc = acc.wrapping_add(800488u64);
    acc = acc.wrapping_add(996588u64);
    acc = acc.wrapping_add(841327u64);
    acc = acc.wrapping_add(879561u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 273056u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(58425u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 722559u64;
    acc = acc.wrapping_mul(35683u64);
    acc = acc ^ 184569u64;
    acc = acc ^ 534044u64;
    acc = acc ^ 567738u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(393046u64);
    acc = acc.wrapping_add(117801u64);
    acc = acc.wrapping_mul(18357u64);
    acc = acc ^ 615286u64;
    acc = acc.wrapping_add(286681u64);
    acc = acc ^ 370955u64;
    acc = acc.wrapping_mul(43005u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(872965u64);
    acc = acc.wrapping_mul(33225u64);
    acc = acc.wrapping_mul(24091u64);
    acc = acc ^ 37453u64;
    acc = acc ^ 121089u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(327845u64);
    acc = acc.wrapping_mul(14729u64);
    acc = acc ^ 496366u64;
    acc = acc ^ 735091u64;
    acc = acc ^ 926910u64;
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 297u64, b: -904i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 196u64, b: -922i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 346u64, b: 302i32, c: false } } }


static SVALS_0: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_0 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((1804373408u64).wrapping_add(1804373408u64).wrapping_sub(1804373408u64) == 1804373408u64);
        debug_assert!((1435399118u32) | (403847347u32) | (1435399118u32) == (1435399118u32) | (403847347u32));
        debug_assert!((3826828477u32) | (4124125236u32) | (3826828477u32) == (3826828477u32) | (4124125236u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3049287369u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1560296130u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1574200946u64)));
        {

    let mut u = SU_0 { r: &SVALS_0[0] };
    u.r = &SVALS_0[0];
    let read_0_0 = unsafe { *u.r };
    print!("case_38:"); println!("{}", read_0_0);
    u.p = &SVALS_0[3] as *const i64;
    let read_0_1 = unsafe { *u.p };
    print!("case_38:"); println!("{}", read_0_1);

        }
        assert!((2858036666u64).wrapping_add(2858036666u64).wrapping_sub(2858036666u64) == 2858036666u64);
        assert!(!(1081202565u64) == (1081202565u64) ^ u64::MAX);
        debug_assert!((3512632794u64).wrapping_mul(1) == 3512632794u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3232185130u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1204132286u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3923725636u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_39 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(154u64).rotate_left(26) ^ 1048u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(4432u64).rotate_left(28) ^ 2972u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(9489u64).rotate_left(28) ^ 9017u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(3631u64).rotate_left(28) ^ 7436u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(374138u64);
    acc = acc ^ 199093u64;
    acc = acc.wrapping_add(37517u64);
    acc = acc.wrapping_mul(26579u64);
    acc = acc.wrapping_mul(27341u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(49203u64);
    acc = acc.wrapping_add(576909u64);
    acc = acc ^ 177945u64;
    acc = acc.wrapping_mul(34573u64);
    acc = acc.wrapping_mul(19613u64);
    acc = acc.wrapping_mul(5043u64);
    acc = acc ^ 485652u64;
    acc = acc ^ 328193u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(430910u64);
    acc = acc.wrapping_mul(32991u64);
    acc = acc ^ 389722u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(30037u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(697729u64);
    acc = acc ^ 318160u64;
    acc = acc ^ 372043u64;
    acc = acc.wrapping_add(747696u64);
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc ^ 268741u64;
    acc = acc.wrapping_add(967387u64);
    acc = acc ^ 835782u64;
    acc = acc.wrapping_mul(26351u64);
    acc = acc.wrapping_add(201662u64);
    acc = acc ^ 661788u64;
    acc = acc.wrapping_add(605870u64);
    acc = acc.wrapping_add(959452u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(6715u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(33597u64);
    acc = acc ^ 431825u64;
    acc = acc.wrapping_mul(22287u64);
    acc = acc.wrapping_add(464672u64);
    acc = acc.wrapping_mul(29839u64);
    acc = acc.wrapping_mul(36059u64);
    acc = acc.wrapping_add(553330u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 705981u64;
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(157503u64);
    acc = acc.wrapping_add(831180u64);
    acc = acc.wrapping_mul(577u64);
    acc = acc.wrapping_mul(18563u64);
    acc = acc ^ 115008u64;
    acc = acc ^ 697284u64;
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(35159u64);
    acc = acc.wrapping_mul(333u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(256095u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(60593u64);
    acc = acc ^ 670565u64;
    acc = acc.wrapping_mul(50845u64);
    acc = acc.wrapping_add(176591u64);
    acc = acc ^ 876131u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(23731u64);
    acc = acc.wrapping_mul(46345u64);
    acc = acc ^ 247425u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(25029u64);
    acc = acc.wrapping_mul(64361u64);
    acc = acc.wrapping_mul(64715u64);
    acc = acc.wrapping_add(755662u64);
    acc = acc.wrapping_add(441521u64);
    acc = acc.wrapping_mul(62799u64);
    acc = acc.wrapping_add(604450u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(101835u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(785898u64);
    acc = acc.wrapping_mul(14283u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 145485u64;
    acc = acc.rotate_left(22);
    acc = acc ^ 391386u64;
    acc = acc.wrapping_mul(26897u64);
    acc = acc.wrapping_add(319482u64);
    acc = acc ^ 568800u64;
    acc = acc.wrapping_add(106215u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(19221u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 800065u64;
    acc = acc ^ 752692u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(48391u64);
    acc = acc.wrapping_mul(30013u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(894205u64);
    acc = acc.wrapping_add(572217u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc ^ 295462u64;
    acc = acc.wrapping_add(471022u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 634440u64;
    acc = acc ^ 65042u64;
    acc = acc ^ 25054u64;
    acc = acc.wrapping_mul(40081u64);
    acc = acc.wrapping_add(855733u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(314293u64);
    acc = acc.wrapping_mul(22591u64);
    acc = acc ^ 179030u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 602533u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(971312u64);
    acc = acc ^ 580141u64;
    acc = acc ^ 810005u64;
    acc = acc.wrapping_mul(9719u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(30639u64);
    acc = acc ^ 701477u64;
    acc = acc.wrapping_add(988412u64);
    acc = acc ^ 422941u64;
    acc = acc ^ 768223u64;
    acc = acc.wrapping_mul(8111u64);
    acc = acc ^ 694820u64;
    acc = acc ^ 291691u64;
    acc = acc.wrapping_mul(58755u64);
    acc = acc.wrapping_add(91142u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(144808u64);
    acc = acc ^ 852389u64;
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc ^ 184084u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(605639u64);
    acc = acc.wrapping_mul(61415u64);
    acc = acc.wrapping_add(93048u64);
    acc = acc.wrapping_add(292915u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(654014u64);
    acc = acc ^ 273255u64;
    acc = acc.wrapping_add(55459u64);
    acc = acc.wrapping_mul(64417u64);
    acc = acc.wrapping_mul(48567u64);
    acc = acc.wrapping_mul(35611u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 715395u64;
    acc = acc.wrapping_mul(54249u64);
    acc = acc.wrapping_add(288136u64);
    acc = acc.wrapping_mul(12033u64);
    acc = acc.wrapping_add(932151u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc ^ 29237u64;
    acc = acc.rotate_left(28);
    acc = acc ^ 933900u64;
    acc = acc ^ 41379u64;
    acc = acc ^ 626197u64;
    acc = acc ^ 292632u64;
    acc = acc ^ 430090u64;
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(33327u64);
    acc = acc.wrapping_mul(14905u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(48081u64);
    acc = acc.wrapping_add(130501u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(681u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(650u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(491u64) } }


static SVALS_1: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_1 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((2512981682u64) ^ (2512981682u64) == 0u64);
        debug_assert!((3465915600u32) | (521414321u32) | (3465915600u32) == (3465915600u32) | (521414321u32));
        debug_assert!((3765664281u64).wrapping_mul(1) == 3765664281u64);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(14992159u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3852476779u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(474118811u64)));
        {

    let mut u = SU_1 { p: &SVALS_1[0] as *const i64 };
    u.r = &SVALS_1[0];
    let read_1_0 = unsafe { *u.r };
    print!("case_39:"); println!("{}", read_1_0);
    u.p = &SVALS_1[3] as *const i64;
    let read_1_1 = unsafe { *u.p };
    print!("case_39:"); println!("{}", read_1_1);

        }
        assert!(!(3312153954u64) == (3312153954u64) ^ u64::MAX);
        debug_assert!((3049377379u32) | (1267199045u32) | (3049377379u32) == (3049377379u32) | (1267199045u32));
        assert!(!(485661646u64) == (485661646u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3571110265u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(555012061u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3569827654u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_40 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(1814u64).rotate_left(27) ^ 3188u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(8389u64).rotate_left(6) ^ 1276u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(3425u64).rotate_left(22) ^ 3276u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(6496u64).rotate_left(21) ^ 9258u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 150881u64;
    acc = acc.wrapping_add(356842u64);
    acc = acc ^ 108539u64;
    acc = acc ^ 881260u64;
    acc = acc.wrapping_add(183911u64);
    acc = acc ^ 519818u64;
    acc = acc.wrapping_mul(10911u64);
    acc = acc.wrapping_mul(50907u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 585166u64;
    acc = acc.wrapping_mul(27763u64);
    acc = acc.wrapping_mul(26003u64);
    acc = acc ^ 323944u64;
    acc = acc.rotate_left(2);
    acc = acc ^ 497883u64;
    acc = acc.wrapping_add(435666u64);
    acc = acc ^ 52196u64;
    acc = acc ^ 971905u64;
    acc = acc.wrapping_add(652646u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(18283u64);
    acc = acc.wrapping_mul(3803u64);
    acc = acc ^ 449778u64;
    acc = acc.wrapping_add(543155u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(29731u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 328779u64;
    acc = acc.wrapping_add(792073u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(255673u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 712242u64;
    acc = acc.wrapping_add(192756u64);
    acc = acc ^ 595625u64;
    acc = acc ^ 315646u64;
    acc = acc.wrapping_mul(37751u64);
    acc = acc.wrapping_add(181938u64);
    acc = acc.wrapping_mul(15703u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 88742u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(808290u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(64043u64);
    acc = acc.wrapping_mul(15731u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 734439u64;
    acc = acc ^ 779788u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(55511u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 79998u64;
    acc = acc ^ 741798u64;
    acc = acc.wrapping_add(183990u64);
    acc = acc.wrapping_add(993280u64);
    acc = acc.wrapping_add(848345u64);
    acc = acc ^ 915753u64;
    acc = acc.wrapping_mul(61261u64);
    acc = acc.wrapping_add(907574u64);
    acc = acc.wrapping_mul(58247u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(45593u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(44397u64);
    acc = acc.wrapping_add(953528u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(637966u64);
    acc = acc.wrapping_add(445187u64);
    acc = acc.wrapping_add(102230u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(17657u64);
    acc = acc.wrapping_mul(63793u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 907963u64;
    acc = acc.wrapping_mul(36619u64);
    acc = acc ^ 229853u64;
    acc = acc.wrapping_mul(64967u64);
    acc = acc.wrapping_add(884997u64);
    acc = acc.wrapping_mul(56253u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(19845u64);
    acc = acc.wrapping_mul(56767u64);
    acc = acc ^ 407008u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 347671u64;
    acc = acc ^ 632633u64;
    acc = acc ^ 798861u64;
    acc = acc.wrapping_add(859271u64);
    acc = acc.wrapping_add(611110u64);
    acc = acc.wrapping_add(557545u64);
    acc = acc.wrapping_add(195492u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 864853u64;
    acc = acc.wrapping_add(145500u64);
    acc = acc.wrapping_mul(58683u64);
    acc = acc ^ 153694u64;
    acc = acc ^ 977180u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(37457u64);
    acc = acc.wrapping_add(2454u64);
    acc = acc ^ 959808u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(419670u64);
    acc = acc ^ 811743u64;
    acc = acc.wrapping_mul(34259u64);
    acc = acc.wrapping_mul(48169u64);
    acc = acc.wrapping_mul(791u64);
    acc = acc.wrapping_add(102442u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(566478u64);
    acc = acc ^ 601712u64;
    acc = acc.wrapping_add(359647u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(41719u64);
    acc = acc.wrapping_add(419714u64);
    acc = acc.wrapping_mul(15989u64);
    acc = acc ^ 756816u64;
    acc = acc ^ 721756u64;
    acc = acc.wrapping_add(566288u64);
    acc = acc.wrapping_add(382295u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 550390u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(31957u64);
    acc = acc.wrapping_mul(41945u64);
    acc = acc.wrapping_add(342794u64);
    acc = acc.wrapping_add(696941u64);
    acc = acc.wrapping_mul(50941u64);
    acc = acc.wrapping_add(779349u64);
    acc = acc.wrapping_add(890878u64);
    acc = acc ^ 616314u64;
    acc = acc.wrapping_mul(9645u64);
    acc = acc.wrapping_add(787883u64);
    acc = acc.wrapping_add(851513u64);
    acc = acc ^ 810411u64;
    acc = acc.wrapping_add(898489u64);
    acc = acc.wrapping_mul(53933u64);
    acc = acc.wrapping_add(272880u64);
    acc = acc.wrapping_add(344221u64);
    acc = acc.wrapping_mul(59431u64);
    acc = acc.wrapping_add(381870u64);
    acc = acc ^ 326576u64;
    acc = acc.wrapping_mul(2991u64);
    acc = acc ^ 275013u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 828320u64;
    acc = acc.wrapping_add(710920u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 198037u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(45263u64);
    acc = acc ^ 142792u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(143073u64);
    acc = acc ^ 571554u64;
    acc = acc.wrapping_mul(21611u64);
    acc = acc.wrapping_add(914712u64);
    acc = acc.wrapping_mul(41811u64);
    acc = acc.wrapping_mul(5433u64);
    acc = acc ^ 377785u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(216555u64);
    acc = acc.wrapping_add(525107u64);
    acc = acc ^ 304937u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(7031u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 452673u64;
    acc = acc ^ 807622u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(41897u64);
    acc = acc.wrapping_add(526780u64);
    acc = acc.wrapping_mul(40149u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(63845u64);
    acc = acc ^ 659412u64;
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(846u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 790u64, b: -964i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 597u64, b: -138i32, c: true } } }


static SVALS_2: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_2 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((587522447u32) | (2922530216u32) | (587522447u32) == (587522447u32) | (2922530216u32));
        assert!((2861415918u64) ^ (2861415918u64) == 0u64);
        debug_assert!((768647183u32) | (4105040530u32) | (768647183u32) == (768647183u32) | (4105040530u32));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3861734748u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1732958639u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3426904669u64)));
        {

    let mut u = SU_2 { r: &SVALS_2[0] };
    u.r = &SVALS_2[0];
    let read_2_0 = unsafe { *u.p };
    print!("case_40:"); println!("{}", read_2_0);
    u.p = &SVALS_2[1] as *const i64;
    let read_2_1 = unsafe { *u.r };
    print!("case_40:"); println!("{}", read_2_1);

        }
        assert!((644739883u64).wrapping_add(644739883u64).wrapping_sub(644739883u64) == 644739883u64);
        debug_assert!((1198128025u32).rotate_left(19).rotate_right(19) == 1198128025u32);
        debug_assert!((3173746715u32).rotate_left(8).rotate_right(8) == 3173746715u32);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1939998358u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2527816029u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2975523503u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_41 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(4759u64).rotate_left(15) ^ 4912u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(67u64).rotate_left(23) ^ 9632u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(9915u64).rotate_left(5) ^ 2856u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(5504u64).rotate_left(31) ^ 7563u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(16881u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 274403u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(19251u64);
    acc = acc.wrapping_add(881825u64);
    acc = acc.wrapping_mul(19161u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(443697u64);
    acc = acc ^ 464371u64;
    acc = acc.wrapping_add(650602u64);
    acc = acc.wrapping_mul(65115u64);
    acc = acc.wrapping_add(371941u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(166614u64);
    acc = acc.wrapping_add(713246u64);
    acc = acc ^ 88000u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(754433u64);
    acc = acc.wrapping_add(567571u64);
    acc = acc ^ 904637u64;
    acc = acc ^ 130819u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(56837u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(15759u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc ^ 221237u64;
    acc = acc.wrapping_add(394623u64);
    acc = acc.wrapping_add(706218u64);
    acc = acc ^ 394357u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 21468u64;
    acc = acc.wrapping_add(804537u64);
    acc = acc.wrapping_mul(17395u64);
    acc = acc ^ 581345u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(844342u64);
    acc = acc ^ 997214u64;
    acc = acc.wrapping_mul(9635u64);
    acc = acc.wrapping_mul(42779u64);
    acc = acc ^ 798837u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(956608u64);
    acc = acc.wrapping_mul(31977u64);
    acc = acc.wrapping_add(520820u64);
    acc = acc.wrapping_mul(34407u64);
    acc = acc ^ 609649u64;
    acc = acc.wrapping_add(22607u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(987791u64);
    acc = acc.wrapping_add(104476u64);
    acc = acc ^ 409446u64;
    acc = acc.wrapping_add(152592u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(37901u64);
    acc = acc ^ 354499u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(182439u64);
    acc = acc.wrapping_mul(6903u64);
    acc = acc.wrapping_add(501738u64);
    acc = acc ^ 240781u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(17399u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 616943u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(872891u64);
    acc = acc ^ 296940u64;
    acc = acc.wrapping_mul(56667u64);
    acc = acc.wrapping_add(468138u64);
    acc = acc.wrapping_add(854388u64);
    acc = acc.wrapping_add(695227u64);
    acc = acc.wrapping_mul(23863u64);
    acc = acc ^ 852670u64;
    acc = acc ^ 533571u64;
    acc = acc.wrapping_add(317790u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(262207u64);
    acc = acc ^ 448470u64;
    acc = acc ^ 113383u64;
    acc = acc.wrapping_add(433066u64);
    acc = acc ^ 568934u64;
    acc = acc.rotate_left(17);
    acc = acc ^ 471317u64;
    acc = acc.wrapping_mul(63471u64);
    acc = acc.wrapping_mul(34439u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(20455u64);
    acc = acc.wrapping_add(186250u64);
    acc = acc.wrapping_add(155273u64);
    acc = acc.wrapping_add(81383u64);
    acc = acc.wrapping_add(339941u64);
    acc = acc ^ 815682u64;
    acc = acc ^ 522043u64;
    acc = acc.wrapping_mul(15623u64);
    acc = acc.wrapping_mul(17891u64);
    acc = acc.wrapping_add(662352u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 758337u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(39503u64);
    acc = acc.wrapping_mul(15281u64);
    acc = acc.wrapping_mul(25303u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(673623u64);
    acc = acc.wrapping_add(154993u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(164799u64);
    acc = acc.wrapping_add(824655u64);
    acc = acc.wrapping_mul(18151u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(1117u64);
    acc = acc.wrapping_add(704481u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(17065u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(201366u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 696080u64;
    acc = acc.wrapping_add(565820u64);
    acc = acc ^ 934370u64;
    acc = acc.rotate_left(10);
    acc = acc ^ 980656u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(414325u64);
    acc = acc.wrapping_mul(47751u64);
    acc = acc.wrapping_add(287044u64);
    acc = acc.wrapping_mul(22183u64);
    acc = acc ^ 228530u64;
    acc = acc.wrapping_mul(6629u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 440645u64;
    acc = acc.wrapping_add(33800u64);
    acc = acc ^ 728378u64;
    acc = acc ^ 907415u64;
    acc = acc.wrapping_add(327643u64);
    acc = acc.wrapping_add(621935u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(532547u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 494324u64;
    acc = acc ^ 578428u64;
    acc = acc.wrapping_add(802833u64);
    acc = acc ^ 722357u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(44497u64);
    acc = acc.wrapping_mul(52051u64);
    acc = acc.wrapping_mul(64593u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 483187u64;
    acc = acc.wrapping_add(729374u64);
    acc = acc.wrapping_mul(35903u64);
    acc = acc ^ 555080u64;
    acc = acc.wrapping_mul(58429u64);
    acc = acc ^ 581448u64;
    acc = acc ^ 5657u64;
    acc = acc.wrapping_add(7827u64);
    acc = acc.wrapping_mul(60079u64);
    acc = acc ^ 639176u64;
    acc = acc ^ 761256u64;
    acc = acc.wrapping_mul(39511u64);
    acc = acc ^ 127538u64;
    acc = acc.wrapping_mul(6213u64);
    acc = acc.wrapping_add(740779u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 252862u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(22341u64);
    acc = acc.wrapping_mul(8925u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(865385u64);
    acc = acc.wrapping_mul(31929u64);
    acc = acc.wrapping_add(556114u64);
    acc = acc.wrapping_add(660452u64);
    acc = acc.wrapping_add(797271u64);
    acc = acc.wrapping_add(665377u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 596620u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(251472u64);
    acc = acc.wrapping_mul(6363u64);
    acc = acc ^ 47690u64;
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(483641u64);
    acc = acc.wrapping_add(663243u64);
    acc = acc ^ 282247u64;
    acc = acc.wrapping_add(838493u64);
    acc = acc.wrapping_mul(48421u64);
    acc = acc.wrapping_mul(49355u64);
    acc = acc ^ 9927u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(90371u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(6237u64);
    acc = acc ^ 496100u64;
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(706380u64);
    acc = acc.wrapping_add(589146u64);
    acc = acc.wrapping_add(776559u64);
    acc = acc ^ 768280u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(318207u64);
    acc = acc.wrapping_add(127217u64);
    acc = acc ^ 415620u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(741597u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(48833u64);
    acc = acc.wrapping_mul(56911u64);
    acc = acc ^ 309390u64;
    acc = acc ^ 921593u64;
    acc = acc.wrapping_mul(52195u64);
    acc = acc.wrapping_add(786958u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 993u64, b: 360i32, c: false } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(62u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 19u64, b: -673i32, c: false } } }


static SVALS_3: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_3 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((3191021357u32).rotate_left(13).rotate_right(13) == 3191021357u32);
        debug_assert!((1211961908u32) | (1716175677u32) | (1211961908u32) == (1211961908u32) | (1716175677u32));
        debug_assert!((2747106893u64).wrapping_mul(1) == 2747106893u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3016800876u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(53011305u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2853072781u64)));
        {

    let mut u = SU_3 { p: &SVALS_3[0] as *const i64 };
    u.r = &SVALS_3[0];
    let read_3_0 = unsafe { *u.p };
    print!("case_41:"); println!("{}", read_3_0);
    u.p = &SVALS_3[1] as *const i64;
    let read_3_1 = unsafe { *u.r };
    print!("case_41:"); println!("{}", read_3_1);

        }
        debug_assert!((2224372525u64).wrapping_mul(1) == 2224372525u64);
        debug_assert!((4132953305u32) | (2552789407u32) | (4132953305u32) == (4132953305u32) | (2552789407u32));
        assert!((3145450072u64) ^ (3145450072u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(307127051u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3436869007u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(675847753u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_42 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(9258u64).rotate_left(11) ^ 1374u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(3376u64).rotate_left(24) ^ 6099u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(737u64).rotate_left(11) ^ 8597u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(7741u64).rotate_left(26) ^ 1203u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(5717u64);
    acc = acc.wrapping_mul(20973u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(242193u64);
    acc = acc ^ 507598u64;
    acc = acc ^ 304582u64;
    acc = acc.wrapping_mul(58891u64);
    acc = acc.wrapping_mul(43489u64);
    acc = acc.wrapping_mul(24005u64);
    acc = acc.wrapping_add(872158u64);
    acc = acc.wrapping_mul(15109u64);
    acc = acc.wrapping_mul(14133u64);
    acc = acc ^ 167976u64;
    acc = acc.wrapping_mul(34983u64);
    acc = acc.wrapping_mul(34837u64);
    acc = acc.wrapping_mul(54063u64);
    acc = acc.wrapping_add(84602u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 278557u64;
    acc = acc.wrapping_mul(43481u64);
    acc = acc ^ 403641u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(19321u64);
    acc = acc ^ 168097u64;
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(18323u64);
    acc = acc ^ 455529u64;
    acc = acc.wrapping_add(695611u64);
    acc = acc ^ 517630u64;
    acc = acc.wrapping_mul(62453u64);
    acc = acc ^ 619933u64;
    acc = acc ^ 465392u64;
    acc = acc.wrapping_add(323764u64);
    acc = acc.wrapping_mul(32927u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(525007u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(810125u64);
    acc = acc.wrapping_mul(465u64);
    acc = acc.wrapping_mul(56797u64);
    acc = acc ^ 680029u64;
    acc = acc.wrapping_add(990157u64);
    acc = acc.wrapping_add(163843u64);
    acc = acc.wrapping_mul(51047u64);
    acc = acc ^ 981100u64;
    acc = acc.wrapping_mul(62363u64);
    acc = acc ^ 155718u64;
    acc = acc.wrapping_add(114442u64);
    acc = acc.wrapping_mul(25133u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 506136u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(32663u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(778764u64);
    acc = acc ^ 432374u64;
    acc = acc.wrapping_mul(50153u64);
    acc = acc.wrapping_mul(42771u64);
    acc = acc.wrapping_mul(50591u64);
    acc = acc ^ 284224u64;
    acc = acc.wrapping_add(251017u64);
    acc = acc.wrapping_add(578981u64);
    acc = acc ^ 213418u64;
    acc = acc.wrapping_mul(6527u64);
    acc = acc.wrapping_mul(46413u64);
    acc = acc ^ 924190u64;
    acc = acc ^ 128975u64;
    acc = acc ^ 840518u64;
    acc = acc.wrapping_add(867674u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(53597u64);
    acc = acc.wrapping_mul(30293u64);
    acc = acc.wrapping_mul(51733u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(557743u64);
    acc = acc.wrapping_add(104197u64);
    acc = acc.wrapping_mul(21455u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 11973u64;
    acc = acc.wrapping_add(983701u64);
    acc = acc.wrapping_add(986077u64);
    acc = acc ^ 34137u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 77237u64;
    acc = acc ^ 911436u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(7843u64);
    acc = acc.wrapping_add(234527u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 472665u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(251061u64);
    acc = acc.wrapping_add(536701u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(13951u64);
    acc = acc.wrapping_mul(57865u64);
    acc = acc.wrapping_mul(28551u64);
    acc = acc.wrapping_mul(23833u64);
    acc = acc.wrapping_add(644377u64);
    acc = acc ^ 33657u64;
    acc = acc ^ 152116u64;
    acc = acc.wrapping_add(697645u64);
    acc = acc.wrapping_add(628573u64);
    acc = acc.wrapping_mul(53769u64);
    acc = acc.wrapping_mul(18075u64);
    acc = acc.wrapping_add(605105u64);
    acc = acc.wrapping_mul(7869u64);
    acc = acc.wrapping_add(380874u64);
    acc = acc ^ 392417u64;
    acc = acc.wrapping_mul(12981u64);
    acc = acc.wrapping_mul(56559u64);
    acc = acc ^ 907868u64;
    acc = acc.rotate_left(21);
    acc = acc.wrapping_mul(20127u64);
    acc = acc.wrapping_add(980300u64);
    acc = acc ^ 635842u64;
    acc = acc ^ 558804u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(213419u64);
    acc = acc ^ 494068u64;
    acc = acc ^ 399108u64;
    acc = acc.wrapping_add(306992u64);
    acc = acc.wrapping_mul(34583u64);
    acc = acc ^ 600898u64;
    acc = acc ^ 264738u64;
    acc = acc ^ 482671u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 311950u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(29375u64);
    acc = acc ^ 306287u64;
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(595830u64);
    acc = acc ^ 374442u64;
    acc = acc.wrapping_add(250327u64);
    acc = acc.wrapping_mul(11071u64);
    acc = acc.wrapping_add(771345u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(286899u64);
    acc = acc.wrapping_mul(41949u64);
    acc = acc ^ 859793u64;
    acc = acc.wrapping_mul(17541u64);
    acc = acc ^ 515031u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(863020u64);
    acc = acc ^ 908318u64;
    acc = acc.wrapping_mul(22539u64);
    acc = acc.wrapping_mul(32393u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc ^ 67365u64;
    acc = acc.wrapping_mul(30513u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(611645u64);
    acc = acc ^ 586465u64;
    acc = acc ^ 753037u64;
    acc = acc ^ 220555u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(737586u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(336701u64);
    acc = acc.wrapping_mul(64251u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(36585u64);
    acc = acc.wrapping_mul(3617u64);
    acc = acc ^ 886423u64;
    acc = acc ^ 475849u64;
    acc = acc.wrapping_mul(19725u64);
    acc = acc ^ 379811u64;
    acc = acc ^ 707300u64;
    acc = acc.wrapping_mul(25157u64);
    acc = acc.wrapping_mul(7641u64);
    acc = acc.wrapping_mul(46745u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 323378u64;
    acc = acc ^ 161775u64;
    acc = acc.wrapping_add(235338u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(29015u64);
    acc = acc ^ 604887u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(46879u64);
    acc = acc.wrapping_mul(2393u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(29855u64);
    acc = acc.wrapping_add(662411u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(314354u64);
    acc = acc.wrapping_add(752721u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(512u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 739u64, b: -816i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(604u64) } }


static SVALS_4: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_4 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((338004651u64) ^ (338004651u64) == 0u64);
        debug_assert!((116083019u32).rotate_left(29).rotate_right(29) == 116083019u32);
        debug_assert!((1663060568u32) | (2870283129u32) | (1663060568u32) == (1663060568u32) | (2870283129u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2221143917u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(271512451u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3674111459u64)));
        {

    let mut u = SU_4 { r: &SVALS_4[0] };
    u.p = &SVALS_4[0] as *const i64;
    let read_4_0 = unsafe { *u.p };
    print!("case_42:"); println!("{}", read_4_0);
    u.r = &SVALS_4[3];
    let read_4_1 = unsafe { *u.r };
    print!("case_42:"); println!("{}", read_4_1);
    u.p = &SVALS_4[2] as *const i64;
    let read_4_2 = unsafe { *u.r };
    print!("case_42:"); println!("{}", read_4_2);

        }
        assert!((3570501271u64).wrapping_add(3570501271u64).wrapping_sub(3570501271u64) == 3570501271u64);
        assert!((4129035994u64).wrapping_add(4129035994u64).wrapping_sub(4129035994u64) == 4129035994u64);
        debug_assert!((1553288390u32).rotate_left(17).rotate_right(17) == 1553288390u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3707213689u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3587830075u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(4163981897u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_43 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(5211u64).rotate_left(19) ^ 452u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(6179u64).rotate_left(31) ^ 9764u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(4199u64).rotate_left(9) ^ 7634u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(3809u64).rotate_left(30) ^ 2788u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 681271u64;
    acc = acc ^ 999375u64;
    acc = acc.wrapping_add(58541u64);
    acc = acc.wrapping_add(185276u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(75975u64);
    acc = acc.wrapping_add(248232u64);
    acc = acc ^ 524327u64;
    acc = acc.wrapping_mul(20083u64);
    acc = acc.wrapping_add(275554u64);
    acc = acc.wrapping_add(543302u64);
    acc = acc.wrapping_mul(30791u64);
    acc = acc.wrapping_add(57260u64);
    acc = acc.wrapping_add(32598u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 180795u64;
    acc = acc ^ 640153u64;
    acc = acc.wrapping_add(728172u64);
    acc = acc.wrapping_mul(30219u64);
    acc = acc ^ 287883u64;
    acc = acc ^ 970022u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(220215u64);
    acc = acc ^ 750531u64;
    acc = acc.wrapping_mul(16089u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 927901u64;
    acc = acc ^ 509623u64;
    acc = acc.wrapping_mul(24053u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(54061u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(224171u64);
    acc = acc.wrapping_add(981660u64);
    acc = acc.wrapping_mul(30109u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(364526u64);
    acc = acc.wrapping_mul(53967u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(837u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc ^ 150092u64;
    acc = acc ^ 638728u64;
    acc = acc.wrapping_mul(51543u64);
    acc = acc ^ 287230u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(62179u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(764822u64);
    acc = acc ^ 750491u64;
    acc = acc ^ 992115u64;
    acc = acc ^ 653221u64;
    acc = acc ^ 992377u64;
    acc = acc ^ 593092u64;
    acc = acc.rotate_left(21);
    acc = acc ^ 568418u64;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(9369u64);
    acc = acc.wrapping_mul(4415u64);
    acc = acc.wrapping_mul(54731u64);
    acc = acc.wrapping_mul(48481u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(717927u64);
    acc = acc.wrapping_add(44238u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(324986u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(535339u64);
    acc = acc.wrapping_add(837821u64);
    acc = acc ^ 94734u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(337436u64);
    acc = acc ^ 30289u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(367478u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(38818u64);
    acc = acc ^ 175169u64;
    acc = acc ^ 799067u64;
    acc = acc.wrapping_mul(38833u64);
    acc = acc.wrapping_add(42727u64);
    acc = acc.wrapping_add(300991u64);
    acc = acc.wrapping_add(232991u64);
    acc = acc ^ 904095u64;
    acc = acc.wrapping_add(388590u64);
    acc = acc ^ 516452u64;
    acc = acc.wrapping_add(700969u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(729673u64);
    acc = acc.wrapping_add(512387u64);
    acc = acc ^ 741799u64;
    acc = acc.wrapping_add(5973u64);
    acc = acc ^ 209861u64;
    acc = acc.wrapping_mul(57401u64);
    acc = acc.wrapping_mul(40089u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 815468u64;
    acc = acc.rotate_left(7);
    acc = acc ^ 564196u64;
    acc = acc ^ 540630u64;
    acc = acc ^ 347253u64;
    acc = acc.wrapping_mul(26793u64);
    acc = acc.wrapping_mul(51553u64);
    acc = acc ^ 255222u64;
    acc = acc.wrapping_add(612327u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(538880u64);
    acc = acc.wrapping_add(112361u64);
    acc = acc.wrapping_mul(55733u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(13799u64);
    acc = acc.wrapping_mul(53899u64);
    acc = acc.wrapping_mul(13539u64);
    acc = acc ^ 948417u64;
    acc = acc.wrapping_add(640356u64);
    acc = acc ^ 621210u64;
    acc = acc.wrapping_mul(22055u64);
    acc = acc.wrapping_add(251662u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 89098u64;
    acc = acc.wrapping_add(872831u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(15419u64);
    acc = acc.wrapping_mul(64567u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(259686u64);
    acc = acc.wrapping_mul(52039u64);
    acc = acc.wrapping_mul(61263u64);
    acc = acc.wrapping_add(195177u64);
    acc = acc.wrapping_add(495185u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(43463u64);
    acc = acc.wrapping_add(75621u64);
    acc = acc.wrapping_mul(11665u64);
    acc = acc ^ 66813u64;
    acc = acc.wrapping_mul(18205u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 813416u64;
    acc = acc ^ 953688u64;
    acc = acc.wrapping_mul(1671u64);
    acc = acc.wrapping_mul(14527u64);
    acc = acc ^ 733138u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(586145u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(53797u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(228u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(845u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 693u64, b: -850i32, c: true } } }


static SVALS_5: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_5 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((3190178346u32) | (3301602320u32) | (3190178346u32) == (3190178346u32) | (3301602320u32));
        assert!(!(2156357312u64) == (2156357312u64) ^ u64::MAX);
        assert!((2453427210u64) ^ (2453427210u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1717704804u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2148043468u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(750072988u64)));
        {

    let mut u = SU_5 { p: &SVALS_5[0] as *const i64 };
    u.p = &SVALS_5[0] as *const i64;
    let read_5_0 = unsafe { *u.p };
    print!("case_43:"); println!("{}", read_5_0);
    u.r = &SVALS_5[3];
    let read_5_1 = unsafe { *u.r };
    print!("case_43:"); println!("{}", read_5_1);
    u.p = &SVALS_5[2] as *const i64;
    let read_5_2 = unsafe { *u.r };
    print!("case_43:"); println!("{}", read_5_2);

        }
        assert!(!(2832744994u64) == (2832744994u64) ^ u64::MAX);
        debug_assert!((3246833068u32).rotate_left(11).rotate_right(11) == 3246833068u32);
        debug_assert!((3722409801u64).wrapping_mul(1) == 3722409801u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2872840392u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1871452370u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1771480299u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_44 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(2396u64).rotate_left(17) ^ 3364u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(9961u64).rotate_left(1) ^ 4495u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(3766u64).rotate_left(30) ^ 9360u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(304u64).rotate_left(25) ^ 1760u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(419676u64);
    acc = acc.wrapping_add(669842u64);
    acc = acc.wrapping_mul(13417u64);
    acc = acc ^ 981736u64;
    acc = acc.wrapping_mul(59701u64);
    acc = acc.wrapping_add(691327u64);
    acc = acc.wrapping_mul(14207u64);
    acc = acc.wrapping_add(46673u64);
    acc = acc.wrapping_add(473241u64);
    acc = acc.wrapping_add(677953u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(41131u64);
    acc = acc.wrapping_mul(12493u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(7658u64);
    acc = acc ^ 3967u64;
    acc = acc ^ 97569u64;
    acc = acc.wrapping_add(995920u64);
    acc = acc ^ 679551u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(568941u64);
    acc = acc.wrapping_mul(40995u64);
    acc = acc.wrapping_mul(42601u64);
    acc = acc ^ 8211u64;
    acc = acc.wrapping_add(880358u64);
    acc = acc.wrapping_add(578109u64);
    acc = acc.wrapping_add(424970u64);
    acc = acc.wrapping_mul(65447u64);
    acc = acc ^ 209862u64;
    acc = acc.wrapping_mul(23551u64);
    acc = acc.wrapping_mul(19287u64);
    acc = acc.wrapping_mul(42453u64);
    acc = acc ^ 646594u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 230093u64;
    acc = acc.wrapping_add(62503u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(24417u64);
    acc = acc ^ 92513u64;
    acc = acc.wrapping_add(565230u64);
    acc = acc.wrapping_mul(873u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(26069u64);
    acc = acc.wrapping_add(329991u64);
    acc = acc.wrapping_mul(44265u64);
    acc = acc.wrapping_mul(61695u64);
    acc = acc.wrapping_mul(15549u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(255509u64);
    acc = acc ^ 638818u64;
    acc = acc.wrapping_mul(50623u64);
    acc = acc.wrapping_add(118859u64);
    acc = acc ^ 709284u64;
    acc = acc.wrapping_add(102343u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc ^ 527222u64;
    acc = acc.rotate_left(7);
    acc = acc ^ 239957u64;
    acc = acc ^ 52915u64;
    acc = acc.wrapping_add(737949u64);
    acc = acc.wrapping_mul(50649u64);
    acc = acc.wrapping_add(694991u64);
    acc = acc ^ 460056u64;
    acc = acc.wrapping_add(580816u64);
    acc = acc.wrapping_add(61058u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(392318u64);
    acc = acc.wrapping_add(458270u64);
    acc = acc.wrapping_add(168528u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(85664u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(13671u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(564808u64);
    acc = acc.wrapping_add(940815u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 443057u64;
    acc = acc.wrapping_mul(3185u64);
    acc = acc.wrapping_add(412298u64);
    acc = acc.wrapping_add(347580u64);
    acc = acc.wrapping_add(439790u64);
    acc = acc.rotate_left(24);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(46121u64);
    acc = acc.wrapping_add(248933u64);
    acc = acc.wrapping_add(669391u64);
    acc = acc ^ 629356u64;
    acc = acc.wrapping_add(879140u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(35677u64);
    acc = acc ^ 102945u64;
    acc = acc ^ 870974u64;
    acc = acc ^ 202637u64;
    acc = acc.rotate_left(12);
    acc = acc ^ 865150u64;
    acc = acc.wrapping_mul(50725u64);
    acc = acc ^ 831501u64;
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(362208u64);
    acc = acc ^ 169856u64;
    acc = acc.wrapping_add(292203u64);
    acc = acc.wrapping_mul(19205u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(17277u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(746472u64);
    acc = acc ^ 8150u64;
    acc = acc ^ 823624u64;
    acc = acc.wrapping_mul(38523u64);
    acc = acc.wrapping_add(864248u64);
    acc = acc ^ 581194u64;
    acc = acc ^ 820669u64;
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc ^ 800535u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(932088u64);
    acc = acc.wrapping_mul(34133u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(716387u64);
    acc = acc ^ 997525u64;
    acc = acc ^ 934264u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(14445u64);
    acc = acc.wrapping_mul(4335u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(52731u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 47588u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(46741u64);
    acc = acc.wrapping_add(751057u64);
    acc = acc ^ 271274u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(209412u64);
    acc = acc ^ 298031u64;
    acc = acc.wrapping_add(662997u64);
    acc = acc ^ 944193u64;
    acc = acc.wrapping_add(408925u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(72067u64);
    acc = acc ^ 944713u64;
    acc = acc.wrapping_mul(38017u64);
    acc = acc ^ 707580u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(207u64);
    acc = acc.wrapping_mul(51851u64);
    acc = acc.wrapping_mul(28395u64);
    acc = acc.wrapping_add(138891u64);
    acc = acc.wrapping_add(167418u64);
    acc = acc ^ 764872u64;
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(157232u64);
    acc = acc ^ 679535u64;
    acc = acc.wrapping_add(742779u64);
    acc = acc.wrapping_add(86276u64);
    acc = acc.wrapping_add(521641u64);
    acc = acc.wrapping_mul(36689u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(786963u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(35345u64);
    acc = acc.wrapping_mul(27599u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 528651u64;
    acc = acc.wrapping_mul(13859u64);
    acc = acc.wrapping_add(814683u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(159789u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(684582u64);
    acc = acc.wrapping_add(986038u64);
    acc = acc.wrapping_mul(22575u64);
    acc = acc.wrapping_add(875524u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(704599u64);
    acc = acc ^ 24734u64;
    acc = acc ^ 898186u64;
    acc = acc ^ 278588u64;
    acc = acc ^ 248649u64;
    acc = acc.wrapping_add(121756u64);
    acc = acc.wrapping_mul(5763u64);
    acc = acc.wrapping_add(703783u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(9861u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 256121u64;
    acc = acc.wrapping_add(894787u64);
    acc = acc.wrapping_add(83806u64);
    acc = acc ^ 898888u64;
    acc = acc ^ 623724u64;
    acc = acc.wrapping_mul(2187u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(636u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 157u64, b: 444i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(405u64) } }


static SVALS_6: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_6 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((2902374634u64).wrapping_mul(1) == 2902374634u64);
        assert!((1517976768u64) ^ (1517976768u64) == 0u64);
        assert!((2742113822u64) ^ (2742113822u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2944901650u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(546749730u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3124053464u64)));
        {

    let mut u = SU_6 { r: &SVALS_6[0] };
    u.r = &SVALS_6[0];
    let read_6_0 = unsafe { *u.r };
    print!("case_44:"); println!("{}", read_6_0);
    u.r = &SVALS_6[3];
    let read_6_1 = unsafe { *u.p };
    print!("case_44:"); println!("{}", read_6_1);
    u.p = &SVALS_6[0] as *const i64;
    let read_6_2 = unsafe { *u.p };
    print!("case_44:"); println!("{}", read_6_2);
    u.p = &SVALS_6[3] as *const i64;
    let read_6_3 = unsafe { *u.r };
    print!("case_44:"); println!("{}", read_6_3);

        }
        debug_assert!((3155832098u32).rotate_left(15).rotate_right(15) == 3155832098u32);
        debug_assert!((2739988595u32) | (2213370521u32) | (2739988595u32) == (2739988595u32) | (2213370521u32));
        debug_assert!((1847140685u32) | (3624199191u32) | (1847140685u32) == (1847140685u32) | (3624199191u32));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3021478102u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(694957612u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1009741443u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_45 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(1187u64).rotate_left(7) ^ 5524u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(275u64).rotate_left(15) ^ 1440u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(1869u64).rotate_left(31) ^ 3347u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(7721u64).rotate_left(22) ^ 8422u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc ^ 406071u64;
    acc = acc.wrapping_add(493815u64);
    acc = acc.wrapping_mul(17903u64);
    acc = acc.wrapping_mul(58049u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(35985u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc ^ 404394u64;
    acc = acc.wrapping_mul(19331u64);
    acc = acc.wrapping_add(99487u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 584107u64;
    acc = acc ^ 93839u64;
    acc = acc.wrapping_mul(43477u64);
    acc = acc.wrapping_mul(63831u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(33145u64);
    acc = acc.wrapping_add(532176u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 530179u64;
    acc = acc ^ 450245u64;
    acc = acc.wrapping_add(581301u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(168605u64);
    acc = acc.wrapping_mul(36393u64);
    acc = acc ^ 825519u64;
    acc = acc ^ 832456u64;
    acc = acc.wrapping_mul(10821u64);
    acc = acc.wrapping_add(469131u64);
    acc = acc.wrapping_add(93532u64);
    acc = acc.wrapping_mul(1881u64);
    acc = acc.wrapping_mul(21005u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 176469u64;
    acc = acc.wrapping_add(948782u64);
    acc = acc ^ 483382u64;
    acc = acc.wrapping_add(436461u64);
    acc = acc.wrapping_mul(7195u64);
    acc = acc.wrapping_add(641865u64);
    acc = acc ^ 617024u64;
    acc = acc.wrapping_mul(42371u64);
    acc = acc.wrapping_mul(8749u64);
    acc = acc ^ 912905u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(49369u64);
    acc = acc.wrapping_add(349117u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 896115u64;
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(48353u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(614538u64);
    acc = acc.wrapping_mul(37811u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(999726u64);
    acc = acc ^ 779579u64;
    acc = acc ^ 935630u64;
    acc = acc.wrapping_mul(34395u64);
    acc = acc ^ 197048u64;
    acc = acc.rotate_left(11);
    acc = acc ^ 259135u64;
    acc = acc.wrapping_mul(31151u64);
    acc = acc ^ 315546u64;
    acc = acc.wrapping_mul(16315u64);
    acc = acc.wrapping_mul(29815u64);
    acc = acc.wrapping_mul(20637u64);
    acc = acc.wrapping_add(171904u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(52451u64);
    acc = acc ^ 466435u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(317198u64);
    acc = acc.wrapping_mul(31049u64);
    acc = acc.wrapping_add(770268u64);
    acc = acc.wrapping_add(4462u64);
    acc = acc.wrapping_mul(57225u64);
    acc = acc ^ 395666u64;
    acc = acc.wrapping_mul(40469u64);
    acc = acc ^ 598647u64;
    acc = acc ^ 117668u64;
    acc = acc.rotate_left(3);
    acc = acc ^ 8493u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(45585u64);
    acc = acc ^ 86984u64;
    acc = acc.wrapping_add(311444u64);
    acc = acc ^ 788254u64;
    acc = acc.wrapping_mul(13455u64);
    acc = acc.wrapping_mul(57517u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(34495u64);
    acc = acc.wrapping_mul(60805u64);
    acc = acc.wrapping_add(771102u64);
    acc = acc ^ 811197u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(371424u64);
    acc = acc.wrapping_mul(22049u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(488342u64);
    acc = acc.wrapping_add(530199u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(11841u64);
    acc = acc ^ 217289u64;
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(7899u64);
    acc = acc.wrapping_add(873244u64);
    acc = acc ^ 467399u64;
    acc = acc.wrapping_add(522957u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 905002u64;
    acc = acc.wrapping_add(200355u64);
    acc = acc ^ 899828u64;
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(16023u64);
    acc = acc.wrapping_mul(56565u64);
    acc = acc.wrapping_mul(30757u64);
    acc = acc.wrapping_mul(48013u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 118545u64;
    acc = acc.wrapping_add(165913u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 86641u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(31581u64);
    acc = acc.wrapping_add(784918u64);
    acc = acc.wrapping_mul(1765u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 216797u64;
    acc = acc.wrapping_mul(11477u64);
    acc = acc.wrapping_mul(28855u64);
    acc = acc ^ 214697u64;
    acc = acc ^ 193642u64;
    acc = acc.wrapping_mul(42895u64);
    acc = acc.wrapping_add(447734u64);
    acc = acc ^ 35524u64;
    acc = acc.wrapping_mul(45785u64);
    acc = acc ^ 953640u64;
    acc = acc.wrapping_mul(56727u64);
    acc = acc.wrapping_add(382706u64);
    acc = acc.wrapping_mul(62909u64);
    acc = acc.wrapping_add(918085u64);
    acc = acc.wrapping_add(515264u64);
    acc = acc ^ 403903u64;
    acc = acc ^ 766713u64;
    acc = acc.wrapping_add(700291u64);
    acc = acc ^ 633383u64;
    acc = acc ^ 213120u64;
    acc = acc.wrapping_add(62237u64);
    acc = acc.wrapping_mul(24219u64);
    acc = acc.wrapping_add(893209u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc ^ 122263u64;
    acc = acc.wrapping_mul(9395u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(37511u64);
    acc = acc.wrapping_add(355232u64);
    acc = acc.wrapping_add(305249u64);
    acc = acc.wrapping_mul(10557u64);
    acc = acc.wrapping_mul(6685u64);
    acc = acc.wrapping_add(726076u64);
    acc = acc.wrapping_mul(63597u64);
    acc = acc.wrapping_mul(8927u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(55997u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc ^ 308011u64;
    acc = acc.wrapping_mul(32215u64);
    acc = acc.wrapping_add(858670u64);
    acc = acc.wrapping_mul(19435u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(363247u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(118802u64);
    acc = acc.wrapping_add(881882u64);
    acc = acc.wrapping_add(791542u64);
    acc = acc.wrapping_mul(33895u64);
    acc = acc.wrapping_mul(39137u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(784150u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 929243u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 712870u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(563763u64);
    acc = acc.wrapping_add(10737u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 64807u64;
    acc = acc ^ 198255u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(408131u64);
    acc = acc ^ 821191u64;
    acc = acc.wrapping_mul(669u64);
    acc = acc.wrapping_mul(12797u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 532482u64;
    acc = acc.wrapping_add(760469u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(752260u64);
    acc = acc ^ 163919u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(34081u64);
    acc = acc.wrapping_mul(24717u64);
    acc = acc.wrapping_mul(20475u64);
    acc = acc ^ 820234u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(46417u64);
    acc = acc ^ 908593u64;
    acc = acc.wrapping_add(655991u64);
    acc = acc.wrapping_add(467585u64);
    acc = acc.wrapping_add(614689u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 996899u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(826289u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(733u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 567u64, b: -826i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 946u64, b: -612i32, c: false } } }


static SVALS_7: [i64; 4] = [1000003i64, 2000006i64, 3000009i64, 4000012i64];

union SU_7 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((1232789623u64) ^ (1232789623u64) == 0u64);
        assert!((1883875944u64).wrapping_add(1883875944u64).wrapping_sub(1883875944u64) == 1883875944u64);
        assert!((2867153546u64).wrapping_add(2867153546u64).wrapping_sub(2867153546u64) == 2867153546u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(130751515u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(597334803u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3234501473u64)));
        {

    let mut u = SU_7 { p: &SVALS_7[0] as *const i64 };
    u.r = &SVALS_7[0];
    let read_7_0 = unsafe { *u.r };
    print!("case_45:"); println!("{}", read_7_0);
    u.r = &SVALS_7[3];
    let read_7_1 = unsafe { *u.p };
    print!("case_45:"); println!("{}", read_7_1);
    u.p = &SVALS_7[0] as *const i64;
    let read_7_2 = unsafe { *u.p };
    print!("case_45:"); println!("{}", read_7_2);
    u.p = &SVALS_7[3] as *const i64;
    let read_7_3 = unsafe { *u.r };
    print!("case_45:"); println!("{}", read_7_3);

        }
        debug_assert!((3227670892u32) | (348591266u32) | (3227670892u32) == (3227670892u32) | (348591266u32));
        assert!((3566050071u64) ^ (3566050071u64) == 0u64);
        debug_assert!((1714889926u32).rotate_left(24).rotate_right(24) == 1714889926u32);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3707586513u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(717523131u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3031619072u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_46 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(568u64).rotate_left(31) ^ 6602u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(357u64).rotate_left(24) ^ 3185u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(633u64).rotate_left(7) ^ 533u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(4075u64).rotate_left(26) ^ 8612u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 327127u64;
    acc = acc.wrapping_add(423917u64);
    acc = acc ^ 491306u64;
    acc = acc ^ 554446u64;
    acc = acc ^ 535543u64;
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc ^ 39928u64;
    acc = acc.wrapping_add(218485u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(10621u64);
    acc = acc ^ 691088u64;
    acc = acc ^ 786054u64;
    acc = acc ^ 194072u64;
    acc = acc.wrapping_mul(48291u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(32507u64);
    acc = acc ^ 312846u64;
    acc = acc.wrapping_add(522028u64);
    acc = acc.wrapping_mul(47221u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(312956u64);
    acc = acc.wrapping_mul(10493u64);
    acc = acc.wrapping_mul(60683u64);
    acc = acc.wrapping_mul(60199u64);
    acc = acc ^ 755213u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(60495u64);
    acc = acc.wrapping_add(820951u64);
    acc = acc ^ 387699u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(311175u64);
    acc = acc.wrapping_add(248501u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(702389u64);
    acc = acc.wrapping_mul(2855u64);
    acc = acc.wrapping_mul(24003u64);
    acc = acc.wrapping_mul(61089u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 572928u64;
    acc = acc.wrapping_add(195159u64);
    acc = acc.wrapping_add(515180u64);
    acc = acc.wrapping_mul(46863u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(178223u64);
    acc = acc.wrapping_mul(7915u64);
    acc = acc.wrapping_mul(31101u64);
    acc = acc.wrapping_add(513909u64);
    acc = acc.wrapping_add(218548u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc ^ 712959u64;
    acc = acc.wrapping_add(930834u64);
    acc = acc.wrapping_mul(9583u64);
    acc = acc ^ 277784u64;
    acc = acc ^ 487082u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(48837u64);
    acc = acc.wrapping_mul(42745u64);
    acc = acc.wrapping_mul(35447u64);
    acc = acc.wrapping_mul(61445u64);
    acc = acc.wrapping_add(163408u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(382654u64);
    acc = acc.wrapping_add(696656u64);
    acc = acc.wrapping_add(194458u64);
    acc = acc.wrapping_add(434933u64);
    acc = acc ^ 956964u64;
    acc = acc.rotate_left(8);
    acc = acc ^ 581682u64;
    acc = acc.rotate_left(22);
    acc = acc ^ 268403u64;
    acc = acc ^ 534884u64;
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc ^ 357811u64;
    acc = acc.wrapping_add(99393u64);
    acc = acc.wrapping_add(50359u64);
    acc = acc.wrapping_add(392053u64);
    acc = acc ^ 972650u64;
    acc = acc.wrapping_add(925419u64);
    acc = acc.wrapping_mul(22245u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(235539u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc ^ 194381u64;
    acc = acc.wrapping_mul(27305u64);
    acc = acc.wrapping_mul(525u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 782753u64;
    acc = acc.wrapping_add(668798u64);
    acc = acc.wrapping_mul(7773u64);
    acc = acc.wrapping_mul(35453u64);
    acc = acc.wrapping_add(108179u64);
    acc = acc.wrapping_mul(42269u64);
    acc = acc.wrapping_add(646807u64);
    acc = acc ^ 819971u64;
    acc = acc ^ 380874u64;
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 825861u64;
    acc = acc.wrapping_add(494541u64);
    acc = acc.wrapping_mul(35513u64);
    acc = acc ^ 488779u64;
    acc = acc.wrapping_add(270984u64);
    acc = acc.wrapping_mul(12401u64);
    acc = acc.wrapping_add(291994u64);
    acc = acc.wrapping_add(628788u64);
    acc = acc ^ 137981u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(47203u64);
    acc = acc.wrapping_add(914116u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(45869u64);
    acc = acc.wrapping_mul(56625u64);
    acc = acc.wrapping_mul(51919u64);
    acc = acc.wrapping_add(113431u64);
    acc = acc.wrapping_add(657635u64);
    acc = acc.wrapping_mul(19657u64);
    acc = acc.wrapping_add(400855u64);
    acc = acc.wrapping_add(176760u64);
    acc = acc.wrapping_mul(48853u64);
    acc = acc.wrapping_add(286005u64);
    acc = acc.wrapping_add(75066u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V3(959u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 972u64, b: 149i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 309u64, b: -484i32, c: true } } }


static SVALS_8: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_8 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!(!(2358331792u64) == (2358331792u64) ^ u64::MAX);
        assert!((2220578553u64) ^ (2220578553u64) == 0u64);
        assert!((1341033846u64).wrapping_add(1341033846u64).wrapping_sub(1341033846u64) == 1341033846u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2533615176u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3260703121u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3285596052u64)));
        {

    let mut u = SU_8 { r: &SVALS_8[0] };
    u.r = &SVALS_8[0];
    let read_8_0 = unsafe { *u.r };
    print!("case_46:"); println!("{}", read_8_0);
    u.p = &SVALS_8[5] as *const i64;
    let read_8_1 = unsafe { *u.p };
    print!("case_46:"); println!("{}", read_8_1);

        }
        assert!((3994069614u64) ^ (3994069614u64) == 0u64);
        assert!(!(3669831480u64) == (3669831480u64) ^ u64::MAX);
        debug_assert!((1322177460u32) | (1692180320u32) | (1322177460u32) == (1322177460u32) | (1692180320u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1658837831u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(55181700u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2720688059u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_47 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(2667u64).rotate_left(17) ^ 6743u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(1439u64).rotate_left(8) ^ 7354u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(6009u64).rotate_left(24) ^ 7091u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(8236u64).rotate_left(26) ^ 2536u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(31205u64);
    acc = acc.wrapping_mul(39943u64);
    acc = acc.wrapping_add(574500u64);
    acc = acc.wrapping_mul(38411u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(60089u64);
    acc = acc.wrapping_mul(731u64);
    acc = acc.wrapping_add(761180u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 646897u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(26263u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(40567u64);
    acc = acc.wrapping_add(107794u64);
    acc = acc.wrapping_add(134721u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 627841u64;
    acc = acc.wrapping_mul(39975u64);
    acc = acc ^ 708881u64;
    acc = acc.wrapping_add(931240u64);
    acc = acc ^ 713037u64;
    acc = acc.wrapping_add(161010u64);
    acc = acc ^ 835768u64;
    acc = acc.wrapping_add(642573u64);
    acc = acc ^ 862011u64;
    acc = acc.wrapping_add(677287u64);
    acc = acc.wrapping_mul(61177u64);
    acc = acc.wrapping_mul(2107u64);
    acc = acc.wrapping_mul(51423u64);
    acc = acc ^ 813827u64;
    acc = acc.wrapping_mul(15151u64);
    acc = acc.wrapping_add(542363u64);
    acc = acc ^ 920158u64;
    acc = acc.rotate_left(25);
    acc = acc ^ 934969u64;
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(50644u64);
    acc = acc ^ 662127u64;
    acc = acc.rotate_left(29);
    acc = acc ^ 76028u64;
    acc = acc.wrapping_mul(11597u64);
    acc = acc.wrapping_mul(64249u64);
    acc = acc.wrapping_mul(11979u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 763241u64;
    acc = acc ^ 589915u64;
    acc = acc.wrapping_add(72837u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(8669u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(39443u64);
    acc = acc.wrapping_add(279827u64);
    acc = acc ^ 526114u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(361821u64);
    acc = acc.wrapping_add(652458u64);
    acc = acc.wrapping_add(416285u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(760591u64);
    acc = acc.wrapping_add(444500u64);
    acc = acc ^ 172792u64;
    acc = acc ^ 143983u64;
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(482249u64);
    acc = acc ^ 791826u64;
    acc = acc.wrapping_mul(56087u64);
    acc = acc.wrapping_mul(52099u64);
    acc = acc.wrapping_add(457042u64);
    acc = acc.wrapping_mul(61627u64);
    acc = acc ^ 385113u64;
    acc = acc.wrapping_mul(26925u64);
    acc = acc ^ 28182u64;
    acc = acc ^ 20977u64;
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(363809u64);
    acc = acc.wrapping_add(806862u64);
    acc = acc ^ 268859u64;
    acc = acc.wrapping_add(626834u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(35791u64);
    acc = acc ^ 488476u64;
    acc = acc ^ 764332u64;
    acc = acc.wrapping_mul(50299u64);
    acc = acc ^ 458992u64;
    acc = acc.rotate_left(14);
    acc = acc ^ 338415u64;
    acc = acc.wrapping_mul(26805u64);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(3993u64);
    acc = acc.wrapping_add(766799u64);
    acc = acc.wrapping_mul(27787u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(840127u64);
    acc = acc.wrapping_add(604221u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc ^ 268009u64;
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(59617u64);
    acc = acc.wrapping_mul(54051u64);
    acc = acc ^ 755290u64;
    acc = acc.wrapping_add(506217u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(41921u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(868965u64);
    acc = acc.wrapping_mul(25261u64);
    acc = acc.wrapping_mul(38923u64);
    acc = acc.wrapping_add(19024u64);
    acc = acc ^ 976426u64;
    acc = acc.rotate_left(6);
    acc = acc ^ 66377u64;
    acc = acc.wrapping_add(413093u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc ^ 269445u64;
    acc = acc.wrapping_mul(50315u64);
    acc = acc.wrapping_mul(19275u64);
    acc = acc.wrapping_mul(27131u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(381145u64);
    acc = acc.wrapping_mul(16217u64);
    acc = acc.wrapping_add(436296u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc ^ 507312u64;
    acc = acc.wrapping_mul(12883u64);
    acc = acc ^ 66526u64;
    acc = acc.wrapping_add(758510u64);
    acc = acc.wrapping_mul(54157u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 522775u64;
    acc = acc.wrapping_mul(11919u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(532445u64);
    acc = acc.wrapping_add(829448u64);
    acc = acc.wrapping_mul(8349u64);
    acc = acc ^ 216869u64;
    acc = acc.wrapping_mul(14653u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(2577u64);
    acc = acc ^ 236773u64;
    acc = acc ^ 919190u64;
    acc = acc.rotate_left(10);
    acc = acc ^ 898464u64;
    acc = acc ^ 369194u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(568387u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 469173u64;
    acc = acc ^ 98223u64;
    acc = acc ^ 969579u64;
    acc = acc.wrapping_add(259439u64);
    acc = acc.wrapping_mul(49847u64);
    acc = acc.wrapping_mul(26085u64);
    acc = acc.wrapping_mul(36381u64);
    acc = acc.wrapping_mul(12661u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 774162u64;
    acc = acc.wrapping_mul(27439u64);
    acc = acc ^ 568209u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(338692u64);
    acc = acc ^ 922519u64;
    acc = acc.wrapping_add(329234u64);
    acc = acc.wrapping_mul(14177u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(281626u64);
    acc = acc ^ 47712u64;
    acc = acc ^ 483146u64;
    acc = acc.wrapping_mul(27943u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 173u64, b: -947i32, c: true } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(882u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 829u64, b: -683i32, c: true } } }


static SVALS_9: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_9 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((3874488223u32).rotate_left(23).rotate_right(23) == 3874488223u32);
        debug_assert!((651935087u64).wrapping_mul(1) == 651935087u64);
        debug_assert!((4058653954u32) | (2710314307u32) | (4058653954u32) == (4058653954u32) | (2710314307u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2945861721u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2840811745u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3797214309u64)));
        {

    let mut u = SU_9 { p: &SVALS_9[0] as *const i64 };
    u.r = &SVALS_9[0];
    let read_9_0 = unsafe { *u.r };
    print!("case_47:"); println!("{}", read_9_0);
    u.p = &SVALS_9[5] as *const i64;
    let read_9_1 = unsafe { *u.p };
    print!("case_47:"); println!("{}", read_9_1);

        }
        assert!(!(1430658011u64) == (1430658011u64) ^ u64::MAX);
        debug_assert!((3875122624u64).wrapping_mul(1) == 3875122624u64);
        debug_assert!((3886471439u64).wrapping_mul(1) == 3886471439u64);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1932341433u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(4073061381u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1232613663u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_48 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(6269u64).rotate_left(8) ^ 9834u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(2869u64).rotate_left(18) ^ 730u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(607u64).rotate_left(25) ^ 4953u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(1079u64).rotate_left(31) ^ 2913u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(49517u64);
    acc = acc.wrapping_mul(50097u64);
    acc = acc.wrapping_add(532322u64);
    acc = acc.wrapping_add(450999u64);
    acc = acc.wrapping_mul(39217u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(271142u64);
    acc = acc ^ 299720u64;
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(925065u64);
    acc = acc.wrapping_add(833676u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(636854u64);
    acc = acc.wrapping_add(842738u64);
    acc = acc.wrapping_add(87559u64);
    acc = acc ^ 557123u64;
    acc = acc.wrapping_add(602097u64);
    acc = acc ^ 158003u64;
    acc = acc.rotate_left(13);
    acc = acc ^ 261883u64;
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(29041u64);
    acc = acc ^ 520817u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(16975u64);
    acc = acc.wrapping_mul(1361u64);
    acc = acc.wrapping_add(341715u64);
    acc = acc ^ 713416u64;
    acc = acc.wrapping_mul(5403u64);
    acc = acc.wrapping_mul(21811u64);
    acc = acc ^ 944358u64;
    acc = acc.wrapping_mul(64923u64);
    acc = acc.wrapping_add(994563u64);
    acc = acc.wrapping_add(186157u64);
    acc = acc.wrapping_mul(54927u64);
    acc = acc.wrapping_mul(44559u64);
    acc = acc.wrapping_mul(19933u64);
    acc = acc.wrapping_add(511594u64);
    acc = acc.wrapping_add(103003u64);
    acc = acc.wrapping_mul(48651u64);
    acc = acc ^ 2131u64;
    acc = acc ^ 212780u64;
    acc = acc ^ 601243u64;
    acc = acc.wrapping_add(870974u64);
    acc = acc.wrapping_mul(45937u64);
    acc = acc.wrapping_mul(5847u64);
    acc = acc.wrapping_add(158866u64);
    acc = acc.wrapping_mul(62405u64);
    acc = acc.wrapping_add(762776u64);
    acc = acc ^ 278496u64;
    acc = acc.wrapping_add(63312u64);
    acc = acc.wrapping_add(3252u64);
    acc = acc.wrapping_mul(56071u64);
    acc = acc.wrapping_mul(18855u64);
    acc = acc.wrapping_mul(5297u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(198051u64);
    acc = acc.wrapping_mul(65317u64);
    acc = acc.wrapping_add(479930u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc ^ 945520u64;
    acc = acc.wrapping_add(319077u64);
    acc = acc.wrapping_mul(20043u64);
    acc = acc ^ 334367u64;
    acc = acc ^ 385491u64;
    acc = acc.wrapping_mul(32927u64);
    acc = acc.wrapping_add(675792u64);
    acc = acc.wrapping_mul(45973u64);
    acc = acc ^ 56763u64;
    acc = acc ^ 814016u64;
    acc = acc.wrapping_add(140737u64);
    acc = acc ^ 77224u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(57113u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(9715u64);
    acc = acc.wrapping_add(526383u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(10333u64);
    acc = acc.wrapping_mul(9959u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(48969u64);
    acc = acc.wrapping_mul(63279u64);
    acc = acc.wrapping_add(24193u64);
    acc = acc.wrapping_add(312377u64);
    acc = acc.rotate_left(14);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(314845u64);
    acc = acc.wrapping_mul(6695u64);
    acc = acc.wrapping_add(386613u64);
    acc = acc.wrapping_add(326371u64);
    acc = acc ^ 695925u64;
    acc = acc ^ 45344u64;
    acc = acc ^ 294092u64;
    acc = acc.rotate_left(15);
    acc = acc ^ 929027u64;
    acc = acc.wrapping_mul(46341u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 147865u64;
    acc = acc.wrapping_add(821992u64);
    acc = acc ^ 231416u64;
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(530117u64);
    acc = acc.wrapping_mul(19063u64);
    acc = acc.wrapping_add(892533u64);
    acc = acc ^ 763797u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 683512u64;
    acc = acc.rotate_left(11);
    acc = acc ^ 217485u64;
    acc = acc.wrapping_add(901020u64);
    acc = acc.wrapping_add(344271u64);
    acc = acc ^ 751982u64;
    acc = acc.wrapping_mul(62327u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(35461u64);
    acc = acc.wrapping_add(944482u64);
    acc = acc.wrapping_mul(49447u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(264259u64);
    acc = acc.wrapping_add(182440u64);
    acc = acc.wrapping_mul(10893u64);
    acc = acc.wrapping_mul(57701u64);
    acc = acc.wrapping_mul(3285u64);
    acc = acc.wrapping_mul(51927u64);
    acc = acc.wrapping_mul(11975u64);
    acc = acc.wrapping_add(475520u64);
    acc = acc ^ 408065u64;
    acc = acc.wrapping_mul(13645u64);
    acc = acc.wrapping_add(375479u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 217813u64;
    acc = acc ^ 400406u64;
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(398790u64);
    acc = acc.wrapping_add(102659u64);
    acc = acc ^ 646286u64;
    acc = acc ^ 553048u64;
    acc = acc.wrapping_mul(25105u64);
    acc = acc.wrapping_mul(44689u64);
    acc = acc ^ 243590u64;
    acc = acc.wrapping_add(498613u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(664199u64);
    acc = acc ^ 646646u64;
    acc = acc.wrapping_mul(7243u64);
    acc = acc.wrapping_mul(35713u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 623540u64;
    acc = acc.wrapping_mul(49189u64);
    acc = acc ^ 377849u64;
    acc = acc ^ 667693u64;
    acc = acc.wrapping_mul(11813u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(54989u64);
    acc = acc ^ 995776u64;
    acc = acc ^ 677767u64;
    acc = acc ^ 705117u64;
    acc = acc ^ 632010u64;
    acc = acc.rotate_left(24);
    acc = acc ^ 971594u64;
    acc = acc.wrapping_add(672178u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(594733u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(3803u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(916618u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(59785u64);
    acc = acc ^ 428677u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(790668u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(20505u64);
    acc = acc.wrapping_add(507235u64);
    acc = acc.wrapping_mul(9497u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(35u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V2(792u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 361u64, b: -571i32, c: false } } }


static SVALS_10: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_10 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((1628984695u64).wrapping_add(1628984695u64).wrapping_sub(1628984695u64) == 1628984695u64);
        assert!(!(2355314492u64) == (2355314492u64) ^ u64::MAX);
        debug_assert!((1278102421u64).wrapping_mul(1) == 1278102421u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3424865631u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3730627819u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(560688397u64)));
        {

    let mut u = SU_10 { r: &SVALS_10[0] };
    u.r = &SVALS_10[0];
    let read_10_0 = unsafe { *u.p };
    print!("case_48:"); println!("{}", read_10_0);
    u.p = &SVALS_10[1] as *const i64;
    let read_10_1 = unsafe { *u.r };
    print!("case_48:"); println!("{}", read_10_1);

        }
        assert!((1394432696u64) ^ (1394432696u64) == 0u64);
        debug_assert!((4046586919u64).wrapping_mul(1) == 4046586919u64);
        assert!((2614863012u64).wrapping_add(2614863012u64).wrapping_sub(2614863012u64) == 2614863012u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2207031150u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2676265059u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3938446911u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_49 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(247u64).rotate_left(16) ^ 4324u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(4576u64).rotate_left(30) ^ 4121u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(3377u64).rotate_left(8) ^ 162u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(340u64).rotate_left(16) ^ 8292u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(674330u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 483639u64;
    acc = acc.rotate_left(15);
    acc = acc ^ 56731u64;
    acc = acc.wrapping_add(43980u64);
    acc = acc.wrapping_add(10414u64);
    acc = acc.wrapping_mul(16367u64);
    acc = acc ^ 528218u64;
    acc = acc.wrapping_add(137079u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(228090u64);
    acc = acc.wrapping_add(78351u64);
    acc = acc.wrapping_add(382024u64);
    acc = acc.wrapping_add(27386u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(87472u64);
    acc = acc ^ 46413u64;
    acc = acc.wrapping_add(503808u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(8235u64);
    acc = acc.wrapping_add(63612u64);
    acc = acc.rotate_left(7);
    acc = acc ^ 900836u64;
    acc = acc.wrapping_add(611104u64);
    acc = acc ^ 581121u64;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(61981u64);
    acc = acc.wrapping_add(972207u64);
    acc = acc.wrapping_mul(41527u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(501u64);
    acc = acc ^ 694358u64;
    acc = acc.wrapping_mul(46215u64);
    acc = acc.wrapping_mul(30331u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(21871u64);
    acc = acc.wrapping_add(211481u64);
    acc = acc.wrapping_mul(59349u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 584039u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(632648u64);
    acc = acc.wrapping_mul(22403u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(31911u64);
    acc = acc ^ 634970u64;
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(746019u64);
    acc = acc.wrapping_add(675908u64);
    acc = acc ^ 753413u64;
    acc = acc ^ 500500u64;
    acc = acc ^ 507369u64;
    acc = acc ^ 766487u64;
    acc = acc.wrapping_mul(15935u64);
    acc = acc ^ 637621u64;
    acc = acc.wrapping_add(538199u64);
    acc = acc ^ 189907u64;
    acc = acc ^ 773546u64;
    acc = acc.wrapping_add(896107u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(542315u64);
    acc = acc.wrapping_add(840962u64);
    acc = acc.wrapping_add(828318u64);
    acc = acc ^ 690584u64;
    acc = acc ^ 945284u64;
    acc = acc.wrapping_mul(63481u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(788997u64);
    acc = acc.wrapping_mul(26647u64);
    acc = acc.wrapping_add(27634u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(219953u64);
    acc = acc.wrapping_mul(6585u64);
    acc = acc.wrapping_mul(19291u64);
    acc = acc.wrapping_mul(29251u64);
    acc = acc ^ 735128u64;
    acc = acc ^ 274583u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(247293u64);
    acc = acc.rotate_left(7);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 745663u64;
    acc = acc ^ 800286u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(20973u64);
    acc = acc.wrapping_add(479810u64);
    acc = acc ^ 570664u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(45365u64);
    acc = acc.wrapping_mul(55219u64);
    acc = acc ^ 109156u64;
    acc = acc ^ 191593u64;
    acc = acc.wrapping_mul(44287u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 662597u64;
    acc = acc.wrapping_mul(31585u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(42129u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(7295u64);
    acc = acc ^ 539553u64;
    acc = acc.wrapping_mul(40067u64);
    acc = acc.wrapping_add(351043u64);
    acc = acc.wrapping_mul(12915u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 429874u64;
    acc = acc.wrapping_add(479308u64);
    acc = acc.wrapping_mul(3225u64);
    acc = acc.wrapping_mul(8065u64);
    acc = acc ^ 823974u64;
    acc = acc.wrapping_mul(44755u64);
    acc = acc ^ 892473u64;
    acc = acc ^ 431903u64;
    acc = acc.wrapping_add(795999u64);
    acc = acc ^ 18778u64;
    acc = acc.wrapping_mul(24573u64);
    acc = acc ^ 327147u64;
    acc = acc.wrapping_add(357803u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(426608u64);
    acc = acc.wrapping_add(753585u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(291939u64);
    acc = acc ^ 240840u64;
    acc = acc ^ 742941u64;
    acc = acc ^ 581675u64;
    acc = acc.wrapping_add(543568u64);
    acc = acc ^ 728309u64;
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(856428u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(59051u64);
    acc = acc ^ 596228u64;
    acc = acc.wrapping_add(326416u64);
    acc = acc ^ 962690u64;
    acc = acc ^ 780854u64;
    acc = acc.wrapping_mul(12637u64);
    acc = acc.wrapping_add(980663u64);
    acc = acc.wrapping_add(41831u64);
    acc = acc.wrapping_mul(23939u64);
    acc = acc ^ 308638u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 438725u64;
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(62773u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(499231u64);
    acc = acc.wrapping_mul(32377u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 118626u64;
    acc = acc ^ 504016u64;
    acc = acc.wrapping_add(963786u64);
    acc = acc.wrapping_mul(25305u64);
    acc = acc.wrapping_add(800462u64);
    acc = acc.wrapping_add(537574u64);
    acc = acc.wrapping_mul(59563u64);
    acc = acc.wrapping_add(298270u64);
    acc = acc ^ 902571u64;
    acc = acc.wrapping_add(879867u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 353399u64;
    acc = acc.wrapping_add(621653u64);
    acc = acc.wrapping_add(547818u64);
    acc = acc.wrapping_add(575096u64);
    acc = acc.wrapping_mul(46293u64);
    acc = acc.wrapping_mul(43255u64);
    acc = acc.wrapping_mul(36189u64);
    acc = acc ^ 227840u64;
    acc = acc.wrapping_mul(11997u64);
    acc = acc ^ 846018u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 146047u64;
    acc = acc.wrapping_add(982705u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(827248u64);
    acc = acc ^ 560565u64;
    acc = acc.wrapping_mul(39085u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(669u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 944u64, b: -270i32, c: false } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 926u64, b: -990i32, c: false } } }


static SVALS_11: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_11 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!(!(3044687371u64) == (3044687371u64) ^ u64::MAX);
        assert!((1900909746u64) ^ (1900909746u64) == 0u64);
        debug_assert!((3770797393u32) | (3869019102u32) | (3770797393u32) == (3770797393u32) | (3869019102u32));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1154125673u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1592144270u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2119968277u64)));
        {

    let mut u = SU_11 { p: &SVALS_11[0] as *const i64 };
    u.r = &SVALS_11[0];
    let read_11_0 = unsafe { *u.p };
    print!("case_49:"); println!("{}", read_11_0);
    u.p = &SVALS_11[1] as *const i64;
    let read_11_1 = unsafe { *u.r };
    print!("case_49:"); println!("{}", read_11_1);

        }
        assert!(!(2217925416u64) == (2217925416u64) ^ u64::MAX);
        assert!((2912849617u64) ^ (2912849617u64) == 0u64);
        assert!((2515667284u64) ^ (2515667284u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2191127243u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3692194507u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2513834834u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_50 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(5674u64).rotate_left(24) ^ 6657u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(9515u64).rotate_left(25) ^ 3744u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(8122u64).rotate_left(6) ^ 7445u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(396u64).rotate_left(3) ^ 3691u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc ^ 595062u64;
    acc = acc ^ 502664u64;
    acc = acc.wrapping_mul(21351u64);
    acc = acc.wrapping_add(897133u64);
    acc = acc ^ 254459u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(44599u64);
    acc = acc.wrapping_add(695507u64);
    acc = acc.wrapping_add(732288u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(37719u64);
    acc = acc ^ 797186u64;
    acc = acc.wrapping_mul(15883u64);
    acc = acc.wrapping_mul(39331u64);
    acc = acc ^ 185600u64;
    acc = acc ^ 940703u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(378374u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 412813u64;
    acc = acc.wrapping_mul(54711u64);
    acc = acc.wrapping_mul(23565u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(14221u64);
    acc = acc.wrapping_add(543699u64);
    acc = acc.wrapping_add(262971u64);
    acc = acc ^ 538278u64;
    acc = acc ^ 721917u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(667964u64);
    acc = acc.wrapping_mul(60589u64);
    acc = acc ^ 524106u64;
    acc = acc ^ 524850u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 391663u64;
    acc = acc.rotate_left(15);
    acc = acc ^ 171606u64;
    acc = acc.wrapping_mul(44985u64);
    acc = acc.wrapping_add(895690u64);
    acc = acc.wrapping_add(215890u64);
    acc = acc ^ 98507u64;
    acc = acc.wrapping_mul(43177u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(7245u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(6047u64);
    acc = acc ^ 939654u64;
    acc = acc.rotate_left(13);
    acc = acc ^ 332234u64;
    acc = acc.rotate_left(27);
    acc = acc ^ 931058u64;
    acc = acc.wrapping_mul(50991u64);
    acc = acc ^ 15213u64;
    acc = acc.wrapping_add(572366u64);
    acc = acc.wrapping_mul(16063u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(47599u64);
    acc = acc.wrapping_mul(27093u64);
    acc = acc ^ 679805u64;
    acc = acc.wrapping_mul(51301u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(994194u64);
    acc = acc ^ 896959u64;
    acc = acc ^ 244368u64;
    acc = acc ^ 72969u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(50745u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(12029u64);
    acc = acc.wrapping_add(205887u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(39551u64);
    acc = acc.wrapping_mul(40935u64);
    acc = acc.wrapping_mul(65389u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(760323u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(665940u64);
    acc = acc.wrapping_add(380140u64);
    acc = acc ^ 897737u64;
    acc = acc.wrapping_mul(1671u64);
    acc = acc.wrapping_add(972360u64);
    acc = acc.wrapping_add(238678u64);
    acc = acc.rotate_left(14);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 288911u64;
    acc = acc ^ 861191u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 436440u64;
    acc = acc ^ 733674u64;
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(47343u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 172234u64;
    acc = acc ^ 278292u64;
    acc = acc.wrapping_mul(32659u64);
    acc = acc.wrapping_mul(19341u64);
    acc = acc.wrapping_mul(48853u64);
    acc = acc.wrapping_mul(38039u64);
    acc = acc ^ 155746u64;
    acc = acc.wrapping_mul(3039u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc ^ 235393u64;
    acc = acc.wrapping_add(630073u64);
    acc = acc.wrapping_add(681576u64);
    acc = acc.wrapping_mul(19075u64);
    acc = acc ^ 832786u64;
    acc = acc.wrapping_mul(58941u64);
    acc = acc ^ 34439u64;
    acc = acc ^ 616656u64;
    acc = acc.wrapping_add(953660u64);
    acc = acc ^ 862759u64;
    acc = acc.wrapping_add(124048u64);
    acc = acc.wrapping_mul(59073u64);
    acc = acc.wrapping_mul(9527u64);
    acc = acc.wrapping_add(966986u64);
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(840990u64);
    acc = acc.wrapping_add(677221u64);
    acc = acc ^ 600812u64;
    acc = acc ^ 978045u64;
    acc = acc.wrapping_mul(50477u64);
    acc = acc.wrapping_mul(23159u64);
    acc = acc ^ 110466u64;
    acc = acc.wrapping_add(112408u64);
    acc = acc ^ 589212u64;
    acc = acc.wrapping_mul(59705u64);
    acc = acc.wrapping_mul(2945u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(33707u64);
    acc = acc.wrapping_mul(56509u64);
    acc = acc ^ 514274u64;
    acc = acc ^ 135098u64;
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 645486u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(40965u64);
    acc = acc.wrapping_add(912165u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(6359u64);
    acc = acc.wrapping_mul(15955u64);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(28927u64);
    acc = acc ^ 839947u64;
    acc = acc ^ 712466u64;
    acc = acc.wrapping_add(970469u64);
    acc = acc ^ 524022u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(60787u64);
    acc = acc ^ 578544u64;
    acc = acc ^ 664875u64;
    acc = acc.wrapping_add(468965u64);
    acc = acc.rotate_left(26);
    acc = acc ^ 815183u64;
    acc = acc.wrapping_add(910617u64);
    acc = acc.wrapping_mul(17461u64);
    acc = acc ^ 795150u64;
    acc = acc.wrapping_add(573608u64);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(243091u64);
    acc = acc ^ 451614u64;
    acc = acc.wrapping_mul(5003u64);
    acc = acc ^ 368553u64;
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(937u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 877u64, b: -170i32, c: false } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(449u64) } }


static SVALS_12: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_12 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((2385845937u64).wrapping_add(2385845937u64).wrapping_sub(2385845937u64) == 2385845937u64);
        debug_assert!((1349264669u32).rotate_left(31).rotate_right(31) == 1349264669u32);
        assert!(!(1645683857u64) == (1645683857u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(87190748u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2135563494u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(2769020960u64)));
        {

    let mut u = SU_12 { r: &SVALS_12[0] };
    u.p = &SVALS_12[0] as *const i64;
    let read_12_0 = unsafe { *u.p };
    print!("case_50:"); println!("{}", read_12_0);
    u.r = &SVALS_12[5];
    let read_12_1 = unsafe { *u.r };
    print!("case_50:"); println!("{}", read_12_1);
    u.p = &SVALS_12[3] as *const i64;
    let read_12_2 = unsafe { *u.r };
    print!("case_50:"); println!("{}", read_12_2);

        }
        assert!((2504875382u64) ^ (2504875382u64) == 0u64);
        debug_assert!((3063120488u32).rotate_left(30).rotate_right(30) == 3063120488u32);
        assert!((281423106u64) ^ (281423106u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(4161294366u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1718842831u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2749040759u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_51 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(6854u64).rotate_left(5) ^ 748u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_mul(8939u64).rotate_left(19) ^ 3583u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(3596u64).rotate_left(21) ^ 7925u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(66u64).rotate_left(13) ^ 1190u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 167538u64;
    acc = acc.wrapping_mul(17741u64);
    acc = acc.wrapping_mul(45097u64);
    acc = acc ^ 680510u64;
    acc = acc.wrapping_add(754615u64);
    acc = acc.wrapping_mul(4159u64);
    acc = acc.wrapping_mul(25455u64);
    acc = acc ^ 359309u64;
    acc = acc ^ 479295u64;
    acc = acc ^ 661561u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(18727u64);
    acc = acc ^ 821987u64;
    acc = acc.wrapping_mul(997u64);
    acc = acc.wrapping_add(909295u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(117u64);
    acc = acc.wrapping_mul(52467u64);
    acc = acc.wrapping_add(520326u64);
    acc = acc.wrapping_mul(34989u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(814955u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 157257u64;
    acc = acc ^ 527328u64;
    acc = acc.wrapping_add(384653u64);
    acc = acc.wrapping_mul(12423u64);
    acc = acc ^ 925077u64;
    acc = acc.wrapping_mul(51203u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 684850u64;
    acc = acc ^ 983418u64;
    acc = acc ^ 617551u64;
    acc = acc ^ 920371u64;
    acc = acc.wrapping_mul(39013u64);
    acc = acc ^ 460515u64;
    acc = acc.rotate_left(8);
    acc = acc ^ 574716u64;
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(856339u64);
    acc = acc ^ 164148u64;
    acc = acc ^ 262317u64;
    acc = acc.wrapping_add(296733u64);
    acc = acc.wrapping_mul(25721u64);
    acc = acc.wrapping_add(371901u64);
    acc = acc ^ 6688u64;
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(461811u64);
    acc = acc.wrapping_mul(55727u64);
    acc = acc ^ 726938u64;
    acc = acc.wrapping_add(23547u64);
    acc = acc.wrapping_add(312715u64);
    acc = acc.wrapping_mul(21463u64);
    acc = acc.wrapping_add(959096u64);
    acc = acc ^ 762388u64;
    acc = acc.wrapping_mul(17809u64);
    acc = acc.wrapping_mul(54799u64);
    acc = acc.wrapping_mul(7065u64);
    acc = acc.wrapping_add(158138u64);
    acc = acc ^ 43102u64;
    acc = acc.wrapping_add(909469u64);
    acc = acc.wrapping_add(406428u64);
    acc = acc.wrapping_add(487434u64);
    acc = acc ^ 622591u64;
    acc = acc.wrapping_mul(60183u64);
    acc = acc ^ 152907u64;
    acc = acc.wrapping_mul(51301u64);
    acc = acc ^ 969413u64;
    acc = acc ^ 274964u64;
    acc = acc.rotate_left(5);
    acc = acc ^ 438794u64;
    acc = acc.wrapping_mul(3265u64);
    acc = acc.wrapping_mul(62005u64);
    acc = acc ^ 972352u64;
    acc = acc.wrapping_add(694097u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc ^ 198252u64;
    acc = acc.wrapping_mul(31643u64);
    acc = acc.wrapping_add(77355u64);
    acc = acc ^ 845532u64;
    acc = acc.wrapping_mul(63269u64);
    acc = acc.wrapping_mul(39573u64);
    acc = acc.rotate_left(10);
    acc = acc ^ 540955u64;
    acc = acc.wrapping_add(29144u64);
    acc = acc ^ 804331u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 560907u64;
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc ^ 123867u64;
    acc = acc.wrapping_add(993499u64);
    acc = acc.wrapping_add(215048u64);
    acc = acc ^ 906657u64;
    acc = acc ^ 154343u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(726156u64);
    acc = acc.rotate_left(12);
    acc = acc ^ 937066u64;
    acc = acc.wrapping_mul(30391u64);
    acc = acc.wrapping_mul(47333u64);
    acc = acc ^ 66661u64;
    acc = acc ^ 567287u64;
    acc = acc ^ 197680u64;
    acc = acc.wrapping_add(890697u64);
    acc = acc ^ 410217u64;
    acc = acc.wrapping_add(584586u64);
    acc = acc ^ 935102u64;
    acc = acc.wrapping_mul(45183u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(30713u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 813394u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(408705u64);
    acc = acc.wrapping_add(903258u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 16170u64;
    acc = acc.wrapping_mul(38017u64);
    acc = acc.wrapping_add(254077u64);
    acc = acc.wrapping_mul(54825u64);
    acc = acc ^ 926644u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(178960u64);
    acc = acc ^ 964519u64;
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(53735u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(26021u64);
    acc = acc.wrapping_add(509244u64);
    acc = acc ^ 965621u64;
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(21343u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 834451u64;
    acc = acc.wrapping_add(799854u64);
    acc = acc.wrapping_add(371810u64);
    acc = acc.wrapping_mul(46947u64);
    acc = acc ^ 813979u64;
    acc = acc.wrapping_mul(12795u64);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(57497u64);
    acc = acc ^ 71034u64;
    acc = acc ^ 444227u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(173567u64);
    acc = acc ^ 732946u64;
    acc = acc.wrapping_add(608144u64);
    acc = acc ^ 812055u64;
    acc = acc ^ 818240u64;
    acc = acc.wrapping_mul(26161u64);
    acc = acc.wrapping_add(220495u64);
    acc = acc.wrapping_add(163976u64);
    acc = acc ^ 219757u64;
    acc = acc ^ 92348u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(12069u64);
    acc = acc ^ 156522u64;
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(5479u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 692727u64;
    acc = acc.wrapping_mul(31567u64);
    acc = acc ^ 592125u64;
    acc = acc.wrapping_mul(6877u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 30237u64;
    acc = acc.wrapping_mul(25843u64);
    acc = acc.wrapping_mul(23175u64);
    acc = acc.wrapping_mul(47281u64);
    acc = acc ^ 296284u64;
    acc = acc.rotate_left(31);
    acc = acc ^ 238455u64;
    acc = acc ^ 901430u64;
    acc = acc.wrapping_add(898546u64);
    acc = acc ^ 710727u64;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_mul(31809u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(47203u64);
    acc = acc.wrapping_add(361106u64);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(309634u64);
    acc = acc.wrapping_add(210021u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(9833u64);
    acc = acc ^ 51118u64;
    acc = acc.wrapping_mul(28467u64);
    acc = acc.wrapping_add(578502u64);
    acc = acc.wrapping_add(33388u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 620489u64;
    acc = acc.wrapping_mul(61925u64);
    acc = acc ^ 309961u64;
    acc = acc.wrapping_mul(29561u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 524u64, b: 961i32, c: true } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(864u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(361u64) } }


static SVALS_13: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_13 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((3011286207u32).rotate_left(26).rotate_right(26) == 3011286207u32);
        assert!(!(2847089476u64) == (2847089476u64) ^ u64::MAX);
        debug_assert!((64820992u64).wrapping_mul(1) == 64820992u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(128689050u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3677791932u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2944818445u64)));
        {

    let mut u = SU_13 { p: &SVALS_13[0] as *const i64 };
    u.p = &SVALS_13[0] as *const i64;
    let read_13_0 = unsafe { *u.p };
    print!("case_51:"); println!("{}", read_13_0);
    u.r = &SVALS_13[5];
    let read_13_1 = unsafe { *u.r };
    print!("case_51:"); println!("{}", read_13_1);
    u.p = &SVALS_13[3] as *const i64;
    let read_13_2 = unsafe { *u.r };
    print!("case_51:"); println!("{}", read_13_2);

        }
        debug_assert!((1818598674u32).rotate_left(30).rotate_right(30) == 1818598674u32);
        assert!((3958433797u64).wrapping_add(3958433797u64).wrapping_sub(3958433797u64) == 3958433797u64);
        debug_assert!((3888147177u32).rotate_left(17).rotate_right(17) == 3888147177u32);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3523801848u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3143835103u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1502715913u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_52 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(6411u64).rotate_left(5) ^ 8343u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(3082u64).rotate_left(31) ^ 4990u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(7270u64).rotate_left(24) ^ 6998u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(2460u64).rotate_left(16) ^ 8943u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(868011u64);
    acc = acc.wrapping_add(46186u64);
    acc = acc.wrapping_add(848299u64);
    acc = acc.wrapping_add(1067u64);
    acc = acc.wrapping_add(746403u64);
    acc = acc.wrapping_mul(42791u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(522104u64);
    acc = acc ^ 263610u64;
    acc = acc.wrapping_add(724606u64);
    acc = acc ^ 402247u64;
    acc = acc ^ 860015u64;
    acc = acc.wrapping_add(517874u64);
    acc = acc.wrapping_add(632608u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(927079u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(758868u64);
    acc = acc.wrapping_add(195067u64);
    acc = acc.wrapping_add(870559u64);
    acc = acc.wrapping_mul(48055u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(313100u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(41899u64);
    acc = acc ^ 619886u64;
    acc = acc.wrapping_mul(29055u64);
    acc = acc.wrapping_add(47189u64);
    acc = acc.rotate_left(1);
    acc = acc ^ 406442u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(245120u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(55019u64);
    acc = acc.wrapping_add(543815u64);
    acc = acc.wrapping_add(199388u64);
    acc = acc.wrapping_mul(53095u64);
    acc = acc.wrapping_add(98190u64);
    acc = acc ^ 63697u64;
    acc = acc ^ 327827u64;
    acc = acc.wrapping_add(984386u64);
    acc = acc.wrapping_add(26334u64);
    acc = acc ^ 135908u64;
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(432293u64);
    acc = acc ^ 887516u64;
    acc = acc.rotate_left(31);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(969572u64);
    acc = acc.wrapping_add(236834u64);
    acc = acc ^ 249447u64;
    acc = acc.wrapping_mul(12615u64);
    acc = acc.wrapping_mul(43037u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(689194u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(34419u64);
    acc = acc.wrapping_add(151377u64);
    acc = acc.wrapping_add(910193u64);
    acc = acc.wrapping_add(189226u64);
    acc = acc.wrapping_mul(2167u64);
    acc = acc ^ 781293u64;
    acc = acc.wrapping_mul(4881u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(518716u64);
    acc = acc.wrapping_add(759494u64);
    acc = acc.wrapping_add(22458u64);
    acc = acc ^ 560329u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(31345u64);
    acc = acc.wrapping_add(952729u64);
    acc = acc ^ 984272u64;
    acc = acc.wrapping_add(748229u64);
    acc = acc.wrapping_add(993826u64);
    acc = acc.wrapping_add(163323u64);
    acc = acc.wrapping_add(779808u64);
    acc = acc.wrapping_mul(20387u64);
    acc = acc.wrapping_add(296162u64);
    acc = acc.wrapping_mul(60713u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(17187u64);
    acc = acc.wrapping_mul(6945u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc ^ 769392u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 392263u64;
    acc = acc ^ 224845u64;
    acc = acc.wrapping_mul(51973u64);
    acc = acc ^ 544056u64;
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(642326u64);
    acc = acc.wrapping_add(154451u64);
    acc = acc.wrapping_mul(54061u64);
    acc = acc.wrapping_add(90636u64);
    acc = acc.wrapping_mul(22471u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(391041u64);
    acc = acc.wrapping_add(89014u64);
    acc = acc ^ 789616u64;
    acc = acc.wrapping_mul(2071u64);
    acc = acc.wrapping_mul(489u64);
    acc = acc.wrapping_add(457215u64);
    acc = acc ^ 578458u64;
    acc = acc ^ 810533u64;
    acc = acc.wrapping_mul(45205u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(24987u64);
    acc = acc.wrapping_add(532258u64);
    acc = acc.wrapping_mul(37433u64);
    acc = acc.wrapping_add(174707u64);
    acc = acc ^ 469114u64;
    acc = acc.wrapping_mul(24647u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(59577u64);
    acc = acc ^ 324795u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(681120u64);
    acc = acc.wrapping_add(505141u64);
    acc = acc.wrapping_add(900409u64);
    acc = acc.wrapping_mul(52543u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 491927u64;
    acc = acc.wrapping_mul(4059u64);
    acc = acc.wrapping_add(267417u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 170175u64;
    acc = acc.wrapping_add(738380u64);
    acc = acc ^ 898429u64;
    acc = acc.wrapping_mul(6277u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(299358u64);
    acc = acc ^ 465767u64;
    acc = acc.wrapping_mul(63433u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 751840u64;
    acc = acc ^ 739699u64;
    acc = acc.wrapping_mul(50661u64);
    acc = acc.wrapping_add(245315u64);
    acc = acc.wrapping_add(252348u64);
    acc = acc.wrapping_add(948433u64);
    acc = acc.rotate_left(9);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(7611u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(33531u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 957321u64;
    acc = acc.wrapping_mul(42995u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(660557u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(963257u64);
    acc = acc.wrapping_add(112915u64);
    acc = acc.wrapping_add(67778u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 133587u64;
    acc = acc.wrapping_add(372417u64);
    acc = acc.wrapping_mul(31323u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 739803u64;
    acc = acc.wrapping_mul(45185u64);
    acc = acc.wrapping_mul(42971u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(62577u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 457150u64;
    acc = acc.wrapping_add(877136u64);
    acc = acc.wrapping_mul(18023u64);
    acc = acc.wrapping_mul(15549u64);
    acc = acc.wrapping_add(971187u64);
    acc = acc.wrapping_mul(7263u64);
    acc = acc.wrapping_mul(39059u64);
    acc = acc.wrapping_add(377844u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(119752u64);
    acc = acc.wrapping_add(261220u64);
    acc = acc ^ 728707u64;
    acc = acc.wrapping_mul(3283u64);
    acc = acc ^ 417903u64;
    acc = acc.wrapping_add(771761u64);
    acc = acc ^ 422828u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(181126u64);
    acc = acc ^ 142990u64;
    acc = acc ^ 349412u64;
    acc = acc.wrapping_add(388993u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 456u64, b: -398i32, c: false } } }

enum NoiseType1 { V0(u64), V1(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(463u64) } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 983u64, b: 210i32, c: true } } }


static SVALS_14: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_14 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!(!(281750369u64) == (281750369u64) ^ u64::MAX);
        debug_assert!((2768490500u32).rotate_left(26).rotate_right(26) == 2768490500u32);
        assert!((1026978296u64) ^ (1026978296u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1297111533u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1668478833u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3126813056u64)));
        {

    let mut u = SU_14 { r: &SVALS_14[0] };
    u.r = &SVALS_14[0];
    let read_14_0 = unsafe { *u.r };
    print!("case_52:"); println!("{}", read_14_0);
    u.r = &SVALS_14[5];
    let read_14_1 = unsafe { *u.p };
    print!("case_52:"); println!("{}", read_14_1);
    u.p = &SVALS_14[0] as *const i64;
    let read_14_2 = unsafe { *u.p };
    print!("case_52:"); println!("{}", read_14_2);
    u.p = &SVALS_14[5] as *const i64;
    let read_14_3 = unsafe { *u.r };
    print!("case_52:"); println!("{}", read_14_3);

        }
        debug_assert!((802210987u32).rotate_left(3).rotate_right(3) == 802210987u32);
        debug_assert!((452540622u64).wrapping_mul(1) == 452540622u64);
        debug_assert!((3356121190u32).rotate_left(22).rotate_right(22) == 3356121190u32);
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(747566535u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1248074109u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(4238888303u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_53 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(7514u64).rotate_left(5) ^ 1306u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(1084u64).rotate_left(20) ^ 4638u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(67u64).rotate_left(28) ^ 4305u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(9109u64).rotate_left(31) ^ 5765u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 302761u64;
    acc = acc.wrapping_mul(53129u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 106211u64;
    acc = acc.wrapping_add(273980u64);
    acc = acc.wrapping_mul(30201u64);
    acc = acc.wrapping_mul(21449u64);
    acc = acc.wrapping_add(951196u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(62255u64);
    acc = acc.wrapping_mul(4379u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(768014u64);
    acc = acc ^ 932922u64;
    acc = acc.wrapping_add(468865u64);
    acc = acc ^ 876040u64;
    acc = acc ^ 360967u64;
    acc = acc ^ 585497u64;
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 549315u64;
    acc = acc.wrapping_add(143121u64);
    acc = acc ^ 458388u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(32133u64);
    acc = acc ^ 393066u64;
    acc = acc.wrapping_mul(57003u64);
    acc = acc.rotate_left(21);
    acc = acc.wrapping_add(680115u64);
    acc = acc.wrapping_mul(20617u64);
    acc = acc.wrapping_add(973516u64);
    acc = acc.wrapping_add(667410u64);
    acc = acc.wrapping_mul(23631u64);
    acc = acc ^ 199784u64;
    acc = acc.rotate_left(8);
    acc = acc ^ 351115u64;
    acc = acc.wrapping_add(32832u64);
    acc = acc ^ 578769u64;
    acc = acc.wrapping_add(279110u64);
    acc = acc.wrapping_mul(43183u64);
    acc = acc.wrapping_add(274540u64);
    acc = acc.wrapping_mul(22233u64);
    acc = acc.wrapping_add(474267u64);
    acc = acc.wrapping_add(524694u64);
    acc = acc ^ 763990u64;
    acc = acc.wrapping_add(433307u64);
    acc = acc.wrapping_add(375457u64);
    acc = acc ^ 762741u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(7457u64);
    acc = acc.wrapping_add(494466u64);
    acc = acc.wrapping_mul(16389u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(37463u64);
    acc = acc.wrapping_mul(12853u64);
    acc = acc ^ 608407u64;
    acc = acc ^ 996852u64;
    acc = acc.wrapping_add(56269u64);
    acc = acc ^ 928175u64;
    acc = acc ^ 562290u64;
    acc = acc ^ 928110u64;
    acc = acc ^ 319859u64;
    acc = acc ^ 749308u64;
    acc = acc.wrapping_add(954326u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 746229u64;
    acc = acc ^ 533166u64;
    acc = acc.wrapping_add(806695u64);
    acc = acc.wrapping_mul(40217u64);
    acc = acc ^ 267243u64;
    acc = acc.wrapping_add(680496u64);
    acc = acc.wrapping_mul(13063u64);
    acc = acc ^ 454194u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(4383u64);
    acc = acc.wrapping_mul(43943u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(9669u64);
    acc = acc.wrapping_mul(9727u64);
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(13015u64);
    acc = acc ^ 350130u64;
    acc = acc.wrapping_mul(59187u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(451u64);
    acc = acc.wrapping_mul(42249u64);
    acc = acc.wrapping_add(698507u64);
    acc = acc.wrapping_add(646465u64);
    acc = acc.wrapping_add(181572u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(959315u64);
    acc = acc.wrapping_add(111767u64);
    acc = acc.wrapping_add(439032u64);
    acc = acc.wrapping_add(831010u64);
    acc = acc.rotate_left(15);
    acc = acc ^ 796207u64;
    acc = acc.wrapping_add(541197u64);
    acc = acc ^ 594136u64;
    acc = acc.wrapping_mul(28891u64);
    acc = acc.wrapping_mul(64141u64);
    acc = acc.wrapping_mul(35929u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(24465u64);
    acc = acc.wrapping_add(356749u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(42431u64);
    acc = acc.wrapping_mul(64879u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(875286u64);
    acc = acc ^ 809613u64;
    acc = acc.wrapping_mul(65077u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(70883u64);
    acc = acc ^ 727300u64;
    acc = acc.wrapping_mul(33661u64);
    acc = acc ^ 76217u64;
    acc = acc.wrapping_add(36580u64);
    acc = acc.wrapping_add(163855u64);
    acc = acc.wrapping_add(801607u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(7743u64);
    acc = acc.wrapping_add(219993u64);
    acc = acc ^ 441791u64;
    acc = acc ^ 546427u64;
    acc = acc ^ 73357u64;
    acc = acc ^ 201480u64;
    acc = acc.wrapping_add(243875u64);
    acc = acc ^ 900312u64;
    acc = acc ^ 338921u64;
    acc = acc ^ 590938u64;
    acc = acc ^ 761196u64;
    acc = acc.wrapping_mul(55591u64);
    acc = acc.wrapping_add(335692u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 314142u64;
    acc = acc.wrapping_add(130929u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(664682u64);
    acc = acc.wrapping_add(530493u64);
    acc = acc.wrapping_add(883665u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(54419u64);
    acc = acc.wrapping_add(203172u64);
    acc = acc ^ 739797u64;
    acc = acc.wrapping_add(109291u64);
    acc = acc ^ 640084u64;
    acc = acc.wrapping_mul(29383u64);
    acc = acc ^ 672659u64;
    acc = acc.rotate_left(16);
    acc = acc ^ 144243u64;
    acc = acc.wrapping_add(544448u64);
    acc = acc.rotate_left(6);
    acc = acc ^ 22399u64;
    acc = acc.wrapping_mul(21453u64);
    acc = acc.wrapping_mul(39903u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(3269u64);
    acc = acc.wrapping_add(98416u64);
    acc = acc ^ 575870u64;
    acc = acc ^ 875914u64;
    acc = acc.wrapping_add(708395u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc ^ 318559u64;
    acc = acc ^ 951948u64;
    acc = acc ^ 17299u64;
    acc = acc.wrapping_mul(45679u64);
    acc = acc.wrapping_add(115636u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 977651u64;
    acc = acc.wrapping_mul(55905u64);
    acc = acc.wrapping_add(63070u64);
    acc = acc.wrapping_mul(58489u64);
    acc = acc ^ 417220u64;
    acc = acc.wrapping_mul(30617u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(39659u64);
    acc = acc.wrapping_mul(37043u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(13671u64);
    acc = acc ^ 676200u64;
    acc = acc ^ 327646u64;
    acc = acc.wrapping_mul(25505u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(29317u64);
    acc = acc ^ 474800u64;
    acc = acc ^ 986908u64;
    acc = acc ^ 126393u64;
    acc = acc.wrapping_mul(36421u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V1(557u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 265u64, b: 99i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(563u64) } }


static SVALS_15: [i64; 6] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64];

union SU_15 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((474299599u64).wrapping_add(474299599u64).wrapping_sub(474299599u64) == 474299599u64);
        debug_assert!((3099318375u64).wrapping_mul(1) == 3099318375u64);
        assert!((2307491353u64) ^ (2307491353u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1877126710u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(2545235169u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2593922028u64)));
        {

    let mut u = SU_15 { p: &SVALS_15[0] as *const i64 };
    u.r = &SVALS_15[0];
    let read_15_0 = unsafe { *u.r };
    print!("case_53:"); println!("{}", read_15_0);
    u.r = &SVALS_15[5];
    let read_15_1 = unsafe { *u.p };
    print!("case_53:"); println!("{}", read_15_1);
    u.p = &SVALS_15[0] as *const i64;
    let read_15_2 = unsafe { *u.p };
    print!("case_53:"); println!("{}", read_15_2);
    u.p = &SVALS_15[5] as *const i64;
    let read_15_3 = unsafe { *u.r };
    print!("case_53:"); println!("{}", read_15_3);

        }
        debug_assert!((3127694958u32) | (63605881u32) | (3127694958u32) == (3127694958u32) | (63605881u32));
        debug_assert!((3135463386u64).wrapping_mul(1) == 3135463386u64);
        assert!((4057892319u64).wrapping_add(4057892319u64).wrapping_sub(4057892319u64) == 4057892319u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1559701898u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2980336714u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(19621714u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_54 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(5196u64).rotate_left(6) ^ 3043u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(8936u64).rotate_left(5) ^ 4871u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(1760u64).rotate_left(24) ^ 2193u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(1939u64).rotate_left(31) ^ 9609u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(709198u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 541183u64;
    acc = acc.wrapping_add(240301u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(1599u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(127556u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(25429u64);
    acc = acc.wrapping_add(182798u64);
    acc = acc.wrapping_mul(11471u64);
    acc = acc.wrapping_add(128873u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(54761u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(443168u64);
    acc = acc.wrapping_add(782777u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(11593u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(276556u64);
    acc = acc ^ 177307u64;
    acc = acc ^ 750390u64;
    acc = acc.wrapping_mul(35433u64);
    acc = acc.wrapping_mul(19901u64);
    acc = acc.wrapping_mul(9979u64);
    acc = acc ^ 152607u64;
    acc = acc.rotate_left(14);
    acc = acc ^ 931000u64;
    acc = acc.wrapping_mul(55937u64);
    acc = acc.wrapping_add(227376u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 686447u64;
    acc = acc.wrapping_mul(55369u64);
    acc = acc.wrapping_mul(9903u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(83985u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(731153u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 438522u64;
    acc = acc.wrapping_add(80223u64);
    acc = acc ^ 869342u64;
    acc = acc ^ 748238u64;
    acc = acc.wrapping_mul(37529u64);
    acc = acc ^ 702011u64;
    acc = acc.wrapping_mul(48359u64);
    acc = acc.wrapping_mul(48749u64);
    acc = acc.wrapping_mul(32369u64);
    acc = acc ^ 772146u64;
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 729014u64;
    acc = acc.wrapping_mul(11085u64);
    acc = acc ^ 150255u64;
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(975524u64);
    acc = acc.wrapping_add(305569u64);
    acc = acc.wrapping_add(321262u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 353575u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(911512u64);
    acc = acc.wrapping_add(836005u64);
    acc = acc ^ 939054u64;
    acc = acc.wrapping_add(607442u64);
    acc = acc.wrapping_add(595621u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(43701u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(411898u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc ^ 803784u64;
    acc = acc.wrapping_mul(21985u64);
    acc = acc ^ 257464u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 202680u64;
    acc = acc ^ 35985u64;
    acc = acc.wrapping_add(543698u64);
    acc = acc.wrapping_add(452892u64);
    acc = acc ^ 581330u64;
    acc = acc.wrapping_mul(1563u64);
    acc = acc ^ 287104u64;
    acc = acc ^ 381182u64;
    acc = acc.wrapping_mul(56063u64);
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(1003u64);
    acc = acc.wrapping_mul(35883u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(26613u64);
    acc = acc.wrapping_mul(12561u64);
    acc = acc ^ 456467u64;
    acc = acc ^ 309754u64;
    acc = acc.wrapping_add(740610u64);
    acc = acc ^ 330449u64;
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(60619u64);
    acc = acc.wrapping_mul(35039u64);
    acc = acc.rotate_left(23);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_mul(38085u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(24075u64);
    acc = acc.wrapping_mul(37361u64);
    acc = acc.wrapping_mul(10323u64);
    acc = acc.wrapping_add(895034u64);
    acc = acc.wrapping_add(590134u64);
    acc = acc.wrapping_add(668354u64);
    acc = acc ^ 793012u64;
    acc = acc ^ 115817u64;
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(143378u64);
    acc = acc ^ 938904u64;
    acc = acc ^ 168923u64;
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(56379u64);
    acc = acc.wrapping_mul(28775u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(17705u64);
    acc = acc.wrapping_mul(2897u64);
    acc = acc ^ 941539u64;
    acc = acc.wrapping_mul(26309u64);
    acc = acc.wrapping_add(841143u64);
    acc = acc ^ 153918u64;
    acc = acc.wrapping_mul(2995u64);
    acc = acc.wrapping_mul(37393u64);
    acc = acc.wrapping_add(743457u64);
    acc = acc.wrapping_mul(27629u64);
    acc = acc ^ 834830u64;
    acc = acc.wrapping_mul(44673u64);
    acc = acc ^ 31785u64;
    acc = acc.rotate_left(18);
    acc = acc ^ 430124u64;
    acc = acc.wrapping_add(400444u64);
    acc = acc.wrapping_mul(22015u64);
    acc = acc.wrapping_add(300543u64);
    acc = acc.wrapping_add(893331u64);
    acc = acc.wrapping_mul(24521u64);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(332316u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(499584u64);
    acc = acc.wrapping_mul(8993u64);
    acc = acc.rotate_left(23);
    acc = acc ^ 835412u64;
    acc = acc.wrapping_add(451173u64);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(31725u64);
    acc = acc.wrapping_mul(23657u64);
    acc = acc ^ 340712u64;
    acc = acc.wrapping_add(2413u64);
    acc = acc.wrapping_mul(63139u64);
    acc = acc ^ 575701u64;
    acc = acc ^ 944546u64;
    acc = acc.wrapping_add(184444u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 215157u64;
    acc = acc.wrapping_add(593830u64);
    acc = acc ^ 371970u64;
    acc = acc.wrapping_add(85635u64);
    acc = acc.wrapping_mul(58681u64);
    acc = acc ^ 334502u64;
    acc = acc.wrapping_add(334403u64);
    acc = acc ^ 875403u64;
    acc = acc ^ 652868u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(17800u64);
    acc = acc.wrapping_mul(43663u64);
    acc = acc ^ 760119u64;
    acc = acc ^ 34u64;
    acc = acc.wrapping_add(518368u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 342118u64;
    acc = acc.wrapping_mul(35673u64);
    acc = acc.wrapping_add(792346u64);
    acc = acc.wrapping_add(127209u64);
    acc = acc ^ 214315u64;
    acc = acc ^ 392964u64;
    acc = acc.wrapping_mul(55929u64);
    acc = acc.wrapping_add(950451u64);
    acc = acc.wrapping_add(329339u64);
    acc = acc.wrapping_mul(44417u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 502420u64;
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(948556u64);
    acc = acc.wrapping_mul(16125u64);
    acc = acc ^ 964831u64;
    acc = acc ^ 417169u64;
    acc = acc.wrapping_mul(62211u64);
    acc = acc.wrapping_mul(4523u64);
    acc = acc.wrapping_mul(50903u64);
    acc = acc.wrapping_add(684449u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(58289u64);
    acc = acc ^ 746405u64;
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(28937u64);
    acc = acc.wrapping_add(793531u64);
    acc = acc.wrapping_add(36572u64);
    acc = acc ^ 630022u64;
    acc = acc.wrapping_add(324709u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(269598u64);
    acc = acc.rotate_left(23);
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(102u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(796u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(859u64) } }


static SVALS_16: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_16 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((1679953868u32).rotate_left(12).rotate_right(12) == 1679953868u32);
        debug_assert!((2111117455u64).wrapping_mul(1) == 2111117455u64);
        assert!(!(2954560172u64) == (2954560172u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(108808020u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1395440361u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(4006568203u64)));
        {

    let mut u = SU_16 { r: &SVALS_16[0] };
    u.r = &SVALS_16[0];
    let read_16_0 = unsafe { *u.r };
    print!("case_54:"); println!("{}", read_16_0);
    u.p = &SVALS_16[7] as *const i64;
    let read_16_1 = unsafe { *u.p };
    print!("case_54:"); println!("{}", read_16_1);

        }
        assert!(!(301419274u64) == (301419274u64) ^ u64::MAX);
        assert!((867913580u64) ^ (867913580u64) == 0u64);
        debug_assert!((1762675529u32) | (720648871u32) | (1762675529u32) == (1762675529u32) | (720648871u32));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(483180870u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2616011943u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2123852852u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_55 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(4011u64).rotate_left(1) ^ 7936u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(4045u64).rotate_left(26) ^ 3869u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(4781u64).rotate_left(11) ^ 9248u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(4422u64).rotate_left(1) ^ 3131u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(42267u64);
    acc = acc.wrapping_add(897493u64);
    acc = acc.wrapping_mul(46683u64);
    acc = acc.wrapping_add(504891u64);
    acc = acc.wrapping_add(48209u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(941361u64);
    acc = acc.wrapping_add(717456u64);
    acc = acc.wrapping_add(42085u64);
    acc = acc.wrapping_mul(50641u64);
    acc = acc.wrapping_mul(59217u64);
    acc = acc.wrapping_add(165967u64);
    acc = acc.wrapping_mul(7225u64);
    acc = acc ^ 824378u64;
    acc = acc ^ 917968u64;
    acc = acc.wrapping_add(239625u64);
    acc = acc ^ 179645u64;
    acc = acc.wrapping_mul(31815u64);
    acc = acc.wrapping_add(206589u64);
    acc = acc.wrapping_mul(43893u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_add(5345u64);
    acc = acc.wrapping_add(255822u64);
    acc = acc.wrapping_mul(22783u64);
    acc = acc.wrapping_add(651040u64);
    acc = acc.wrapping_mul(59685u64);
    acc = acc.wrapping_mul(38115u64);
    acc = acc.wrapping_mul(35021u64);
    acc = acc ^ 701522u64;
    acc = acc.wrapping_mul(28113u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(32923u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 85426u64;
    acc = acc.wrapping_mul(16731u64);
    acc = acc.wrapping_add(273695u64);
    acc = acc.wrapping_add(204650u64);
    acc = acc ^ 829833u64;
    acc = acc.wrapping_mul(33103u64);
    acc = acc.wrapping_mul(25583u64);
    acc = acc ^ 491956u64;
    acc = acc.wrapping_add(597441u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(712817u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 246682u64;
    acc = acc.wrapping_add(633375u64);
    acc = acc ^ 296830u64;
    acc = acc ^ 437382u64;
    acc = acc.wrapping_add(616146u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(2589u64);
    acc = acc.wrapping_add(685177u64);
    acc = acc.wrapping_mul(483u64);
    acc = acc ^ 987155u64;
    acc = acc ^ 904402u64;
    acc = acc.wrapping_mul(28045u64);
    acc = acc.wrapping_mul(351u64);
    acc = acc ^ 90273u64;
    acc = acc.wrapping_mul(3389u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(27629u64);
    acc = acc.wrapping_add(713614u64);
    acc = acc ^ 575209u64;
    acc = acc ^ 932589u64;
    acc = acc ^ 430137u64;
    acc = acc ^ 805886u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(841644u64);
    acc = acc.wrapping_mul(2423u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(1935u64);
    acc = acc.wrapping_mul(20337u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 610210u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(894043u64);
    acc = acc.wrapping_mul(22071u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(53583u64);
    acc = acc.wrapping_add(369296u64);
    acc = acc.wrapping_add(860523u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(37183u64);
    acc = acc.wrapping_mul(22741u64);
    acc = acc ^ 333565u64;
    acc = acc.wrapping_mul(29635u64);
    acc = acc.wrapping_add(899426u64);
    acc = acc.rotate_left(4);
    acc = acc ^ 159323u64;
    acc = acc ^ 540360u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(59477u64);
    acc = acc.wrapping_mul(47873u64);
    acc = acc.wrapping_add(183887u64);
    acc = acc.wrapping_add(959558u64);
    acc = acc.wrapping_add(174925u64);
    acc = acc.wrapping_mul(41707u64);
    acc = acc.wrapping_mul(19549u64);
    acc = acc.wrapping_add(669466u64);
    acc = acc.wrapping_mul(50759u64);
    acc = acc ^ 106835u64;
    acc = acc.wrapping_add(50845u64);
    acc = acc.wrapping_mul(23111u64);
    acc = acc ^ 129331u64;
    acc = acc.wrapping_mul(6425u64);
    acc = acc ^ 844519u64;
    acc = acc.rotate_left(13);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(26839u64);
    acc = acc ^ 339589u64;
    acc = acc.wrapping_mul(18569u64);
    acc = acc.wrapping_mul(37343u64);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc ^ 938741u64;
    acc = acc.wrapping_mul(58053u64);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(785445u64);
    acc = acc.wrapping_add(969238u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(294430u64);
    acc = acc ^ 999438u64;
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(26841u64);
    acc = acc.wrapping_add(729337u64);
    acc = acc ^ 102729u64;
    acc = acc ^ 932994u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(752998u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(62921u64);
    acc = acc.wrapping_add(217002u64);
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(5313u64);
    acc = acc.wrapping_mul(11435u64);
    acc = acc.wrapping_add(217281u64);
    acc = acc ^ 33610u64;
    acc = acc.rotate_left(21);
    acc = acc ^ 492931u64;
    acc = acc.wrapping_mul(52013u64);
    acc = acc.wrapping_add(636028u64);
    acc = acc ^ 403683u64;
    acc = acc ^ 812184u64;
    acc = acc.wrapping_mul(18163u64);
    acc = acc.wrapping_mul(44199u64);
    acc = acc ^ 719032u64;
    acc = acc ^ 657635u64;
    acc = acc.wrapping_add(564378u64);
    acc = acc ^ 745950u64;
    acc = acc ^ 520326u64;
    acc = acc.wrapping_mul(5509u64);
    acc = acc ^ 610385u64;
    acc = acc.wrapping_mul(31043u64);
    acc = acc ^ 893550u64;
    acc = acc.wrapping_mul(23507u64);
    acc = acc ^ 545989u64;
    acc = acc ^ 164046u64;
    acc = acc.wrapping_add(140653u64);
    acc = acc.wrapping_add(528931u64);
    acc = acc ^ 748012u64;
    acc = acc.wrapping_add(751309u64);
    acc = acc.wrapping_add(461875u64);
    acc = acc.wrapping_add(666841u64);
    acc = acc.rotate_left(17);
    acc = acc ^ 364378u64;
    acc = acc ^ 223317u64;
    acc = acc.wrapping_add(218109u64);
    acc = acc.wrapping_add(332259u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 329557u64;
    acc = acc.wrapping_add(630906u64);
    acc = acc ^ 368795u64;
    acc = acc.wrapping_add(715732u64);
    acc = acc.rotate_left(25);
    acc = acc ^ 554071u64;
    acc = acc.wrapping_mul(7407u64);
    acc = acc.wrapping_add(831659u64);
    acc = acc ^ 60271u64;
    acc = acc.wrapping_mul(39899u64);
    acc = acc.wrapping_mul(39305u64);
    acc = acc.wrapping_add(975905u64);
    acc = acc.wrapping_add(989137u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(364232u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(60177u64);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc ^ 809589u64;
    acc = acc.rotate_left(27);
    acc = acc ^ 122342u64;
    acc = acc.wrapping_add(715291u64);
    acc = acc.wrapping_mul(10311u64);
    acc = acc ^ 540869u64;
    acc = acc.wrapping_add(661746u64);
    acc = acc.wrapping_mul(58319u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 17u64, b: 336i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 116u64, b: -961i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 394u64, b: 310i32, c: true } } }


static SVALS_17: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_17 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((1127891205u64) ^ (1127891205u64) == 0u64);
        assert!((3233119340u64) ^ (3233119340u64) == 0u64);
        debug_assert!((3851891331u32) | (1235937549u32) | (3851891331u32) == (3851891331u32) | (1235937549u32));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1058089335u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(1604073958u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(298831552u64)));
        {

    let mut u = SU_17 { p: &SVALS_17[0] as *const i64 };
    u.r = &SVALS_17[0];
    let read_17_0 = unsafe { *u.r };
    print!("case_55:"); println!("{}", read_17_0);
    u.p = &SVALS_17[7] as *const i64;
    let read_17_1 = unsafe { *u.p };
    print!("case_55:"); println!("{}", read_17_1);

        }
        debug_assert!((786473376u64).wrapping_mul(1) == 786473376u64);
        debug_assert!((1694375929u32).rotate_left(22).rotate_right(22) == 1694375929u32);
        assert!(!(2234346575u64) == (2234346575u64) ^ u64::MAX);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(820765397u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2022121248u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(444123029u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_56 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(9756u64).rotate_left(1) ^ 9021u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(4301u64).rotate_left(31) ^ 2541u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_add(5536u64).rotate_left(30) ^ 4648u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(9956u64).rotate_left(14) ^ 9127u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc ^ 92455u64;
    acc = acc ^ 95898u64;
    acc = acc ^ 47197u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(7581u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(625535u64);
    acc = acc.wrapping_add(875492u64);
    acc = acc ^ 597693u64;
    acc = acc ^ 528234u64;
    acc = acc ^ 991558u64;
    acc = acc ^ 637826u64;
    acc = acc ^ 571282u64;
    acc = acc.wrapping_add(24480u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 998258u64;
    acc = acc.wrapping_mul(40321u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(335196u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 403011u64;
    acc = acc.rotate_left(3);
    acc = acc ^ 998630u64;
    acc = acc.wrapping_add(640039u64);
    acc = acc ^ 569859u64;
    acc = acc ^ 256432u64;
    acc = acc.wrapping_add(313895u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(977451u64);
    acc = acc ^ 252341u64;
    acc = acc.wrapping_add(274738u64);
    acc = acc.wrapping_mul(22797u64);
    acc = acc.wrapping_mul(43029u64);
    acc = acc.wrapping_add(919030u64);
    acc = acc.wrapping_mul(15685u64);
    acc = acc ^ 603011u64;
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(44529u64);
    acc = acc.wrapping_add(169606u64);
    acc = acc.wrapping_add(948301u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(34535u64);
    acc = acc ^ 722459u64;
    acc = acc.wrapping_mul(28969u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(38979u64);
    acc = acc ^ 60078u64;
    acc = acc ^ 692150u64;
    acc = acc ^ 286816u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(5385u64);
    acc = acc.wrapping_add(198820u64);
    acc = acc.wrapping_add(855768u64);
    acc = acc ^ 189083u64;
    acc = acc ^ 769770u64;
    acc = acc ^ 34555u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_mul(62047u64);
    acc = acc ^ 456111u64;
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc ^ 507003u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(20269u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(22075u64);
    acc = acc.wrapping_mul(12813u64);
    acc = acc ^ 83973u64;
    acc = acc.wrapping_add(119994u64);
    acc = acc.rotate_left(5);
    acc = acc ^ 523978u64;
    acc = acc ^ 829174u64;
    acc = acc.wrapping_add(71768u64);
    acc = acc.wrapping_add(127688u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(727118u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(742737u64);
    acc = acc ^ 213798u64;
    acc = acc ^ 627152u64;
    acc = acc ^ 763261u64;
    acc = acc.wrapping_mul(4035u64);
    acc = acc ^ 558965u64;
    acc = acc ^ 74108u64;
    acc = acc.wrapping_mul(65205u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(64989u64);
    acc = acc.wrapping_add(668937u64);
    acc = acc.wrapping_add(119625u64);
    acc = acc ^ 489054u64;
    acc = acc.wrapping_mul(51447u64);
    acc = acc ^ 165901u64;
    acc = acc.wrapping_mul(48707u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 222527u64;
    acc = acc.rotate_left(5);
    acc = acc.wrapping_mul(51731u64);
    acc = acc.rotate_left(19);
    acc = acc ^ 288828u64;
    acc = acc.wrapping_mul(4841u64);
    acc = acc.wrapping_mul(15873u64);
    acc = acc.wrapping_mul(50971u64);
    acc = acc ^ 899017u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(168534u64);
    acc = acc.wrapping_mul(46051u64);
    acc = acc.wrapping_mul(33887u64);
    acc = acc.wrapping_add(569847u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(495416u64);
    acc = acc ^ 645225u64;
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(87369u64);
    acc = acc ^ 769521u64;
    acc = acc ^ 665376u64;
    acc = acc.wrapping_mul(43451u64);
    acc = acc ^ 57684u64;
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(838498u64);
    acc = acc.wrapping_add(944703u64);
    acc = acc.wrapping_mul(963u64);
    acc = acc.wrapping_add(622237u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 572296u64;
    acc = acc ^ 697362u64;
    acc = acc.wrapping_mul(29983u64);
    acc = acc.wrapping_mul(12677u64);
    acc = acc.wrapping_mul(52367u64);
    acc = acc.wrapping_mul(61761u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(62979u64);
    acc = acc.wrapping_mul(33297u64);
    acc = acc ^ 621305u64;
    acc = acc.wrapping_add(599664u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(228734u64);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(7239u64);
    acc = acc.wrapping_mul(15635u64);
    acc = acc.wrapping_mul(19929u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(354137u64);
    acc = acc.wrapping_add(792565u64);
    acc = acc.wrapping_mul(51849u64);
    acc = acc.wrapping_mul(65527u64);
    acc = acc.rotate_left(11);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(325469u64);
    acc = acc.wrapping_add(347178u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_mul(12495u64);
    acc = acc.wrapping_add(484922u64);
    acc = acc.wrapping_mul(737u64);
    acc = acc ^ 765568u64;
    acc = acc ^ 379320u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_add(749986u64);
    acc = acc.wrapping_mul(2933u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 512710u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(60123u64);
    acc = acc.wrapping_add(469901u64);
    acc = acc.wrapping_add(230402u64);
    acc = acc ^ 325095u64;
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc ^ 941123u64;
    acc = acc.rotate_left(14);
    acc = acc ^ 747690u64;
    acc = acc.wrapping_mul(41875u64);
    acc = acc ^ 570267u64;
    acc = acc.wrapping_mul(1781u64);
    acc = acc.wrapping_mul(34775u64);
    acc = acc.wrapping_add(618198u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(34301u64);
    acc = acc.wrapping_mul(61553u64);
    acc = acc.wrapping_mul(58881u64);
    acc = acc ^ 751080u64;
    acc = acc.wrapping_mul(34785u64);
    acc = acc.wrapping_mul(47089u64);
    acc = acc ^ 359692u64;
    acc = acc ^ 609753u64;
    acc = acc.wrapping_mul(34157u64);
    acc = acc.wrapping_add(296367u64);
    acc = acc.rotate_left(16);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 608u64, b: -881i32, c: true } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 384u64, b: -890i32, c: true } } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(677u64) } }


static SVALS_18: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_18 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!(!(1533125607u64) == (1533125607u64) ^ u64::MAX);
        debug_assert!((3518644378u32).rotate_left(6).rotate_right(6) == 3518644378u32);
        assert!((2544980227u64).wrapping_add(2544980227u64).wrapping_sub(2544980227u64) == 2544980227u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(982666907u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2621714065u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(3337354775u64)));
        {

    let mut u = SU_18 { r: &SVALS_18[0] };
    u.r = &SVALS_18[0];
    let read_18_0 = unsafe { *u.p };
    print!("case_56:"); println!("{}", read_18_0);
    u.p = &SVALS_18[1] as *const i64;
    let read_18_1 = unsafe { *u.r };
    print!("case_56:"); println!("{}", read_18_1);

        }
        assert!((2827347751u64) ^ (2827347751u64) == 0u64);
        assert!((3597935196u64).wrapping_add(3597935196u64).wrapping_sub(3597935196u64) == 3597935196u64);
        assert!((874964562u64).wrapping_add(874964562u64).wrapping_sub(874964562u64) == 874964562u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2894917866u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(797487686u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(321614861u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_57 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(7181u64).rotate_left(25) ^ 3403u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(9674u64).rotate_left(18) ^ 9069u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(204u64).rotate_left(30) ^ 6508u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(4956u64).rotate_left(9) ^ 2767u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_add(374570u64);
    acc = acc.wrapping_add(401993u64);
    acc = acc.wrapping_add(214781u64);
    acc = acc ^ 223193u64;
    acc = acc.wrapping_mul(12373u64);
    acc = acc ^ 7016u64;
    acc = acc.wrapping_mul(21857u64);
    acc = acc.wrapping_mul(30621u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(239907u64);
    acc = acc.wrapping_add(610881u64);
    acc = acc.wrapping_add(615292u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(60109u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.rotate_left(30);
    acc = acc ^ 32309u64;
    acc = acc.wrapping_mul(51451u64);
    acc = acc.wrapping_mul(6621u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(52501u64);
    acc = acc.wrapping_mul(7935u64);
    acc = acc ^ 260082u64;
    acc = acc.wrapping_mul(20223u64);
    acc = acc.wrapping_add(877250u64);
    acc = acc.wrapping_add(291115u64);
    acc = acc.wrapping_add(213355u64);
    acc = acc.wrapping_add(589349u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(47047u64);
    acc = acc ^ 48661u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(166247u64);
    acc = acc.wrapping_add(210486u64);
    acc = acc ^ 995628u64;
    acc = acc ^ 931956u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(806120u64);
    acc = acc ^ 504386u64;
    acc = acc ^ 591032u64;
    acc = acc.wrapping_add(235290u64);
    acc = acc.wrapping_mul(33077u64);
    acc = acc.wrapping_mul(40745u64);
    acc = acc ^ 953413u64;
    acc = acc.wrapping_add(909161u64);
    acc = acc.wrapping_add(100613u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(30933u64);
    acc = acc ^ 228410u64;
    acc = acc.wrapping_mul(14677u64);
    acc = acc.wrapping_mul(46745u64);
    acc = acc.wrapping_mul(25959u64);
    acc = acc.wrapping_add(375641u64);
    acc = acc ^ 970955u64;
    acc = acc.wrapping_mul(41321u64);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_add(111534u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(12021u64);
    acc = acc ^ 301669u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 64042u64;
    acc = acc.wrapping_add(21260u64);
    acc = acc ^ 94176u64;
    acc = acc.rotate_left(1);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc ^ 21588u64;
    acc = acc.wrapping_add(304758u64);
    acc = acc.wrapping_mul(60373u64);
    acc = acc ^ 62636u64;
    acc = acc ^ 567353u64;
    acc = acc.wrapping_add(430804u64);
    acc = acc ^ 858289u64;
    acc = acc ^ 444552u64;
    acc = acc.rotate_left(12);
    acc = acc ^ 346104u64;
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(52145u64);
    acc = acc.wrapping_add(622686u64);
    acc = acc.wrapping_add(26443u64);
    acc = acc.wrapping_mul(24737u64);
    acc = acc ^ 815250u64;
    acc = acc.wrapping_mul(33795u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 494688u64;
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc ^ 795721u64;
    acc = acc.rotate_left(17);
    acc = acc ^ 932734u64;
    acc = acc.wrapping_mul(36989u64);
    acc = acc.wrapping_mul(38301u64);
    acc = acc.wrapping_mul(5831u64);
    acc = acc.wrapping_mul(60401u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(25101u64);
    acc = acc ^ 295825u64;
    acc = acc.wrapping_add(530221u64);
    acc = acc.rotate_left(25);
    acc = acc ^ 545480u64;
    acc = acc ^ 432754u64;
    acc = acc.wrapping_add(948706u64);
    acc = acc.rotate_left(22);
    acc = acc.rotate_left(29);
    acc = acc.wrapping_mul(25799u64);
    acc = acc.wrapping_add(556472u64);
    acc = acc.wrapping_add(812269u64);
    acc = acc.wrapping_add(482184u64);
    acc = acc ^ 30801u64;
    acc = acc.wrapping_mul(37215u64);
    acc = acc.wrapping_add(678725u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(814661u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(830183u64);
    acc = acc.wrapping_add(790755u64);
    acc = acc.wrapping_mul(57943u64);
    acc = acc ^ 578713u64;
    acc = acc.wrapping_add(339375u64);
    acc = acc.wrapping_mul(21349u64);
    acc = acc.wrapping_add(63977u64);
    acc = acc.wrapping_add(694166u64);
    acc = acc ^ 758807u64;
    acc = acc.wrapping_mul(5051u64);
    acc = acc.wrapping_mul(29603u64);
    acc = acc ^ 540219u64;
    acc = acc.wrapping_add(559241u64);
    acc = acc.wrapping_mul(4743u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 431341u64;
    acc = acc.wrapping_add(402009u64);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(132874u64);
    acc = acc.wrapping_add(654774u64);
    acc = acc.wrapping_mul(3093u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 228u64, b: -611i32, c: true } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V1(153u64) } }

enum NoiseType2 { V0(u64), V1(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(524u64) } }


static SVALS_19: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_19 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((1428691190u32) | (3661288948u32) | (1428691190u32) == (1428691190u32) | (3661288948u32));
        assert!(!(3009097484u64) == (3009097484u64) ^ u64::MAX);
        debug_assert!((4147320687u32).rotate_left(11).rotate_right(11) == 4147320687u32);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2231032643u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(537730400u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(949954329u64)));
        {

    let mut u = SU_19 { p: &SVALS_19[0] as *const i64 };
    u.r = &SVALS_19[0];
    let read_19_0 = unsafe { *u.p };
    print!("case_57:"); println!("{}", read_19_0);
    u.p = &SVALS_19[1] as *const i64;
    let read_19_1 = unsafe { *u.r };
    print!("case_57:"); println!("{}", read_19_1);

        }
        debug_assert!((344662851u64).wrapping_mul(1) == 344662851u64);
        debug_assert!((2307169621u32).rotate_left(27).rotate_right(27) == 2307169621u32);
        debug_assert!((4208678411u64).wrapping_mul(1) == 4208678411u64);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(4061525814u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(888328766u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(1463833670u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_58 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(5295u64).rotate_left(4) ^ 8125u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_sub(9030u64).rotate_left(23) ^ 8826u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(9893u64).rotate_left(20) ^ 5847u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_mul(5237u64).rotate_left(19) ^ 421u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 474629u64;
    acc = acc ^ 663290u64;
    acc = acc.wrapping_add(612832u64);
    acc = acc.wrapping_add(779477u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(7049u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(47657u64);
    acc = acc.wrapping_mul(23549u64);
    acc = acc.wrapping_add(667356u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(818195u64);
    acc = acc ^ 932285u64;
    acc = acc ^ 616395u64;
    acc = acc.wrapping_add(678571u64);
    acc = acc.wrapping_add(766712u64);
    acc = acc.wrapping_mul(38181u64);
    acc = acc.wrapping_mul(17633u64);
    acc = acc ^ 166640u64;
    acc = acc.rotate_left(13);
    acc = acc.wrapping_add(870480u64);
    acc = acc.wrapping_mul(61749u64);
    acc = acc.wrapping_add(354093u64);
    acc = acc.wrapping_mul(7195u64);
    acc = acc.wrapping_add(290033u64);
    acc = acc.wrapping_mul(30595u64);
    acc = acc.wrapping_mul(59639u64);
    acc = acc.wrapping_add(876511u64);
    acc = acc ^ 695739u64;
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(380080u64);
    acc = acc.rotate_left(18);
    acc = acc ^ 888144u64;
    acc = acc.wrapping_add(207119u64);
    acc = acc ^ 497098u64;
    acc = acc.wrapping_add(867466u64);
    acc = acc ^ 404606u64;
    acc = acc ^ 90381u64;
    acc = acc ^ 667224u64;
    acc = acc.wrapping_add(355353u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(37457u64);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(20375u64);
    acc = acc ^ 970076u64;
    acc = acc ^ 643797u64;
    acc = acc.wrapping_add(526144u64);
    acc = acc.wrapping_mul(31441u64);
    acc = acc.wrapping_add(535541u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(823688u64);
    acc = acc ^ 789822u64;
    acc = acc.wrapping_add(685781u64);
    acc = acc.rotate_left(24);
    acc = acc ^ 554584u64;
    acc = acc ^ 522064u64;
    acc = acc.wrapping_mul(38037u64);
    acc = acc.wrapping_mul(51821u64);
    acc = acc.wrapping_add(970031u64);
    acc = acc ^ 207544u64;
    acc = acc ^ 970078u64;
    acc = acc ^ 262266u64;
    acc = acc ^ 703411u64;
    acc = acc.wrapping_mul(25061u64);
    acc = acc.wrapping_mul(40057u64);
    acc = acc ^ 613493u64;
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(185288u64);
    acc = acc.wrapping_mul(22153u64);
    acc = acc.wrapping_add(208781u64);
    acc = acc.wrapping_mul(19263u64);
    acc = acc.wrapping_add(827999u64);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_mul(3659u64);
    acc = acc ^ 188129u64;
    acc = acc.wrapping_mul(30991u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 520981u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(11019u64);
    acc = acc.wrapping_add(760692u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_mul(23199u64);
    acc = acc.wrapping_add(522436u64);
    acc = acc.wrapping_add(362192u64);
    acc = acc.wrapping_add(392563u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(419072u64);
    acc = acc.wrapping_add(620002u64);
    acc = acc.wrapping_mul(17603u64);
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.wrapping_add(141743u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(841525u64);
    acc = acc.wrapping_add(128844u64);
    acc = acc ^ 33882u64;
    acc = acc.wrapping_mul(12955u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(961307u64);
    acc = acc.wrapping_add(958788u64);
    acc = acc ^ 623673u64;
    acc = acc.wrapping_mul(5235u64);
    acc = acc ^ 935409u64;
    acc = acc.rotate_left(31);
    acc = acc ^ 733100u64;
    acc = acc.wrapping_mul(58817u64);
    acc = acc.wrapping_add(905307u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(18719u64);
    acc = acc.wrapping_mul(64523u64);
    acc = acc ^ 102395u64;
    acc = acc.wrapping_add(867828u64);
    acc = acc.rotate_left(1);
    acc = acc.wrapping_mul(45417u64);
    acc = acc.wrapping_add(688851u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(879602u64);
    acc = acc ^ 449726u64;
    acc = acc ^ 757223u64;
    acc = acc.wrapping_add(968010u64);
    acc = acc ^ 851290u64;
    acc = acc.wrapping_add(753289u64);
    acc = acc.wrapping_mul(45775u64);
    acc = acc.wrapping_add(514333u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(42327u64);
    acc = acc ^ 338170u64;
    acc = acc.wrapping_add(304742u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc ^ 272004u64;
    acc = acc ^ 443457u64;
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(11151u64);
    acc = acc ^ 895846u64;
    acc = acc.wrapping_add(673413u64);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(44669u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 837264u64;
    acc = acc ^ 933548u64;
    acc = acc ^ 981394u64;
    acc = acc.wrapping_mul(9705u64);
    acc = acc.wrapping_mul(54749u64);
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 535u64, b: -833i32, c: false } } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(472u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V0(613u64) } }


static SVALS_20: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_20 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((1918523146u32) | (674319223u32) | (1918523146u32) == (1918523146u32) | (674319223u32));
        debug_assert!((845976173u64).wrapping_mul(1) == 845976173u64);
        debug_assert!((3138752526u32).rotate_left(27).rotate_right(27) == 3138752526u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1833928167u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(177687833u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(699861638u64)));
        {

    let mut u = SU_20 { r: &SVALS_20[0] };
    u.p = &SVALS_20[0] as *const i64;
    let read_20_0 = unsafe { *u.p };
    print!("case_58:"); println!("{}", read_20_0);
    u.r = &SVALS_20[7];
    let read_20_1 = unsafe { *u.r };
    print!("case_58:"); println!("{}", read_20_1);
    u.p = &SVALS_20[4] as *const i64;
    let read_20_2 = unsafe { *u.r };
    print!("case_58:"); println!("{}", read_20_2);

        }
        debug_assert!((4285462627u64).wrapping_mul(1) == 4285462627u64);
        debug_assert!((905367304u32).rotate_left(5).rotate_right(5) == 905367304u32);
        assert!((821202153u64) ^ (821202153u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(1203824031u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(3646055410u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(286471429u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_59 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_sub(5328u64).rotate_left(12) ^ 5405u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(7479u64).rotate_left(25) ^ 3992u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(1716u64).rotate_left(5) ^ 7909u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(7297u64).rotate_left(23) ^ 1758u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(920819u64);
    acc = acc ^ 445478u64;
    acc = acc.rotate_left(25);
    acc = acc.rotate_left(1);
    acc = acc.rotate_left(8);
    acc = acc.wrapping_add(547826u64);
    acc = acc.wrapping_mul(15765u64);
    acc = acc.wrapping_mul(27361u64);
    acc = acc.wrapping_mul(27451u64);
    acc = acc.wrapping_mul(22315u64);
    acc = acc.wrapping_add(841762u64);
    acc = acc.wrapping_mul(45355u64);
    acc = acc.wrapping_add(735523u64);
    acc = acc.wrapping_add(196490u64);
    acc = acc.wrapping_add(864428u64);
    acc = acc ^ 989029u64;
    acc = acc ^ 29310u64;
    acc = acc.wrapping_mul(39477u64);
    acc = acc ^ 797957u64;
    acc = acc.wrapping_mul(25147u64);
    acc = acc ^ 176470u64;
    acc = acc.wrapping_mul(25085u64);
    acc = acc.wrapping_add(34007u64);
    acc = acc ^ 959640u64;
    acc = acc ^ 620310u64;
    acc = acc.wrapping_add(543534u64);
    acc = acc ^ 90562u64;
    acc = acc.rotate_left(14);
    acc = acc ^ 205695u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(57869u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 718210u64;
    acc = acc.wrapping_mul(5217u64);
    acc = acc.wrapping_add(532697u64);
    acc = acc.wrapping_add(409801u64);
    acc = acc.wrapping_mul(35097u64);
    acc = acc ^ 250903u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(639829u64);
    acc = acc ^ 84377u64;
    acc = acc.wrapping_add(610197u64);
    acc = acc.wrapping_add(716138u64);
    acc = acc.rotate_left(2);
    acc = acc ^ 139663u64;
    acc = acc ^ 854292u64;
    acc = acc.wrapping_mul(58671u64);
    acc = acc ^ 478357u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(918317u64);
    acc = acc.wrapping_add(852510u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(438218u64);
    acc = acc.wrapping_add(322304u64);
    acc = acc.wrapping_add(893246u64);
    acc = acc.wrapping_mul(45199u64);
    acc = acc.wrapping_add(161912u64);
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(453491u64);
    acc = acc ^ 833127u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(6453u64);
    acc = acc ^ 908903u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(581626u64);
    acc = acc ^ 615692u64;
    acc = acc ^ 980896u64;
    acc = acc ^ 450702u64;
    acc = acc.wrapping_add(129366u64);
    acc = acc.wrapping_add(222745u64);
    acc = acc.wrapping_mul(46493u64);
    acc = acc ^ 431427u64;
    acc = acc.rotate_left(19);
    acc = acc.rotate_left(26);
    acc = acc ^ 665830u64;
    acc = acc ^ 682218u64;
    acc = acc ^ 969729u64;
    acc = acc.wrapping_add(448763u64);
    acc = acc ^ 357730u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(150221u64);
    acc = acc.wrapping_mul(38923u64);
    acc = acc.rotate_left(27);
    acc = acc ^ 846530u64;
    acc = acc.wrapping_mul(54007u64);
    acc = acc.wrapping_mul(40803u64);
    acc = acc ^ 871694u64;
    acc = acc ^ 787794u64;
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc ^ 25947u64;
    acc = acc ^ 866389u64;
    acc = acc ^ 602072u64;
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(659615u64);
    acc = acc ^ 754149u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 329158u64;
    acc = acc.wrapping_add(313093u64);
    acc = acc.wrapping_add(348365u64);
    acc = acc ^ 481125u64;
    acc = acc ^ 37394u64;
    acc = acc.wrapping_add(825443u64);
    acc = acc.wrapping_add(184492u64);
    acc = acc ^ 342063u64;
    acc = acc ^ 829586u64;
    acc = acc ^ 461679u64;
    acc = acc ^ 661530u64;
    acc = acc.wrapping_add(865865u64);
    acc = acc ^ 778091u64;
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(53095u64);
    acc = acc.wrapping_add(972167u64);
    acc = acc.wrapping_mul(44101u64);
    acc = acc ^ 508920u64;
    acc = acc ^ 107163u64;
    acc = acc.wrapping_add(580292u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_mul(33783u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc ^ 939911u64;
    acc = acc.wrapping_add(202528u64);
    acc = acc.wrapping_mul(29161u64);
    acc = acc.wrapping_add(39887u64);
    acc = acc ^ 280257u64;
    acc = acc.wrapping_add(311497u64);
    acc = acc.wrapping_mul(52723u64);
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_add(243492u64);
    acc = acc.rotate_left(26);
    acc = acc.wrapping_mul(13379u64);
    acc = acc ^ 550049u64;
    acc = acc.wrapping_add(332669u64);
    acc = acc.wrapping_mul(8441u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(941u64);
    acc = acc.wrapping_mul(41895u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_mul(43115u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(411978u64);
    acc = acc ^ 299047u64;
    acc = acc ^ 936617u64;
    acc = acc.wrapping_add(882323u64);
    acc = acc.rotate_left(7);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(783681u64);
    acc = acc ^ 926173u64;
    acc = acc.wrapping_add(356595u64);
    acc = acc.wrapping_add(810431u64);
    acc = acc.wrapping_mul(10077u64);
    acc = acc.wrapping_mul(11375u64);
    acc = acc.wrapping_mul(64107u64);
    acc = acc.wrapping_mul(14203u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(81593u64);
    acc = acc.wrapping_mul(28047u64);
    acc = acc.wrapping_add(522853u64);
    acc = acc.wrapping_mul(17071u64);
    acc = acc.wrapping_mul(39447u64);
    acc = acc.wrapping_add(83364u64);
    acc = acc ^ 794850u64;
    acc = acc ^ 809779u64;
    acc = acc.wrapping_add(873214u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(42109u64);
    acc = acc ^ 183240u64;
    acc = acc.wrapping_mul(7419u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 129075u64;
    acc = acc.rotate_left(3);
    acc = acc.wrapping_add(508719u64);
    acc = acc.wrapping_mul(15453u64);
    acc = acc.wrapping_add(404951u64);
    acc = acc.wrapping_mul(64565u64);
    acc = acc.wrapping_add(857238u64);
    acc = acc.wrapping_add(403226u64);
    acc = acc.rotate_left(21);
    acc = acc ^ 828301u64;
    acc = acc.wrapping_mul(25067u64);
    acc = acc ^ 406266u64;
    acc = acc.wrapping_add(991306u64);
    acc = acc ^ 770978u64;
    acc = acc.rotate_left(1);
    acc = acc ^ 777923u64;
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(120899u64);
    acc = acc.rotate_left(29);
    acc = acc ^ 699717u64;
    acc = acc ^ 417781u64;
    acc = acc ^ 30976u64;
    acc = acc.rotate_left(26);
    acc = acc ^ 878458u64;
    acc = acc ^ 599447u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(839178u64);
    acc = acc ^ 225121u64;
    acc = acc.rotate_left(6);
    acc = acc.rotate_left(13);
    acc = acc.wrapping_mul(23067u64);
    acc = acc.wrapping_add(717044u64);
    acc = acc.wrapping_add(795962u64);
    acc = acc.wrapping_add(160453u64);
    acc = acc ^ 138136u64;
    acc = acc.wrapping_mul(40029u64);
    acc = acc ^ 170229u64;
    acc = acc ^ 809772u64;
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(31677u64);
    acc = acc.wrapping_mul(28925u64);
    acc = acc.wrapping_mul(12301u64);
    acc = acc ^ 988005u64;
    acc = acc ^ 966521u64;
    acc = acc.wrapping_mul(46221u64);
    acc = acc.wrapping_mul(10177u64);
    acc = acc.wrapping_add(827153u64);
    acc = acc.wrapping_mul(29877u64);
    acc = acc.rotate_left(22);
    acc = acc ^ 129999u64;
    acc = acc.wrapping_mul(40721u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_add(354040u64);
    acc = acc.wrapping_mul(12703u64);
    acc = acc.wrapping_add(249950u64);
    acc = acc ^ 113586u64;
    acc = acc.wrapping_add(295377u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc ^ 279547u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(632378u64);
    acc
}

enum NoiseType0 { V0(u64), V1(u64), V2(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V2(511u64) } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 934u64, b: 521i32, c: false } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 979u64, b: 455i32, c: true } } }


static SVALS_21: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_21 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((2439878794u64).wrapping_add(2439878794u64).wrapping_sub(2439878794u64) == 2439878794u64);
        assert!((368519001u64).wrapping_add(368519001u64).wrapping_sub(368519001u64) == 368519001u64);
        debug_assert!((3052787911u64).wrapping_mul(1) == 3052787911u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(1827694860u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(3691846533u64)));
        let _ = std::hint::black_box(noise_fn_1(std::hint::black_box(795136410u64)));
        {

    let mut u = SU_21 { p: &SVALS_21[0] as *const i64 };
    u.p = &SVALS_21[0] as *const i64;
    let read_21_0 = unsafe { *u.p };
    print!("case_59:"); println!("{}", read_21_0);
    u.r = &SVALS_21[7];
    let read_21_1 = unsafe { *u.r };
    print!("case_59:"); println!("{}", read_21_1);
    u.p = &SVALS_21[4] as *const i64;
    let read_21_2 = unsafe { *u.r };
    print!("case_59:"); println!("{}", read_21_2);

        }
        debug_assert!((3437817020u32) | (1329785278u32) | (3437817020u32) == (3437817020u32) | (1329785278u32));
        assert!(!(639703957u64) == (639703957u64) ^ u64::MAX);
        debug_assert!((229471131u32).rotate_left(30).rotate_right(30) == 229471131u32);
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(504948876u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2463249106u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3614348009u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_60 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_mul(2311u64).rotate_left(8) ^ 8222u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(3704u64).rotate_left(12) ^ 414u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_sub(4374u64).rotate_left(8) ^ 6145u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_sub(3613u64).rotate_left(25) ^ 427u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 481367u64;
    acc = acc ^ 22913u64;
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(538580u64);
    acc = acc ^ 471115u64;
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc ^ 944920u64;
    acc = acc.rotate_left(29);
    acc = acc.rotate_left(5);
    acc = acc ^ 724010u64;
    acc = acc.wrapping_add(81085u64);
    acc = acc.wrapping_add(312983u64);
    acc = acc.wrapping_mul(7937u64);
    acc = acc ^ 84028u64;
    acc = acc.rotate_left(16);
    acc = acc.wrapping_mul(13217u64);
    acc = acc.wrapping_add(946513u64);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(12677u64);
    acc = acc.wrapping_mul(20051u64);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_mul(22795u64);
    acc = acc.wrapping_mul(62107u64);
    acc = acc.wrapping_mul(29813u64);
    acc = acc ^ 52255u64;
    acc = acc.wrapping_mul(44833u64);
    acc = acc.wrapping_mul(57103u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 99167u64;
    acc = acc ^ 120279u64;
    acc = acc.rotate_left(4);
    acc = acc ^ 911153u64;
    acc = acc.wrapping_add(874575u64);
    acc = acc.wrapping_add(744335u64);
    acc = acc.wrapping_mul(15905u64);
    acc = acc.wrapping_add(570392u64);
    acc = acc.wrapping_mul(7877u64);
    acc = acc ^ 215850u64;
    acc = acc.wrapping_add(30663u64);
    acc = acc ^ 720820u64;
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(48715u64);
    acc = acc.wrapping_add(638047u64);
    acc = acc.wrapping_add(662196u64);
    acc = acc ^ 668468u64;
    acc = acc.wrapping_add(780858u64);
    acc = acc ^ 773358u64;
    acc = acc.wrapping_add(996690u64);
    acc = acc.wrapping_add(179560u64);
    acc = acc.rotate_left(13);
    acc = acc ^ 530411u64;
    acc = acc.wrapping_mul(21175u64);
    acc = acc.rotate_left(3);
    acc = acc.wrapping_mul(55451u64);
    acc = acc.wrapping_mul(9011u64);
    acc = acc ^ 199359u64;
    acc = acc ^ 329857u64;
    acc = acc.rotate_left(7);
    acc = acc.wrapping_add(221186u64);
    acc = acc.wrapping_mul(51617u64);
    acc = acc ^ 402721u64;
    acc = acc.rotate_left(4);
    acc = acc.wrapping_add(136090u64);
    acc = acc.rotate_left(18);
    acc = acc.wrapping_add(275891u64);
    acc = acc.wrapping_add(729385u64);
    acc = acc.wrapping_add(440837u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_mul(38557u64);
    acc = acc.wrapping_mul(26323u64);
    acc = acc ^ 169191u64;
    acc = acc.wrapping_add(943143u64);
    acc = acc.wrapping_add(120757u64);
    acc = acc.rotate_left(26);
    acc = acc.rotate_left(2);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_mul(56521u64);
    acc = acc ^ 455351u64;
    acc = acc.wrapping_mul(60793u64);
    acc = acc ^ 301200u64;
    acc = acc ^ 353632u64;
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(199269u64);
    acc = acc.wrapping_add(615495u64);
    acc = acc.wrapping_mul(40691u64);
    acc = acc.wrapping_mul(23771u64);
    acc = acc.rotate_left(24);
    acc = acc.wrapping_add(490117u64);
    acc = acc.wrapping_add(543318u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_add(452581u64);
    acc = acc ^ 798167u64;
    acc = acc ^ 108574u64;
    acc = acc.wrapping_mul(21587u64);
    acc = acc ^ 909504u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(3075u64);
    acc = acc.wrapping_mul(28949u64);
    acc = acc.wrapping_add(359466u64);
    acc = acc.wrapping_mul(20233u64);
    acc = acc.wrapping_mul(54639u64);
    acc = acc.wrapping_add(489159u64);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(61159u64);
    acc = acc.wrapping_add(506327u64);
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.wrapping_mul(48859u64);
    acc = acc ^ 340931u64;
    acc = acc.wrapping_add(799917u64);
    acc = acc.wrapping_mul(35525u64);
    acc = acc ^ 968488u64;
    acc = acc ^ 64815u64;
    acc = acc ^ 782006u64;
    acc = acc.wrapping_add(734165u64);
    acc = acc.wrapping_mul(57169u64);
    acc = acc.wrapping_mul(34805u64);
    acc = acc.wrapping_mul(49917u64);
    acc = acc.wrapping_mul(53121u64);
    acc = acc.wrapping_add(318043u64);
    acc = acc.wrapping_mul(43215u64);
    acc = acc.wrapping_add(851171u64);
    acc = acc.rotate_left(16);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_add(346146u64);
    acc = acc.wrapping_mul(7089u64);
    acc = acc.wrapping_add(522329u64);
    acc = acc.wrapping_add(619727u64);
    acc = acc.wrapping_mul(47217u64);
    acc = acc.wrapping_add(576203u64);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_mul(37405u64);
    acc = acc ^ 330337u64;
    acc = acc.wrapping_add(624882u64);
    acc = acc.wrapping_add(309067u64);
    acc = acc ^ 106479u64;
    acc = acc ^ 555793u64;
    acc = acc.wrapping_mul(22933u64);
    acc = acc.wrapping_add(504298u64);
    acc = acc.wrapping_add(323920u64);
    acc = acc.wrapping_add(199920u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(2909u64);
    acc = acc.wrapping_add(539715u64);
    acc = acc.wrapping_add(346792u64);
    acc = acc ^ 784039u64;
    acc = acc.rotate_left(29);
    acc = acc.wrapping_add(408885u64);
    acc = acc.wrapping_mul(30379u64);
    acc = acc ^ 860045u64;
    acc = acc.wrapping_mul(9041u64);
    acc = acc ^ 724023u64;
    acc = acc ^ 343617u64;
    acc = acc.wrapping_mul(60645u64);
    acc = acc.wrapping_add(908113u64);
    acc = acc.rotate_left(30);
    acc = acc ^ 538374u64;
    acc = acc.wrapping_mul(30201u64);
    acc = acc.rotate_left(20);
    acc = acc.rotate_left(27);
    acc = acc.wrapping_add(536840u64);
    acc = acc.wrapping_add(83352u64);
    acc = acc.wrapping_add(905210u64);
    acc = acc ^ 98990u64;
    acc = acc ^ 927888u64;
    acc = acc.wrapping_add(713132u64);
    acc = acc.rotate_left(14);
    acc = acc.wrapping_mul(45189u64);
    acc = acc.wrapping_mul(643u64);
    acc = acc ^ 678822u64;
    acc = acc.wrapping_mul(55369u64);
    acc = acc ^ 859695u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(876248u64);
    acc = acc.rotate_left(8);
    acc = acc.rotate_left(15);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_add(781303u64);
    acc = acc.wrapping_mul(30611u64);
    acc = acc ^ 778224u64;
    acc = acc.wrapping_add(808880u64);
    acc = acc.wrapping_mul(36785u64);
    acc = acc.wrapping_mul(2839u64);
    acc = acc.wrapping_mul(13139u64);
    acc = acc.wrapping_mul(23107u64);
    acc = acc ^ 108256u64;
    acc = acc ^ 538521u64;
    acc = acc.wrapping_add(833486u64);
    acc = acc.rotate_left(13);
    acc = acc.rotate_left(20);
    acc = acc ^ 148380u64;
    acc = acc.wrapping_mul(5379u64);
    acc = acc.wrapping_mul(4057u64);
    acc = acc.wrapping_add(633153u64);
    acc = acc ^ 921825u64;
    acc
}

enum NoiseType0 { V0(u64), V1(u64) }
impl NoiseType0 { fn sample() -> Self { NoiseType0::V0(935u64) } }

enum NoiseType1 { V0(u64), V1(u64), V2(u64), V3(u64) }
impl NoiseType1 { fn sample() -> Self { NoiseType1::V0(790u64) } }

enum NoiseType2 { V0(u64), V1(u64), V2(u64) }
impl NoiseType2 { fn sample() -> Self { NoiseType2::V1(266u64) } }


static SVALS_22: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_22 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        assert!((194262774u64) ^ (194262774u64) == 0u64);
        assert!((1072215186u64) ^ (1072215186u64) == 0u64);
        debug_assert!((2509358979u32).rotate_left(17).rotate_right(17) == 2509358979u32);
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(1097995380u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(2425166583u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1619270071u64)));
        {

    let mut u = SU_22 { r: &SVALS_22[0] };
    u.r = &SVALS_22[0];
    let read_22_0 = unsafe { *u.r };
    print!("case_60:"); println!("{}", read_22_0);
    u.r = &SVALS_22[7];
    let read_22_1 = unsafe { *u.p };
    print!("case_60:"); println!("{}", read_22_1);
    u.p = &SVALS_22[0] as *const i64;
    let read_22_2 = unsafe { *u.p };
    print!("case_60:"); println!("{}", read_22_2);
    u.p = &SVALS_22[7] as *const i64;
    let read_22_3 = unsafe { *u.r };
    print!("case_60:"); println!("{}", read_22_3);

        }
        assert!((473665760u64) ^ (473665760u64) == 0u64);
        debug_assert!((1067324873u64).wrapping_mul(1) == 1067324873u64);
        assert!((3676549425u64) ^ (3676549425u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(3050642335u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(4086725455u64)));
        let _ = std::hint::black_box(noise_fn_2(std::hint::black_box(1177473976u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

mod case_61 {
    #![allow(dead_code)]

fn noise_fn_0(x: u64) -> u64 { x.wrapping_add(8180u64).rotate_left(15) ^ 944u64 }

fn noise_fn_1(x: u64) -> u64 { x.wrapping_add(7837u64).rotate_left(13) ^ 7111u64 }

fn noise_fn_2(x: u64) -> u64 { x.wrapping_mul(9207u64).rotate_left(29) ^ 4616u64 }

fn noise_fn_3(x: u64) -> u64 { x.wrapping_add(5719u64).rotate_left(11) ^ 6117u64 }

fn noise_fn_4(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc.rotate_left(4);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(483647u64);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(514254u64);
    acc = acc.rotate_left(8);
    acc = acc ^ 173317u64;
    acc = acc.wrapping_mul(40859u64);
    acc = acc.wrapping_add(945755u64);
    acc = acc.wrapping_add(993735u64);
    acc = acc.rotate_left(12);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(8131u64);
    acc = acc.wrapping_mul(62359u64);
    acc = acc.rotate_left(9);
    acc = acc.wrapping_add(352402u64);
    acc = acc.wrapping_add(765190u64);
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_mul(39281u64);
    acc = acc.rotate_left(20);
    acc = acc ^ 494786u64;
    acc = acc.wrapping_mul(8005u64);
    acc = acc.wrapping_add(227067u64);
    acc = acc.wrapping_add(15991u64);
    acc = acc.wrapping_mul(57429u64);
    acc = acc.rotate_left(31);
    acc = acc ^ 165710u64;
    acc = acc.wrapping_add(709996u64);
    acc = acc ^ 123909u64;
    acc = acc ^ 431039u64;
    acc = acc.wrapping_add(836953u64);
    acc = acc.rotate_left(11);
    acc = acc.wrapping_add(887413u64);
    acc = acc ^ 607742u64;
    acc = acc.rotate_left(1);
    acc = acc.wrapping_add(952492u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(676197u64);
    acc = acc.wrapping_mul(42103u64);
    acc = acc.wrapping_mul(47757u64);
    acc = acc.wrapping_mul(33983u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(11855u64);
    acc = acc.wrapping_add(931973u64);
    acc = acc.wrapping_add(185126u64);
    acc = acc.wrapping_mul(4355u64);
    acc = acc.rotate_left(23);
    acc = acc.wrapping_mul(9339u64);
    acc = acc ^ 141514u64;
    acc = acc.wrapping_add(842248u64);
    acc = acc.rotate_left(20);
    acc = acc.wrapping_add(263910u64);
    acc = acc.rotate_left(3);
    acc = acc ^ 733751u64;
    acc = acc.rotate_left(17);
    acc = acc.wrapping_mul(20039u64);
    acc = acc.wrapping_add(622177u64);
    acc = acc.wrapping_mul(22393u64);
    acc = acc.wrapping_mul(11945u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(59933u64);
    acc = acc.rotate_left(11);
    acc = acc ^ 987930u64;
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(55379u64);
    acc = acc.wrapping_add(39749u64);
    acc = acc.rotate_left(15);
    acc = acc.wrapping_add(442300u64);
    acc = acc.wrapping_add(855642u64);
    acc = acc.wrapping_add(574750u64);
    acc = acc.wrapping_add(894289u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(7971u64);
    acc = acc.wrapping_mul(1783u64);
    acc = acc.rotate_left(9);
    acc = acc ^ 255860u64;
    acc = acc ^ 386942u64;
    acc = acc ^ 390807u64;
    acc = acc.wrapping_add(190578u64);
    acc = acc ^ 1258u64;
    acc
}

fn noise_fn_5(x: u64) -> u64 {
    let mut acc: u64 = x;
    acc = acc ^ 464542u64;
    acc = acc.wrapping_mul(39365u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_add(922952u64);
    acc = acc ^ 557967u64;
    acc = acc.wrapping_mul(31313u64);
    acc = acc ^ 951423u64;
    acc = acc.wrapping_mul(32503u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(35102u64);
    acc = acc.rotate_left(19);
    acc = acc.wrapping_mul(50845u64);
    acc = acc.wrapping_mul(8987u64);
    acc = acc.wrapping_mul(51507u64);
    acc = acc.rotate_left(16);
    acc = acc ^ 822580u64;
    acc = acc.rotate_left(30);
    acc = acc.rotate_left(6);
    acc = acc.wrapping_add(360954u64);
    acc = acc ^ 344073u64;
    acc = acc.rotate_left(27);
    acc = acc.rotate_left(3);
    acc = acc.rotate_left(10);
    acc = acc.wrapping_add(744667u64);
    acc = acc.wrapping_add(238478u64);
    acc = acc.wrapping_add(228074u64);
    acc = acc ^ 632213u64;
    acc = acc.wrapping_mul(4465u64);
    acc = acc.rotate_left(21);
    acc = acc.rotate_left(28);
    acc = acc.wrapping_add(456991u64);
    acc = acc ^ 114623u64;
    acc = acc ^ 790876u64;
    acc = acc.wrapping_mul(32473u64);
    acc = acc.wrapping_add(90606u64);
    acc = acc ^ 823763u64;
    acc = acc.wrapping_mul(24695u64);
    acc = acc.rotate_left(22);
    acc = acc.wrapping_mul(53253u64);
    acc = acc.rotate_left(5);
    acc = acc.wrapping_add(823520u64);
    acc = acc.wrapping_add(714196u64);
    acc = acc.wrapping_mul(34375u64);
    acc = acc ^ 78329u64;
    acc = acc.wrapping_add(624440u64);
    acc = acc.wrapping_add(237971u64);
    acc = acc ^ 532883u64;
    acc = acc.rotate_left(30);
    acc = acc.wrapping_add(368623u64);
    acc = acc.wrapping_mul(41227u64);
    acc = acc.wrapping_add(418912u64);
    acc = acc ^ 218653u64;
    acc = acc.wrapping_mul(29915u64);
    acc = acc.rotate_left(10);
    acc = acc.rotate_left(17);
    acc = acc.rotate_left(24);
    acc = acc.rotate_left(31);
    acc = acc.wrapping_mul(25655u64);
    acc = acc.rotate_left(14);
    acc = acc ^ 710501u64;
    acc = acc.rotate_left(28);
    acc = acc.wrapping_mul(24765u64);
    acc = acc.wrapping_mul(13953u64);
    acc = acc.rotate_left(18);
    acc = acc.rotate_left(25);
    acc = acc.wrapping_mul(52303u64);
    acc = acc.wrapping_add(139804u64);
    acc = acc ^ 304256u64;
    acc = acc.wrapping_add(911798u64);
    acc = acc ^ 78723u64;
    acc = acc.rotate_left(5);
    acc = acc.rotate_left(12);
    acc = acc.wrapping_mul(11987u64);
    acc = acc.wrapping_add(984740u64);
    acc = acc.wrapping_add(114948u64);
    acc = acc.wrapping_mul(14099u64);
    acc = acc.rotate_left(16);
    acc = acc.wrapping_add(432001u64);
    acc = acc.rotate_left(30);
    acc = acc.wrapping_mul(12403u64);
    acc = acc.wrapping_add(1969u64);
    acc = acc ^ 825106u64;
    acc = acc.wrapping_add(778594u64);
    acc = acc.wrapping_mul(44215u64);
    acc = acc.wrapping_mul(51515u64);
    acc = acc.wrapping_add(404359u64);
    acc = acc ^ 763625u64;
    acc = acc ^ 882812u64;
    acc
}

struct NoiseType0 { a: u64, b: i32, c: bool }
impl NoiseType0 { fn sample() -> Self { NoiseType0 { a: 838u64, b: 541i32, c: false } } }

struct NoiseType1 { a: u64, b: i32, c: bool }
impl NoiseType1 { fn sample() -> Self { NoiseType1 { a: 346u64, b: 587i32, c: true } } }

struct NoiseType2 { a: u64, b: i32, c: bool }
impl NoiseType2 { fn sample() -> Self { NoiseType2 { a: 535u64, b: 575i32, c: false } } }


static SVALS_23: [i64; 8] = [1000003i64, 2000006i64, 3000009i64, 4000012i64, 5000015i64, 6000018i64, 7000021i64, 8000024i64];

union SU_23 {
    r: &'static i64,
    p: *const i64,
}

    pub fn run() {
        debug_assert!((7402076u32).rotate_left(4).rotate_right(4) == 7402076u32);
        debug_assert!((3278283220u32) | (793625106u32) | (3278283220u32) == (3278283220u32) | (793625106u32));
        assert!((3673380312u64) ^ (3673380312u64) == 0u64);
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(3620918483u64)));
        let _ = std::hint::black_box(noise_fn_4(std::hint::black_box(980222459u64)));
        let _ = std::hint::black_box(noise_fn_5(std::hint::black_box(2610228227u64)));
        {

    let mut u = SU_23 { p: &SVALS_23[0] as *const i64 };
    u.r = &SVALS_23[0];
    let read_23_0 = unsafe { *u.r };
    print!("case_61:"); println!("{}", read_23_0);
    u.r = &SVALS_23[7];
    let read_23_1 = unsafe { *u.p };
    print!("case_61:"); println!("{}", read_23_1);
    u.p = &SVALS_23[0] as *const i64;
    let read_23_2 = unsafe { *u.p };
    print!("case_61:"); println!("{}", read_23_2);
    u.p = &SVALS_23[7] as *const i64;
    let read_23_3 = unsafe { *u.r };
    print!("case_61:"); println!("{}", read_23_3);

        }
        assert!((1346480582u64).wrapping_add(1346480582u64).wrapping_sub(1346480582u64) == 1346480582u64);
        assert!(!(2563295003u64) == (2563295003u64) ^ u64::MAX);
        assert!((163893241u64).wrapping_add(163893241u64).wrapping_sub(163893241u64) == 163893241u64);
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(3074582860u64)));
        let _ = std::hint::black_box(noise_fn_3(std::hint::black_box(2631925535u64)));
        let _ = std::hint::black_box(noise_fn_0(std::hint::black_box(2613960942u64)));
        let _ = std::hint::black_box(NoiseType0::sample());
        let _ = std::hint::black_box(NoiseType1::sample());
        let _ = std::hint::black_box(NoiseType2::sample());
    }
}

fn main() {
    case_0::run();
    case_1::run();
    case_2::run();
    case_3::run();
    case_4::run();
    case_5::run();
    case_6::run();
    case_7::run();
    case_8::run();
    case_9::run();
    case_10::run();
    case_11::run();
    case_12::run();
    case_13::run();
    case_14::run();
    case_15::run();
    case_16::run();
    case_17::run();
    case_18::run();
    case_19::run();
    case_20::run();
    case_21::run();
    case_22::run();
    case_23::run();
    case_24::run();
    case_25::run();
    case_26::run();
    case_27::run();
    case_28::run();
    case_29::run();
    case_30::run();
    case_31::run();
    case_32::run();
    case_33::run();
    case_34::run();
    case_35::run();
    case_36::run();
    case_37::run();
    case_38::run();
    case_39::run();
    case_40::run();
    case_41::run();
    case_42::run();
    case_43::run();
    case_44::run();
    case_45::run();
    case_46::run();
    case_47::run();
    case_48::run();
    case_49::run();
    case_50::run();
    case_51::run();
    case_52::run();
    case_53::run();
    case_54::run();
    case_55::run();
    case_56::run();
    case_57::run();
    case_58::run();
    case_59::run();
    case_60::run();
    case_61::run();
}
