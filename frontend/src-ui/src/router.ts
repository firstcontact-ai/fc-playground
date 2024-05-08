import { getFirst, hub } from 'dom-native';

const route_hub = hub("Route");

const MAIN_IDS = ["drives", "agents", "spaces"] as const;
export type MainId = typeof MAIN_IDS[number];
export function isMainId(str: any): str is MainId {
	return MAIN_IDS.includes(str);
}

/** 
 * Route states for the whole application. 
 * 
 * Currently, the best practice is to keep the Route states as simple
 * as possible, meaning, flat and just "ids" like names/values.
 * 
 * Example paths: 
 * 
 * - `spaces?space_id=123`
 * 
 **/
export interface RouteState {
	main_id?: MainId,
	space_id?: number,
	agent_id?: number,
}

class Router {

	init() {
		debugHash();
		window.addEventListener('hashchange', () => {
			this.#broadcast_change();
		}, false);
	}

	// #current_route: Route = {};

	update_state(state: Partial<RouteState>) {
		let current_state = parse_state();

		// Note: Will need to deep clone at some point. 
		Object.assign(current_state, state);

		save_state(current_state);

		// Note: Needs to broadcast event manually 
		//       (the Window 'hashchange' event get triggered when hash was change by a link click)
		//       ^^^ Not sure if this is true, seems we are getting multiple event.
		// this.#broadcast_change();
	}

	get_current(): RouteState {
		return parse_state();
	}

	#broadcast_change() {
		debugHash();

		// -- Broadcast the event
		// NOTE: Perhaps would be better to send the parsed RouteState, so that not too many parse calls later. 
		route_hub.pub("change", null);
	}


}

function debugHash() {
	const hash = window.location.hash;
	// -- DEBUG - for now, show in search placeholder 
	const mainSearchEl = getFirst("#main-search");
	mainSearchEl.setAttribute("placeholder", hash);
}

// Save the state to the window.location.hash

function save_state(route: RouteState) {
	let hash = "";

	if (route.main_id) {
		hash = route.main_id ?? "";
	}

	if (route.space_id) {
		hash = `spaces?space_id=${route.space_id}`;
	}

	if (route.agent_id) {
		hash = `agents?agent_id=${route.agent_id}`;
	}

	window.location.hash = hash
}

/**
 * Prase the state from the window.location.hash
 * 
 * @param hash e.g. `#spaces?space_id=1`
 * @returns 
 */
function parse_state(): RouteState {
	let hash = window.location.hash;
	if (hash.startsWith('#')) {
		hash = hash.substring(1);
	}
	const url = new URL(hash, 'http://placeholder.com');
	// TODO: Need to see if we still want the replace '/' by ''
	const main_id = url.pathname.replace('/', '') as MainId;

	if (!isMainId(main_id)) {
		return {};
	}

	const route: RouteState = { main_id };

	switch (main_id) {
		case 'spaces':
			const space_id = url.searchParams.get('space_id');
			if (space_id) route.space_id = Number(space_id);
			break;
		case 'agents':
			const agent_id = url.searchParams.get('agent_id');
			if (agent_id) route.agent_id = Number(agent_id);
			break;
		// Add more cases if needed
	}

	return route;
}



export const router = new Router();