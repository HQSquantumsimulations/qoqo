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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="circuits/intro.html"><strong aria-hidden="true">2.</strong> Quantum Circuits</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="circuits/unitary.html"><strong aria-hidden="true">2.1.</strong> Unitary Operations</a></li><li class="chapter-item expanded "><a href="circuits/readout.html"><strong aria-hidden="true">2.2.</strong> Readout</a></li><li class="chapter-item expanded "><a href="circuits/pragma.html"><strong aria-hidden="true">2.3.</strong> Pragma Operations</a></li><li class="chapter-item expanded "><a href="circuits/noise.html"><strong aria-hidden="true">2.4.</strong> Noise Operations</a></li></ol></li><li class="chapter-item expanded "><a href="high-level/intro.html"><strong aria-hidden="true">3.</strong> High-Level Interface: Quantum Programs</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="high-level/pauliz.html"><strong aria-hidden="true">3.1.</strong> PauliZProduct Measurement</a></li><li class="chapter-item expanded "><a href="high-level/program.html"><strong aria-hidden="true">3.2.</strong> QuantumProgram and Variable-Replacement</a></li><li class="chapter-item expanded "><a href="high-level/classical.html"><strong aria-hidden="true">3.3.</strong> ClassicalRegister Measurement</a></li><li class="chapter-item expanded "><a href="high-level/pauliz_cheated.html"><strong aria-hidden="true">3.4.</strong> CheatedPauliZProduct Measurement</a></li><li class="chapter-item expanded "><a href="high-level/cheated.html"><strong aria-hidden="true">3.5.</strong> Cheated Measurement</a></li></ol></li><li class="chapter-item expanded "><a href="backends.html"><strong aria-hidden="true">4.</strong> Backends</a></li><li class="chapter-item expanded "><a href="devices.html"><strong aria-hidden="true">5.</strong> Devices</a></li><li class="chapter-item expanded "><a href="conventions.html"><strong aria-hidden="true">6.</strong> Conventions</a></li><li class="chapter-item expanded "><a href="gate_operations/intro.html"><strong aria-hidden="true">7.</strong> List of Gate Operations</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="gate_operations/single_qubit_gates.html"><strong aria-hidden="true">7.1.</strong> Single-Qubit Gates</a></li><li class="chapter-item expanded "><a href="gate_operations/two_qubit_gates.html"><strong aria-hidden="true">7.2.</strong> Two-Qubit Gates</a></li><li class="chapter-item expanded "><a href="gate_operations/multi_qubit_gates.html"><strong aria-hidden="true">7.3.</strong> Multi-Qubit Gates</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
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
