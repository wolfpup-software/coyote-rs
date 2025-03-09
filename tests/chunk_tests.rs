use coyote::{tmpl, ClientHtml, Html};

#[test]
fn test_empty_element() {
    let template = tmpl(
        "
		<p>
		</p>
		",
        [],
    );
    let expected = "<p></p>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_fragment() {
    let template = tmpl(
        "
		<>
		</>
		",
        [],
    );
    let expected = "";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_text_element() {
    let template = tmpl(
        "

            Beasts tread softly underfoot.
            
		",
        [],
    );
    let expected = "Beasts tread softly underfoot.";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_element_with_text() {
    let template = tmpl(
        "
		<p>hello!</p>
		",
        [],
    );
    let expected = "<p>\n\thello!\n</p>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_inline_element_with_text() {
    let template = tmpl(
        "
		<b>   hello!    </b>
		",
        [],
    );
    let expected = "<b>hello!</b>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_achor_element_with_text() {
    let template = tmpl(
        "
		<a>
            hello!    </a>
		",
        [],
    );
    let expected = "<a>hello!</a>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_void_element() {
    let template = tmpl(
        "
		<input />
		",
        [],
    );
    let expected = "<input>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn test_non_void_element() {
    let template = tmpl(
        "
		<p />
		",
        [],
    );
    let expected = "<p></p>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

// #[test]
// fn test_comment_element() {
//     // edge case
//     // will include all the spaces
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

#[test]
fn test_alt_element() {
    // edge case
    // will include all the spaces
    let template = tmpl(
        "
		<script>
			{}
		</script>
		",
        [],
    );
    let expected = "<script>\n\t{}\n</script>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
