use html::Injection;
use txml::Template;

const template_str_0: &str = "<hello>world</hello>";
const template_str_1: &str = "<hello mygood=\"sir\">{}</hello>";
const injection_template_0: &str = "<howdy>{}</howdy>";
const text_injection: &str = "pardner!
what's good
		we should hang out
	with all these dumb tabs
";

/*

type TestTemplate<'a> = Template<'a, Injection<'a, ()>>;

#[test]
fn it_works<'a>() {
    let mut builder = StaticHtmlBuilder::new();

    let template1 = TestTemplate {
        kind: "html",
        template_str: template_str_0,
        injections: Vec::<Injection<'a, ()>>::new(),
    };

    build(&mut builder, template1);
}

/*
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

*/

#[test]
fn it_works_with_multiple_injections_returnable() {
    let mut doc_builder = StaticHtmlBuilder::new();

    let template = nested_test_component();
    build(&mut doc_builder, template);

    let rendered_str = doc_builder.build();
    let expected = "<wolf whatup howsit=\"going\">
    <hello>
        world
    </hello>
    pardner!
    what's good
    we should hang out
    with all these dumb tabs
</wolf>
pardner!
what's good
we should hang out
with all these dumb tabs
";

    assert_eq!(expected, rendered_str);
    println!("{}", rendered_str);
}

fn nested_test_component<'a>() -> TestTemplate<'a> {
    let descendant_template = html(template_str_0, Vec::new());

    let scoped_attr = "hello".to_string();
    let attributes = Vec::from([
        Injection::Attr(scoped_attr),
        Injection::AttrValue("howsit".to_string(), "going".to_string()),
    ]);

    let descendants = Vec::from([
        Injection::Template(descendant_template),
        Injection::Text(text_injection.to_string()),
    ]);

    return html(
        "<wolf {}>{}</wolf>{}",
        Vec::from([
            Injection::List(attributes),
            Injection::List(descendants),
            Injection::Text(text_injection.to_string()),
        ]),
    );
}

*/
