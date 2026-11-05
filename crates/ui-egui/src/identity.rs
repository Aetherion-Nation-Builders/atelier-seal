//! Who this app is: the product name and where it lives. Atelier Seal is built on PdfCraft
//! (storytold/pdfcraft, MIT OR Apache-2.0). The code keeps its upstream names (crates, command
//! ids, the settings and recovery formats) so upstream work keeps merging cleanly; only what a person sees
//! is renamed, through the constants here and the catalog rewrite in [`rewrite`]. The places that
//! carry the name outside this crate are listed in `docs/brand.md`.

/// The product name as shown in the title bar, menus, dialogs and the start screen.
pub const PRODUCT: &str = "Atelier Seal";
/// The short name, for places where the suite name is already in view.
pub const SHORT: &str = "Seal";
/// The suite this app belongs to.
pub const SUITE: &str = "Atelier";
/// Who makes it.
pub const VENDOR: &str = "Aetherion Nation";
/// The upstream project this app is built on, as named in its own strings and catalogs.
pub const UPSTREAM: &str = "PdfCraft";

/// Rewrites an upstream string so it names this product. Applied to every translation catalog
/// entry as it is read, so the UI follows the brand in every language without editing the
/// catalogs (which would make every upstream catalog change a merge conflict).
pub fn rewrite(s: &str) -> String {
    // "a PdfCraft window" reads "an Atelier Seal window".
    s.replace(&format!(" a {UPSTREAM}"), &format!(" an {PRODUCT}")).replace(UPSTREAM, PRODUCT)
}

/// Like [`rewrite`], borrowing when there is nothing to change (the common case).
pub fn rewrite_cow(s: &str) -> std::borrow::Cow<'_, str> {
    if s.contains(UPSTREAM) { std::borrow::Cow::Owned(rewrite(s)) } else { std::borrow::Cow::Borrowed(s) }
}

/// Upstream strings the brand layer retires: anything naming the upstream community and site.
/// Their UI strings are replaced by ours (`i18n/brand.tsv`), so their catalog rows are skipped
/// on load (a row is retired when its source or its translation names them, which also drops the
/// command-id rows of the retired links); keeping them would leave translations for strings
/// nothing shows.
pub fn retired(s: &str) -> bool {
    ["ArtCraft", "Discord", "getartcraft", "discord.gg"].iter().any(|w| s.contains(w))
}

/// Our own UI strings, with their translations: `i18n/brand.tsv`. The catalogs are upstream
/// files, so our strings live beside them and are added to each language's catalog as it loads.
/// The file's `product` line is the rename [`rewrite`] applies; a test keeps it equal to
/// [`PRODUCT`].
const BRAND_TSV: &str = include_str!("i18n/brand.tsv");

/// `(code, source, translation)` for every line of `brand.tsv` (the `product` line included).
fn brand_lines() -> impl Iterator<Item = (&'static str, &'static str, &'static str)> {
    BRAND_TSV.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).filter_map(|l| {
        let mut cols = l.split('\t');
        match (cols.next(), cols.next(), cols.next(), cols.next()) {
            (Some(code), Some(src), Some(tr), None) if !src.is_empty() && !tr.is_empty() => Some((code, src, tr)),
            _ => None,
        }
    })
}

/// The brand strings for a language (English for a language the file does not cover).
pub fn strings(code: &str) -> Vec<(&'static str, &'static str)> {
    let of = |c: &str| brand_lines().filter(|(l, _, _)| *l == c).map(|(_, s, t)| (s, t)).collect::<Vec<_>>();
    let own = of(code);
    if own.is_empty() { of("en") } else { own }
}

/// The embedded craft-i18n catalogs with the brand layer applied: in every locale, rows naming
/// the upstream community are dropped ([`retired`]), the rest name this product ([`rewrite`]),
/// and our own strings ([`strings`]) are added to the app's catalog. The catalogs themselves
/// stay upstream's (craft-i18n), so this runs once, as they are first read.
pub fn branded(embedded: &[(&'static str, &'static str, &'static str)]) -> Vec<(&'static str, &'static str, &'static str)> {
    embedded
        .iter()
        .map(|&(code, shared, app)| {
            let mut app = brand_catalog(app);
            for (source, translation) in strings(code) {
                app.push('\t');
                app.push_str(source);
                app.push('\t');
                app.push_str(translation);
                app.push('\n');
            }
            // Built once per locale for the life of the process (the catalogs are `'static`).
            (code, &*Box::leak(brand_catalog(shared).into_boxed_str()), &*Box::leak(app.into_boxed_str()))
        })
        .collect()
}

/// One catalog file (`context<TAB>source<TAB>translation` rows, `#` comments) with the brand
/// layer applied: retired rows are dropped, and translations name this product. A row whose
/// source names the upstream product is kept under that source too: messages from below the UI
/// crate (engine errors and the like) still name it, while our own code asks for the rewritten
/// text. The context column is kept as is.
pub fn brand_catalog(tsv: &str) -> String {
    let mut out = String::with_capacity(tsv.len().saturating_add(tsv.len() / 8));
    for line in tsv.lines() {
        let row = if line.starts_with('#') { None } else { line.split_once('\t') };
        let Some((context, row)) = row else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        if retired(row) {
            continue;
        }
        let (source, translation) = row.split_once('\t').unwrap_or((row, ""));
        let translation = rewrite_cow(translation);
        let mut push = |source: &str| {
            out.push_str(context);
            out.push('\t');
            out.push_str(source);
            if row.contains('\t') {
                out.push('\t');
                out.push_str(&translation);
            }
            out.push('\n');
        };
        push(source);
        // `@id` rows are keyed by command ids, which keep upstream's names.
        if context != "@id" && source.contains(UPSTREAM) {
            push(&rewrite(source));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_upstream_name_is_rewritten_wherever_it_appears() {
        assert_eq!(rewrite("About PdfCraft"), "About Atelier Seal");
        assert_eq!(rewrite("PdfCraft — PdfCraft"), "Atelier Seal — Atelier Seal");
        assert_eq!(rewrite("Acrobat"), "Acrobat");
        assert_eq!(rewrite("open in a PdfCraft window"), "open in an Atelier Seal window");
        assert!(retired("Join the ArtCraft Discord…") && retired("ArtCraft Website") && retired("PdfCraft on getartcraft.com"));
        assert!(!retired("Join the Nation") && !retired("Save As…"));
    }

    #[test]
    fn catalogs_are_branded_as_they_load() {
        let branded =
            brand_catalog("# PdfCraft\n\tAbout PdfCraft\tÀ propos de PdfCraft\n@id\thelp.discord\tRejoindre le Discord ArtCraft\nno tabs\n");
        assert_eq!(branded, "# PdfCraft\n\tAbout PdfCraft\tÀ propos de Atelier Seal\n\tAbout Atelier Seal\tÀ propos de Atelier Seal\nno tabs\n");
        let all = branded_for_test();
        let (code, shared, app) = all.first().copied().unwrap();
        assert_eq!((code, shared), ("fr", "\tSave\tEnregistrer\n"));
        assert!(app.starts_with("\tQuit PdfCraft\tQuitter Atelier Seal\n\tQuit Atelier Seal\tQuitter Atelier Seal\n"), "{app}");
        assert!(app.contains("\tJoin the Nation\t"), "our own strings are added: {app}");
    }

    fn branded_for_test() -> Vec<(&'static str, &'static str, &'static str)> {
        branded(&[("fr", "\tSave\tEnregistrer\n", "\tQuit PdfCraft\tQuitter PdfCraft\n")])
    }

    #[test]
    fn the_names_hang_together() {
        assert!(PRODUCT.starts_with(SUITE));
        assert!(PRODUCT.ends_with(SHORT));
        assert_ne!(PRODUCT, UPSTREAM);
    }

    #[test]
    fn the_product_line_matches_the_constants() {
        let product = brand_lines().find(|(code, _, _)| *code == "product");
        assert_eq!(product, Some(("product", UPSTREAM, PRODUCT)));
    }

    #[test]
    fn every_line_parses_and_every_language_covers_the_english_set() {
        let lines = BRAND_TSV.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')).count();
        assert_eq!(lines, brand_lines().count(), "every non-comment line of brand.tsv has three columns");
        let en: Vec<&str> = strings("en").iter().map(|(k, _)| *k).collect();
        assert!(en.len() >= 4);
        let codes: std::collections::BTreeSet<&str> = brand_lines().map(|(c, _, _)| c).filter(|c| *c != "product" && *c != "en").collect();
        assert!(codes.len() >= 15, "{codes:?}");
        for code in codes {
            let keys: Vec<&str> = strings(code).iter().map(|(k, _)| *k).collect();
            assert_eq!(keys, en, "{code}");
            for (k, v) in strings(code) {
                assert_eq!(v.matches('…').count(), k.matches('…').count(), "{code}: {k:?} -> {v:?}");
            }
        }
        assert_eq!(strings("xx"), strings("en"));
    }
}
