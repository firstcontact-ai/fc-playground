

import { BaseHTMLElement, OnEvent, append, cherryChild, customElement, html, onDoc, onEvent, trigger } from 'dom-native';
import { bhvPopup } from '../behaviors';

const HTML = html`
<header>
</header>
<section>
</section>
<footer>
</footer>
`;

type SimpleDlgOnCallback = (type: "DO-CANCEL" | "DO-OK", dlg: HTMLElement & SimpleDlg) => void | boolean;


export interface SimpleDlgData {
	onDlgEvent?: SimpleDlgOnCallback,
	title?: string | HTMLElement | DocumentFragment,
	content?: string | HTMLElement | DocumentFragment,
	footer?: string | HTMLElement | DocumentFragment
}

@customElement('simple-dlg')
export class SimpleDlg extends BaseHTMLElement { // extends HTMLElement
	_data!: SimpleDlgData

	// -- Callbacks
	#onDlgEvent?: SimpleDlgOnCallback

	// -- Els
	#headerEl!: HTMLElement
	#contentEl!: HTMLElement
	#footerEl!: HTMLElement

	// #region    --- DocEvents

	// #endregion --- DocEvents

	constructor() {
		super();
		bhvPopup(this);
	}

	// #region    --- UI Events

	// DO-OK
	@onEvent("pointerup", ".do-ok:not(:disabled)")
	onButOkClick(evt: Event & OnEvent) {
		this.#onDlgEvent?.("DO-OK", this);
	}

	// DO-CANCEL
	@onEvent("pointerup", ".do-cancel")
	onButCancelClick(evt: Event & OnEvent) {
		this.#onDlgEvent?.("DO-CANCEL", this);
		this.remove();
	}

	// #endregion --- UI Events

	init() {

		const content = document.importNode(HTML, true);
		[this.#headerEl, this.#contentEl, this.#footerEl] = cherryChild(content, "header", "section", "footer");

		this.#onDlgEvent = this._data.onDlgEvent;

		add_content(this.#headerEl, this._data.title);
		add_content(this.#contentEl, this._data.content);
		add_content(this.#footerEl, this._data.footer);

		this.replaceChildren(content);
	}
}

function add_content(baseEl: HTMLElement, content?: string | HTMLElement | DocumentFragment) {
	if (content == null) return;

	if (typeof content == "string") {
		baseEl.textContent = content;
	} else {
		baseEl.replaceChildren(content);
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'simple-dlg': SimpleDlg;
	}
}
