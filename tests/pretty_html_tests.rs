use coyote::{tmpl, ClientHtml, Html};

#[test]
fn test_pretty_html_no_empty_space() {
    let template = tmpl("<html></html>", []);
    let expected = "<html></html>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_void_el() {
    let template = tmpl(
        "<input>   <input>
            <input><input> ",
        [],
    );
    let expected = "<input>\n<input>\n<input>\n<input>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_void_el_with_attributes() {
    let template = tmpl(
        "
        <!DOCTYPE html><input type=checkbox>   <input woof=\"bark\">
            <input grrr><input> ",
        [],
    );
    let expected =
        "<!DOCTYPE html>\n<input type=checkbox>\n<input woof=\"bark\">\n<input grrr>\n<input>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_void_el_and_others() {
    let template = tmpl(
        "
            <input><p>hai :3</p>    ",
        [],
    );
    let expected = "<input>\n<p>\n\thai :3\n</p>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_nested_void_el() {
    let template = tmpl(
        "
        <section>
            <input><p>hai :3</p>
        </section>
    ",
        [],
    );
    let expected = "<section>\n\t<input>\n\t<p>\n\t\thai :3\n\t</p>\n</section>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_preserved_space_el() {
    let template = tmpl(
        "<style>#woof .bark {
    color: doggo;
}</style>",
        [],
    );
    let expected = "<style>\n\t#woof .bark {\n\t    color: doggo;\n\t}\n</style>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_doc() {
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
        "<!DOCTYPE>\n<html>\n\t<head>\n\t\t<style>\n\t\t\t#woof .bark {\n\t\t\t\tcolor: doggo;\n\t\t\t}\n\t\t</style>\n\t\t<script>\n\t\t\tif 2 < 3 {\n\t\t\t\tconsole.log();\n\t\t\t}\n\t\t</script>\n\t</head>\n\t<body>\n\t\t<article></article>\n\t\t<footer></footer>\n\t</body>\n</html>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_client() {
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

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_without_indents_server() {
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
    "<!DOCTYPE>\n<html>\n\t<head></head>\n\t<body>\n\t\t<article>\n\t\t\tYou're a <span>boy kisser</span> aren't you?\n\t\t\tClick\n\t\t\t<a>\n\t\t\t\there\n\t\t\t</a>\n\t\t\tand go somewhere else.\n\t\t</article>\n\t\t<footer></footer>\n\t</body>\n</html>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_without_indents_client() {
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

    assert_eq!(expected, results);
}

/* complicated inline cases */
// there should be some logic for img tags
//
#[test]
fn test_pretty_html_without_indents_and_text() {
    let template = tmpl("<a><label><input type=woofer>bark!</label><img></a>", []);

    let expected = "<a>\n\t<label>\n\t\t<input type=woofer>\n\t\tbark!\n\t</label>\n\t<img>\n</a>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_without_indents_and_text_client() {
    let template = tmpl("<a><label><input type=woofer>bark!</label><img></a>", []);
    let expected = "<a><label><input type=woofer>bark!</label><img></a>";

    let mut html = ClientHtml::new();
    let results = html.build(&template);

    assert_eq!(expected, results);
}
