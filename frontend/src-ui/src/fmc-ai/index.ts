import { RunUserPromptParams, UserPrompt } from '../bindings';
import { invoke_rpc } from '../ipc';


class AiFmc {

	async list_models(): Promise<string[]> {
		let res = await invoke_rpc(`ai_list_models`, {});
		return res.data;
	}

	async gen(space_id: number, data: UserPrompt): Promise<any> {
		let res = await invoke_rpc(`gen_ai`, { space_id, data });
		return res.data;
	}

	async run_user_prompt(params: RunUserPromptParams): Promise<any> {
		let res = await invoke_rpc(`ai_run_user_prompt`, params);
		return res.data;
	}

}

export const aiFmc = new AiFmc();