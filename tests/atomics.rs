// use coyote::{tmpl, Html};

// #[test]
// fn text_element() {
//     let template = tmpl(
//         "

//             Beasts tread
//             softly underfoot.

// 		",
//         [],
//     );
//     let expected = "Beasts tread\nsoftly underfoot.";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn empty_element() {
//     let template = tmpl(
//         "
// 		<p>
// 		</p>
// 		",
//         [],
//     );
//     let expected = "<p></p>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn fragment() {
//     let template = tmpl(
//         "
// 		<>
// 		</>
// 		",
//         [],
//     );
//     let expected = "";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn element_with_text() {
//     let template = tmpl(
//         "
// 		<p>hello!</p>
// 		",
//         [],
//     );
//     let expected = "<p>\n\thello!\n</p>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn inline_element_with_text() {
//     let template = tmpl(
//         "
// 		<b>   hello!
//             </b>
// 		",
//         [],
//     );
//     let expected = "<b>hello!</b>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn achor_element_with_text() {
//     let template = tmpl(
//         "
// 		<a>
//             hello!    </a>
// 		",
//         [],
//     );
//     let expected = "<a>\n\thello!\n</a>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn void_element() {
//     let template = tmpl(
//         "
// 		<input />
// 		",
//         [],
//     );
//     let expected = "<input>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn non_void_element() {
//     let template = tmpl(
//         "
// 		<p />
// 		",
//         [],
//     );
//     let expected = "<p></p>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn comment_element() {
//     let template = tmpl(
//         "
// 		<!--
//             Hello!
//         -->
// 		",
//         [],
//     );
//     let expected = "<!--\n\tHello!\n-->";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn alt_text_element() {
//     let template = tmpl(
//         "<style>#woof .bark {
//     color: doggo;
// }</style>",
//         [],
//     );
//     let expected = "<style>\n\t#woof .bark {\n\t    color: doggo;\n\t}\n</style>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn alt_element_no_descendants() {
//     let template = tmpl(
//         "
// 		<script>
// 			{}
// 		</script>
// 		",
//         [],
//     );
//     let expected = "<script>\n\t{}\n</script>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }

// #[test]
// fn pretty_preserved_text_elements() {
//     let template = tmpl(
//         "
// <pre>
// 	U w U
// 	  woof woof!
// </pre>
// 		",
//         [],
//     );

//     let expected = "<pre>\n\tU w U\n\t  woof woof!\n</pre>";

//     let mut html = Html::new();
//     let results = html.build(&template);

//     assert_eq!(Ok(expected.to_string()), results);
// }
