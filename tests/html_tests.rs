use html::{build, html, Injection};

const template_str_0: &str = "<hello>world</hello>";
const template_str_1: &str = "<hello mygood=\"sir\">{}</hello>";
const injection_template_0: &str = "<howdy>{}</howdy>";
const text_injection: &str = "pardner!
what's good
		we should hang out
	with all these dumb tabs
";

// #[test]
fn it_works() {
    let template1 = html(template_str_0, &[]);

    let finished_template = build(&template1);
}

#[test]
fn it_works_with_str_injections() {
		let text = "pardner!
what's good
		we should hang out
	with all these dumb tabs
";
		let injections = [Injection::Text(text)];
    
    let template1 = html(
        &"<hello mygood=\"sir\">{}</hello>",
        &injections,
    );

    let finished_template = build(&template1);
    
    println!("{}", finished_template)
}

/*
// #[test]
fn it_works_with_template_injections() {
    let inj = Injection::Text(text_injection);
    let template = html(injection_template_0, Vec::from([Vec::from([inj])]));

    let temp_inj = Injection::Template(template);
    let template1 = html(
        "<hello mygood=\"sir\">{}</hello>",
        Vec::from([Vec::from([temp_inj])]),
    );

    let finished_template = build(&template1);
    println!("{}", finished_template);
}

#[test]
fn it_works_with_multiple_injections() {
    let text_inj = Injection::Text(text_injection);
    let text_inj2 = Injection::Text(text_injection);
    let template = html(template_str_0, Vec::from([]));
    let temp_inj = Injection::Template(template);

    let template1 = html(
        "<hello {}>{}</hello>{}",
        &Vec::from([
            Injection::List(&[
                Injection::Attr("howdy"),
                Injection::AttrValue("howsit", "going"),
            ]),
            Injection::List(&[temp_inj, text_inj]),
            text_inj2,
        ]),
    );

    let finished_template = build(&template1);
    println!("{}", finished_template);
}
*/

