import { BaseHTMLElement, cherryChild, customElement, elem, first, frag, html, onHub } from 'dom-native';
import { driveFmc } from '../../fmc-model';

const HTML = html`

<drives-nav>
	<a class="all sel">ALL DRIVES</a>
	<section>
	<a> Drive Two </a>
	</section>
</drives-nav>
<section class="main">
	<all-drives-p></all-drives-p>
</section>

`;


@customElement('drives-v') // same as customElements.define('app-v', AppView)
export class DrivesView extends BaseHTMLElement { // extends HTMLElement
	#drivesNavCtn!: HTMLElement;

	@onHub("dataHub", "drive")
	onDataDrive(drive_id: number) {
		this.refreshNav();
	}

	init() {
		const content = document.importNode(HTML, true);

		this.#drivesNavCtn = first(content, "drives-nav > section")!;

		this.replaceChildren(content);

		this.refreshNav();
	}


	async refreshNav() {
		let drives = await driveFmc.list();

		let frg = frag();
		for (const drive of drives) {
			frg.append(elem("a", { $: { textContent: drive.name } }));
		}

		this.#drivesNavCtn.replaceChildren(frg);
	}


}