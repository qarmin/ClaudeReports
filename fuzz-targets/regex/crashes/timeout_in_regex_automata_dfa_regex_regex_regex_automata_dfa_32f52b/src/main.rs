use regex_automata::Anchored;
use regex_automata::dfa::regex::Builder;
use regex_automata::nfa::thompson::backtrack::BoundedBacktracker;
use regex_syntax::ast::Ast;
use regex_syntax::ast::ClassPerl;
use regex_syntax::ast::ClassPerlKind;
use regex_syntax::ast::HexLiteralKind;
use regex_syntax::ast::Position;
use regex_syntax::ast::Repetition;
use regex_syntax::ast::RepetitionKind;
use regex_syntax::ast::RepetitionOp;
use regex_syntax::ast::Span;
use regex_syntax::utf8::Utf8Range;
use regex_syntax::utf8::Utf8Sequence;

fn main() {
    let _ = Repetition { span: Span { start: Position { offset: 255usize, line: 1usize, column: 2147483648usize }, end: Position::new(5usize, 4294967295usize, 13648082024310080854usize) }.with_end(Position { offset: 0usize, line: 2129180065523257685usize, column: 8usize }), op: RepetitionOp { span: Span::splat(Position { offset: 2usize, line: 2usize, column: 8usize }), kind: RepetitionKind::OneOrMore }, greedy: true, ast: Box::new(Ast::class_perl(ClassPerl { span: Span::new(Position { offset: 8393290005580950419usize, line: 4294967295usize, column: 2usize }, Position::new(11usize, 8796093022208usize, 4usize)), kind: ClassPerlKind::Word, negated: false })) };
    let _ = HexLiteralKind::UnicodeLong;
    let utf8_sequence = Utf8Sequence::One(Utf8Range { start: 227u8, end: 8u8 });
    let _ = utf8_sequence.matches(&vec![8u8, 166u8, 161u8, 112u8, 92u8, 44u8, 50u8, 29u8, 60u8, 168u8, 172u8, 45u8, 20u8]);
    let bounded_backtracker = BoundedBacktracker::new("                                                                                                                                                                                                                                                                                                                                                                                                                                             ").unwrap();
    let _ = bounded_backtracker.get_nfa();
    let _ = Anchored::No;
    let regex = Builder::default().build_sparse("# This file represent tests that may be expensive to run on some regex engines.\n# For example, tests that build a full DFA ahead of time and minimize it can\n# take a horrendously long time on regexes that are large (or result in an\n# explosion in the number of states). We group these tests together so that\n# such engines can simply skip these tests.\n\n# See: https://github.com/rust-lang/regex/issues/98\n[[test]]\nname = \"regression-many-repeat-no-stack-overflow\"\nregex = '^.{1,2500}'\nhaystack = \"a\"\nmatches = [[0, 1]]\n\n# This test is meant to blow the bounded backtracker's visited capacity. In\n# order to do that, we need a somewhat sizeable regex. The purpose of this\n# is to make sure there's at least one test that exercises this path in the\n# backtracker. All other tests (at time of writing) are small enough that the\n# backtracker can handle them fine.\n[[test]]\nname = \"<a:b xmlns:a=''/>backtrack-blow-visited-capacity\"\nregex = '\\pL{50}'\nhaystack = \"abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyZZ\"\nmatches = [[0, 50], [50, 100], [100, 150]]\nstack = \"a\"\nmatches = [[0, 1]]\n\n# This test is meant to blow the bounded backtracker's visited capacity. In\n# order to do that, we need a somewhat sizeable regex. The purpose of this\n# is to make sure there's at least one test that exercises this path in the\n# backtracker. All other tests (at time of writing) are small enough that the\n# backtracker can handle them fine.\n[[test]]\nname = \"<a:b xmlns:a=''/>backtrack-blow-visited-capacity\"\nregex = '\\pL{50}'\nhaystack = \"abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyZZ\"\nmatches = [[0, 50], [50, 100], [100, 150]]\nt that exercises this path in the\n# backtracker. All other tests (at time of writing) are small enough that the\n# backtracker can handle them fine.\n[[test]]\nname = \"<a:b xmlns:a=''/>backtrack-blow-visited-capacity\"\nregex = '\\pL{50}'\nhaystack = \"abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyZZ\"\nmatches = [[0, 50], [50, 100], [100, 150]]\nstack = \"a\"\nmatches = [[0, 1]]\n\n# This test is meant to blow the bounded backtracker's visited capacity. In\n# order to do that, we need a somewhat sizeable regex. The purpose of this\n# is to make sure there's at least one test that exercises this path in the\n# backtracker. All other tests (at time of writing) are small enough that the\n# backtracker can handle them fine.\n[[test]]\nname = \"<a:b xmlns:a=''/>backtrack-blow-visited-capacity\"\nregex = '\\pL{50}'\nhaystack = \"abcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyabcdefghijklmnopqrstuvwxyZZ\"\nmatches = [[0, 50], [50, 100], [100, 150]]\n").unwrap();
    let _ = regex.forward();
}
