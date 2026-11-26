import { invoke } from "@tauri-apps/api/core";
import type { NewWowInstance, SerializedAppError, WowInstance } from "$lib/types/instance";

export function formatError(err: unknown): string {
  if (typeof err === "object" && err !== null && "message" in err) {
    const e = err as SerializedAppError;
    return e.code ? `[${e.code}] ${e.message}` : e.message;
  }
  return String(err);
}

export async function fetchInstances(): Promise<WowInstance[]> {
  return invoke<WowInstance[]>("list_instances");
}

export async function createInstance(input: NewWowInstance): Promise<WowInstance> {
  return invoke<WowInstance>("add_instance", { input });
}