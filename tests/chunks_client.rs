use coyote::{tmpl, ClientHtml, Html};

#[test]
fn empty_element() {
    let template = tmpl(
        "
		<p>
		</p>
		",
        [],
    );
    let expected = "<p></p>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn fragment() {
    let template = tmpl(
        "
		<>
		</>
		",
        [],
    );
    let expected = "";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_element() {
    let template = tmpl(
        "

            Beasts tread softly underfoot.

		",
        [],
    );
    let expected = "Beasts tread softly underfoot.";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn element_with_text() {
    let template = tmpl(
        "
		<p uwu>hello!</p>
		",
        [],
    );
    let expected = "<p uwu>hello!</p>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_element_with_text() {
    let template = tmpl(
        "
		<b>   hello!
            </b>
		",
        [],
    );
    let expected = "<b>hello!</b>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn achor_element_with_text() {
    let template = tmpl(
        "
		<a>
            hello!    </a>
		",
        [],
    );
    let expected = "<a>hello!</a>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element() {
    let template = tmpl(
        "
		<input />
		",
        [],
    );
    let expected = "<input>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn non_void_element() {
    let template = tmpl(
        "
		<p />
		",
        [],
    );
    let expected = "<p></p>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn comment_element() {
    let template = tmpl(
        "
		<!-- 
            Hello!
        -->
		",
        [],
    );
    let expected = "";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn alt_element() {
    let template = tmpl(
        "
		<script>
			{}
		</script>
		",
        [],
    );
    let expected = "";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
