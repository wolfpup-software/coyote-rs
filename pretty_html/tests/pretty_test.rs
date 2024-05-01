use pretty_html::PretyHtmlBuilder;
use txml::build_template;

#[test]
fn it_works<'a>() {
    let lil_template: &str = "<section><header><p>hai :3</p></header></section>";
    let expected: &str = "<section>
	<header>
		<p>
			hai :3
		</p>
	</header>
</section>
";

    let mut builder = PretyHtmlBuilder::new();
    build_template(&mut builder, lil_template);

    let results = builder.build();

    assert_eq!(expected, results);
}
