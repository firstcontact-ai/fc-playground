//! Main Application View which will initialize the application and display the appropriate 
//!
//! Notes:
//!   - Will listen to Route.change event, and update the main view
//!   - The Nav View `nav-v` will manage it's routing update.
//!
//! TODO: Needs to implement the menu click (min-nav action)
//!

import { BaseHTMLElement, customElement, elem, first, getFirst, html, onEvent, onHub } from 'dom-native';
import { app_init } from '../app-init.js';
import { invoke_rpc, invoke_set_win_sess_value } from '../ipc/index.js';
import { MainId, router } from '../router.js';

// dom-native JS Tagged templates to create a DocumentFragment (parse once)
const HTML = html`
	<header data-tauri-drag-region>
	<d-input id="main-search" placeholder="Search"></d-input>
	</header>
	<main></main>
`;

@customElement('app-v') // same as customElements.define('app-v', AppView)
export class AppView extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Key Els
	#mainEl!: HTMLElement;
	#currentMainId?: MainId;
	// #endregion --- Key Els

	// #region    --- App Events
	@onHub("Route", "change") // @onHub(hubName, topic, label?)
	onRouteChange() {
		this.refresh_view();
	}
	// #endregion --- App Events

	// #region    --- UI Events
	@onEvent("pointerup", "header > c-ico.menu") // @onEvent(eventType, elementSelectorFromThis)
	onMenuClick(evt: PointerEvent) {
		this.classList.toggle("min-nav");
	}
	// #endregion --- UI Events

	init() { // Will be called by BaseHTMLElement once on first connectedCallback
		// clone the HTML documentFragment and get the key elements (to be used later)
		let content = document.importNode(HTML, true);

		this.#mainEl = getFirst(content, "main");

		// replace the children
		this.replaceChildren(content);
		app_init();

	}

	async postDisplay() {

		this.refresh_view();
	}

	refresh_view() {

		const { main_id } = router.get_current();
		// if nil, then, default to spaces
		if (main_id == null) {
			router.update_state({ main_id: "spaces" });
		}
		if (this.#currentMainId != main_id) {
			this.#currentMainId = main_id;

			if (main_id == "drives") {
				this.#mainEl.replaceChildren(elem("drives-v"));
			} if (main_id == "spaces") {
				this.#mainEl.replaceChildren(elem("spaces-v"));
			} else if (main_id == "agents") {
				this.#mainEl.replaceChildren(elem("agents-v"));
			}
			else {
				this.#mainEl.replaceChildren("no view yet");
			}
		}
	}

}

declare global { // trick to augment the global TagName with this component
	interface HTMLElementTagNameMap {
		'app-v': AppView;
	}
}


