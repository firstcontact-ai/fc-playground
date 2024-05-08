import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import debounce from 'debounce';
import { elem, first, hub, trigger } from 'dom-native';
import { router } from './router';

export async function app_init() {

	// Initialize the router
	router.init();


	// Note: Needs to debounce, because when page reload in dev, we get multiple events same eventId. 
	//       Apparently, it is fix in dev branch. 
	let db_tauri_drop = debounce(async function (evt: any) {
		const path = evt.payload.paths[0];

		const { x, y } = await relate_tauri_pos(evt.payload.position);
		//  needs to make it relative in case the dev tool appears

		let el = document.elementFromPoint(x, y);

		// DEBUG
		// let mark = elem("div", {
		// 	style: `width:8px;height:8px;background:red;position:absolute;top:${y}px;left:${x}px`
		// })
		// document.body.appendChild(mark);

		trigger(el, "TAURI_DROP", { detail: { path } });
	}, 50);


	listen('tauri://file-drop', db_tauri_drop);

	listen('hubEvent', handle_hub_event);

}

// Notes: 
// - For now, we handcode it. Might be generated ins bindings.
// - Not used as the listen binding above will fail if not any
interface BackendHubEvent {
	hub: string,
	topic: string,
	label?: string,
	detail: any,
}

async function handle_hub_event(evt: any) {
	const { hub: hub_name, topic, label, detail } = evt.payload;

	let hub_ = hub(hub_name);
	if (typeof label == "string") {
		hub_.pub(topic, label, detail);
	} else {
		hub_.pub(topic, detail);
	}

	// For debug
	// console.log('handle_hub_event:', evt);
}

async function relate_tauri_pos(pos: { x: number, y: number }): Promise<{ x: number, y: number }> {
	let { x, y } = pos;
	let [winW, winH] = await invoke("get_win_size") as [number, number];
	let viewW = window.innerWidth;
	let viewH = window.innerHeight;
	let tx = x; // Nothing on the x axis, somehow, it does not change this one when debug window
	let ty = y + (winH - viewH);

	return { x: tx, y: ty };
}