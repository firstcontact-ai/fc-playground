import { BaseHTMLElement, OnEvent, customElement, html, on, setAttr, setClass } from 'dom-native';

const HTML = html``;

// <do-toggle do-show-steps="Show Steps" do-hide-steps="Hide Steps">Show Steps</do-toggle>

@customElement('do-toggle')
export class DoToggleEl extends BaseHTMLElement { // extends HTMLElement

	init() {
	}

	postDisplay() {
		this.update_class();
	}

	update_class() {
		const { do_attrs, current } = extract_state(this);
		const clss_obj = to_false_vals(do_attrs);
		if (current != null) {
			clss_obj[current] = true;
		}
		setClass(this, clss_obj);
	}

	toggle() {
		const { do_attrs, current } = extract_state(this);
		const other_key = Object.keys(do_attrs).find(key => key != current);
		if (other_key != null) {
			this.textContent = do_attrs[other_key];
			this.update_class();
		}
	}

	current_key(): string | null {
		const { do_attrs, current } = extract_state(this);
		return current;
	}
}


declare global {
	interface HTMLElementTagNameMap {
		'e-action-toggle': DoToggleEl;
	}
}


on(document, "pointerup", "do-toggle", (evt: OnEvent) => {
	let do_el = evt.selectTarget;
	if (do_el instanceof DoToggleEl) {
		do_el.toggle();
	}
});

// #region    --- Support

interface DoState {
	// e.g., `"do-show-steps": "Show Steps"`
	do_attrs: { [name: string]: string },
	// e.g. `"do-show_steps"`
	current: string | null
}
function extract_state(el: HTMLElement): DoState {
	const do_attrs: { [name: string]: string } = {};
	// Loop through all attributes of the element
	for (let attr of el.attributes) {
		// Add each attribute to the object
		do_attrs[attr.name] = attr.value;
	}

	// Determine the current state based on the inner text of the element
	let current = Object.keys(do_attrs).find(key => do_attrs[key] === el.textContent) ?? null;

	return {
		do_attrs,
		current
	};
}

function to_false_vals(orig: any) {
	const obj: any = {};
	// Iterate over the object keys
	Object.keys(orig).forEach(key => {
		obj[key] = false;
	});
	return obj;
}

// #endregion --- Support

