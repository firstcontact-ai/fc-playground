import { DInputElement, DTextareaElement } from '@dom-native/ui';
import { BaseHTMLElement, OnEvent, PositionOptions, cherryChild, closest, customElement, elem, first, frag, getAttr, html, next, onEvent, onHub, setAttr, setClass, style } from 'dom-native';
import { asNum } from 'utils-min';
import { toggle_selector_popup } from '../../comps/selector-pop';
import { aiFmc } from '../../fmc-ai';
import { convFmc, spaceFmc } from '../../fmc-model';

const HTML = html`

<div class="chat-content">
	<div class="inner">
	</div>
</div>
<div class="chat-control"><d-ico name="ico-more" class="show-conv-menu"></d-ico></div>
<d-textarea spellcheck="off" placeholder="Ask your question"></d-textarea>

`;


const DEBUG_CONTENT_LONG = `
The color of the sky can change depending on the time of day, location, and weather conditions, but typically, the sky appears blue during the day. This is because of a process called Rayleigh scattering. When the sun's light reaches Earth's atmosphere, it is made up of different colors, which travel as waves of different lengths. Blue and violet light have the shortest wavelengths and are scattered in all directions by the gas molecules in the Earth's atmosphere.

This scattering of light makes the sky appear blue during the day. However, at sunrise or sunset, the sunlight has to pass through more of the Earth's atmosphere, which scatters the short-wavelength blue and violet light to a greater extent, allowing the longer-wavelength red and orange light to reach your eyes. This is why the sky can appear red or orange during sunrise or sunset.

This scattering of light makes the sky appear blue during the day. However, at sunrise or sunset, the sunlight has to pass through more of the Earth's atmosphere, which scatters the short-wavelength blue and violet light to a greater extent, allowing the longer-wavelength red and orange light to reach your eyes. This is why the sky can appear red or orange during sunrise or sunset.

There are also other factors that can cause the sky to appear red, such as dust, smoke, or pollution in the atmosphere, which can scatter shorter wavelength blue and green light, allowing longer wavelength red light to dominate. This is why the sky might appear red during a dust storm or a volcanic eruption.`;

const DEBUG_CONTENT_SHORT = `{"category": 1}`;

@customElement('space-chat') // same as customElements.define('app-v', AppView)
export class SpaceChat extends BaseHTMLElement { // extends HTMLElement
	// -- Instantiation props
	space_id!: number

	// -- Properties
	conv_id?: number

	// -- Els
	#chatContentParentEl!: HTMLElement
	#chatContentEl!: HTMLElement
	#inputEl!: DTextareaElement

	// #region    --- HubEvents

	@onHub("modelHub", "msg")
	async onModelMsg(detail: any) {
		console.log('->> onModelMsg detail:', detail);
	}

	@onHub("modelHub", "msg")
	async onModelConv(detail: any) {
		console.log('->> onModelConv detail:', detail);
	}

	@onHub("convHub", "conv_work_new")
	async onConvEventNew(detail: any) {
		console.log('->> onConvEventNew detail:', detail);
	}

	@onHub("convHub", "conv_work_done")
	async onConvEventDone(detail: any) {
		console.log('->> onConvEventDone detail:', detail);
		this.refreshMsgs();
	}

	// #endregion --- HubEvents

	// #region    --- UI Events

	@onEvent("pointerup", ".chat-content")
	onRefreshClick() {
		// this.refreshMsgs(); // for debug
	}

	@onEvent("keyup", "d-textarea")
	async onKeyup(evt: KeyboardEvent & OnEvent) {

		let inputEl = evt.selectTarget as DInputElement;
		let special = evt.getModifierState("Meta") || evt.getModifierState("Control");
		if (evt.code == "Enter" && special) {
			let val = inputEl.value;

			// TODO: Needs to handle when no `conv_id`
			await aiFmc.run_user_prompt({
				conv_id: this.conv_id!,
				user_prompt: val
			});

			// TODO: This should be removed, and we should use onHub event on ConvMsg or something
			this.refreshMsgs();
		}
	}

	@onEvent("pointerup", ".do-show-steps")
	onShowSteps(evt: OnEvent) {
		let chat_p = closest(evt.selectTarget, "chat-p");
		if (chat_p instanceof ChatPart) {
			const orig_msg_id = chat_p.msg_id;
			const conv_id = this.conv_id!;
			const step_el = elem("steps-p", { $: { data: { orig_msg_id, conv_id } } });
			chat_p.insertAdjacentElement('afterend', step_el);
		}
	}

	@onEvent("pointerup", ".do-hide-steps")
	onHideSteps(evt: OnEvent) {
		let chat_p = closest(evt.selectTarget, "chat-p");
		let steps_p = next(chat_p);
		if (steps_p instanceof StepsPart) {
			steps_p.remove()
		}
	}

	@onEvent("pointerup", ".show-conv-menu")
	onShowConvMenu(evt: OnEvent) {
		toggle_selector_popup({
			popupId: "conv-menu",
			onDoHandler: async (type) => {
				if (type == "do-clear-conv") {
					console.log('->> do-clear-conv',);
					await convFmc.clear_all(this.conv_id!);
				} else if (type == "do-refresh-conv") {
					console.log('->> do-refresh-conv',);
					this.refreshMsgs();
				}
			},
			content: [
				{ key: "do-clear-conv", label: "Clear Conversation" },
				{ key: "do-refresh-conv", label: "Refresh" }
			],
			triggerEl: evt.selectTarget,
			positionOptions: { refPos: "BL", pos: "TL", hGap: 8 },
		});
	}

	// #endregion --- UI Events

	init() {
		const content = document.importNode(HTML, true);

		// -- Els
		this.#chatContentEl = first(content, ".chat-content .inner")!;
		this.#chatContentParentEl = this.#chatContentEl.parentElement as HTMLElement;
		this.#inputEl = first(content, "d-textarea")!;

		this.replaceChildren(content);

		// xp trick to make the content seems to display faster
		style(this.#chatContentEl, { visibility: "hidden" })
	}

	async postDisplay() {
		const conv = await spaceFmc.get_latest_conv(this.space_id);
		this.conv_id = conv.id;

		this.refreshMsgs();

	}

	async refreshMsgs() {
		let msgs = await convFmc.list_msgs(this.conv_id!);
		let content = frag(msgs, msg => elem("chat-p", { $: { _data: { msg } } }));

		let lastEl = content.lastElementChild as HTMLElement;

		this.#chatContentEl.replaceChildren(content);

		// Note: Might not be needed. Was to solve some problem (need to remove hidden in ui)
		style(this.#chatContentEl, { visibility: "visible" });

		// TODO: Why this needs to be 50 (seems to need to be bigger when more content). 
		//       Does not work with 20. Shoul not even be needed.  
		setTimeout(() => {
			lastEl?.scrollIntoView(true);
			// Note: putting the visibility switch here caused layout issue when bigger content.
		}, 150);

		setTimeout(() => {
			// Find all elements with the class '.do-show-steps'
			const elements = document.querySelectorAll('.do-show-steps');
			console.log('->> e', elements);

			// Check if there are any elements
			if (elements.length > 0) {
				// Select the last element from the NodeList
				const lastElement = elements[elements.length - 1];

				// Create a new pointer event of type 'pointerup'
				const event = new PointerEvent('pointerup', {
					bubbles: true,     // Allows the event to bubble up through the DOM
					cancelable: true,  // Allows the event to be cancellable
				});

				// Dispatch the event on the last element
				lastElement.dispatchEvent(event);
			}
		}, 50);

	}

}

declare global {
	interface HTMLElementTagNameMap {
		'space-chat': SpaceChat;
	}
}


// #region    --- chat-p

const HTML_CHAT_P = html`
<d-ico></d-ico>
<section></section>
<footer></footer>
`;

@customElement('chat-p')
export class ChatPart extends BaseHTMLElement { // extends HTMLElement
	_data!: { msg: any }

	// -- Els
	#icoEl!: HTMLElement
	#contentEl!: HTMLElement
	#footerEl!: HTMLElement

	// -- Getters
	get msg_id(): number {
		return this._data.msg.id;
	}

	init() {
		const content = document.importNode(HTML_CHAT_P, true);
		[this.#icoEl, this.#contentEl, this.#footerEl] = cherryChild(content, "d-ico", "section", "footer");
		this.replaceChildren(content);
	}

	preDisplay() {
		let msg = this._data.msg;
		let author_kind = msg.author_kind;

		// ico-agent
		let ico_name = (author_kind == "User") ? "ico-user" : "ico-agent";
		setAttr(this.#icoEl, { name: ico_name });

		this.#contentEl.innerHTML = `${msg.content}`;
		let addl_content = "";
		if (msg.orig_msg_id != null) {
			addl_content += `<span>(origin: #${msg.orig_msg_id})</span>`
		}
		if (author_kind == "User") {
			addl_content += `<do-toggle do-show-steps="Show Steps" do-hide-steps="Hide Steps">Show Steps</do-toggle>`;
		}
		this.#footerEl.innerHTML = `<span>#${msg.id}</span> ${addl_content}`;
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'chat-p': ChatPart;
	}
}


// #endregion --- chat-p


// #region    --- Steps

const HTML_STEPS_P = html`
  <nav></nav>
  <section></section> 
`;

@customElement('steps-p')
export class StepsPart extends BaseHTMLElement { // extends HTMLElement
	data!: { orig_msg_id: number, conv_id: number }

	// -- Els
	#navEl!: HTMLElement
	#contentEl!: HTMLElement

	// -- UI states
	get minimized() { return (this.classList.contains("minimized")) }

	// #region    --- UI Events

	@onEvent("pointerup", "nav > label")
	onNavItemClick(evt: OnEvent) {
		let step_el = evt.selectTarget;
		let step_id = asNum(getAttr(step_el, "data-step-id"));
		this.show_step(step_id);
	}

	// #endregion --- UI Events

	// #region    --- Lifecycle

	init() {
		const content = document.importNode(HTML_STEPS_P, true);
		[this.#navEl, this.#contentEl] = cherryChild(content, "nav", "section");
		this.replaceChildren(content);
	}

	async postDisplay() {
		let conv_id = this.data?.conv_id!;
		let orig_msg_id = this.data?.orig_msg_id!;

		let steps = await convFmc.list_steps(conv_id, orig_msg_id);
		// <label data-name="inst" class="sel"><d-ico name="ico-step"></d-ico></label>
		let nav_content = frag(steps, step =>
			html`<label data-name="inst" data-step-id="${step.id}"><d-ico name="ico-step"></d-ico></label>`
		);
		this.#navEl.replaceChildren(nav_content);

		if (!this.minimized && steps.length > 0) {
			this.show_step(steps[0].id);
		}
	}

	// #endregion --- Lifecycle

	async show_step(step_id: number | null) {
		if (step_id == null) return;

		// -- update the nav
		let step_el = first(this.#navEl, `[data-step-id='${step_id}']`);
		setClass(this.#navEl.children, { sel: false });
		setClass(step_el, { sel: true });

		// -- update the content
		let step = await convFmc.get_step(this.data.conv_id, step_id);
		let content_inner_el: HTMLElement;
		if (step.closer) {
			content_inner_el = elem("pre", { $: { textContent: "ALL DONE" } });
		} else {
			// serialize step in json string
			content_inner_el = elem("pre", { $: { textContent: JSON.stringify(step, null, 2) } });
			// 			content = `
			// <div class="model"><label>model:</label><span>${step.resolve_model}</span></div>
			// <div class="output">${step.call_out}</div>
			// 			`;
		}
		this.#contentEl.replaceChildren(content_inner_el);
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'steps-p': StepsPart;
	}
}


// #endregion --- Steps
