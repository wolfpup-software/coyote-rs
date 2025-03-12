use coyote::{tmpl, ClientHtml};

#[test]
fn empty_element() {
    let template = tmpl("<html></html>", []);
    let expected = "<html></html>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn unbalanced_empty_element() {
    let template = tmpl("<html>", []);
    let expected = format!(
        "Coyote Err: the following template component is imbalanced:\n{:?}<html>",
        &template
    );

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    if let Err(_) = results {
        return;
    }

    assert_eq!(
        Err("unbalanced template failed to error".to_string()),
        results
    );
}

#[test]
fn mozilla_example() {
    let template = tmpl(
        "
	<h1>   Hello
        <span> World!</span>   </h1>
		",
        [],
    );

    let expected = "<h1>Hello <span>World!</span></h1>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements() {
    let template = tmpl(
        "<input>   <input>
            <input><input> ",
        [],
    );

    let expected = "<input><input><input><input>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_and_inline_elements() {
    let template = tmpl(
        "beasts <span>    tread		</span>     softly <span>    underfoot </span>      .",
        [],
    );

    let expected = "beasts <span>tread</span> softly <span>underfoot</span> .";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn text_and_blocks() {
    let template = tmpl(
        "beasts <p>    tread		</p>     softly <p>    underfoot </p>      .",
        [],
    );

    let expected = "beasts <p>tread</p> softly <p>underfoot</p> .";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_elements_with_attributes() {
    let template = tmpl(
        "
        <!DOCTYPE html><input type=checkbox>   <input woof=\"bark\">
            <input grrr><input> ",
        [],
    );
    let expected = "<!DOCTYPE html><input type=checkbox><input woof=\"bark\"><input grrr><input>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn void_element_with_sibling() {
    let template = tmpl(
        "
            <input><p>hai :3</p>    ",
        [],
    );
    let expected = "<input><p>hai :3</p>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_void_element_with_sibling() {
    let template = tmpl(
        "
        <section>
            <input><p>hai :3</p>
        </section>
    ",
        [],
    );

    let expected = "<section><input><p>hai :3</p></section>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_elements_and_text() {
    let template = tmpl("<a><label><input type=woofer>bark!</label><img></a>", []);
    let expected = "<a><label><input type=woofer>bark!</label><img></a>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document() {
    let template = tmpl(
        "        <!DOCTYPE>
    <html>
    <head>

    </head>
        <body>
            <article>
                You're a <span>boy kisser</span> aren't you?
                Click <a>here</a> and go somewhere else.
            </article>
            <footer/>
        </body>
</html>",
        [],
    );

    let expected =
        "<!DOCTYPE><html><head></head><body><article>You're a <span>boy kisser</span> aren't you? Click <a>here</a> and go somewhere else.</article><footer></footer></body></html>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn doc_with_alt_text_elements() {
    let template = tmpl(
        "        <!DOCTYPE>
    <html>
    <head>
        <style>
#woof .bark {
	color: doggo;
}
        </style>
        <script>
if 2 < 3 {
	console.log();
}
        </script>
    </head>
        <body>
            <article></article>
            <footer/>
        </body>
</html>",
        [],
    );

    let expected =
        "<!DOCTYPE><html><head></head><body><article></article><footer></footer></body></html>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}
