/* eslint-disable */
// @ts-nocheck

// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async isPathValid(path: string) : Promise<Result<null, InvalidPathError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("is_path_valid", { path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async resolvePackage(location: PackageLocation) : Promise<Result<Package[], ResolveError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("resolve_package", { location }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async searchRegistry(registry: Registry, query: string) : Promise<Result<SearchResult[], SearchRegistryError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("search_registry", { registry, query }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/



/** user-defined constants **/



/** user-defined types **/

export type GitCommit = { commit: string } | { tag: string }
export type GitPackageLocation = { git_type: GitPackageType; commit: GitCommit }
export type GitPackageType = { local: string } | { remote: string }
export type GitResolveError = never
export type InvalidPathError = "not_found" | "not_a_directory" | "invalid_name" | "not_absolute" | "unknown"
export type Language = "rust" | "node_js" | "deno" | "python" | "zig"
export type LocalPackageLocation = { path: string }
export type LocalResolveError = "no_packages_in_path" | { invalid_path: InvalidPathError }
export type Package = { location: PackageLocation; name: string; version: string; description: string | null; authors: string[]; language: Language; dependencies: PackageLocation[] }
export type PackageLocation = { local: LocalPackageLocation } | { git: GitPackageLocation } | { registry: RegistryPackageLocation } | { url: string } | { unknown: { name: string; description: string } }
export type Registry = { cargo: string | null } | { npm: string | null } | { jsr: string | null } | { py_pi: string | null }
export type RegistryPackageLocation = { registry: Registry; name: string; version: string }
export type RegistryResolveError = never
export type ResolveError = { local: LocalResolveError } | { git: GitResolveError } | { registry: RegistryResolveError }
export type SearchRegistryError = "network" | "server" | "parse"
export type SearchResult = { name: string }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
