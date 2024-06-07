use parsley::{get_text_from_step, parse_template_str, Step, StepKind};
use pretty_html::compose;
use pretty_html::sieves::HtmlServerSieve;
use txml::{attr_val, list, text, txml, Component};

#[test]
fn test_pretty_html() {
    let template = "
        <!DOCTYPE>
            <html>
            <head>
                <meta stuff=otherstuff>
                <!-- sdfsdf s df sdf-->
                <meta yo yo yo>
            </head>
                <body>
                    <header>hai! :3</header>
                    <pre>

    hiiiii
            </pre>
                    <article></article>
                    <footer/>
                </body>
        </html>
    ";

    let sieve = HtmlServerSieve::new();
    let results = compose(&sieve, &template);
    println!("{:?}", results);
}
