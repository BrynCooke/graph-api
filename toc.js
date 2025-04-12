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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">User Guide</li><li class="chapter-item expanded "><a href="user_guide/getting_started.html"><strong aria-hidden="true">1.</strong> Getting Started</a></li><li class="chapter-item expanded "><a href="user_guide/defining_a_model.html"><strong aria-hidden="true">2.</strong> Defining a Model</a></li><li class="chapter-item expanded "><a href="user_guide/property_graphs.html"><strong aria-hidden="true">3.</strong> Property Graphs</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user_guide/property_graphs/no_index.html"><strong aria-hidden="true">3.1.</strong> Exploring Without Indexes</a></li><li class="chapter-item expanded "><a href="user_guide/property_graphs/hash_index.html"><strong aria-hidden="true">3.2.</strong> Hash Indexes</a></li><li class="chapter-item expanded "><a href="user_guide/property_graphs/range_index.html"><strong aria-hidden="true">3.3.</strong> Range Indexes</a></li><li class="chapter-item expanded "><a href="user_guide/property_graphs/full_text_index.html"><strong aria-hidden="true">3.4.</strong> Full-text Indexes</a></li></ol></li><li class="chapter-item expanded "><a href="user_guide/derive_macros.html"><strong aria-hidden="true">4.</strong> Derive Macros</a></li><li class="chapter-item expanded "><a href="user_guide/basic_operations.html"><strong aria-hidden="true">5.</strong> Basic Operations</a></li><li class="chapter-item expanded "><a href="user_guide/traversal.html"><strong aria-hidden="true">6.</strong> Graph Traversal</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user_guide/walker/overview.html"><strong aria-hidden="true">6.1.</strong> Walker Overview</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps.html"><strong aria-hidden="true">6.2.</strong> Walker Steps</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="user_guide/walker/steps/vertices.html"><strong aria-hidden="true">6.2.1.</strong> vertices</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/vertices_by_id.html"><strong aria-hidden="true">6.2.2.</strong> vertices_by_id</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/edges.html"><strong aria-hidden="true">6.2.3.</strong> edges</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/head.html"><strong aria-hidden="true">6.2.4.</strong> head</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/tail.html"><strong aria-hidden="true">6.2.5.</strong> tail</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/filter.html"><strong aria-hidden="true">6.2.6.</strong> filter</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/control_flow.html"><strong aria-hidden="true">6.2.7.</strong> control_flow</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/map.html"><strong aria-hidden="true">6.2.8.</strong> map</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/fold.html"><strong aria-hidden="true">6.2.9.</strong> fold</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/reduce.html"><strong aria-hidden="true">6.2.10.</strong> reduce</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/take.html"><strong aria-hidden="true">6.2.11.</strong> take</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/first.html"><strong aria-hidden="true">6.2.12.</strong> first</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/context.html"><strong aria-hidden="true">6.2.13.</strong> context</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/default_context.html"><strong aria-hidden="true">6.2.14.</strong> default_context</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/mutate_context.html"><strong aria-hidden="true">6.2.15.</strong> mutate_context</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/detour.html"><strong aria-hidden="true">6.2.16.</strong> detour</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/collect.html"><strong aria-hidden="true">6.2.17.</strong> collect</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/count.html"><strong aria-hidden="true">6.2.18.</strong> count</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/into_iter.html"><strong aria-hidden="true">6.2.19.</strong> into_iter</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/probe.html"><strong aria-hidden="true">6.2.20.</strong> probe</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/mutate.html"><strong aria-hidden="true">6.2.21.</strong> mutate</a></li><li class="chapter-item expanded "><a href="user_guide/walker/steps/dbg.html"><strong aria-hidden="true">6.2.22.</strong> dbg</a></li></ol></li><li class="chapter-item expanded "><a href="user_guide/walker/context_system.html"><strong aria-hidden="true">6.3.</strong> Context System</a></li><li class="chapter-item expanded "><a href="user_guide/walker/best_practices.html"><strong aria-hidden="true">6.4.</strong> Best Practices</a></li></ol></li><li class="chapter-item expanded "><li class="part-title">Implementation Guide</li><li class="chapter-item expanded "><a href="implementation/guide.html"><strong aria-hidden="true">7.</strong> Implementation Guide for Graph Backends</a></li><li class="chapter-item expanded "><a href="implementation/graphs.html"><strong aria-hidden="true">8.</strong> Creating a Graph Implementation</a></li><li class="chapter-item expanded "><a href="implementation/testing.html"><strong aria-hidden="true">9.</strong> Testing Your Implementation</a></li><li class="chapter-item expanded "><a href="implementation/features.html"><strong aria-hidden="true">10.</strong> Features and Extensions</a></li><li class="chapter-item expanded "><a href="implementation/indexes.html"><strong aria-hidden="true">11.</strong> Implementing Indexes</a></li><li class="chapter-item expanded "><a href="implementation/benchmarks.html"><strong aria-hidden="true">12.</strong> Benchmarking</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference</li><li class="chapter-item expanded "><a href="reference/api.html"><strong aria-hidden="true">13.</strong> API Reference</a></li><li class="chapter-item expanded "><a href="reference/support_traits.html"><strong aria-hidden="true">14.</strong> Support Traits</a></li><li class="chapter-item expanded "><a href="reference/derive_macros.html"><strong aria-hidden="true">15.</strong> Derive Macros</a></li><li class="chapter-item expanded "><a href="reference/implementations.html"><strong aria-hidden="true">16.</strong> Existing Implementations</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="reference/implementations/simple_graph.html"><strong aria-hidden="true">16.1.</strong> SimpleGraph</a></li><li class="chapter-item expanded "><a href="reference/implementations/pet_graph.html"><strong aria-hidden="true">16.2.</strong> PetGraph</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
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
