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

    let template1 = html(&"<hello mygood=\"sir\">{}</hello>", &injections);

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

*/

#[test]
fn it_works_with_multiple_injections() {
    let descendant_template = html(template_str_0, &[]);

    let attributes = [
        Injection::Attr("whatup"),
        Injection::AttrValue("howsit", "going"),
    ];

    let descendants = [
        Injection::Template(descendant_template),
        Injection::Text(text_injection),
    ];

    // much better
    let template_str = "<howdy @click=\"onClickCallback\" {}>{}</howdy>{}";
    let injections = [
        Injection::List(&attributes),
        Injection::List(&descendants),
        Injection::Text(text_injection),
    ];

    let template = html(template_str, &injections);

    let finished_template = build(&template);

    println!("{}", finished_template);
}
