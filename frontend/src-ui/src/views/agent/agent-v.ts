import { SelectOption } from '@dom-native/ui';
import { updatedDiff } from 'deep-object-diff';
import { BaseHTMLElement, OnEvent, customElement, first, html, onEvent, onHub, pull, push } from 'dom-native';
import { isEmpty } from 'utils-min';
import { aiFmc } from '../../fmc-ai';
import { agentFmc } from '../../fmc-model';

const HTML = html`
<div class="form">
	<div class="form__field">
		<label>Name:</label>
		<d-input name="name"><d-input>
	</div>

	<div class="form__field">
		<label>Model:</label>
		<d-select name="model" placeholder="Select a model">
		</d-select>
	</div>

	<div class="form__field">
		<label>Description:</label>
		<d-textarea name="desc"><d-textarea>
	</div>	

		<tabs-p>
			<nav>
				<label data-name="inst" class="sel" >Instruction</label>
				<label data-name="prompt_tmpl">Prompt Template</label>
				<label data-name="chain" >Chain</label>
			</nav>
			<section>
				<section data-for="inst" class="agent-tab-section sel">
					<label>Agent instruction:</label>
					<d-textarea name="inst"></d-textarea>
				</section>

				<section data-for="prompt_tmpl" class="agent-tab-section">
					<label>Prompt Template:</label>
					<d-textarea name="prompt_tmpl"></d-textarea>
				</section>				

				<section data-for="chain" class="agent-tab-section">
					<label>Chain:</label>
					<d-textarea name="chain"></d-textarea>
				</section>				
				
			</section> 
		</tabs-p>

	<div class="form__but-bar">
		<d-check name="out_format" label="Json Output" value="Json" unchecked-value="Text"></d-check>
		<button class="save">Save</button>
	</div>
</div>
`;

@customElement('agent-v')
export class AgentView extends BaseHTMLElement { // extends HTMLElement
	// -- Init
	_data!: { agent_id: number }

	// -- Els
	#formEl!: HTMLElement

	// -- States
	#loaded_agent: any

	get agent_id() { return this._data.agent_id }

	// #region    --- Hub Events

	@onHub("modelHub", "agent")
	onModelHub(evt: any, info: any) {
		if (evt.id == this.agent_id && info.label != "delete") {
			this.refresh();
		}
	}

	// #endregion --- Hub Events


	// #region    --- UI Events

	@onEvent("pointerup", ".form button.save")
	async onSaveClick(evt: Event & OnEvent) {
		let data = pull(this);
		let updatedData: any = updatedDiff(this.#loaded_agent, data);
		if (!isEmpty(updatedData)) {
			let agent = await agentFmc.update(this.agent_id, updatedData);
			this.#loaded_agent = agent;
			// TODO: Probably need to be event based, and also make sure the UI is refreshed to make 
			//       sure user can see if there is an issue.
		}
	}

	@onEvent("D-DATA", "d-select[name='model']")
	async onDSelectModelData(evt: OnEvent) {
		let { sendData } = evt.detail;
		// let model_strs = ["mixtral", "llamaasdf2"];
		let model_strs = await aiFmc.list_models();
		let models: SelectOption[] = model_strs.map((m) => { return { content: m, value: m } });
		models.unshift({ content: "None", value: null });
		sendData(models);
	}

	// #endregion --- UI Events

	init() {
		const content = document.importNode(HTML, true);
		this.#formEl = first(content, ".form")!;
		this.replaceChildren(content);
	}

	async postDisplay() {
		this.refresh()
	}

	async refresh() {
		console.log('->> agent-v refresh',);
		let agent = await agentFmc.get(this.agent_id);
		this.#loaded_agent = agent;
		push(this.#formEl, agent);
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'agent-v': AgentView;
	}
}


