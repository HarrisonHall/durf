//! Tests.

use super::*;

#[test_log::test]
fn parse_page_1() {
    let page = r#"
            <!doctype html>
            <html>
              <head>
                <!-- Meta -->
                <meta charset="utf-8" />
                <meta name="description" content="Homepage of Harrison Hall" />
                <meta name="author" content="Harrison Hall" />
                <meta
                  name="keywords"
                  content="Harrison Hall, Harrison, hachha, hocko, blog, tech, projects, resume"
                />

                <!-- Style -->
                <link rel="stylesheet" href="/styles/spectre/spectre.min.css" />
                <link rel="stylesheet" href="/styles/phosphor/style.css" />

                <!-- Custom styles -->
                <link rel="stylesheet" href="/styles/site_colors.css" />
                <link rel="stylesheet" href="/styles/content.css" />

                <!-- Mobile support -->
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <!-- Feeds -->
                <link
                  rel="alternate"
                  type="application/atom+xml"
                  title="hachha.dev blog"
                  href="/blog.feed"
                />
                <link
                  rel="alternate"
                  type="application/atom+xml"
                  title="hachha.dev links"
                  href="/links.feed"
                />
                <link
                  rel="alternate"
                  type="application/atom+xml"
                  title="hachha.dev slipfeed"
                  href="https://feeds.hachha.dev/all/feed"
                />

                <!-- Firefox FOUC fix -->
                <script>
                  let FF_FOUC_FIX;
                </script>
              </head>

              <body class="index-page">
                <header class="navbar" style="z-index: 1">
                  <section class="navbar-section">
                    <a href="/" class="btn btn-link text-secondary"><b>hachha.dev</b></a>
                  </section>
                  <section class="navbar-center">
                    <a href="/blog" class="btn btn-link text-gray">Blog</a>
                    <a href="/links" class="btn btn-link text-gray">Links</a>
                    <a
                      href="https://www.linkedin.com/in/harrison-hall-525b81123/"
                      target="”_blank”"
                      class="btn btn-link text-gray"
                      >Resume</a
                    >
                    <a href="/projects" class="btn btn-link text-gray">Projects</a>
                    <a
                      href="https://github.com/trackl-games"
                      target="”_blank”"
                      class="btn btn-link text-gray"
                      >Games</a
                    >
                  </section>
                  <section class="navbar-section"></section>
                </header>

                <div class="hero">
                  <div class="container grid-lg text-center">
                    <h1 style="font-size: 4em"><b>Harrison Hall</b></h1>
                    <p>Check out <a href="https://github.com/harrisonhall/slipstream">slipstream</a>!</p>
                  </div>
                </div>

                <img
                    src="/media/profile-b.png" alt=""
                    class="image-circle img-responsive"
                style="max-width: 256px; max-height: 256px;"
                >
                <!-- -->
                <div class="section" >
                    <div class="container grid-md">
                        <div class="empty">
                      <div class="container grid-xs">
                        <div
                          class="columns"
                          style="padding-left: 2em; padding-right: 2em; text-align: center"
                        >
                          <div class="column col-4 col-mx-auto">
                            <a
                              href="https://github.com/harrisonhall"
                              class="icon-link"
                              target="_blank"
                            >
                              <i class="ph-fill ph-github-logo"></i>
                            </a>
                          </div>
                          <div class="column col-4 col-mx-auto">
                            <a
                              href="https://www.linkedin.com/in/harrison-hall-525b81123/"
                              class="icon-link"
                              target="_blank"
                            >
                              <i class="ph-fill ph-linkedin-logo"></i>
                            </a>
                          </div>
                          <div class="column col-4 col-mx-auto">
                            <a
                              href="https://mastodon.social/@harryhallyall"
                              class="icon-link"
                              target="_blank"
                            >
                              <i class="ph-fill ph-mastodon-logo"></i>
                            </a>
                          </div>
                        </div>
                      </div>
                    </div>
                </div>
                </div>
                <!-- -->
                <div class="container grid-md" style="padding-left: 2em; padding-right: 2em;">
                    <div class="divider text-center"  ></div>
                </div>

                <div style="text-align: center">
                  <div class="section" >
                      <div class="container grid-md">
                          <div class="container">
                      <div class="columns">
                        <div class="column col-12">
                          <p></p>
                        </div>
                      </div>
                    </div>
                </div>
                  </div>
                </div>

                <div class="footer-spacer"></div>
                <footer class="text-center">
                  <div class="container grid-lg" id="copyright">
                    <p>
                      <span>© Harrison Hall 2025</span>
                      <br />
                      <a href="https://github.com/HarrisonHall/hachha.dev">v0.10.6</a>
                    </p>
                  </div>
                </footer>
              </body>
            </html>
        "#;
    let ast = Ast::from_html(page, ParseFlags::default());
    assert!(ast.is_ok());
    let mut ast = ast.unwrap();
    tracing::trace!("{ast}");
    ast.root.minimize();
    tracing::trace!("{ast}");
}

#[test_log::test]
fn parse_page_2() {
    let page = r#"
        <html><head>
            <!-- Meta -->
            <meta charset="utf-8">
            <meta name="description" content="Homepage of Harrison Hall">
            <meta name="author" content="Harrison Hall">
            <meta name="keywords" content="Harrison Hall, Harrison, hachha, hocko, blog, tech, projects, resume">
    
            <!-- Style -->
            <link rel="stylesheet" href="/styles/spectre/spectre.min.css">
            <link rel="stylesheet" href="/styles/phosphor/style.css">
    
            <!-- Custom styles -->
            <link rel="stylesheet" href="/styles/site_colors.css">
            <link rel="stylesheet" href="/styles/content.css">
    
            <!-- Mobile support -->
            <meta name="viewport" content="width=device-width, initial-scale=1">
    
            <!-- Feeds -->
            <link rel="alternate" type="application/atom+xml" title="hachha.dev blog" href="/blog.feed">
            <link rel="alternate" type="application/atom+xml" title="hachha.dev links" href="/links.feed">
            <link rel="alternate" type="application/atom+xml" title="hachha.dev slipfeed" href="https://feeds.hachha.dev/all/feed">
    
            <!-- Firefox FOUC fix -->
            <script>
              let FF_FOUC_FIX;
            </script>
         <link rel="stylesheet" href="/styles/highlight.js/catppuccin-mocha.min.css">
         <script src="/styles/highlight.js/highlight.min.js"></script>
         <script>
           hljs.highlightAll();
         </script>

          <style>:is([id*='google_ads_iframe'],[id*='taboola-'],.taboolaHeight,.taboola-placeholder,#top-ad,#credential_picker_container,#credentials-picker-container,#credential_picker_iframe,[id*='google-one-tap-iframe'],#google-one-tap-popup-container,.google-one-tap__module,.google-one-tap-modal-div,#amp_floatingAdDiv,#ez-content-blocker-container) {display:none!important;min-height:0!important;height:0!important;}</style></head>

          <body class="blog-page">
            <header class="navbar" style="z-index: 1">
              <section class="navbar-section">
                <a href="/" class="btn btn-link text-secondary"><b>hachha.dev</b></a>
              </section>
              <section class="navbar-center">
                <a href="/blog" class="btn btn-link text-gray">Blog</a>
                <a href="/links" class="btn btn-link text-gray">Links</a>
                <a href="https://www.linkedin.com/in/harrison-hall-525b81123/" target="”_blank”" class="btn btn-link text-gray">Resume</a>
                <a href="/projects" class="btn btn-link text-gray">Projects</a>
                <a href="https://github.com/trackl-games" target="”_blank”" class="btn btn-link text-gray">Games</a>
              </section>
              <section class="navbar-section"></section>
            </header>

            <article class="blog-article">
              <div class="hero">
                <div class="container grid-lg text-center">
                  <h1 style="font-size: 4em"><b>Slipstream!</b></h1>
                  <h2 style="font-size: 2em"><b>Slipstream is out!</b></h2>
                  <span class="chip">2025-02-22</span>
                </div>
              </div>

              <div class="blog-markdown">
                <div class="section">
                    <div class="container grid-md">
                        <p>You heard it here first, <code>slipstream</code> is
        <a href="https://github.com/HarrisonHall/slipstream/releases/tag/slipstream-1.0.0"><em>out</em></a>,
        just in time for
        <a href="https://en.wikipedia.org/wiki/National_Cat_Day#Japan">cat day</a>!</p>
        <p><img src="/blog/slipstream_2/web_ui.png" alt="slipstream"></p>
        <p>I couldn't be happier with the result, but it's worth noting this isn't what I
        originally promised. Where are custom lua filters? Tracking read articles (read:
        headlines)? Super fancy tui?</p>
        <p>After using <code>slipknot</code> for a while, I realized I didn't actually care about many
        of those features. If I need a new filter, I can just push a new version of
        slipstream out. My readers can track what I've read, and I no longer care about
        sharing that between devices.</p>
        <p>So what happened to <code>slipknot</code>? <code>slipstream</code> now contains all of what was
        <code>slipknot</code>. I didn't see a reason to keep them separate or reimplement features.
        <code>slipstream</code> is basically <code>slipknot</code> with the default addresses going to the web
        view (atom feeds are now accessible with an extra <code>/feed</code> in the path).
        Honestly, I felt the name <code>slipknot</code> was a little aggressive, I wanted something
        with "slip" in it, but didn't think too much about it.</p>
        <p>But seriously, <a href="https://feeds.hachha.dev/">check it out</a>! The source remains on
        <a href="https://github.com/HarrisonHall/slipstream">github</a>.</p>
        <h2>Future Plans</h2>
        <p>I may still revisit my own tui in the future, but for now <code>newsraft</code> (tui) and
        <code>feeder</code> (mobile) are completely sufficient for my own needs.</p>
        <p>There are some outstanding tasks I need to eventually finish up.</p>
        <ul>
        <li><code>slipfeed</code>
        <ul>
        <li><input type="checkbox" disabled=""> Add other built-in feed implementations (e.g. activitypub)</li>
        </ul>
        </li>
        <li><code>slipstream</code>
        <ul>
        <li><input type="checkbox" disabled=""> Add more filters (regex/pomsky, allowlists, etc.)</li>
        <li><input type="checkbox" disabled=""> OPML conversion support</li>
        <li><input type="checkbox" disabled=""> Use sqlite for storing entries and feed definitions</li>
        <li><input type="checkbox" disabled=""> Support atom exports</li>
        </ul>
        </li>
        </ul>
        <p>...but I don't need any of these now, so who knows when they'll be completed.
        ¯\_(ツ)_/¯</p>

                    </div>
                </div>
              </div>
            </article>

            <div class="footer-spacer"></div>
            <footer class="text-center">
              <div class="container grid-lg" id="copyright">
                <p>
                  <span>© Harrison Hall 2025</span>
                  <br>
                  <a href="https://github.com/HarrisonHall/hachha.dev">v0.10.6</a>
                </p>
              </div>
            </footer>
  

        </body></html>
        "#;
    let ast = Ast::from_html(page, ParseFlags::default());
    assert!(ast.is_ok());
    let mut ast = ast.unwrap();
    tracing::trace!("{ast}");
    ast.root.minimize();
    tracing::trace!("{ast}");
}

#[test_log::test]
fn parse_page_3_jp() {
    let page = r#"
        <article class="easy-article">
          <div class="article-figure">
                        <figure id="js-article-figure" class="is-show"><img src="https://news.web.nhk/news/easy/ogp/ne2026012811533/8fCtO2v6MrdvTGU7BWUUAXvbchlpXViEvlLPInyG.jpg" alt="" onerror="this.src='/news/easy/images/noimg_default_easy_m.jpg';"></figure>
                      </div>

          <h1 class="article-title">
              29<ruby>日<rt>にち</rt></ruby>から<ruby>日本海<rt>にほんかい</rt></ruby><ruby>側<rt>がわ</rt></ruby>などでまた<ruby>雪<rt>ゆき</rt></ruby>がたくさん<ruby>降<rt>ふ</rt></ruby>りそう
          </h1>
          <p class="article-date" id="js-article-date">2026年1月28日 19時20分</p>
          <div class="article-top-tool">
            <div class="article-buttons">
              <a href="" class="article-buttons__audio js-open-audio">
                <span> ニュースを<ruby>聞<rt>き</rt></ruby>く </span>
              </a>
              <a href="" class="article-buttons__ruby js-toggle-ruby is-ruby --pc">
                <ruby>漢字<rt>かんじ</rt></ruby>の<ruby>読<rt>よ</rt></ruby>み<ruby>方<rt>かた</rt></ruby>を<ruby>消<rt>け</rt></ruby>す
              </a>
            </div>

            <a href="" class="article-buttons__ruby js-toggle-ruby is-ruby --sp">
              <ruby>漢字<rt>かんじ</rt></ruby>の<ruby>読<rt>よ</rt></ruby>み<ruby>方<rt>かた</rt></ruby>を<ruby>消<rt>け</rt></ruby>す
            </a>

            <div class="audio-player" id="js-audio-wrapper">
              <div id="js-audio-inner"></div>
            </div>
          </div>
          <div class="article-body" id="js-article-body">
            <p><span class="colorL"><ruby>東北地方<rt>とうほくちほう</rt></ruby></span><span class="colorB">から</span><span class="colorL"><ruby>中国地方<rt>ちゅうごくちほう</rt></ruby></span><span class="colorB">まで</span><span class="colorB">の</span><span class="colorL"><ruby>日本海<rt>にほんかい</rt></ruby></span><span class="color4"><ruby>側<rt>がわ</rt></ruby></span><span class="color4">など</span><span class="colorB">で</span><span class="colorB">、</span><span class="colorB">29</span><span class="color4"><ruby>日<rt>にち</rt></ruby></span><span class="colorB">から</span><span class="color4">また</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4">たくさん</span><span class="color4"><ruby>降<rt>ふ</rt></ruby>り</span><span class="colorB">そう</span><span class="colorB">です</span><span class="colorB">。</span></p>
            <p><span class="colorC"><ruby>気象庁<rt>きしょうちょう</rt></ruby></span><span class="colorB">によると</span><span class="colorB">、</span><span class="colorB">29</span><span class="color4"><ruby>日<rt>にち</rt></ruby></span><span class="colorB">の</span><span class="color4"><ruby>夕方<rt>ゆうがた</rt></ruby></span><span class="colorB">まで</span><span class="colorB">の</span><span class="colorB">24</span><span class="color4"><ruby>時間<rt>じかん</rt></ruby></span><span class="colorB">に</span><span class="colorB">、</span><span class="colorL"><ruby>新潟県<rt>にいがたけん</rt></ruby></span><span class="colorB">と</span><span class="colorL"><ruby>北陸地方<rt>ほくりくちほう</rt></ruby></span><span class="colorB">の</span><span class="color4"><ruby>多<rt>おお</rt></ruby>い</span><span class="colorB">ところ</span><span class="colorB">で</span><span class="colorB">60</span><span class="color2">cm</span><span class="colorB">、</span><span class="colorL"><ruby>青森県<rt>あおもりけん</rt></ruby></span><span class="colorB">で</span><span class="colorB">50</span><span class="color2">cm</span><span class="colorB">、</span><span class="colorL"><ruby>近畿地方<rt>きんきちほう</rt></ruby></span><span class="colorB">で</span><span class="colorB">40</span><span class="color2">cm</span><span class="color4">ぐらい</span><span class="colorB">の</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>降<rt>ふ</rt></ruby>る</span><span class="color3"><ruby>心配<rt>しんぱい</rt></ruby></span><span class="colorB">が</span><span class="color4">あり</span><span class="colorB">ます</span><span class="colorB">。</span><span class="color4">いつも</span><span class="colorB">は</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>少<rt>すく</rt></ruby>ない</span><span class="colorL"><ruby>太平洋<rt>たいへいよう</rt></ruby></span><span class="color4"><ruby>側<rt>がわ</rt></ruby></span><span class="color4">でも</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>降<rt>ふ</rt></ruby>る</span><span class="colorB">ところ</span><span class="colorB">が</span><span class="color4">あり</span><span class="colorB">そう</span><span class="colorB">です</span><span class="colorB">。</span></p>
            <p><span class="color3"><ruby>交通<rt>こうつう</rt></ruby></span><span class="colorB">が</span><span class="color4"><ruby>止<rt>と</rt></ruby>まる</span><span class="colorB">かも</span><span class="colorB">しれ</span><span class="colorB">ませ</span><span class="colorB">ん</span><span class="colorB">。</span><span class="color4"><ruby>天気<rt>てんき</rt></ruby></span><span class="colorB">や</span><span class="color3"><ruby>交通<rt>こうつう</rt></ruby></span><span class="colorB">の</span><span class="color2"><ruby>情報<rt>じょうほう</rt></ruby></span><span class="colorB">を</span><span class="color4"><ruby>見<rt>み</rt></ruby></span><span class="colorB">て</span><span class="color3">ください</span><span class="colorB">。</span><span class="color4"><ruby>雪<rt>ゆき</rt></ruby></span><span class="colorB">を</span><span class="color3"><ruby>片<rt>かた</rt></ruby>づける</span><span class="color4">とき</span><span class="colorB">の</span><span class="color3"><ruby>事故<rt>じこ</rt></ruby></span><span class="colorB">にも</span><span class="color0"><ruby>気<rt>き</rt></ruby></span><span class="color0">をつけ</span><span class="colorB">て</span><span class="color3">ください</span><span class="colorB">。</span></p>
          </div>

          <div class="article-info">
            <div class="article-info__color">
              <ul class="color__list">
                <li class="--person">
                  … <ruby>人<rt>ひと</rt></ruby>の<ruby>名前<rt>なまえ</rt></ruby>
                </li>
                <li class="--place">
                  … <ruby>国<rt>くに</rt></ruby>や<ruby>県<rt>けん</rt></ruby>、<ruby>町<rt>まち</rt></ruby>、<ruby>場所<rt>ばしょ</rt></ruby>などの<ruby>名前<rt>なまえ</rt></ruby>
                </li>
                <li class="--group">
                  … <ruby>会社<rt>かいしゃ</rt></ruby>やグループなどの<ruby>名前<rt>なまえ</rt></ruby>
                </li>
              </ul>
              <a href="" class="color__toggle" id="js-toggle-color">ことばの<ruby>色<rt>いろ</rt></ruby>を<ruby>消<rt>け</rt></ruby>す</a>
            </div>
          </div>
          <div class="article-share">
            <div class="nhk-snsbtn" data-nhksns-disable="google" data-nhksns-description=" "></div>
          </div>
                    <div class="article-link" id="js-regular-news-wrapper">
            <a href="https://news.web.nhk/newsweb/na/na-k10015037371000" class="btn btn__no-ruby" target="_blank" id="js-regular-news">NEWS WEBでよむ</a>
          </div>
          </article>
        "#;
    let ast = Ast::from_html(page, ParseFlags::default());
    assert!(ast.is_ok());
    let mut ast = ast.unwrap();
    tracing::trace!("{ast}");
    ast.root.minimize();
    tracing::trace!("{ast}");
}

#[test_log::test]
fn parse_page_4_jp() {
    let page = r#"
        <div>
            <p>Test1</p>
            <p class="calibre3"><span xmlns="http://www.w3.org/1999/xhtml" class="kobospan" id="kobo.16.1">　そう、無敵だと信じていた〝</span><ruby><span xmlns="http://www.w3.org/1999/xhtml" class="kobospan" id="kobo.17.1">王宮城塞</span><rt>キャッスルガード</rt></ruby><span xmlns="http://www.w3.org/1999/xhtml" class="kobospan" id="kobo.18.1">〟が破られたのは、フェルドウェイからしても想定外過ぎた。慎重な性格でなかったとしても、撤退を選択するに十分な理由であろう。</span></p>
        </div>
        <p>Test2</p>
        "#;
    let ast = Ast::from_html(page, ParseFlags::default());
    assert!(ast.is_ok());
    let mut ast = ast.unwrap();
    tracing::trace!("{ast}");
    ast.root.minimize();
    tracing::trace!("{ast}");
}
