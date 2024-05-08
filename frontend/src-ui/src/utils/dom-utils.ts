
export type OnDoHandler = (type: string, doEl: HTMLElement) => void;

/**
 * Returns the first `do-...` css class name on this element or parent elements
 * @param el 
 * @returns 
 */
export function first_do_class_name(el: HTMLElement): string | null {
	let tmpEl: HTMLElement | null = el;

	while (tmpEl != null) {
		let doClss = _doClassToTypeEl(tmpEl);
		if (doClss != null) return doClss;

		tmpEl = tmpEl.parentElement;
	}

	return null;
}

function _doClassToTypeEl(el: HTMLElement): string | null {
	for (const cssClass of el.classList.values()) {
		if (cssClass.startsWith("do-")) {
			return cssClass;
		}
	}
	return null;
}