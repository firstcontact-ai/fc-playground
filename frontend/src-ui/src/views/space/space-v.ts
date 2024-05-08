
import * as tauri_dialog from "@tauri-apps/plugin-dialog";
import { BaseHTMLElement, OnEvent, cherryChild, customElement, elem, first, frag, getAttr, html, onEvent, onHub, position } from 'dom-native';
import { asNum } from 'utils-min';
import { toggle_selector_popup } from '../../comps/selector-pop';
import { agentFmc, driveFmc, dsourceFmc, spaceFmc } from '../../fmc-model';
import { invoke_rpc } from '../../ipc';

const HTML = html`

<section class="ctrl-bar">

	<div class="box box-drive">
		<h3>Documents & Folders</h3>
		<section class="box-drive-content">
		<div class="dsources-content">
		</div>
		<div class="drive-info">
		Click or Drag & Drop Files & Folders. 
		</div>
		</section>
	</div>

	<div class="box">
		<h3><a href="#agents">Agents</a></h3>
		<section class="box-agents-content">
			
		</section>
	</div>	

	<div class="box">
		<h3>Privacy</h3>
	</div>	

</section>

<section class="content main-content">
	
</section>

`;

const DRIVE_INFO_DEFAULT = "Click or Drag & Drop Files & Folders."
const DRIVE_INFO_DRAGOVER = "Drop to add to content."

@customElement('space-v') // same as customElements.define('app-v', AppView)
export class SpaceView extends BaseHTMLElement { // extends HTMLElement
	// -- Instantiation props
	_data!: { space_id: number }

	// -- Els
	#driveEl!: HTMLElement
	#driveInfoEl!: HTMLElement
	#dsourcesContentEl!: HTMLElement
	#agentsContentEl!: HTMLElement
	#mainContentEl!: HTMLElement

	// -- Privates
	#space_id!: number;
	#drive_id!: number;

	// #region    --- UI Events

	@onEvent("click", ".drive-info")
	async onDriveClick() {

		let path = await tauri_dialog.open({
			title: "Select Folder Or File",
			defaultPath: "~/Document",
			directory: true,
		});

		if (path) {
			await driveFmc.add_dsource(this.#drive_id, path);
			this.refreshDrive();
		}

	}

	@onEvent("dragover, dragleave, TAURI_DROP", ".box-drive")
	async onDragOver(evt: Event & OnEvent) {
		if (evt.type == "dragover") {
			this.#driveEl.classList.add("dragover");
			this.#driveInfoEl.textContent = DRIVE_INFO_DRAGOVER;
		} else if (evt.type == "TAURI_DROP") {
			this.#driveEl.classList.remove("dragover");
			await driveFmc.add_dsource(this.#drive_id, evt.detail.path);
			this.refreshDrive();
			this.#driveInfoEl.textContent = DRIVE_INFO_DEFAULT;
		} else {
			this.#driveEl.classList.remove("dragover");
			this.#driveInfoEl.textContent = DRIVE_INFO_DEFAULT;
		}
	}

	@onEvent("pointerup", "agent-item")
	onAgentItemClick(evt: OnEvent & Event) {
		let agentEl = evt.selectTarget;
		this.showAgentSelector(agentEl);
	}

	// #endregion --- UI Events

	// #region    --- Component Lifecycle
	init() {
		this.#space_id = this._data.space_id;

		const content = document.importNode(HTML, true);
		// -- Els
		this.#driveEl = first(content, ".box-drive")!;
		this.#driveInfoEl = first(content, ".drive-info")!;
		this.#dsourcesContentEl = first(content, ".dsources-content")!;
		this.#mainContentEl = first(content, ".main-content")!;
		this.#agentsContentEl = first(content, ".box-agents-content")!;

		this.replaceChildren(content);

	}

	async postDisplay() {
		this.#dsourcesContentEl.textContent = "...";
		let drive = await spaceFmc.get_default_drive(this.#space_id);
		this.#drive_id = drive.id;
		this.#dsourcesContentEl.textContent = "" + drive.id;

		this.refreshDrive();
		this.refreshAgents();
		this.#mainContentEl.replaceChildren(elem("space-chat", { $: { space_id: this.#space_id } }));
	}
	// #endregion --- Component Lifecycle


	// #region    --- Refresh Fns

	async refreshAgents() {
		let agent = await spaceFmc.seek_agent(this.#space_id);
		if (agent) {
			let content = html`
<agent-item data-id="${agent.id}" title="${agent.name}">
	<d-ico name="ico-agent"></d-ico>
	<label>${agent.name}</label>
</agent-item>
`;
			this.#agentsContentEl.replaceChildren(content);
		}
	}

	async refreshDrive() {
		let dsources = await dsourceFmc.list({
			filters: {
				drive_id: this.#drive_id
			}
		});

		let content = frag(dsources, (_data) => elem("dsource-c", { $: { _data } }));
		this.#dsourcesContentEl.replaceChildren(content);
	}

	// #endregion --- Refresh Fns


	// #region    --- Others

	async showAgentSelector(agentEl: HTMLElement) {
		let agentId = asNum(getAttr(agentEl, "data-id"));
		let agentTitle = getAttr(agentEl, "title");

		// -- Build the html content
		let agents = await agentFmc.list();
		let content = html`
		  <h3 href="#agents?agent_id=${agentId}" class="item do-close">Edit: <span class="link">${agentTitle}</span></h3>			
			<span class="delim"></span>
			<h3>Change Agent:</h3>
		`;
		for (const agent of agents) {
			if (agentId == agent.id) {
				continue;
			}
			content.appendChild(elem("div", { class: "item link do-select", "data-id": `${agent.id}`, $: { textContent: agent.name } }));
		}

		// -- Event call back
		let onDoHandler = async (type: string, el: HTMLElement) => {
			if (type == "do-select") {
				let agent_id = asNum(getAttr(el, "data-id"));
				if (agentId != null) {
					await spaceFmc.update(this.#space_id, { agent_id });
					this.refreshAgents();
				}
			}
		};

		toggle_selector_popup({
			popupId: "agent-selector",
			onDoHandler,
			content,
			triggerEl: agentEl,
			positionOptions: { refPos: "TR", pos: "BR", hGap: 8 }
		})

		// let popupEl = elem('selector-pop', { $: { _data: { content, onDoHandler } } });
		// document.body.appendChild(popupEl);
		// position(popupEl, agentEl, { refPos: "TR", pos: "BR", hGap: 8 });
	}
	// #endregion --- Others
}

declare global {
	interface HTMLElementTagNameMap {
		'space-v': SpaceView;
	}
}
