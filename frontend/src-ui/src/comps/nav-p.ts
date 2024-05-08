

import { BaseHTMLElement, customElement, html } from 'dom-native';

const HTML = html`
`;

// NOTE: Not needed for now. Just keep in case. Might remove later. 


@customElement('nav-item')
export class NavItem extends BaseHTMLElement { // extends HTMLElement

	init() {
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'nav-item': NavItem;
	}
}
