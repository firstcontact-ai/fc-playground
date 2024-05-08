import debounce from 'debounce';
import { BaseHTMLElement, OnEvent, PositionOptions, cherryChild, customElement, elem, first, frag, html, on, onEvent, onWin, position } from 'dom-native';
import { bhvPopup } from '../behaviors';
import { OnDoHandler, first_do_class_name } from '../utils';

const HTML = html`
<section></section>
`;

export interface ToggleSelectorData {
  /** 
   * The elem ID to identify this popup uniquely (only one at a time)
   * (without '#' or '.'). Up to this function to use htmlelment id or class
   */
  popupId: string,

  onDoHandler: OnDoHandler,

  /** e.g., [["do-clear-conv", "Clear Converssation"], ["do-refresh-conv", "Refresh"]] */
  content: { key: string, label: string }[] | HTMLElement | DocumentFragment,

  triggerEl?: HTMLElement,

  positionOptions?: PositionOptions,
}

/**
 * example: 
 * ```
 *  toggle_selector_popup({
 *     popupId: "conv-menu",
 *     onDoHandler: async (type) => {
 *       if (type == "do-clear-conv") {
 *         console.log('->> do-clear-conv',);
 *         await convFmc.clear_all(this.conv_id!);
 *       } else if (type == "do-refresh-conv") {
 *         console.log('->> do-refresh-conv',);
 *         this.refreshMsgs();
 *       }
 *     },
 *     content: [
 *       { key: "do-clear-conv", label: "Clear Conversation" },
 *       { key: "do-refresh-conv", label: "Refresh" }
 *     ],
 *     triggerEl: evt.selectTarget,
 *     positionOptions: { refPos: "BL", pos: "TL", hGap: 8 },
 *   });
 * ```
 * @param params 
 */
export function toggle_selector_popup(params: ToggleSelectorData) {
  let { popupId } = params;
  let popup_el = first(`#${popupId}`);
  if (popup_el != null) {
    popup_el.remove();
  } else {
    let popup_el = elem('selector-pop', { id: `${popupId}`, $: { _data: params } });
    document.body.appendChild(popup_el);
  }
}

@customElement('selector-pop')
export class SelectorPopup extends BaseHTMLElement { // extends HTMLElement
  _data!: ToggleSelectorData

  get bhvPopupCompNoRemoveEl(): HTMLElement | undefined {
    return this._data?.triggerEl;
  }

  // -- Els
  #contentEl!: HTMLElement

  // -- Privates
  #onDoHandler?: OnDoHandler;

  // #region    --- UI Events

  @onEvent("pointerup")
  onItemClick(evt: Event & OnEvent) {
    let doAction = first_do_class_name(evt.target);
    if (doAction != null) {
      if (this.#onDoHandler) {
        this.#onDoHandler(doAction, evt.target);
      }
      // Note: We do on next frame to let event propagate (in case it's a a link)
      requestAnimationFrame(() => this.remove());
    }
  }

  // #endregion --- UI Events

  constructor() {
    super();
    bhvPopup(this);
  }

  init() {
    this.#onDoHandler = this._data?.onDoHandler;

    // -- Create Structure
    const tag_html = document.importNode(HTML, true);
    this.#contentEl = cherryChild(tag_html, "section");

    // -- Update classList
    this.classList.add("popup");

    // -- Update content from init data
    let content = this._data?.content;
    if (content) {
      if (content instanceof Array) {
        content = frag(content, ({ key, label }) => elem("div", { class: `item ${key}`, $: { textContent: label } }));

      }
      this.#contentEl.replaceChildren(content);
    }

    this.replaceChildren(tag_html);

    // -- Popup specific
    // Not flicker on reposition
    this.style.visibility = "hidden";
  }

  postDisplay() {
    let { triggerEl, positionOptions } = this._data;
    if (triggerEl && positionOptions) {
      position(this, triggerEl, positionOptions);
      this.style.visibility = "visible";
      const rePosition = debounce((evt: any) => {
        position(this, triggerEl, positionOptions);
      }, 5, { immediate: true });

      on(window, "resize", rePosition, this._nsObj);
      this.forceCleanRootEvents();
    } else {
      this.style.visibility = "visible";
    }

  }
}
declare global {
  interface HTMLElementTagNameMap {
    'selector-pop': SelectorPopup;
  }
}
