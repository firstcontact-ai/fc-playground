import { BaseHTMLElement, OnEvent, all, customElement, first, getAttr, html, onEvent, setClass } from 'dom-native';


/**
 * The `tabs-p` component just makes the content below "tabbable" meaning when the 
 * user click on the nav label it will set the `.sel` appropriately. 
 * NOTE: This component does not initialize its html content, it assumes it is already set.
 * 
 * 
 * Content example:
 * ```
 * <tabs-p>
 *   <nav>
 *      <label data-name="inst" class="sel" >Instruction</label>
 *      <label data-name="chain" >Chain Definition</label>
 *   </nav>
 *   <section>
 *     <section data-for="inst" class="agent-tab-section sel">
 *       <label>Agent instruction:</label>
 *       <d-textarea name="inst"><d-textarea>
 *     </section>
 *     <section data-for="chain" class="agent-tab-section">
 *       <label>Chain:</label>
 *       <d-textarea name="chain"><d-textarea>
 *     </section>				
 *   </section> 
 * </tabs-p>
 * ```
 */
@customElement('tabs-p')
export class TabsPart extends BaseHTMLElement { // extends HTMLElement

  @onEvent("pointerup", "nav > label")
  onNavClick(evt: Event & OnEvent) {
    let navItemEl = evt.selectTarget;
    const name = getAttr(evt.selectTarget, "data-name");
    if (name) {
      // update the nav
      setClass(all(this, "nav > label"), { sel: false });
      navItemEl.classList.add("sel");

      // update the content
      setClass(all(this, "section > section"), { sel: false });
      all(this, "section > section").forEach(navEl => navEl.classList.remove("sel"));
      first(this, `section > section[data-for='${name}']`)?.classList.add("sel");
    }
  }
  // TODO: On post display, might to make sure the content section match the nav label

}
declare global {
  interface HTMLElementTagNameMap {
    'tabs-p': TabsPart;
  }
}
