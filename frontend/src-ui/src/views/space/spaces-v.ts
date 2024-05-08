import { BaseHTMLElement, cherryChild, customElement, elem, first, frag, html, onHub } from 'dom-native';
import { driveFmc, spaceFmc } from '../../fmc-model';
import { invoke_rpc } from '../../ipc';
import { RouteState, router } from '../../router';

// NOTE: For now nothing.
const HTML = html`

<header>
	<h2></h2>
</header>

<section class="content">
</section>

`;


@customElement('spaces-v') // same as customElements.define('app-v', AppView)
export class SpacesView extends BaseHTMLElement { // extends HTMLElement
	#spaceTitleEl!: HTMLElement
	#contentEl!: HTMLElement

	current_space_id?: number

	@onHub("Route", "change") // @onHub(hubName, topic, label?)
	onRouteChange() {
		let route = router.get_current();
		if (route.main_id == "spaces") {
			this.refresh_view(route);
		}
	}

	init() {

		const content = document.importNode(HTML, true);
		this.#spaceTitleEl = first(content, "header > h2")!;
		this.#contentEl = cherryChild(content, "section");

		this.replaceChildren(content);
	}

	async postDisplay() {
		let current_route = router.get_current();
		if (current_route.space_id == null) {
			let space = await spaceFmc.get_latest();

			router.update_state({ space_id: space.id });
			// Note: No need to call refresh_view, as it will be caled from the SpacesView.onRouteChange
		} else {
			this.refresh_view(router.get_current())
		}
	}

	async refresh_view(route: RouteState) {
		// -- Get from router
		let space_id = route.space_id;


		if (space_id == null) {
			this.#contentEl.replaceChildren("");
			return;
		}

		// -- Update the view if needed
		if (this.current_space_id != space_id) {
			let space = await spaceFmc.get(space_id);
			this.#spaceTitleEl.textContent = space.name;
			let spaceEl = elem("space-v", { $: { _data: { space_id } } });
			this.#contentEl.replaceChildren(spaceEl);
		}
	}

}

declare global {
	interface HTMLElementTagNameMap {
		'spaces-v': SpacesView;
	}
}
