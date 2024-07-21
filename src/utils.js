import { invoke } from '@tauri-apps/api/tauri';

export function invoke(command) {
  return invoke(command);
}
