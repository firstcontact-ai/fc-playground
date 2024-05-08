
import { BaseHTMLElement, OnEvent, all, closest, customElement, elem, first, frag, html, on, onEvent, onHub, position, pull, setAttr, setClass } from 'dom-native';
import { Agent, AgentLite } from '../../bindings';
import { SimpleDlgData, toggle_selector_popup } from '../../comps';
import { agentFmc } from '../../fmc-model';
import { RouteState, router } from '../../router';
import { AgentView } from './agent-v';

const HTML = html`
<header>
	<h2>Agents <d-ico class="do-add" name="ico-add"></d-ico></h2><a class="back-to-space" href="#spaces">Back to space</a>
</header>

<section class="content">
	
	<nav-p>
		<section class="nav-p-ctn">
		</section>
	</nav-p>

	<section class="agent-v-ctn">

	</section>
</section>
`;

@customElement('agents-v')
export class AgentsView extends BaseHTMLElement { // extends HTMLElement
	// -- Els
	#navContentEl!: HTMLElement
	#agentViewCtn!: HTMLElement

	// -- States
	#current_agent_id?: number
	#loaded_agents?: AgentLite[]

	// #region    --- Hub Events

	@onHub("Route", "change") // @onHub(hubName, topic, label?)
	onRouteChange() {
		let route = router.get_current();
		if (route.main_id == "agents") {
			this.refresh_view();
		}
	}

	@onHub("modelHub", "agent")
	onModelHub(evt: any, info: any) {
		console.log('->> modelHub agent', evt, info);
		this.refresh_view(true);
	}

	// #endregion --- Hub Events

	// #region    --- UI Events

	@onEvent("pointerup", ".do-add")
	onDoAddClick(evt: Event & OnEvent) {
		this.showAddAgentDlg();
	}

	@onEvent("pointerup", "nav-item .do-show-more")
	onNavItemShowMore(evt: OnEvent) {
		let nav_item = closest(evt.selectTarget, "nav-item");
		if (nav_item == null) return;
		console.log('->> nav_item show more', nav_item);

		toggle_selector_popup({
			popupId: "agent-nav-show-more-pop",
			onDoHandler: async (type) => {
				if (type == "do-delete-agent") {
					let agent_id = nav_item.getAttribute("data-id");
					if (agent_id) {
						await agentFmc.delete(parseInt(agent_id));
					}
				}
			},
			content: [
				{ key: "do-delete-agent", label: "Delete Agent" },
			],
			triggerEl: evt.selectTarget,
			positionOptions: { refPos: "TR", pos: "BR", hGap: 0 },
		});
	}

	// #endregion --- UI Events

	// #region    --- Add Agent

	async onDlgEvent(type: any, addDlg: HTMLElement) {
		if (type == "DO-OK") {
			await this.do_add_agent_from_dlg(addDlg);
		}
	}

	showAddAgentDlg() {
		let _data: SimpleDlgData = {
			onDlgEvent: (type, srcEl) => { this.onDlgEvent(type, srcEl) },
			content: html`
			<div class="form">
				<div class=""><d-input name="name" autofocus placeholder="Agent Name"></d-input></div>
				<div class="form__but-bar event-buttons">
				  <button class="do-cancel">Cancel</button>
					<button class="do-ok">Add</button>
				</div>
			</div>
			`,
		}
		let dlgEl = elem("simple-dlg", { $: { _data } });
		document.body.appendChild(dlgEl);
		position(dlgEl, first(this, ".do-add")!, { refPos: "TR", pos: "BR", hGap: 8 });
		on(dlgEl, "keyup", "d-input", (evt: KeyboardEvent) => {
			if (evt.key == "Enter") {
				this.do_add_agent_from_dlg(dlgEl);
			}
		})
	}


	async do_add_agent_from_dlg(addDlg: HTMLElement) {
		let data = pull(addDlg);
		if (data.name) {
			const name = data.name;
			let agent = await agentFmc.create({ name });
			addDlg.remove();
			router.update_state({ agent_id: agent.id });
			// TODO: Need to fix that to be event based. 
		}

	}
	// #endregion --- Add Agent


	init() {
		const content = document.importNode(HTML, true);
		this.#navContentEl = first(content, ".nav-p-ctn")!;
		this.#agentViewCtn = first(content, ".agent-v-ctn")!;
		this.replaceChildren(content);

		this.classList.add("main-view");

		this.refresh_view(true);
	}



	async reload_nav() {
		let agents = await agentFmc.list();
		this.#loaded_agents = agents;
		let route = router.get_current();

		const content = frag(agents, (agent) => {
			let sel = (route.agent_id == agent.id) ? "sel" : "";
			return html`
<nav-item data-id="${agent.id}">
	<d-ico href="#agents?agent_id=${agent.id}"  name="ico-agent"></d-ico>
	<label href="#agents?agent_id=${agent.id}" >${agent.name}</label>
	<d-ico class="do-show-more opts" name="ico-more"></d-ico>
</nav-item>			
			`;
		});

		this.#navContentEl.replaceChildren(content);
	}

	async refresh_view(reload_content = false) {
		let route = router.get_current();

		// reload the nave and this.#loade_agents (lite)
		await this.reload_nav();

		let agent_id = route.agent_id;

		// -- set agenit_id to nul if not found
		if (agent_id != null && !this.#loaded_agents?.find(a => a.id == agent_id)) {
			agent_id = undefined;
		}

		// -- If no agent_id and no empty list
		if (agent_id == null && this.#loaded_agents?.length) {
			router.update_state({ agent_id: this.#loaded_agents[0].id });
			return;
		}

		// -- Make sure nav .sel is up to date
		// clear all of the `.sel`
		setClass(this.#navContentEl.children, { sel: false });
		setClass(first(this.#navContentEl, `nav-item[data-id='${agent_id}']`), { sel: true });

		// TODO: needs to handle empty state

		// -- Update the content if needed
		if (agent_id != this.#current_agent_id) {
			this.#current_agent_id = agent_id;
			// change the content
			this.#agentViewCtn.replaceChildren(elem("agent-v", { $: { _data: { agent_id } } }));

		}


	}
}
declare global {
	interface HTMLElementTagNameMap {
		'agents-v': AgentsView;
	}
}
