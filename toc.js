// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="introduction/_index.html"><strong aria-hidden="true">1.</strong> Introduction</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="introduction/why-rust.html"><strong aria-hidden="true">1.1.</strong> Why use Rust with Python?</a></li><li class="chapter-item "><a href="introduction/learning.html"><strong aria-hidden="true">1.2.</strong> Learning a new language</a></li><li class="chapter-item "><a href="introduction/how-to-read.html"><strong aria-hidden="true">1.3.</strong> How to read this book</a></li></ol></li><li class="chapter-item expanded "><a href="setup.html"><strong aria-hidden="true">2.</strong> Setup &amp; installation</a></li><li class="chapter-item expanded "><a href="pieces/_index.html"><strong aria-hidden="true">3.</strong> Pieces</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="pieces/hello-world.html"><strong aria-hidden="true">3.1.</strong> Hello world!</a></li><li class="chapter-item "><a href="pieces/intro/_index.html"><strong aria-hidden="true">3.2.</strong> Introduction</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="pieces/intro/protocols_traits.html"><strong aria-hidden="true">3.2.1.</strong> Protocols vs. traits</a></li><li class="chapter-item "><a href="pieces/intro/enumerate.html"><strong aria-hidden="true">3.2.2.</strong> Enumerate</a></li><li class="chapter-item "><a href="pieces/intro/zip.html"><strong aria-hidden="true">3.2.3.</strong> Zip</a></li><li class="chapter-item "><a href="pieces/intro/tuple_unpacking.html"><strong aria-hidden="true">3.2.4.</strong> Tuple unpacking</a></li><li class="chapter-item "><a href="pieces/intro/lambdas_vs_closures.html"><strong aria-hidden="true">3.2.5.</strong> Lambdas vs. closures</a></li><li class="chapter-item "><a href="pieces/intro/single_line_if_else.html"><strong aria-hidden="true">3.2.6.</strong> Single-line if-else</a></li><li class="chapter-item "><a href="pieces/intro/list_comprehensions_vs_map.html"><strong aria-hidden="true">3.2.7.</strong> List comprehensions vs. map/filter</a></li><li class="chapter-item "><a href="pieces/intro/dicts_vs_hashmaps.html"><strong aria-hidden="true">3.2.8.</strong> Dicts vs. HashMaps</a></li><li class="chapter-item "><a href="pieces/intro/sets_vs_hashsets.html"><strong aria-hidden="true">3.2.9.</strong> Sets vs. HashSets</a></li></ol></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded "><a href="CONTRIBUTORS.html"><strong aria-hidden="true">4.</strong> Contributors</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
