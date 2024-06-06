use parsley::{get_text_from_step, parse_template_str, Step, StepKind};
use pretty_html::compose;
use pretty_html::sieves::HtmlServerSieve;
use txml::{attr_val, list, text, txml, Component};

#[test]
fn test_pretty_html() {
    let template = "
        <!DOCTYPE>
            <html>
            <head />
                <body>
                    <header></header>
                    <footer/>
                </body>
        </html>
    ";

    let sieve = HtmlServerSieve::new();
    let results = compose(&sieve, &template);
    println!("{:?}", results);
}
