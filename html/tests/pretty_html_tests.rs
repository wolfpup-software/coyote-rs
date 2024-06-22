use parsley::{get_text_from_step, parse_template_str, Step, StepKind};
use pretty_html::compose;
use pretty_html::sieves::{HtmlClientSieve, HtmlServerSieve, HtmlWebComponentSieve};
use txml::{attr_val, list, text, txml, Component};

#[test]
fn test_pretty_html() {
    let template = "        <!DOCTYPE>
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
</html>";
    // "<!DOCTYPE>\n\t<html>\n\t<head>\n\t\t<style>\n\t\t\t#woof .bark {\n\t\t\t\tcolor: doggo;\n\t\t\t}\n\t\t</style>\n\t\t<script>\n\t\t\tif 2 < 3 {\n\t\t\t\tconsole.log();\n\t\t\t}\n\t\t</script>\n\t</head>\n\t\t<body>\n\t\t\t<article></article>\n\t\t\t<footer/>\n\t\t</body>\n</html>";
    let expected =
	"<!DOCTYPE>\n\t<html>\n\t<head>\n\t\t<style>\n\t\t\t#woof .bark {\n\t\t\t\tcolor: doggo;\n\t\t\t}\n\t\t</style>\n\t\t<script>\n\t\t\tif 2 < 3 {\n\t\t\t\tconsole.log();\n\t\t\t}\n\t\t</script>\n\t</head>\n\t\t<body>\n\t\t\t<article></article>\n\t\t\t<footer/>\n\t\t</body>\n</html>";
    let sieve = HtmlServerSieve::new();
    let results = compose(&sieve, &template);
    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_client() {
    let template = "        <!DOCTYPE>
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
</html>";
    let expected =
        "<!DOCTYPE><html><head></head><body><article></article><footer></footer></body></html>";

    let sieve = HtmlClientSieve::new();
    let results = compose(&sieve, &template);
    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_web_component() {
    let template = "        <!DOCTYPE>
    <html>
    <head>

    </head>
        <body>
            <article></article>
            <footer/>
        </body>
</html>";

    let expected =
        "<!DOCTYPE><html><head></head><body><article></article><footer></footer></body></html>";

    let sieve = HtmlClientSieve::new();
    let results = compose(&sieve, &template);
    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_without_indents_server() {
    let template = "        <!DOCTYPE>
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
</html>";

    let expected =
    "<!DOCTYPE>\n<html>\n\t<head></head>\n\t<body>\n\t\t<article>\n\t\t\tYou're a <span>boy kisser</span> aren't you?\n\t\t\tClick <a>here</a> and go somewhere else.\n\t\t</article>\n\t\t<footer></footer>\n\t</body>\n</html>";
    let sieve = HtmlServerSieve::new();
    let results = compose(&sieve, &template);
    assert_eq!(expected, results);
}

#[test]
fn test_pretty_html_without_indents_client() {
    let template = "        <!DOCTYPE>
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
</html>";

    let expected =
        "<!DOCTYPE><html><head></head><body><article>You're a <span>boy kisser</span> aren't you? Click <a>here</a> and go somewhere else.</article><footer></footer></body></html>";
    let sieve = HtmlClientSieve::new();
    let results = compose(&sieve, &template);
    assert_eq!(expected, results);
}
