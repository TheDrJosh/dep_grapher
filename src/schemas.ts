import { z } from "zod";

export const projectRegistry = ["cargo", "npm", "jsr", "pypi"] as const;
export const projectRegistrySchema = z.enum(projectRegistry);
export type ProjectRegistry = z.infer<typeof projectRegistrySchema>;

export const projectSource = [
    ...(["local", "git"] as const),
    ...projectRegistry,
] as const;
export const projectSourceSchema = z.enum(projectSource);
export type ProjectSource = z.infer<typeof projectSourceSchema>;

export const projectType = ["rust", "nodejs", "deno", "python", "zig"] as const;
export const projectTypeSchema = z.enum(projectType);
export type ProjectType = z.infer<typeof projectSourceSchema>;
