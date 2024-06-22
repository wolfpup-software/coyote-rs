use parsley::ParsleySieve;
use txml::Sieve as TxmlSieve;

// make a "true false"
// true -> creating readable static files
// false -> creating server generated files
pub trait Sieve: ParsleySieve + TxmlSieve {}

pub struct HtmlServerSieve {}

impl HtmlServerSieve {
    pub fn new() -> HtmlServerSieve {
        HtmlServerSieve {}
    }
}

impl Sieve for HtmlServerSieve {}

impl ParsleySieve for HtmlServerSieve {
    fn alt_text(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
}

impl TxmlSieve for HtmlServerSieve {
    fn respect_indentation(&self) -> bool {
        true
    }
    fn banned_el(&self, tag: &str) -> bool {
        false
    }
    fn void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }
    fn namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }
    fn preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }
    fn inline_el(&self, tag: &str) -> bool {
        is_inline_el(tag)
    }
}

pub struct HtmlClientSieve {}

impl HtmlClientSieve {
    pub fn new() -> HtmlClientSieve {
        HtmlClientSieve {}
    }
}

impl Sieve for HtmlClientSieve {}

impl ParsleySieve for HtmlClientSieve {
    fn alt_text(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
}

impl TxmlSieve for HtmlClientSieve {
    fn respect_indentation(&self) -> bool {
        false
    }
    fn banned_el(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
    fn void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }
    fn namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }
    fn preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }
    fn inline_el(&self, tag: &str) -> bool {
        is_inline_el(tag)
    }
}

pub struct HtmlWebComponentSieve {}

impl HtmlWebComponentSieve {
    pub fn new() -> HtmlWebComponentSieve {
        HtmlWebComponentSieve {}
    }
}

impl Sieve for HtmlWebComponentSieve {}

impl ParsleySieve for HtmlWebComponentSieve {
    fn alt_text(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            "style" => true,
            _ => false,
        }
    }
}

impl TxmlSieve for HtmlWebComponentSieve {
    fn respect_indentation(&self) -> bool {
        false
    }
    fn banned_el(&self, tag: &str) -> bool {
        match tag {
            "script" => true,
            _ => false,
        }
    }
    fn void_el(&self, tag: &str) -> bool {
        is_void_el(tag)
    }
    fn namespace_el(&self, tag: &str) -> bool {
        is_namespace_el(tag)
    }
    fn preserved_text_el(&self, tag: &str) -> bool {
        is_preserved_text_el(tag)
    }
    fn inline_el(&self, tag: &str) -> bool {
        is_inline_el(tag)
    }
}

fn is_void_el(tag: &str) -> bool {
    match tag {
        "!DOCTYPE" => true,
        "!--" => true,
        "area" => true,
        "base" => true,
        "br" => true,
        "col" => true,
        "embed" => true,
        "hr" => true,
        "img" => true,
        "input" => true,
        "link" => true,
        "meta" => true,
        "param" => true,
        "source" => true,
        "track" => true,
        "wbr" => true,
        _ => false,
    }
}

fn is_namespace_el(tag: &str) -> bool {
    match tag {
        "html" => true,
        "svg" => true,
        "math" => true,
        _ => false,
    }
}

pub fn is_preserved_text_el(tag: &str) -> bool {
    return tag == "pre";
}

pub fn is_inline_el(tag: &str) -> bool {
    match tag {
        "a" => true,
        "abbr" => true,
        "b" => true,
        "bdi" => true,
        "bdo" => true,
        "cite" => true,
        "code" => true,
        "data" => true,
        "dfn" => true,
        "em" => true,
        "i" => true,
        "kbd" => true,
        "mark" => true,
        "q" => true,
        "rp" => true,
        "rt" => true,
        "ruby" => true,
        "s" => true,
        "samp" => true,
        "small" => true,
        "span" => true,
        "strong" => true,
        "sub" => true,
        "sup" => true,
        "time" => true,
        "u" => true,
        "var" => true,
        "wbr" => true,
        "area" => true,
        "audio" => true,
        "img" => true,
        "map" => true,
        "track" => true,
        "video" => true,
        "embed" => true,
        "iframe" => true,
        "object" => true,
        "picture" => true,
        "portal" => true,
        "source" => true,
        _ => false,
    }
}

fn is_html_element(tag: &str) -> bool {
    match tag {
        // document
        "html" => true,
        "base" => true,
        "head" => true,
        "link" => true,
        "meta" => true,
        "style" => true,
        "title" => true,
        // sectioning
        "body" => true,
        "address" => true,
        "article" => true,
        "aside" => true,
        "fo_eloter" => true,
        "header" => true,
        "h1" => true,
        "h2" => true,
        "h3" => true,
        "h4" => true,
        "h5" => true,
        "h6" => true,
        "hgroup" => true,
        "main" => true,
        "nav" => true,
        "section" => true,
        "search" => true,
        // text
        "blockquote" => true,
        "dd" => true,
        "div" => true,
        "dl" => true,
        "dt" => true,
        "figcaption" => true,
        "figure" => true,
        "hr" => true,
        "li" => true,
        "menu" => true,
        "ol" => true,
        "p" => true,
        "pre" => true,
        "ul" => true,
        // inline text
        "a" => true,
        "abbr" => true,
        "b" => true,
        "bdi" => true,
        "bdo" => true,
        "cite" => true,
        "co_elde" => true,
        "data" => true,
        "dfn" => true,
        "em" => true,
        "i" => true,
        "kbd" => true,
        "mark" => true,
        "q" => true,
        "rp" => true,
        "rt" => true,
        "ruby" => true,
        "s" => true,
        "samp" => true,
        "small" => true,
        "span" => true,
        "strong" => true,
        "sub" => true,
        "sup" => true,
        "time" => true,
        "u" => true,
        "var" => true,
        "wbr" => true,
        "area" => true,
        "audio" => true,
        "img" => true,
        "map" => true,
        "track" => true,
        "video" => true,
        "embed" => true,
        "iframe" => true,
        "object" => true,
        "picture" => true,
        "portal" => true,
        "source" => true,
        // svg math
        "svg" => true,
        "math" => true,
        // scripting
        "canvas" => true,
        "noscript" => true,
        "script" => true,
        // edits
        "del" => true,
        "ins" => true,
        // tables
        "caption" => true,
        "col" => true,
        "colgroup" => true,
        "table" => true,
        "tbody" => true,
        "td" => true,
        "tfoot" => true,
        "th" => true,
        "thead" => true,
        "tr" => true,
        // forms
        "button" => true,
        "datalist" => true,
        "fieldset" => true,
        "form" => true,
        "input" => true,
        "label" => true,
        "legend" => true,
        "meter" => true,
        "optgroup" => true,
        "option" => true,
        "output" => true,
        "progress" => true,
        "select" => true,
        "textarea" => true,
        // interactive elements
        "details" => true,
        "dialog" => true,
        "summary" => true,
        // web components
        "slot" => true,
        "template" => true,
        // do checks for valid web component name
        _ => false,
    }
}

fn is_svg_element(tag: &str) -> bool {
    match tag {
        "a" => true,
        "animate" => true,
        "animateMotion" => true,
        "animateTransform" => true,
        "circle" => true,
        "clipPath" => true,
        "defs" => true,
        "desc" => true,
        "feBlend" => true,
        "feColorMatrix" => true,
        "feComponentTransfer" => true,
        "feComponent" => true,
        "feConvolveMatrix" => true,
        "feDiffuseLighting" => true,
        "feDisplacementMap" => true,
        "feDistantLight" => true,
        "feDropShadow" => true,
        "feFlood" => true,
        "feFuncA" => true,
        "feFuncB" => true,
        "feFuncG" => true,
        "feFuncR" => true,
        "feGaussianBlue" => true,
        "feImage" => true,
        "feMerge" => true,
        "feMergeNode" => true,
        "feMorphology" => true,
        "feOffset" => true,
        "fePointLight" => true,
        "feSpecularLighting" => true,
        "feSpotLight" => true,
        "feTile" => true,
        "feTurbulence" => true,
        "filter" => true,
        "foreignObject" => true,
        "g" => true,
        "image" => true,
        "line" => true,
        "linearGradient" => true,
        "marker" => true,
        "mask" => true,
        "metadata" => true,
        "mpath" => true,
        "path" => true,
        "pattern" => true,
        "polygon" => true,
        "polyline" => true,
        "radialGradient" => true,
        "rect" => true,
        "script" => true,
        "set" => true,
        "stop" => true,
        "style" => true,
        "svg" => true,
        "switch" => true,
        "symbol" => true,
        "text" => true,
        "textPath" => true,
        "title" => true,
        "tspan" => true,
        "use" => true,
        "view" => true,
        _ => false,
    }
}

fn is_math_element(tag: &str) -> bool {
    match tag {
        "math" => true,
        "annotation" => true,
        "annotation-xml" => true,
        "merror" => true,
        "mfrac" => true,
        "mi" => true,
        "mmultiscripts" => true,
        "mn" => true,
        "mo" => true,
        "mover" => true,
        "mpadded" => true,
        "mphantom" => true,
        "mprescripts" => true,
        "mroot" => true,
        "mrow" => true,
        "ms" => true,
        "semantics" => true,
        "mspace" => true,
        "msqrt" => true,
        "mstyle" => true,
        "msub" => true,
        "msup" => true,
        "msubsup" => true,
        "mtable" => true,
        "mtd" => true,
        "mtext" => true,
        "mtr" => true,
        "munder" => true,
        "munderover" => true,
        _ => false,
    }
}
