import * as types from '../bindings';
import { invoke_rpc } from '../ipc';
import { dataHub } from './data-hub';


// #region    --- Base Fmc

export interface ListParams {
  filters?: any,
  list_options?: any,
}

/**
 * Base Frontend Model Controller class with basic CRUD except `list` which will be per subclass for now.
 * 
 * - E- For the Enity model type (e.g., Project)
 * - C - For the Create data type (e.g., ProjectForCreate)
 * - U - For the update data type (e.g., ProjectForUpdate)
 */
class BaseFmc<E, C, U> {
  #entity_ns: string // entity namespace, like "space"
  get cmd_suffix() { return this.#entity_ns; }

  constructor(cmd_suffix: string) {
    this.#entity_ns = cmd_suffix;
  }

  async get(id: number): Promise<E> {
    return invoke_rpc(`${this.#entity_ns}_get`, { id }).then(res => res.data);
  }

  async create(data: C): Promise<types.ModelMutateResultData> {
    return invoke_rpc(`${this.#entity_ns}_create`, { data }).then(res => {
      // FIXME: The dataHub event should come from Rust(like the desktop-app pattern)
      dataHub.pub(this.#entity_ns, "create", res.data.id);
      return types.ensure_ModelMutateResultData(res.data);
    });
  }

  async update(id: number, data: U): Promise<types.ModelMutateResultData> {
    return invoke_rpc(`${this.#entity_ns}_update`, { id, data }).then(res => {
      return types.ensure_ModelMutateResultData(res.data);
    });
  }

  async delete(id: number): Promise<types.ModelMutateResultData> {
    return invoke_rpc(`${this.#entity_ns}_delete`, { id }).then(res => res.data);
  }
}

// #endregion --- Base Fmc


// #region    --- AgentFmc

class AgentFmc extends BaseFmc<types.Agent, types.AgentForCreate, types.AgentForUpdate> {
  constructor() {
    super("agent");
  }

  async list(): Promise<types.AgentLite[]> {
    // Note: for now, we just add a 's' for list, might might get rid of plurals
    return invoke_rpc(`${this.cmd_suffix}_list`, {}).then(res => res.data);
  }
}
export const agentFmc = new AgentFmc();

// #endregion --- AgentFmc

// #region    --- SpaceFmc

class SpaceFmc extends BaseFmc<types.Space, types.SpaceForCreate, types.SpaceForUpdate> {
  constructor() {
    super("space");
  }

  async list(): Promise<types.Space[]> {
    // Note: for now, we just add a 's' for list, might might get rid of plurals
    return invoke_rpc(`${this.cmd_suffix}_list`, {}).then(res => res.data);
  }

  async get_latest(): Promise<types.Space> {
    let res = await invoke_rpc("space_get_latest", {});
    return res.data;
  }

  async get_default_drive(space_id: number): Promise<types.Drive> {
    let res = await invoke_rpc("space_get_default_drive", { id: space_id });
    return res.data;
  }

  async get_latest_conv(space_id: number): Promise<types.Conv> {
    let res = await invoke_rpc("space_get_latest_conv", { id: space_id });
    return res.data;
  }

  async seek_agent(space_id: number): Promise<types.Agent> {
    let res = await invoke_rpc("space_seek_agent", { id: space_id });
    return res.data;
  }
}
export const spaceFmc = new SpaceFmc();


// #endregion --- SpaceFmc

// #region    --- ConvFmc

class ConvFmc extends BaseFmc<types.Drive, void, types.DriveForUpdate> {
  constructor() {
    super("conv");
  }

  async list(listParams: ListParams): Promise<types.Conv[]> {
    // Note: for now, we just add a 's' for list, might might get rid of plurals
    return invoke_rpc(`${this.cmd_suffix}_list`, listParams).then(res => res.data);
  }

  // TODO: need to add return type
  async list_msgs(conv_id: number): Promise<any[]> {
    // Note: for now, we just add a 's' for list, might might get rid of plurals
    return invoke_rpc(`${this.cmd_suffix}_list_msgs`, { id: conv_id }).then(res => res.data);
  }

  async list_steps(conv_id: number, orig_msg_id: number): Promise<any[]> {
    return invoke_rpc(`${this.cmd_suffix}_list_steps`, { conv_id, orig_msg_id }).then(res => res.data);
  }

  async get_step(conv_id: number, step_id: number): Promise<any> {
    return invoke_rpc(`${this.cmd_suffix}_get_step`, { conv_id, step_id }).then(res => res.data);
  }

  async clear_all(conv_id: number): Promise<void> {
    return invoke_rpc(`${this.cmd_suffix}_clear_all`, { id: conv_id }).then(res => res.data);
  }
}

export const convFmc = new ConvFmc();

// #endregion --- ConvFmc



// #region    --- DriveFmc

class DriveFmc extends BaseFmc<types.Drive, types.DriveForCreate, types.DriveForUpdate> {
  constructor() {
    super("drive");
  }

  async list(): Promise<types.Drive[]> {
    // Note: for now, we just add a 's' for list, might might get rid of plurals
    return invoke_rpc(`list_${this.cmd_suffix}s`, {}).then(res => res.data);
  }

  async add_dsource(drive_id: number, rref: string): Promise<types.DSource> {
    let dsource_c: types.DSourceForCreate = {
      drive_id,
      rref,
    };
    let res = await invoke_rpc(`drive_add_dsource`, { data: dsource_c });
    return res.data;
  }

}

export const driveFmc = new DriveFmc();

// #endregion --- DriveFmc

// #region    --- DSource

class DSourceFmc extends BaseFmc<types.Drive, void, types.DriveForUpdate> {
  constructor() {
    super("dsource");
  }

  async list(listParams: ListParams): Promise<types.DSource[]> {
    // Note: for now, we just add a 's' for list, might might get rid of plurals
    return invoke_rpc(`${this.cmd_suffix}_list`, listParams).then(res => res.data);
  }
}

export const dsourceFmc = new DSourceFmc();
// #endregion --- DSource


