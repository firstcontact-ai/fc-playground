import { BaseHTMLElement } from 'dom-native';
import { OnListenerByTypeSelector } from 'dom-native/dist/event';

interface BhvPopupComp {
	bhvPopupCompNoRemoveEl?: HTMLElement
}

/** 
 * FIXME: 
 *   Right now, this will override the eventual component document events 'keyup' and 'ponterup'
 *   manual bindings (different from the @onDoc...).
 *   So, eventaully, might want to have a the event, but for this, 
 *   dom-native needs to support multiple binding with same keys. 
 *   So `docEvents["keyup"]` could be one function or array of functions. (dom-native feature)
 *   Right now, no big issue, as components use `@onDoc` for event binding
 */
export function bhvPopup(comp: BaseHTMLElement & BhvPopupComp) {

	comp.docEvents ??= {};
	let docEvents = comp.docEvents;

	// -- add keyup for Escape to remove popup
	if (!checkTypeSelectorAvailability(comp, docEvents, "keyup")) return;
	docEvents["keyup"] = (evt: KeyboardEvent) => {
		if (evt.key == "Escape") {
			comp.remove()
		}
	}

	// -- add pointerup
	if (!checkTypeSelectorAvailability(comp, docEvents, "pointerup")) return;
	docEvents["pointerup"] = (evt: Event) => {
		// if the target clicked is not a sub el of his comp, then, we should remove
		let comp_contains_target = comp.contains(evt.target as HTMLElement);
		// in addition, if 
		let no_remove_el_contains_target = false;

		if (comp.bhvPopupCompNoRemoveEl) {
			no_remove_el_contains_target = comp.bhvPopupCompNoRemoveEl.contains(evt.target as HTMLElement);
		}

		if (!comp_contains_target && !no_remove_el_contains_target) comp.remove();
	}

}


function checkTypeSelectorAvailability(comp: BaseHTMLElement, events: OnListenerByTypeSelector, typesel: string): boolean {
	if (events[typesel]) {
		let compName = comp.tagName;
		console.log(`ERROR - bhvPopup - '${compName}' '${typesel}' already bound`);
		return false;
	}
	return true;
}