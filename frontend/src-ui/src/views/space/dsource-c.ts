import { BaseHTMLElement, customElement, html } from 'dom-native';
import { DSource } from '../../bindings';

const HTML = html`
<d-ico name="ico-add"></d-ico>
<label></label>
`;

@customElement('dsource-c') // same as customElements.define('app-v', AppView)
export class DSourceComp extends BaseHTMLElement { // extends HTMLElement
	_data!: DSource

	init() {
		let dsource = this._data;

		let content = html`
<d-ico name="ico-${dsource.kind.toLowerCase()}"></d-ico>
<label>${dsource.name}</label>
`;
		this.replaceChildren(content);
	}

}
declare global {
	interface HTMLElementTagNameMap {
		'dsource-c': DSourceComp;
	}
}
