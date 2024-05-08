import { closest } from 'dom-native';


// `href=..` attribute handler
document.addEventListener("pointerup", function (evt) {

	let el = evt.target as HTMLElement; // Note: we might change this assumption later
	// if it's a <a> tag we let it handle the href
	if (el.tagName != "A") {
		let closest_el = closest(el, "[href]");
		if (closest_el && closest_el.tagName != "A") {
			evt.stopPropagation();
			evt.preventDefault();
			let hash = closest_el.getAttribute("href")!;
			if (!hash.startsWith("#")) {
				console.log(`WARNING - href '${hash}' is not a hash`);
				return;
			} else {
				hash = hash.substring(1);
			}

			window.location.hash = hash;
		} else {
			// console.log('no href tag');
		}

	}

});