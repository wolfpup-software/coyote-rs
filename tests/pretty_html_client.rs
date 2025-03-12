use coyote::{tmpl, ClientHtml, Html};

#[test]
fn text_element() {
    let template = tmpl(
        "

            Beasts tread
            softly underfoot.
            
		",
        [],
    );
    let expected = "Beasts tread softly underfoot.";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn pretty_nested_elements_and_text() {
    let template = tmpl("<a><label><input type=woofer>bark!</label><img></a>", []);
    let expected = "<a><label><input type=woofer>bark!</label><img></a>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn pretty_doc() {
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
fn pretty_doc_with_alt_text_elements() {
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
