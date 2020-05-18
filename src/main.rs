use cursive::view::Scrollable;

fn main() {
    better_panic::install();
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("provide filename please");
    let doc = epub::doc::EpubDoc::new(std::path::PathBuf::from(&path).as_path())
        .expect("couldn't open the book");
    let select = cursive::views::SelectView::new()
        .autojump()
        .with_all(doc.toc.iter().map(|n| (n.label.clone(), n.content.clone())))
        .on_submit(
            move |siv: &mut cursive::Cursive, resource: &std::path::PathBuf| {
                let mut doc = epub::doc::EpubDoc::new(std::path::PathBuf::from(&path).as_path())
                    .expect("couldn't open the book");
                let mut url = url::Url::parse("epub:///")
                    .unwrap()
                    .join(
                        resource
                            .as_path()
                            .to_str()
                            .expect("invalid table of contents entry"),
                    )
                    .expect("invalid table of contents entry");
                url.set_fragment(None);
                let html = doc
                    .get_resource_str_by_path(url.path().trim_start_matches('/'))
                    .expect("failed to load chapter");
                let markdown = html2md::parse_html(&html);
                let styled = cursive::utils::markup::markdown::parse(markdown);
                siv.add_fullscreen_layer(
                    cursive::views::Dialog::around(cursive::views::Panel::new(
                        cursive::views::TextView::new(styled).scrollable(),
                    ))
                    .button("Back", |s| {
                        s.pop_layer();
                    }),
                );
            },
        );
    let mut siv = cursive::default();
    siv.add_layer(
        cursive::views::Dialog::around(select.scrollable())
            .title("Select a chapter")
            .button("Back", |s| {
                s.quit();
            }),
    );
    let xdg_dirs = xdg::BaseDirectories::with_prefix("myne").unwrap();
    match xdg_dirs.find_config_file("theme.toml") {
        Some(pb) => {
            siv.load_theme_file(pb).unwrap();
        }
        None => {}
    }
    siv.run();
}
