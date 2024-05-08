import { DInputElement } from '@dom-native/ui';
import { BaseHTMLElement, OnEvent, cherryChild, customElement, html, onEvent, onHub } from 'dom-native';
import { driveFmc } from '../../fmc-model';

const HTML = html`

<header>
	<h1>All Drives</h1>
	<d-input class="new-task" placeholder="Create new drive (press enter)"></d-input>
</header>
<section>
	<section><h1>Folders</h1></section>
	<section><h1>Files</h1></section>
	<section><h1>DataSources</h1></section>
	<section><h1>Connectors</h1></section>
</section>

`;


@customElement('all-drives-p')
export class AllDrivesPanel extends BaseHTMLElement { // extends HTMLElement
	#headerEl!: HTMLElement;
	#contentEl!: HTMLElement;

	@onEvent("CHANGE", "d-input")
	onValueChange(evt: OnEvent) {
		let dInput = evt.selectTarget as DInputElement;
		const name = dInput.value;
		driveFmc.create({ name });
	}


	init() {
		const content = document.importNode(HTML, true);
		// [this.#headerEl, this.#contentEl] = cherryChild(content, 'header', 'section');

		this.replaceChildren(content);
	}


}

function OnHub(arg0: string, arg1: string): (target: (evt: any) => void, context: ClassMethodDecoratorContext<AllDrivesPanel, (evt: any) => void> & { name: "onValueChange"; private: false; static: false; }) => void | ((evt: any) => void) {
	throw new Error('Function not implemented.');
}
