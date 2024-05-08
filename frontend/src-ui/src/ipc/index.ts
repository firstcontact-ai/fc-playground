import core, { invoke, transformCallback } from '@tauri-apps/api/core';
import { EventCallback, EventName, Options } from '@tauri-apps/api/event';

interface RpcResponse {
	id: any,
	result?: any,
	error?: any
}

export async function invoke_rpc(method: string, params: any): Promise<any> {
	// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
	try {
		let res = await invoke("rpc", { rpcRequest: { jsonrpc: "2.0", id: 1, method, params } }) as RpcResponse;

		// TODO: Eventually need to fully handle. 
		if (res.error) {
			console.log("RPC CALL ERROR ", res.error);
			throw new Error(res.error);
		}

		return res.result;
	} catch (ex) {
		let msg = {
			method,
			params,
			error: ex
		}
		console.log("RPC CALL EXCEPTION ", msg);
		throw ex;
	}


}


export async function invoke_get_win_sess_value(key: string): Promise<boolean> {
	let res = await invoke("get_win_sess_value", { key }) as boolean;
	return res
}

export async function invoke_set_win_sess_value(key: string, value: any) {
	await invoke("set_win_sess_value", { key, value });
}

