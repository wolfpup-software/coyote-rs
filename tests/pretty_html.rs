use coyote::{tmpl, Html};

#[test]
fn empty_element() {
    let template = tmpl("<html></html>", []);
    let expected = "<html></html>";

    let mut html = Html::new();
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

    let mut html = Html::new();
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
                <span> World!</span>   </h1>",
        [],
    );

    let expected = "<h1>\n\tHello <span>World!</span>\n</h1>";

    let mut html = Html::new();
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

    let expected = "<input>\n<input>\n<input>\n<input>";

    let mut html = Html::new();
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
    let expected =
        "<!DOCTYPE html>\n<input type=checkbox>\n<input woof=\"bark\">\n<input grrr>\n<input>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn inline_elements() {
    let template = tmpl("<span>hai <span>:3</span></span> ", []);
    let expected = "<span>hai <span>:3</span></span>";
    let mut html = Html::new();
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

    let mut html = Html::new();
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
    let expected = "<input>\n<p>\n\thai :3\n</p>";

    let mut html = Html::new();
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

    let expected = "<section>\n\t<input>\n\t<p>\n\t\thai :3\n\t</p>\n</section>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn alt_text_element() {
    let template = tmpl(
        "<style>#woof .bark {
    color: doggo;
}</style>",
        [],
    );
    let expected = "<style>\n\t#woof .bark {\n\t    color: doggo;\n\t}\n</style>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn nested_void_elements() {
    let template = tmpl("<a><label><input type=woofer>bark!</label></a>", []);
    let expected = "<a>\n\t<label>\n\t\t<input type=woofer>\n\t\tbark!\n\t</label>\n</a>";

    let mut html = Html::new();
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
    "<!DOCTYPE>\n<html>\n\t<head></head>\n\t<body>\n\t\t<article>\n\t\t\tYou're a <span>boy kisser</span> aren't you?\n\t\t\tClick\n\t\t\t<a>\n\t\t\t\there\n\t\t\t</a>\n\t\t\tand go somewhere else.\n\t\t</article>\n\t\t<footer></footer>\n\t</body>\n</html>";

    let mut html = Html::new();
    let results = html.build(&template);

    assert_eq!(Ok(expected.to_string()), results);
}

#[test]
fn document_with_alt_text_elements() {
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

    assert_eq!(Ok(expected.to_string()), results);
}
