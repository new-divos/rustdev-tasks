use xml_builder::{attribute::XmlAttribute, document::XmlDocument, element::XmlElement};

fn main() {
    let mut article1 = XmlElement::new("article");

    let mut title = XmlElement::new("title");
    title.add_text(
        "Алгоритмы фрактального анализа временных рядов в системах мониторинга сенсорных сетей",
    );

    let mut authors = XmlElement::new("authors");
    authors
        .add_child(XmlElement::with_text("author", "Аксенов Владислав"))
        .add_child(XmlElement::with_text("author", "Дмитриев Владимир"));

    article1
        .add_attribute(XmlAttribute::new("issn", "2072-9502"))
        .add_attribute(XmlAttribute::new("date", "2012"))
        .add_child(title)
        .add_child(authors);

    let mut article2 = XmlElement::new("article");

    let mut title = XmlElement::new("title");
    title.add_text("On the Time Series Length for an Accurate Fractal Analysis in Network Systems");

    let mut authors = XmlElement::new("authors");
    authors.add_child(XmlElement::with_text("author", "Millán G."));

    article2
        .add_attribute(XmlAttribute::new("date", "2021-05-06"))
        .add_child(title)
        .add_child(authors);

    let mut articles = XmlElement::new("articles");
    articles.add_child(article1).add_child(article2);

    let mut doc = XmlDocument::new();
    doc.add_comment("Статьи о самоподобии во временных рядах")
        .add_child(articles);

    println!("{}", doc);
}
