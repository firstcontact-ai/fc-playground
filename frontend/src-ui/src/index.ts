
import { SVG_SYMBOLS } from './svg-symbols.js';

// important, this will load the customElements

import { html } from 'dom-native';
import './common-handlers.js';
import './comps/index.js';
import './elems/index.js';
import './views/index.js';

import * as _ from '@dom-native/ui';
import { loadDefaultIcons } from '@dom-native/ui';

// load the default icons from @dom-native/ui
loadDefaultIcons();

// --- Initialize some assets on DOMContentLoaded
document.addEventListener("DOMContentLoaded", async function (event) {

	// Append the app custom icons 
	// (similar to what loadDefaultIcons does for @dom-native/ui icons)
	// (this allows to use the <use xlink:href="#symbol_id" ...> and update fill from css)
	const svgEl = html(SVG_SYMBOLS).firstElementChild!;
	svgEl.setAttribute('style', 'display: none'); // in case dom engine move it to body
	document.head.appendChild(svgEl);



});


