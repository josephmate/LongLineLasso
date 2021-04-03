<?xml version="1.0" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd"><svg version="1.1" width="1200" height="710" onload="init(evt)" viewBox="0 0 1200 710" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><!--Flame graph stack visualization. See https://github.com/brendangregg/FlameGraph for latest version, and http://www.brendangregg.com/flamegraphs.html for examples.--><!--NOTES: --><defs><linearGradient id="background" y1="0" y2="1" x1="0" x2="0"><stop stop-color="#eeeeee" offset="5%"/><stop stop-color="#eeeeb0" offset="95%"/></linearGradient></defs><style type="text/css">
text { font-family:"Verdana"; font-size:12px; fill:rgb(0,0,0); }
#title { text-anchor:middle; font-size:17px; }
#search { opacity:0.1; cursor:pointer; }
#search:hover, #search.show { opacity:1; }
#subtitle { text-anchor:middle; font-color:rgb(160,160,160); }
#unzoom { cursor:pointer; }
#frames > *:hover { stroke:black; stroke-width:0.5; cursor:pointer; }
.hide { display:none; }
.parent { opacity:0.5; }
</style><script type="text/ecmascript"><![CDATA[var nametype = 'Function:';
var fontsize = 12;
var fontwidth = 0.59;
var xpad = 10;
var inverted = false;
var searchcolor = 'rgb(230,0,230)';
var fluiddrawing = true;
var truncate_text_right = false;]]><![CDATA["use strict";
var details, searchbtn, unzoombtn, matchedtxt, svg, searching, frames;
function init(evt) {
    details = document.getElementById("details").firstChild;
    searchbtn = document.getElementById("search");
    unzoombtn = document.getElementById("unzoom");
    matchedtxt = document.getElementById("matched");
    svg = document.getElementsByTagName("svg")[0];
    frames = document.getElementById("frames");
    searching = 0;

    // Use GET parameters to restore a flamegraph's state.
    var restore_state = function() {
        var params = get_params();
        if (params.x && params.y)
            zoom(find_group(document.querySelector('[x="' + params.x + '"][y="' + params.y + '"]')));
        if (params.s)
            search(params.s);
    };

    if (fluiddrawing) {
        // Make width dynamic so the SVG fits its parent's width.
        svg.removeAttribute("width");
        // Edge requires us to have a viewBox that gets updated with size changes.
        var isEdge = /Edge\/\d./i.test(navigator.userAgent);
        if (!isEdge) {
          svg.removeAttribute("viewBox");
        }
        var update_for_width_change = function() {
            if (isEdge) {
                svg.attributes.viewBox.value = "0 0 " + svg.width.baseVal.value + " " + svg.height.baseVal.value;
            }

            // Keep consistent padding on left and right of frames container.
            frames.attributes.width.value = svg.width.baseVal.value - xpad * 2;

            // Text truncation needs to be adjusted for the current width.
            var el = frames.children;
            for(var i = 0; i < el.length; i++) {
                update_text(el[i]);
            }

            // Keep search elements at a fixed distance from right edge.
            var svgWidth = svg.width.baseVal.value;
            searchbtn.attributes.x.value = svgWidth - xpad - 100;
            matchedtxt.attributes.x.value = svgWidth - xpad - 100;
        };
        window.addEventListener('resize', function() {
            update_for_width_change();
        });
        // This needs to be done asynchronously for Safari to work.
        setTimeout(function() {
            unzoom();
            update_for_width_change();
            restore_state();
        }, 0);
    } else {
        restore_state();
    }
}
// event listeners
window.addEventListener("click", function(e) {
    var target = find_group(e.target);
    if (target) {
        if (target.nodeName == "a") {
            if (e.ctrlKey === false) return;
            e.preventDefault();
        }
        if (target.classList.contains("parent")) unzoom();
        zoom(target);

        // set parameters for zoom state
        var el = target.querySelector("rect");
        if (el && el.attributes && el.attributes.y && el.attributes._orig_x) {
            var params = get_params()
            params.x = el.attributes._orig_x.value;
            params.y = el.attributes.y.value;
            history.replaceState(null, null, parse_params(params));
        }
    }
    else if (e.target.id == "unzoom") {
        unzoom();

        // remove zoom state
        var params = get_params();
        if (params.x) delete params.x;
        if (params.y) delete params.y;
        history.replaceState(null, null, parse_params(params));
    }
    else if (e.target.id == "search") search_prompt();
}, false)
// mouse-over for info
// show
window.addEventListener("mouseover", function(e) {
    var target = find_group(e.target);
    if (target) details.nodeValue = nametype + " " + g_to_text(target);
}, false)
// clear
window.addEventListener("mouseout", function(e) {
    var target = find_group(e.target);
    if (target) details.nodeValue = ' ';
}, false)
// ctrl-F for search
window.addEventListener("keydown",function (e) {
    if (e.keyCode === 114 || (e.ctrlKey && e.keyCode === 70)) {
        e.preventDefault();
        search_prompt();
    }
}, false)
// functions
function get_params() {
    var params = {};
    var paramsarr = window.location.search.substr(1).split('&');
    for (var i = 0; i < paramsarr.length; ++i) {
        var tmp = paramsarr[i].split("=");
        if (!tmp[0] || !tmp[1]) continue;
        params[tmp[0]]  = decodeURIComponent(tmp[1]);
    }
    return params;
}
function parse_params(params) {
    var uri = "?";
    for (var key in params) {
        uri += key + '=' + encodeURIComponent(params[key]) + '&';
    }
    if (uri.slice(-1) == "&")
        uri = uri.substring(0, uri.length - 1);
    if (uri == '?')
        uri = window.location.href.split('?')[0];
    return uri;
}
function find_child(node, selector) {
    var children = node.querySelectorAll(selector);
    if (children.length) return children[0];
    return;
}
function find_group(node) {
    var parent = node.parentElement;
    if (!parent) return;
    if (parent.id == "frames") return node;
    return find_group(parent);
}
function orig_save(e, attr, val) {
    if (e.attributes["_orig_" + attr] != undefined) return;
    if (e.attributes[attr] == undefined) return;
    if (val == undefined) val = e.attributes[attr].value;
    e.setAttribute("_orig_" + attr, val);
}
function orig_load(e, attr) {
    if (e.attributes["_orig_"+attr] == undefined) return;
    e.attributes[attr].value = e.attributes["_orig_" + attr].value;
    e.removeAttribute("_orig_" + attr);
}
function g_to_text(e) {
    var text = find_child(e, "title").firstChild.nodeValue;
    return (text)
}
function g_to_func(e) {
    var func = g_to_text(e);
    // if there's any manipulation we want to do to the function
    // name before it's searched, do it here before returning.
    return (func);
}
function update_text(e) {
    var r = find_child(e, "rect");
    var t = find_child(e, "text");
    var w = parseFloat(r.attributes.width.value) * frames.attributes.width.value / 100 - 3;
    var txt = find_child(e, "title").textContent.replace(/\([^(]*\)$/,"");
    t.attributes.x.value = format_percent((parseFloat(r.attributes.x.value) + (100 * 3 / frames.attributes.width.value)));
    // Smaller than this size won't fit anything
    if (w < 2 * fontsize * fontwidth) {
        t.textContent = "";
        return;
    }
    t.textContent = txt;
    // Fit in full text width
    if (/^ *\$/.test(txt) || t.getComputedTextLength() < w)
        return;
    if (truncate_text_right) {
        // Truncate the right side of the text.
        for (var x = txt.length - 2; x > 0; x--) {
            if (t.getSubStringLength(0, x + 2) <= w) {
                t.textContent = txt.substring(0, x) + "..";
                return;
            }
        }
    } else {
        // Truncate the left side of the text.
        for (var x = 2; x < txt.length; x++) {
            if (t.getSubStringLength(x - 2, txt.length) <= w) {
                t.textContent = ".." + txt.substring(x, txt.length);
                return;
            }
        }
    }
    t.textContent = "";
}
// zoom
function zoom_reset(e) {
    if (e.attributes != undefined) {
        orig_load(e, "x");
        orig_load(e, "width");
    }
    if (e.childNodes == undefined) return;
    for(var i = 0, c = e.childNodes; i < c.length; i++) {
        zoom_reset(c[i]);
    }
}
function zoom_child(e, x, ratio) {
    if (e.attributes != undefined) {
        if (e.attributes.x != undefined) {
            orig_save(e, "x");
            e.attributes.x.value = format_percent((parseFloat(e.attributes.x.value) - x) * ratio);
            if (e.tagName == "text") {
                e.attributes.x.value = format_percent(parseFloat(find_child(e.parentNode, "rect[x]").attributes.x.value) + (100 * 3 / frames.attributes.width.value));
            }
        }
        if (e.attributes.width != undefined) {
            orig_save(e, "width");
            e.attributes.width.value = format_percent(parseFloat(e.attributes.width.value) * ratio);
        }
    }
    if (e.childNodes == undefined) return;
    for(var i = 0, c = e.childNodes; i < c.length; i++) {
        zoom_child(c[i], x, ratio);
    }
}
function zoom_parent(e) {
    if (e.attributes) {
        if (e.attributes.x != undefined) {
            orig_save(e, "x");
            e.attributes.x.value = "0.0%";
        }
        if (e.attributes.width != undefined) {
            orig_save(e, "width");
            e.attributes.width.value = "100.0%";
        }
    }
    if (e.childNodes == undefined) return;
    for(var i = 0, c = e.childNodes; i < c.length; i++) {
        zoom_parent(c[i]);
    }
}
function zoom(node) {
    var attr = find_child(node, "rect").attributes;
    var width = parseFloat(attr.width.value);
    var xmin = parseFloat(attr.x.value);
    var xmax = xmin + width;
    var ymin = parseFloat(attr.y.value);
    var ratio = 100 / width;
    // XXX: Workaround for JavaScript float issues (fix me)
    var fudge = 0.001;
    unzoombtn.classList.remove("hide");
    var el = frames.children;
    for (var i = 0; i < el.length; i++) {
        var e = el[i];
        var a = find_child(e, "rect").attributes;
        var ex = parseFloat(a.x.value);
        var ew = parseFloat(a.width.value);
        // Is it an ancestor
        if (!inverted) {
            var upstack = parseFloat(a.y.value) > ymin;
        } else {
            var upstack = parseFloat(a.y.value) < ymin;
        }
        if (upstack) {
            // Direct ancestor
            if (ex <= xmin && (ex+ew+fudge) >= xmax) {
                e.classList.add("parent");
                zoom_parent(e);
                update_text(e);
            }
            // not in current path
            else
                e.classList.add("hide");
        }
        // Children maybe
        else {
            // no common path
            if (ex < xmin || ex + fudge >= xmax) {
                e.classList.add("hide");
            }
            else {
                zoom_child(e, xmin, ratio);
                update_text(e);
            }
        }
    }
}
function unzoom() {
    unzoombtn.classList.add("hide");
    var el = frames.children;
    for(var i = 0; i < el.length; i++) {
        el[i].classList.remove("parent");
        el[i].classList.remove("hide");
        zoom_reset(el[i]);
        update_text(el[i]);
    }
}
// search
function reset_search() {
    var el = document.querySelectorAll("#frames rect");
    for (var i = 0; i < el.length; i++) {
        orig_load(el[i], "fill")
    }
    var params = get_params();
    delete params.s;
    history.replaceState(null, null, parse_params(params));
}
function search_prompt() {
    if (!searching) {
        var term = prompt("Enter a search term (regexp " +
            "allowed, eg: ^ext4_)", "");
        if (term != null) {
            search(term)
        }
    } else {
        reset_search();
        searching = 0;
        searchbtn.classList.remove("show");
        searchbtn.firstChild.nodeValue = "Search"
        matchedtxt.classList.add("hide");
        matchedtxt.firstChild.nodeValue = ""
    }
}
function search(term) {
    var re = new RegExp(term);
    var el = frames.children;
    var matches = new Object();
    var maxwidth = 0;
    for (var i = 0; i < el.length; i++) {
        var e = el[i];
        var func = g_to_func(e);
        var rect = find_child(e, "rect");
        if (func == null || rect == null)
            continue;
        // Save max width. Only works as we have a root frame
        var w = parseFloat(rect.attributes.width.value);
        if (w > maxwidth)
            maxwidth = w;
        if (func.match(re)) {
            // highlight
            var x = parseFloat(rect.attributes.x.value);
            orig_save(rect, "fill");
            rect.attributes.fill.value = searchcolor;
            // remember matches
            if (matches[x] == undefined) {
                matches[x] = w;
            } else {
                if (w > matches[x]) {
                    // overwrite with parent
                    matches[x] = w;
                }
            }
            searching = 1;
        }
    }
    if (!searching)
        return;
    var params = get_params();
    params.s = term;
    history.replaceState(null, null, parse_params(params));

    searchbtn.classList.add("show");
    searchbtn.firstChild.nodeValue = "Reset Search";
    // calculate percent matched, excluding vertical overlap
    var count = 0;
    var lastx = -1;
    var lastw = 0;
    var keys = Array();
    for (k in matches) {
        if (matches.hasOwnProperty(k))
            keys.push(k);
    }
    // sort the matched frames by their x location
    // ascending, then width descending
    keys.sort(function(a, b){
        return a - b;
    });
    // Step through frames saving only the biggest bottom-up frames
    // thanks to the sort order. This relies on the tree property
    // where children are always smaller than their parents.
    var fudge = 0.0001;    // JavaScript floating point
    for (var k in keys) {
        var x = parseFloat(keys[k]);
        var w = matches[keys[k]];
        if (x >= lastx + lastw - fudge) {
            count += w;
            lastx = x;
            lastw = w;
        }
    }
    // display matched percent
    matchedtxt.classList.remove("hide");
    var pct = 100 * count / maxwidth;
    if (pct != 100) pct = pct.toFixed(1);
    matchedtxt.firstChild.nodeValue = "Matched: " + pct + "%";
}
function format_percent(n) {
    return n.toFixed(4) + "%";
}
]]></script><rect x="0" y="0" width="100%" height="710" fill="url(#background)"/><text id="title" x="50.0000%" y="24.00">Flame Graph</text><text id="details" x="10" y="693.00"> </text><text id="unzoom" class="hide" x="10" y="24.00">Reset Zoom</text><text id="search" x="1090" y="24.00">Search</text><text id="matched" x="1090" y="693.00"> </text><svg id="frames" x="10" width="1180"><g><title>&lt;alloc::boxed::Box&lt;I&gt; as core::iter::traits::iterator::Iterator&gt;::next (2,500 samples, 1.29%)</title><rect x="45.6151%" y="581" width="1.2909%" height="15" fill="rgb(227,0,7)"/><text x="45.8651%" y="591.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="46.8957%" y="565" width="0.0103%" height="15" fill="rgb(217,0,24)"/><text x="47.1457%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="46.8957%" y="549" width="0.0103%" height="15" fill="rgb(221,193,54)"/><text x="47.1457%" y="559.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="46.8957%" y="533" width="0.0103%" height="15" fill="rgb(248,212,6)"/><text x="47.1457%" y="543.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="46.8957%" y="517" width="0.0103%" height="15" fill="rgb(208,68,35)"/><text x="47.1457%" y="527.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="46.8957%" y="501" width="0.0103%" height="15" fill="rgb(232,128,0)"/><text x="47.1457%" y="511.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="46.8957%" y="485" width="0.0103%" height="15" fill="rgb(207,160,47)"/><text x="47.1457%" y="495.50"></text></g><g><title>&lt;std::io::buffered::BufReader&lt;R&gt; as std::io::BufRead&gt;::consume (1,998 samples, 1.03%)</title><rect x="59.4425%" y="533" width="1.0317%" height="15" fill="rgb(228,23,34)"/><text x="59.6925%" y="543.50"></text></g><g><title>__GI___libc_read (1,138 samples, 0.59%)</title><rect x="61.5358%" y="469" width="0.5876%" height="15" fill="rgb(218,30,26)"/><text x="61.7858%" y="479.50"></text></g><g><title>[[kernel.kallsyms]] (1,084 samples, 0.56%)</title><rect x="61.5637%" y="453" width="0.5597%" height="15" fill="rgb(220,122,19)"/><text x="61.8137%" y="463.50"></text></g><g><title>[[kernel.kallsyms]] (1,061 samples, 0.55%)</title><rect x="61.5756%" y="437" width="0.5479%" height="15" fill="rgb(250,228,42)"/><text x="61.8256%" y="447.50"></text></g><g><title>[[kernel.kallsyms]] (1,042 samples, 0.54%)</title><rect x="61.5854%" y="421" width="0.5380%" height="15" fill="rgb(240,193,28)"/><text x="61.8354%" y="431.50"></text></g><g><title>[[kernel.kallsyms]] (1,040 samples, 0.54%)</title><rect x="61.5864%" y="405" width="0.5370%" height="15" fill="rgb(216,20,37)"/><text x="61.8364%" y="415.50"></text></g><g><title>[[kernel.kallsyms]] (1,026 samples, 0.53%)</title><rect x="61.5937%" y="389" width="0.5298%" height="15" fill="rgb(206,188,39)"/><text x="61.8437%" y="399.50"></text></g><g><title>[[kernel.kallsyms]] (988 samples, 0.51%)</title><rect x="61.6133%" y="373" width="0.5102%" height="15" fill="rgb(217,207,13)"/><text x="61.8633%" y="383.50"></text></g><g><title>[[kernel.kallsyms]] (966 samples, 0.50%)</title><rect x="61.6247%" y="357" width="0.4988%" height="15" fill="rgb(231,73,38)"/><text x="61.8747%" y="367.50"></text></g><g><title>[[kernel.kallsyms]] (933 samples, 0.48%)</title><rect x="61.6417%" y="341" width="0.4818%" height="15" fill="rgb(225,20,46)"/><text x="61.8917%" y="351.50"></text></g><g><title>[[kernel.kallsyms]] (909 samples, 0.47%)</title><rect x="61.6541%" y="325" width="0.4694%" height="15" fill="rgb(210,31,41)"/><text x="61.9041%" y="335.50"></text></g><g><title>[[kernel.kallsyms]] (885 samples, 0.46%)</title><rect x="61.6665%" y="309" width="0.4570%" height="15" fill="rgb(221,200,47)"/><text x="61.9165%" y="319.50"></text></g><g><title>[[kernel.kallsyms]] (833 samples, 0.43%)</title><rect x="61.6933%" y="293" width="0.4301%" height="15" fill="rgb(226,26,5)"/><text x="61.9433%" y="303.50"></text></g><g><title>[[kernel.kallsyms]] (797 samples, 0.41%)</title><rect x="61.7119%" y="277" width="0.4115%" height="15" fill="rgb(249,33,26)"/><text x="61.9619%" y="287.50"></text></g><g><title>[[kernel.kallsyms]] (62 samples, 0.03%)</title><rect x="62.0914%" y="261" width="0.0320%" height="15" fill="rgb(235,183,28)"/><text x="62.3414%" y="271.50"></text></g><g><title>&lt;std::io::stdio::StdinRaw as std::io::Read&gt;::read (1,141 samples, 0.59%)</title><rect x="61.5348%" y="517" width="0.5892%" height="15" fill="rgb(221,5,38)"/><text x="61.7848%" y="527.50"></text></g><g><title>&lt;std::sys::unix::stdio::Stdin as std::io::Read&gt;::read (1,141 samples, 0.59%)</title><rect x="61.5348%" y="501" width="0.5892%" height="15" fill="rgb(247,18,42)"/><text x="61.7848%" y="511.50"></text></g><g><title>std::sys::unix::fd::FileDesc::read (1,141 samples, 0.59%)</title><rect x="61.5348%" y="485" width="0.5892%" height="15" fill="rgb(241,131,45)"/><text x="61.7848%" y="495.50"></text></g><g><title>&lt;std::io::buffered::BufReader&lt;R&gt; as std::io::BufRead&gt;::fill_buf (4,240 samples, 2.19%)</title><rect x="60.4742%" y="533" width="2.1893%" height="15" fill="rgb(249,31,29)"/><text x="60.7242%" y="543.50">&lt;..</text></g><g><title>core::slice::&lt;impl core::ops::index::Index&lt;I&gt; for [T]&gt;::index (1,039 samples, 0.54%)</title><rect x="62.1271%" y="517" width="0.5365%" height="15" fill="rgb(225,111,53)"/><text x="62.3771%" y="527.50"></text></g><g><title>&lt;core::ops::range::Range&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::index (1,039 samples, 0.54%)</title><rect x="62.1271%" y="501" width="0.5365%" height="15" fill="rgb(238,160,17)"/><text x="62.3771%" y="511.50"></text></g><g><title>&lt;core::ops::range::Range&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::get_unchecked (1,037 samples, 0.54%)</title><rect x="62.1281%" y="485" width="0.5355%" height="15" fill="rgb(214,148,48)"/><text x="62.3781%" y="495.50"></text></g><g><title>core::ptr::const_ptr::&lt;impl *const T&gt;::add (1,037 samples, 0.54%)</title><rect x="62.1281%" y="469" width="0.5355%" height="15" fill="rgb(232,36,49)"/><text x="62.3781%" y="479.50"></text></g><g><title>core::ptr::const_ptr::&lt;impl *const T&gt;::offset (1,037 samples, 0.54%)</title><rect x="62.1281%" y="453" width="0.5355%" height="15" fill="rgb(209,103,24)"/><text x="62.3781%" y="463.50"></text></g><g><title>[[kernel.kallsyms]] (29 samples, 0.01%)</title><rect x="62.6636%" y="533" width="0.0150%" height="15" fill="rgb(229,88,8)"/><text x="62.9136%" y="543.50"></text></g><g><title>[[kernel.kallsyms]] (26 samples, 0.01%)</title><rect x="62.6651%" y="517" width="0.0134%" height="15" fill="rgb(213,181,19)"/><text x="62.9151%" y="527.50"></text></g><g><title>[[kernel.kallsyms]] (26 samples, 0.01%)</title><rect x="62.6651%" y="501" width="0.0134%" height="15" fill="rgb(254,191,54)"/><text x="62.9151%" y="511.50"></text></g><g><title>[[kernel.kallsyms]] (25 samples, 0.01%)</title><rect x="62.6656%" y="485" width="0.0129%" height="15" fill="rgb(241,83,37)"/><text x="62.9156%" y="495.50"></text></g><g><title>[[kernel.kallsyms]] (24 samples, 0.01%)</title><rect x="62.6661%" y="469" width="0.0124%" height="15" fill="rgb(233,36,39)"/><text x="62.9161%" y="479.50"></text></g><g><title>[[kernel.kallsyms]] (24 samples, 0.01%)</title><rect x="62.6661%" y="453" width="0.0124%" height="15" fill="rgb(226,3,54)"/><text x="62.9161%" y="463.50"></text></g><g><title>&lt;std::io::buffered::BufReader&lt;R&gt; as std::io::Read&gt;::read (21,378 samples, 11.04%)</title><rect x="53.1823%" y="549" width="11.0386%" height="15" fill="rgb(245,192,40)"/><text x="53.4323%" y="559.50">&lt;std::io::buffer..</text></g><g><title>std::io::impls::&lt;impl std::io::Read for &amp;[u8]&gt;::read (2,987 samples, 1.54%)</title><rect x="62.6785%" y="533" width="1.5423%" height="15" fill="rgb(238,167,29)"/><text x="62.9285%" y="543.50"></text></g><g><title>&lt;std::io::stdio::StdinLock as std::io::Read&gt;::read (24,495 samples, 12.65%)</title><rect x="51.5764%" y="565" width="12.6481%" height="15" fill="rgb(232,182,51)"/><text x="51.8264%" y="575.50">&lt;std::io::stdio::St..</text></g><g><title>[[kernel.kallsyms]] (21 samples, 0.01%)</title><rect x="64.2276%" y="469" width="0.0108%" height="15" fill="rgb(231,60,39)"/><text x="64.4776%" y="479.50"></text></g><g><title>&lt;lll::string_stream_editor::CharIterator as core::iter::traits::iterator::Iterator&gt;::next (33,571 samples, 17.33%)</title><rect x="46.9060%" y="581" width="17.3345%" height="15" fill="rgb(208,69,12)"/><text x="47.1560%" y="591.50">&lt;lll::string_stream_editor:..</text></g><g><title>[[kernel.kallsyms]] (31 samples, 0.02%)</title><rect x="64.2245%" y="565" width="0.0160%" height="15" fill="rgb(235,93,37)"/><text x="64.4745%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (31 samples, 0.02%)</title><rect x="64.2245%" y="549" width="0.0160%" height="15" fill="rgb(213,116,39)"/><text x="64.4745%" y="559.50"></text></g><g><title>[[kernel.kallsyms]] (29 samples, 0.01%)</title><rect x="64.2255%" y="533" width="0.0150%" height="15" fill="rgb(222,207,29)"/><text x="64.4755%" y="543.50"></text></g><g><title>[[kernel.kallsyms]] (28 samples, 0.01%)</title><rect x="64.2260%" y="517" width="0.0145%" height="15" fill="rgb(206,96,30)"/><text x="64.4760%" y="527.50"></text></g><g><title>[[kernel.kallsyms]] (28 samples, 0.01%)</title><rect x="64.2260%" y="501" width="0.0145%" height="15" fill="rgb(218,138,4)"/><text x="64.4760%" y="511.50"></text></g><g><title>[[kernel.kallsyms]] (27 samples, 0.01%)</title><rect x="64.2266%" y="485" width="0.0139%" height="15" fill="rgb(250,191,14)"/><text x="64.4766%" y="495.50"></text></g><g><title>[[kernel.kallsyms]] (49 samples, 0.03%)</title><rect x="64.2513%" y="485" width="0.0253%" height="15" fill="rgb(239,60,40)"/><text x="64.5013%" y="495.50"></text></g><g><title>[[kernel.kallsyms]] (33 samples, 0.02%)</title><rect x="64.2596%" y="469" width="0.0170%" height="15" fill="rgb(206,27,48)"/><text x="64.5096%" y="479.50"></text></g><g><title>[[kernel.kallsyms]] (28 samples, 0.01%)</title><rect x="64.2622%" y="453" width="0.0145%" height="15" fill="rgb(225,35,8)"/><text x="64.5122%" y="463.50"></text></g><g><title>[[kernel.kallsyms]] (28 samples, 0.01%)</title><rect x="64.2622%" y="437" width="0.0145%" height="15" fill="rgb(250,213,24)"/><text x="64.5122%" y="447.50"></text></g><g><title>[[kernel.kallsyms]] (82 samples, 0.04%)</title><rect x="64.2405%" y="581" width="0.0423%" height="15" fill="rgb(247,123,22)"/><text x="64.4905%" y="591.50"></text></g><g><title>[[kernel.kallsyms]] (70 samples, 0.04%)</title><rect x="64.2467%" y="565" width="0.0361%" height="15" fill="rgb(231,138,38)"/><text x="64.4967%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (69 samples, 0.04%)</title><rect x="64.2472%" y="549" width="0.0356%" height="15" fill="rgb(231,145,46)"/><text x="64.4972%" y="559.50"></text></g><g><title>[[kernel.kallsyms]] (67 samples, 0.03%)</title><rect x="64.2482%" y="533" width="0.0346%" height="15" fill="rgb(251,118,11)"/><text x="64.4982%" y="543.50"></text></g><g><title>[[kernel.kallsyms]] (64 samples, 0.03%)</title><rect x="64.2498%" y="517" width="0.0330%" height="15" fill="rgb(217,147,25)"/><text x="64.4998%" y="527.50"></text></g><g><title>[[kernel.kallsyms]] (64 samples, 0.03%)</title><rect x="64.2498%" y="501" width="0.0330%" height="15" fill="rgb(247,81,37)"/><text x="64.4998%" y="511.50"></text></g><g><title>&lt;core::slice::Iter&lt;T&gt; as core::iter::traits::iterator::Iterator&gt;::next (1,237 samples, 0.64%)</title><rect x="78.3023%" y="533" width="0.6387%" height="15" fill="rgb(209,12,38)"/><text x="78.5523%" y="543.50"></text></g><g><title>&lt;core::str::Chars as core::iter::traits::iterator::Iterator&gt;::next (3,258 samples, 1.68%)</title><rect x="77.2614%" y="565" width="1.6823%" height="15" fill="rgb(227,1,9)"/><text x="77.5114%" y="575.50"></text></g><g><title>core::str::next_code_point (3,258 samples, 1.68%)</title><rect x="77.2614%" y="549" width="1.6823%" height="15" fill="rgb(248,47,43)"/><text x="77.5114%" y="559.50"></text></g><g><title>[[kernel.kallsyms]] (34 samples, 0.02%)</title><rect x="78.9602%" y="469" width="0.0176%" height="15" fill="rgb(221,10,30)"/><text x="79.2102%" y="479.50"></text></g><g><title>[[kernel.kallsyms]] (22 samples, 0.01%)</title><rect x="78.9664%" y="453" width="0.0114%" height="15" fill="rgb(210,229,1)"/><text x="79.2164%" y="463.50"></text></g><g><title>[[kernel.kallsyms]] (72 samples, 0.04%)</title><rect x="78.9467%" y="565" width="0.0372%" height="15" fill="rgb(222,148,37)"/><text x="79.1967%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (60 samples, 0.03%)</title><rect x="78.9529%" y="549" width="0.0310%" height="15" fill="rgb(234,67,33)"/><text x="79.2029%" y="559.50"></text></g><g><title>[[kernel.kallsyms]] (59 samples, 0.03%)</title><rect x="78.9535%" y="533" width="0.0305%" height="15" fill="rgb(247,98,35)"/><text x="79.2035%" y="543.50"></text></g><g><title>[[kernel.kallsyms]] (58 samples, 0.03%)</title><rect x="78.9540%" y="517" width="0.0299%" height="15" fill="rgb(247,138,52)"/><text x="79.2040%" y="527.50"></text></g><g><title>[[kernel.kallsyms]] (48 samples, 0.02%)</title><rect x="78.9591%" y="501" width="0.0248%" height="15" fill="rgb(213,79,30)"/><text x="79.2091%" y="511.50"></text></g><g><title>[[kernel.kallsyms]] (47 samples, 0.02%)</title><rect x="78.9597%" y="485" width="0.0243%" height="15" fill="rgb(246,177,23)"/><text x="79.2097%" y="495.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::len (2,063 samples, 1.07%)</title><rect x="78.9844%" y="565" width="1.0652%" height="15" fill="rgb(230,62,27)"/><text x="79.2344%" y="575.50"></text></g><g><title>alloc::collections::vec_deque::count (1,024 samples, 0.53%)</title><rect x="79.5209%" y="549" width="0.5287%" height="15" fill="rgb(216,154,8)"/><text x="79.7709%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::pop_back (1,675 samples, 0.86%)</title><rect x="80.0497%" y="565" width="0.8649%" height="15" fill="rgb(244,35,45)"/><text x="80.2997%" y="575.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::is_empty (1,058 samples, 0.55%)</title><rect x="80.3683%" y="549" width="0.5463%" height="15" fill="rgb(251,115,12)"/><text x="80.6183%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::ptr (1,031 samples, 0.53%)</title><rect x="83.0832%" y="533" width="0.5324%" height="15" fill="rgb(240,54,50)"/><text x="83.3332%" y="543.50"></text></g><g><title>alloc::raw_vec::RawVec&lt;T,A&gt;::ptr (1,031 samples, 0.53%)</title><rect x="83.0832%" y="517" width="0.5324%" height="15" fill="rgb(233,84,52)"/><text x="83.3332%" y="527.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::buffer_read (1,032 samples, 0.53%)</title><rect x="83.0832%" y="549" width="0.5329%" height="15" fill="rgb(207,117,47)"/><text x="83.3332%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::is_empty (926 samples, 0.48%)</title><rect x="83.6161%" y="549" width="0.4781%" height="15" fill="rgb(249,43,39)"/><text x="83.8661%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::pop_front (8,337 samples, 4.30%)</title><rect x="80.9146%" y="565" width="4.3048%" height="15" fill="rgb(209,38,44)"/><text x="81.1646%" y="575.50">alloc..</text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::wrap_add (2,179 samples, 1.13%)</title><rect x="84.0943%" y="549" width="1.1251%" height="15" fill="rgb(236,212,23)"/><text x="84.3443%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::wrap_index (2,179 samples, 1.13%)</title><rect x="84.0943%" y="533" width="1.1251%" height="15" fill="rgb(242,79,21)"/><text x="84.3443%" y="543.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::buffer_write (2,031 samples, 1.05%)</title><rect x="85.8994%" y="549" width="1.0487%" height="15" fill="rgb(211,96,35)"/><text x="86.1494%" y="559.50"></text></g><g><title>core::ptr::write (2,031 samples, 1.05%)</title><rect x="85.8994%" y="533" width="1.0487%" height="15" fill="rgb(253,215,40)"/><text x="86.1494%" y="543.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::is_full (6,230 samples, 3.22%)</title><rect x="86.9481%" y="549" width="3.2169%" height="15" fill="rgb(211,81,21)"/><text x="87.1981%" y="559.50">all..</text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::len (4,089 samples, 2.11%)</title><rect x="88.0537%" y="533" width="2.1114%" height="15" fill="rgb(208,190,38)"/><text x="88.3037%" y="543.50">a..</text></g><g><title>alloc::collections::vec_deque::count (4,089 samples, 2.11%)</title><rect x="88.0537%" y="517" width="2.1114%" height="15" fill="rgb(235,213,38)"/><text x="88.3037%" y="527.50">a..</text></g><g><title>core::num::&lt;impl usize&gt;::wrapping_sub (1,962 samples, 1.01%)</title><rect x="89.1519%" y="501" width="1.0131%" height="15" fill="rgb(237,122,38)"/><text x="89.4019%" y="511.50"></text></g><g><title>alloc::collections::vec_deque::wrap_index (307 samples, 0.16%)</title><rect x="90.1650%" y="533" width="0.1585%" height="15" fill="rgb(244,218,35)"/><text x="90.4150%" y="543.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::push_back (11,906 samples, 6.15%)</title><rect x="85.2194%" y="565" width="6.1477%" height="15" fill="rgb(240,68,47)"/><text x="85.4694%" y="575.50">alloc::c..</text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::wrap_add (2,328 samples, 1.20%)</title><rect x="90.1650%" y="549" width="1.2021%" height="15" fill="rgb(210,16,53)"/><text x="90.4150%" y="559.50"></text></g><g><title>core::num::&lt;impl usize&gt;::wrapping_add (2,021 samples, 1.04%)</title><rect x="90.3235%" y="533" width="1.0435%" height="15" fill="rgb(235,124,12)"/><text x="90.5735%" y="543.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::buffer_write (1,100 samples, 0.57%)</title><rect x="92.3502%" y="549" width="0.5680%" height="15" fill="rgb(224,169,11)"/><text x="92.6002%" y="559.50"></text></g><g><title>core::ptr::write (1,099 samples, 0.57%)</title><rect x="92.3507%" y="533" width="0.5675%" height="15" fill="rgb(250,166,2)"/><text x="92.6007%" y="543.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::is_full (1,042 samples, 0.54%)</title><rect x="92.9182%" y="549" width="0.5380%" height="15" fill="rgb(242,216,29)"/><text x="93.1682%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::push_front (5,025 samples, 2.59%)</title><rect x="91.3671%" y="565" width="2.5947%" height="15" fill="rgb(230,116,27)"/><text x="91.6171%" y="575.50">al..</text></g><g><title>alloc::collections::vec_deque::VecDeque&lt;T&gt;::wrap_sub (979 samples, 0.51%)</title><rect x="93.4563%" y="549" width="0.5055%" height="15" fill="rgb(228,99,48)"/><text x="93.7063%" y="559.50"></text></g><g><title>alloc::collections::vec_deque::wrap_index (979 samples, 0.51%)</title><rect x="93.4563%" y="533" width="0.5055%" height="15" fill="rgb(253,11,6)"/><text x="93.7063%" y="543.50"></text></g><g><title>&lt;lll::matcher::MatchIterator as core::iter::traits::iterator::Iterator&gt;::next (127,930 samples, 66.06%)</title><rect x="28.4025%" y="597" width="66.0570%" height="15" fill="rgb(247,143,39)"/><text x="28.6525%" y="607.50">&lt;lll::matcher::MatchIterator as core::iter::traits::iterator::Iterator&gt;::next</text></g><g><title>lll::matcher::MatchIterator::advance_char (58,442 samples, 30.18%)</title><rect x="64.2828%" y="581" width="30.1767%" height="15" fill="rgb(236,97,10)"/><text x="64.5328%" y="591.50">lll::matcher::MatchIterator::advance_char</text></g><g><title>core::option::Option&lt;T&gt;::unwrap (955 samples, 0.49%)</title><rect x="93.9664%" y="565" width="0.4931%" height="15" fill="rgb(233,208,19)"/><text x="94.2164%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (68 samples, 0.04%)</title><rect x="94.4797%" y="501" width="0.0351%" height="15" fill="rgb(216,164,2)"/><text x="94.7297%" y="511.50"></text></g><g><title>[[kernel.kallsyms]] (50 samples, 0.03%)</title><rect x="94.4890%" y="485" width="0.0258%" height="15" fill="rgb(220,129,5)"/><text x="94.7390%" y="495.50"></text></g><g><title>[[kernel.kallsyms]] (44 samples, 0.02%)</title><rect x="94.4921%" y="469" width="0.0227%" height="15" fill="rgb(242,17,10)"/><text x="94.7421%" y="479.50"></text></g><g><title>[[kernel.kallsyms]] (37 samples, 0.02%)</title><rect x="94.4957%" y="453" width="0.0191%" height="15" fill="rgb(242,107,0)"/><text x="94.7457%" y="463.50"></text></g><g><title>[[kernel.kallsyms]] (26 samples, 0.01%)</title><rect x="94.5014%" y="437" width="0.0134%" height="15" fill="rgb(251,28,31)"/><text x="94.7514%" y="447.50"></text></g><g><title>[[kernel.kallsyms]] (20 samples, 0.01%)</title><rect x="94.5045%" y="421" width="0.0103%" height="15" fill="rgb(233,223,10)"/><text x="94.7545%" y="431.50"></text></g><g><title>[[kernel.kallsyms]] (124 samples, 0.06%)</title><rect x="94.4595%" y="597" width="0.0640%" height="15" fill="rgb(215,21,27)"/><text x="94.7095%" y="607.50"></text></g><g><title>[[kernel.kallsyms]] (112 samples, 0.06%)</title><rect x="94.4657%" y="581" width="0.0578%" height="15" fill="rgb(232,23,21)"/><text x="94.7157%" y="591.50"></text></g><g><title>[[kernel.kallsyms]] (108 samples, 0.06%)</title><rect x="94.4678%" y="565" width="0.0558%" height="15" fill="rgb(244,5,23)"/><text x="94.7178%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (105 samples, 0.05%)</title><rect x="94.4693%" y="549" width="0.0542%" height="15" fill="rgb(226,81,46)"/><text x="94.7193%" y="559.50"></text></g><g><title>[[kernel.kallsyms]] (95 samples, 0.05%)</title><rect x="94.4745%" y="533" width="0.0491%" height="15" fill="rgb(247,70,30)"/><text x="94.7245%" y="543.50"></text></g><g><title>[[kernel.kallsyms]] (92 samples, 0.05%)</title><rect x="94.4761%" y="517" width="0.0475%" height="15" fill="rgb(212,68,19)"/><text x="94.7261%" y="527.50"></text></g><g><title>core::char::methods::&lt;impl char&gt;::encode_utf8 (2,967 samples, 1.53%)</title><rect x="95.5537%" y="581" width="1.5320%" height="15" fill="rgb(240,187,13)"/><text x="95.8037%" y="591.50"></text></g><g><title>core::char::methods::encode_utf8_raw (2,967 samples, 1.53%)</title><rect x="95.5537%" y="565" width="1.5320%" height="15" fill="rgb(223,113,26)"/><text x="95.8037%" y="575.50"></text></g><g><title>core::slice::&lt;impl core::ops::index::IndexMut&lt;I&gt; for [T]&gt;::index_mut (1,022 samples, 0.53%)</title><rect x="96.5580%" y="549" width="0.5277%" height="15" fill="rgb(206,192,2)"/><text x="96.8080%" y="559.50"></text></g><g><title>&lt;core::ops::range::RangeTo&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::index_mut (1,022 samples, 0.53%)</title><rect x="96.5580%" y="533" width="0.5277%" height="15" fill="rgb(241,108,4)"/><text x="96.8080%" y="543.50"></text></g><g><title>&lt;core::ops::range::Range&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::index_mut (1,022 samples, 0.53%)</title><rect x="96.5580%" y="517" width="0.5277%" height="15" fill="rgb(247,173,49)"/><text x="96.8080%" y="527.50"></text></g><g><title>&lt;core::ops::range::RangeFrom&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::get_unchecked_mut (1,009 samples, 0.52%)</title><rect x="97.5571%" y="549" width="0.5210%" height="15" fill="rgb(224,114,35)"/><text x="97.8071%" y="559.50"></text></g><g><title>&lt;core::ops::range::Range&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::get_unchecked_mut (1,009 samples, 0.52%)</title><rect x="97.5571%" y="533" width="0.5210%" height="15" fill="rgb(245,159,27)"/><text x="97.8071%" y="543.50"></text></g><g><title>core::slice::&lt;impl core::ops::index::IndexMut&lt;I&gt; for [T]&gt;::index_mut (1,924 samples, 0.99%)</title><rect x="97.0857%" y="581" width="0.9935%" height="15" fill="rgb(245,172,44)"/><text x="97.3357%" y="591.50"></text></g><g><title>&lt;core::ops::range::RangeFrom&lt;usize&gt; as core::slice::SliceIndex&lt;[T]&gt;&gt;::index_mut (1,924 samples, 0.99%)</title><rect x="97.0857%" y="565" width="0.9935%" height="15" fill="rgb(236,23,11)"/><text x="97.3357%" y="575.50"></text></g><g><title>[[kernel.kallsyms]] (167 samples, 0.09%)</title><rect x="99.8420%" y="101" width="0.0862%" height="15" fill="rgb(205,117,38)"/><text x="100.0920%" y="111.50"></text></g><g><title>[[kernel.kallsyms]] (35 samples, 0.02%)</title><rect x="99.9102%" y="85" width="0.0181%" height="15" fill="rgb(237,72,25)"/><text x="100.1602%" y="95.50"></text></g><g><title>[[kernel.kallsyms]] (32 samples, 0.02%)</title><rect x="99.9117%" y="69" width="0.0165%" height="15" fill="rgb(244,70,9)"/><text x="100.1617%" y="79.50"></text></g><g><title>[[kernel.kallsyms]] (26 samples, 0.01%)</title><rect x="99.9148%" y="53" width="0.0134%" height="15" fill="rgb(217,125,39)"/><text x="100.1648%" y="63.50"></text></g><g><title>[[kernel.kallsyms]] (24 samples, 0.01%)</title><rect x="99.9158%" y="37" width="0.0124%" height="15" fill="rgb(235,36,10)"/><text x="100.1658%" y="47.50"></text></g><g><title>[[kernel.kallsyms]] (378 samples, 0.20%)</title><rect x="99.7336%" y="133" width="0.1952%" height="15" fill="rgb(251,123,47)"/><text x="99.9836%" y="143.50"></text></g><g><title>[[kernel.kallsyms]] (241 samples, 0.12%)</title><rect x="99.8043%" y="117" width="0.1244%" height="15" fill="rgb(221,13,13)"/><text x="100.0543%" y="127.50"></text></g><g><title>__GI___libc_write (3,582 samples, 1.85%)</title><rect x="98.0797%" y="437" width="1.8496%" height="15" fill="rgb(238,131,9)"/><text x="98.3297%" y="447.50">_..</text></g><g><title>[[kernel.kallsyms]] (3,580 samples, 1.85%)</title><rect x="98.0807%" y="421" width="1.8485%" height="15" fill="rgb(211,50,8)"/><text x="98.3307%" y="431.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,579 samples, 1.85%)</title><rect x="98.0812%" y="405" width="1.8480%" height="15" fill="rgb(245,182,24)"/><text x="98.3312%" y="415.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,578 samples, 1.85%)</title><rect x="98.0817%" y="389" width="1.8475%" height="15" fill="rgb(242,14,37)"/><text x="98.3317%" y="399.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,575 samples, 1.85%)</title><rect x="98.0833%" y="373" width="1.8460%" height="15" fill="rgb(246,228,12)"/><text x="98.3333%" y="383.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,573 samples, 1.84%)</title><rect x="98.0843%" y="357" width="1.8449%" height="15" fill="rgb(213,55,15)"/><text x="98.3343%" y="367.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,564 samples, 1.84%)</title><rect x="98.0890%" y="341" width="1.8403%" height="15" fill="rgb(209,9,3)"/><text x="98.3390%" y="351.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,562 samples, 1.84%)</title><rect x="98.0900%" y="325" width="1.8392%" height="15" fill="rgb(230,59,30)"/><text x="98.3400%" y="335.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,558 samples, 1.84%)</title><rect x="98.0921%" y="309" width="1.8372%" height="15" fill="rgb(209,121,21)"/><text x="98.3421%" y="319.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,556 samples, 1.84%)</title><rect x="98.0931%" y="293" width="1.8362%" height="15" fill="rgb(220,109,13)"/><text x="98.3431%" y="303.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,549 samples, 1.83%)</title><rect x="98.0967%" y="277" width="1.8325%" height="15" fill="rgb(232,18,1)"/><text x="98.3467%" y="287.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,510 samples, 1.81%)</title><rect x="98.1169%" y="261" width="1.8124%" height="15" fill="rgb(215,41,42)"/><text x="98.3669%" y="271.50">[..</text></g><g><title>[[kernel.kallsyms]] (3,368 samples, 1.74%)</title><rect x="98.1902%" y="245" width="1.7391%" height="15" fill="rgb(224,123,36)"/><text x="98.4402%" y="255.50"></text></g><g><title>[[kernel.kallsyms]] (2,970 samples, 1.53%)</title><rect x="98.3957%" y="229" width="1.5336%" height="15" fill="rgb(240,125,3)"/><text x="98.6457%" y="239.50"></text></g><g><title>[[kernel.kallsyms]] (2,824 samples, 1.46%)</title><rect x="98.4711%" y="213" width="1.4582%" height="15" fill="rgb(205,98,50)"/><text x="98.7211%" y="223.50"></text></g><g><title>[[kernel.kallsyms]] (2,520 samples, 1.30%)</title><rect x="98.6281%" y="197" width="1.3012%" height="15" fill="rgb(205,185,37)"/><text x="98.8781%" y="207.50"></text></g><g><title>[[kernel.kallsyms]] (2,171 samples, 1.12%)</title><rect x="98.8083%" y="181" width="1.1210%" height="15" fill="rgb(238,207,15)"/><text x="99.0583%" y="191.50"></text></g><g><title>[[kernel.kallsyms]] (1,788 samples, 0.92%)</title><rect x="99.0060%" y="165" width="0.9232%" height="15" fill="rgb(213,199,42)"/><text x="99.2560%" y="175.50"></text></g><g><title>[[kernel.kallsyms]] (1,292 samples, 0.67%)</title><rect x="99.2621%" y="149" width="0.6671%" height="15" fill="rgb(235,201,11)"/><text x="99.5121%" y="159.50"></text></g><g><title>&lt;std::io::stdio::StdoutRaw as std::io::Write&gt;::write (3,584 samples, 1.85%)</title><rect x="98.0797%" y="485" width="1.8506%" height="15" fill="rgb(207,46,11)"/><text x="98.3297%" y="495.50">&lt;..</text></g><g><title>&lt;std::sys::unix::stdio::Stdout as std::io::Write&gt;::write (3,584 samples, 1.85%)</title><rect x="98.0797%" y="469" width="1.8506%" height="15" fill="rgb(241,35,35)"/><text x="98.3297%" y="479.50">&lt;..</text></g><g><title>std::sys::unix::fd::FileDesc::write (3,584 samples, 1.85%)</title><rect x="98.0797%" y="453" width="1.8506%" height="15" fill="rgb(243,32,47)"/><text x="98.3297%" y="463.50">s..</text></g><g><title>&lt;std::io::buffered::BufWriter&lt;W&gt; as std::io::Write&gt;::write (3,586 samples, 1.85%)</title><rect x="98.0797%" y="501" width="1.8516%" height="15" fill="rgb(247,202,23)"/><text x="98.3297%" y="511.50">&lt;..</text></g><g><title>__memrchr_sse2 (110 samples, 0.06%)</title><rect x="99.9313%" y="453" width="0.0568%" height="15" fill="rgb(219,102,11)"/><text x="100.1813%" y="463.50"></text></g><g><title>&lt;std::io::stdio::StdoutLock as std::io::Write&gt;::write_all (3,698 samples, 1.91%)</title><rect x="98.0792%" y="549" width="1.9095%" height="15" fill="rgb(243,110,44)"/><text x="98.3292%" y="559.50">&lt;..</text></g><g><title>std::io::Write::write_all (3,698 samples, 1.91%)</title><rect x="98.0792%" y="533" width="1.9095%" height="15" fill="rgb(222,74,54)"/><text x="98.3292%" y="543.50">s..</text></g><g><title>&lt;std::io::buffered::LineWriter&lt;W&gt; as std::io::Write&gt;::write (3,697 samples, 1.91%)</title><rect x="98.0797%" y="517" width="1.9090%" height="15" fill="rgb(216,99,12)"/><text x="98.3297%" y="527.50">&lt;..</text></g><g><title>std::memchr::memrchr (111 samples, 0.06%)</title><rect x="99.9313%" y="501" width="0.0573%" height="15" fill="rgb(226,22,26)"/><text x="100.1813%" y="511.50"></text></g><g><title>std::sys::unix::memchr::memrchr (111 samples, 0.06%)</title><rect x="99.9313%" y="485" width="0.0573%" height="15" fill="rgb(217,163,10)"/><text x="100.1813%" y="495.50"></text></g><g><title>std::sys::unix::memchr::memrchr::memrchr_specific (111 samples, 0.06%)</title><rect x="99.9313%" y="469" width="0.0573%" height="15" fill="rgb(213,25,53)"/><text x="100.1813%" y="479.50"></text></g><g><title>[unknown] (193,640 samples, 99.99%)</title><rect x="0.0041%" y="629" width="99.9866%" height="15" fill="rgb(252,105,26)"/><text x="0.2541%" y="639.50">[unknown]</text></g><g><title>lll::string_stream_editor::process_string_stream_bufread_bufwrite (193,639 samples, 99.99%)</title><rect x="0.0046%" y="613" width="99.9861%" height="15" fill="rgb(220,39,43)"/><text x="0.2546%" y="623.50">lll::string_stream_editor::process_string_stream_bufread_bufwrite</text></g><g><title>lll::string_stream_editor::OutputBuffer::append_char (10,587 samples, 5.47%)</title><rect x="94.5241%" y="597" width="5.4666%" height="15" fill="rgb(229,68,48)"/><text x="94.7741%" y="607.50">lll::st..</text></g><g><title>lll::string_stream_editor::OutputBuffer::flush (3,702 samples, 1.91%)</title><rect x="98.0792%" y="581" width="1.9115%" height="15" fill="rgb(252,8,32)"/><text x="98.3292%" y="591.50">l..</text></g><g><title>&lt;std::io::stdio::Stdout as std::io::Write&gt;::write_all (3,702 samples, 1.91%)</title><rect x="98.0792%" y="565" width="1.9115%" height="15" fill="rgb(223,20,43)"/><text x="98.3292%" y="575.50">&lt;..</text></g><g><title>lll (193,655 samples, 99.99%)</title><rect x="0.0000%" y="645" width="99.9943%" height="15" fill="rgb(229,81,49)"/><text x="0.2500%" y="655.50">lll</text></g><g><title>all (193,666 samples, 100%)</title><rect x="0.0000%" y="661" width="100.0000%" height="15" fill="rgb(236,28,36)"/><text x="0.2500%" y="671.50"></text></g></svg></svg>